#![cfg_attr(not(feature = "std"), no_std)]

pub mod app {
    #[cfg(feature = "std")]
    use std::fmt;
    #[cfg(feature = "std")]
    use std::fmt::Debug;

    use codec::{Compact, Decode, Encode, Error, Input};
    use sp_application_crypto::{app_crypto, sr25519};
    use sp_io::hashing::blake2_256;
    use sp_runtime::generic::Era;
    use sp_runtime::{AccountId32, MultiAddress, MultiSignature, RuntimeDebug};
    use sp_std::vec::Vec;

    pub const CIRCUIT_CRYPTO_ID: sp_application_crypto::KeyTypeId =
        sp_application_crypto::KeyTypeId(*b"circ");
    app_crypto!(sr25519, CIRCUIT_CRYPTO_ID);

    pub type GenericAddress = MultiAddress<sp_runtime::AccountId32, ()>;

    /// Message signing types
    ///
    /// Simple generic extra mirroring the SignedExtra currently used in extrinsics. Does not implement
    /// the SignedExtension trait. It simply encodes to the same bytes as the real SignedExtra. The
    /// Order is (CheckVersion, CheckGenesis, Check::Era, CheckNonce, CheckWeight, transactionPayment::ChargeTransactionPayment).
    /// This can be locked up in the System module. Fields that are merely PhantomData are not encoded and are
    /// therefore omitted here.
    #[cfg_attr(feature = "std", derive(Debug))]
    #[derive(Decode, Encode, Clone, Eq, PartialEq)]
    pub struct GenericExtra(Era, Compact<u32>, Compact<u128>);

    impl GenericExtra {
        pub fn new(era: Era, nonce: u32) -> GenericExtra {
            GenericExtra(era, Compact(nonce), Compact(0_u128))
        }
    }

    impl Default for GenericExtra {
        fn default() -> Self {
            Self::new(Era::Immortal, 0)
        }
    }

    /// AdditionalSigned fields of the respective SignedExtra fields.
    /// Order is the same as declared in the extra.
    pub type AdditionalSigned<Hash> = (u32, u32, Hash, Hash, (), (), ());

    #[derive(Encode, Clone, RuntimeDebug)]
    pub struct SignedPayload<Call, Hash>((Call, GenericExtra, AdditionalSigned<Hash>));

    impl<Call, Hash> SignedPayload<Call, Hash>
    where
        Call: Encode,
        Hash: Encode + sp_std::fmt::Debug,
    {
        pub fn from_raw(
            call: Call,
            extra: GenericExtra,
            additional_signed: AdditionalSigned<Hash>,
        ) -> Self {
            Self((call, extra, additional_signed))
        }

        /// Get an encoded version of this payload.
        ///
        /// Payloads longer than 256 bytes are going to be `blake2_256`-hashed.
        pub fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
            self.0.using_encoded(|payload| {
                if payload.len() > 256 {
                    f(&blake2_256(payload)[..])
                } else {
                    f(payload)
                }
            })
        }
    }

    /// Mirrors the currently used Extrinsic format (V3) from substrate. Has less traits and methods though.
    /// The SingedExtra used does not need to implement SingedExtension here.
    #[derive(Clone, PartialEq)]
    pub struct UncheckedExtrinsicV4<Call> {
        pub signature: Option<(GenericAddress, MultiSignature, GenericExtra)>,
        pub function: Call,
    }

    impl<Call> UncheckedExtrinsicV4<Call>
    where
        Call: Encode,
    {
        pub fn new_signed(
            function: Call,
            signed: GenericAddress,
            signature: MultiSignature,
            extra: GenericExtra,
        ) -> Self {
            UncheckedExtrinsicV4 {
                signature: Some((signed, signature, extra)),
                function,
            }
        }

        #[cfg(feature = "std")]
        pub fn hex_encode(&self) -> String {
            let mut hex_str = hex::encode(self.encode());
            hex_str.insert_str(0, "0x");
            hex_str
        }
    }

    #[cfg(feature = "std")]
    impl<Call> fmt::Debug for UncheckedExtrinsicV4<Call>
    where
        Call: fmt::Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "UncheckedExtrinsic({:?}, {:?})",
                self.signature.as_ref().map(|x| (&x.0, &x.2)),
                self.function
            )
        }
    }

    const V4: u8 = 4;

    impl<Call> Encode for UncheckedExtrinsicV4<Call>
    where
        Call: Encode,
    {
        fn encode(&self) -> Vec<u8> {
            encode_with_vec_prefix::<Self, _>(|v| {
                match self.signature.as_ref() {
                    Some(s) => {
                        v.push(V4 | 0b1000_0000);
                        s.encode_to(v);
                    }
                    None => {
                        v.push(V4 & 0b0111_1111);
                    }
                }
                self.function.encode_to(v);
            })
        }
    }

    impl<Call> Decode for UncheckedExtrinsicV4<Call>
    where
        Call: Decode + Encode,
    {
        fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
            // This is a little more complicated than usual since the binary format must be compatible
            // with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
            // will be a prefix of vector length (we don't need
            // to use this).
            let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

            let version = input.read_byte()?;

            let is_signed = version & 0b1000_0000 != 0;
            let version = version & 0b0111_1111;
            if version != V4 {
                return Err("Invalid transaction version".into());
            }

            Ok(UncheckedExtrinsicV4 {
                signature: if is_signed {
                    Some(Decode::decode(input)?)
                } else {
                    None
                },
                function: Decode::decode(input)?,
            })
        }
    }

    /// Same function as in primitives::generic. Needed to be copied as it is private there.
    fn encode_with_vec_prefix<T: Encode, F: Fn(&mut Vec<u8>)>(encoder: F) -> Vec<u8> {
        let size = sp_std::mem::size_of::<T>();
        let reserve = match size {
            0..=0b0011_1111 => 1,
            0b0100_0000..=0b0011_1111_1111_1111 => 2,
            _ => 4,
        };
        let mut v = Vec::with_capacity(reserve + size);
        v.resize(reserve, 0);
        encoder(&mut v);

        // need to prefix with the total length to ensure it's binary compatible with
        // Vec<u8>.
        let mut length: Vec<()> = Vec::new();
        length.resize(v.len() - reserve, ());
        length.using_encoded(|s| {
            v.splice(0..reserve, s.iter().cloned());
        });

        v
    }

    impl From<Public> for GenericAddress {
        fn from(public: Public) -> Self {
            MultiAddress::<AccountId32, ()>::Address32(public.0 .0)
        }
    }

    impl From<Signature> for MultiSignature {
        fn from(sig: Signature) -> Self {
            MultiSignature::Sr25519(sig.into())
        }
    }
}

/// Generates the extrinsic's call field for a given module and call passed as &str
/// # Arguments
///
/// * 'node_metadata' - This crate's parsed node metadata as field of the API.
/// * 'module' - Module name as &str for which the call is composed.
/// * 'call' - Call name as &str
/// * 'args' - Optional sequence of arguments of the call. They are not checked against the metadata.
/// As of now the user needs to check himself that the correct arguments are supplied.
#[macro_export]
macro_rules! compose_call {
($node_metadata: expr, $module: expr, $call_name: expr $(, $args: expr) *) => {
        {
            use frame_support::ensure;

            let _lookup_result = $node_metadata.lookup_module_and_call_indices($module, $call_name);

            ensure!(_lookup_result.is_ok(), "Could not assemble call");

            let (module_index, call_index) = _lookup_result.unwrap();

            ([module_index as u8, call_index as u8] $(, ($args)) *)
        }
    };
}

/// Generates an Unchecked extrinsic for a given call
/// # Arguments
///
/// * 'signer' - AccountKey that is used to sign the extrinsic.
/// * 'call' - call as returned by the compose_call! macro or via substrate's call enums.
/// * 'nonce' - signer's account nonce: u32
/// * 'era' - Era for extrinsic to be valid
/// * 'genesis_hash' - sp-runtime::Hash256/[u8; 32].
/// * 'runtime_spec_version' - RuntimeVersion.spec_version/u32
#[macro_export]
macro_rules! compose_extrinsic_offline {
    ($signer: expr,
    $call: expr,
    $nonce: expr,
    $era: expr,
    $genesis_hash: expr,
    $genesis_or_current_hash: expr,
    $runtime_spec_version: expr,
    $transaction_version: expr) => {{
        //     use t3rn_primitives::*;
        //
        //     let extra = GenericExtra::new($era, $nonce);
        //
        //     let raw_payload = SignedPayload::from_raw(
        //         $call.clone(),
        //         extra.clone(),
        //         (
        //             $runtime_spec_version,
        //             $transaction_version,
        //             $genesis_hash,
        //             $genesis_or_current_hash,
        //             (),
        //             (),
        //             (),
        //         ),
        //     );
        //
        //     let signature = raw_payload.using_encoded(|payload| $signer.sign(payload));
        //
        //     let mut arr = Default::default();
        //     arr.clone_from_slice($signer.public().as_ref());
        //
        //     UncheckedExtrinsicV4::new_signed(
        //         $call,
        //         GenericAddress::from(AccountId::from(arr)),
        //         signature.into(),
        //         extra,
        //     )
        vec![]
    }};
}

#[cfg(test)]
mod tests {
    use codec::{Decode, Encode};
    use sp_runtime::MultiSignature;

    use super::app::{GenericAddress, GenericExtra, UncheckedExtrinsicV4};

    #[test]
    fn encode_decode_roundtrip_works() {
        let xt = UncheckedExtrinsicV4::new_signed(
            vec![1, 1, 1],
            GenericAddress::default(),
            MultiSignature::default(),
            GenericExtra::default(),
        );

        let xt_enc = xt.encode();
        assert_eq!(xt, Decode::decode(&mut xt_enc.as_slice()).unwrap())
    }
}
