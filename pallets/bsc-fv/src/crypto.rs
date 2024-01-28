#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::convert::TryFrom;
pub use ethereum_types::{H256, H160, U256};
use generic_array::GenericArray;
pub use keccak_hash::{keccak};
use k256::{
    ecdsa::{
        recoverable::{Id as RecoveryId, Signature as RecoverableSignature},
        Signature as K256Signature,
    },
    PublicKey as K256PublicKey,
};
use elliptic_curve::{consts::U32, sec1::ToEncodedPoint};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
/// An ECDSA signature
pub struct Signature {
    /// R value
    pub r: U256,
    /// S Value
    pub s: U256,
    /// V value
    pub v: u64,
}

impl<'a> TryFrom<&'a [u8]> for Signature {
    type Error = &'a str;

    /// Parses a raw signature which is expected to be 65 bytes long where
    /// the first 32 bytes is the `r` value, the second 32 bytes the `s` value
    /// and the final byte is the `v` value in 'Electrum' notation.
    fn try_from(bytes: &'a [u8]) -> Result<Self, &'a str> {
        if bytes.len() != 65 {
            return Err("Invalid Length!")
        }

        let v = bytes[64];
        let r = U256::from_big_endian(&bytes[0..32]);
        let s = U256::from_big_endian(&bytes[32..64]);

        Ok(Signature { r, s, v: v.into() })
    }
}

impl Signature {
    /// Verifies that signature on `message` was produced by `address`
    pub fn verify(&self, message: H256, address: H160) -> Result<(), &str>
    {
        let recovered = self.recover(message)?;
        if recovered != address {
            return Err("Signature Invalid!")
        }

        Ok(())
    }

    /// Recovers the Ethereum address which was used to sign the given message.
    ///
    /// Recovery signature data uses 'Electrum' notation, this means the `v`
    /// value is expected to be either `27` or `28`.
    pub fn recover(&self, message: H256) -> Result<H160, &str>
    {

        let (recoverable_sig, _recovery_id) = self.as_signature()?;
        let verify_key =
            recoverable_sig.recover_verify_key_from_digest_bytes(message.as_ref().into()).unwrap();

        let public_key = K256PublicKey::from(&verify_key);
        let public_key = public_key.to_encoded_point(/* compress = */ false);
        let public_key = public_key.as_bytes();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak(&public_key[1..]);
        Ok(H160::from_slice(&hash[12..]))
    }

    /// Retrieves the recovery signature.
    fn as_signature(&self) -> Result<(RecoverableSignature, RecoveryId), &str> {
        let recovery_id = self.recovery_id()?;
        let signature = {
            let mut r_bytes = [0u8; 32];
            let mut s_bytes = [0u8; 32];
            self.r.to_big_endian(&mut r_bytes);
            self.s.to_big_endian(&mut s_bytes);
            let gar: &GenericArray<u8, U32> = GenericArray::from_slice(&r_bytes);
            let gas: &GenericArray<u8, U32> = GenericArray::from_slice(&s_bytes);
            let sig = K256Signature::from_scalars(*gar, *gas).unwrap();
            RecoverableSignature::new(&sig, recovery_id).unwrap()
        };

        Ok((signature, recovery_id))
    }

    /// Retrieve the recovery ID.
    pub fn recovery_id(&self) -> Result<RecoveryId, &str> {
        let standard_v = normalize_recovery_id(self.v);
        Ok(RecoveryId::new(standard_v).unwrap())
    }

}

fn normalize_recovery_id(v: u64) -> u8 {
    match v {
        0 => 0,
        1 => 1,
        27 => 0,
        28 => 1,
        v if v >= 35 => ((v - 1) % 2) as _,
        _ => 4,
    }
}