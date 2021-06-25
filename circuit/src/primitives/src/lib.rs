// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

//! A crate that hosts a common definitions that are relevant for the pallet-contracts.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::fmt;

use codec::{Compact, Decode, Encode, Error, Input};
use frame_support::traits::{Currency, Time};
use sp_core::blake2_256;

use sp_runtime::generic::Era;
use sp_runtime::MultiSignature;

use serde::{Deserialize, Serialize};
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

#[cfg(feature = "std")]
use std::fmt::Debug;

use sp_std::prelude::*;

pub mod abi;
pub mod transfers;

pub type InstanceId = [u8; 4];

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
pub enum GatewayType {
    ProgrammableInternal,
    ProgrammableExternal,
    TxOnly,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
pub enum GatewayVendor {
    Substrate,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
/// Structure used at gateway registration as a starting point for multi-finality-verifier
pub struct GenericPrimitivesHeader {
    pub parent_hash: Option<sp_core::hash::H256>,
    pub number: u64,
    pub state_root: Option<sp_core::hash::H256>,
    pub extrinsics_root: Option<sp_core::hash::H256>,
    pub digest: Option<sp_runtime::generic::Digest<sp_core::hash::H256>>,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
pub struct GatewayPointer {
    pub id: InstanceId,
    pub vendor: GatewayVendor,
    pub gateway_type: GatewayType,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
pub struct GatewayGenesisConfig {
    /// SCALE-encoded modules following the format of selected frame_metadata::RuntimeMetadataVXX
    pub modules_encoded: Option<Vec<u8>>,
    /// SCALE-encoded signed extension - see more at frame_metadata::ExtrinsicMetadata
    pub signed_extension: Option<Vec<u8>>,
    /// Runtime version
    pub runtime_version: sp_version::RuntimeVersion,
    /// Genesis hash - block id of the genesis block use to distinct the network and sign messages
    /// Length depending on parameter passed in abi::GatewayABIConfig
    pub genesis_hash: Vec<u8>,
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
#[derive(Eq, PartialEq, Encode, Decode, Serialize, Deserialize, Debug, Clone, Default)]
pub struct Compose<Account, Balance> {
    pub name: Vec<u8>,
    pub code_txt: Vec<u8>,
    pub gateway_id: [u8; 4],
    pub exec_type: Vec<u8>,
    pub dest: Account,
    pub value: Balance,
    pub bytes: Vec<u8>,
    pub input_data: Vec<u8>,
}
/// A result type of a get storage call.
pub type FetchContractsResult = Result<Option<Vec<u8>>, ContractAccessError>;

/// A result of execution of a contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize, Clone)]
pub enum ComposableExecResult {
    /// The contract returned successfully.
    ///
    /// There is a status code and, optionally, some data returned by the contract.
    Success {
        /// Flags that the contract passed along on returning to alter its exit behaviour.
        /// Described in `pallet_contracts::exec::ReturnFlags`.
        flags: u32,
        /// Output data returned by the contract.
        ///
        /// Can be empty.
        data: Vec<u8>,
        /// How much gas was consumed by the call.
        gas_consumed: u64,
    },
    /// The contract execution either trapped or returned an error.
    Error,
}

/// The possible errors that can happen querying the storage of a contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize, Clone)]
pub enum ContractAccessError {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// The specified contract is a tombstone and thus cannot have any storage.
    IsTombstone,
}

#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecPhase<Account, Balance> {
    pub steps: Vec<ExecStep<Account, Balance>>,
}

#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecStep<Account, Balance> {
    pub compose: Compose<Account, Balance>,
}

pub type GenericAddress = sp_runtime::MultiAddress<sp_runtime::AccountId32, ()>;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
pub struct InterExecSchedule<Account, Balance> {
    pub phases: Vec<ExecPhase<Account, Balance>>,
}

pub trait EscrowTrait: frame_system::Config + pallet_sudo::Config {
    type Currency: Currency<Self::AccountId>;
    type Time: Time;
}

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

#[derive(Encode, Clone)]
pub struct SignedPayload<Call, Hash>((Call, GenericExtra, AdditionalSigned<Hash>));

impl<Call, Hash> SignedPayload<Call, Hash>
where
    Call: Encode,
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
        // Added the .0.0 below to call the function on the call_bytes vector
        self.0 .0.using_encoded(|payload| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GenericAddress, GenericExtra};
    use sp_runtime::MultiSignature;

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
