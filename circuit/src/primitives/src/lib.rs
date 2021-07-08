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
use sp_io::hashing::blake2_256;

use sp_runtime::generic::Era;
use sp_runtime::MultiSignature;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

#[cfg(feature = "std")]
use std::fmt::Debug;

use sp_std::prelude::*;

pub mod abi;
pub mod transfers;
pub mod gateway_inbound_protocol;

pub use gateway_inbound_protocol::GatewayInboundProtocol;

pub type InstanceId = [u8; 4];

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayType {
    ProgrammableInternal,
    ProgrammableExternal,
    TxOnly,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayVendor {
    Substrate,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Structure used at gateway registration as a starting point for multi-finality-verifier
pub struct GenericPrimitivesHeader {
    pub parent_hash: Option<sp_core::hash::H256>,
    pub number: u64,
    pub state_root: Option<sp_core::hash::H256>,
    pub extrinsics_root: Option<sp_core::hash::H256>,
    pub digest: Option<sp_runtime::generic::Digest<sp_core::hash::H256>>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayPointer {
    pub id: InstanceId,
    pub vendor: GatewayVendor,
    pub gateway_type: GatewayType,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ContractAccessError {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// The specified contract is a tombstone and thus cannot have any storage.
    IsTombstone,
}

#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ExecPhase<Account, Balance> {
    pub steps: Vec<ExecStep<Account, Balance>>,
}

#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ExecStep<Account, Balance> {
    pub compose: Compose<Account, Balance>,
}

pub type GenericAddress = sp_runtime::MultiAddress<sp_runtime::AccountId32, ()>;

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct InterExecSchedule<Account, Balance> {
    pub phases: Vec<ExecPhase<Account, Balance>>,
}

pub trait EscrowTrait: frame_system::Config + pallet_sudo::Config {
    type Currency: Currency<Self::AccountId>;
    type Time: Time;
}

type Bytes = Vec<u8>;

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CircuitOutboundMessage {
    /// Request compatible with JSON-RPC API of receiving node
    Read {
        /// Method name on the VM
        name: Bytes,
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results that will be decoded and checked against the format
        expected_output: Vec<GatewayExpectedOutput>,
        /// Expected results
        payload: MessagePayload,
    },
    /// Transaction (in substrate extrinics), signed offline and including dispatch call(s)
    Write {
        /// Method name on the VM
        name: Bytes,
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results
        expected_output: Vec<GatewayExpectedOutput>,
        /// Expected results
        payload: MessagePayload,
    },
    /// Custom transmission medium (like Substrate's XCMP)
    Escrowed {
        /// Method name on the VM
        name: Bytes,
        /// Encoded sender's public key
        sender: Bytes,
        /// Encoded target's public key
        target: Bytes,
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results
        expected_output: Vec<GatewayExpectedOutput>,
        /// Expected results
        payload: MessagePayload,
    },
}


/// Inclusion proofs of different tries
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ProofTriePointer {
    /// Proof is a merkle path in the state trie
    State,
    /// Proof is a merkle path in the transaction trie (extrisics in Substrate)
    Transaction,
    /// Proof is a merkle path in the receipts trie (in Substrate logs are entries in state trie, this doesn't apply)
    Receipts,
}

/// Inbound Steps that specifie expected data deposited by relayers back to the Circuit after each step
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CircuitInboundResult {
    pub result_format: Bytes,
    pub proof_type: ProofTriePointer,
}

/// Inbound Steps that specifie expected data deposited by relayers back to the Circuit after each step
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayExpectedOutput {
    /// Effect would be the modified storage key
    Storage {
        key: Vec<[u8; 32]>,
        // key: Vec<sp_core::storage::StorageKey>,
        // value: Vec<Option<sp_core::storage::StorageData>>,
        value: Vec<Option<Bytes>>,
    },

    /// Expect events as a result of that call - will be described with signature
    /// and check against the corresponding types upon receiving
    Events { signatures: Vec<Bytes> },

    /// Yet another event or Storage output
    Extrinsic {
        /// Optionally expect dispatch of extrinsic only at the certain block height
        block_height: Option<u64>,
    },

    /// Yet another event or Storage output. If expecting output u can define its type format.
    Output { output: Bytes },
}

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MessagePayload {
    Signed {
        signer: Bytes,
        /// Encoded utf-8 string of module name that implements requested entrypoint
        module_name: Bytes,
        /// Encoded utf-8 string of method name that implements requested entrypoint
        method_name: Bytes,
        /// Encoded call bytes
        call_bytes: Bytes,
        /// Encoded tx signature
        signature: Bytes,
        /// Encoded extras to that transctions, like versions and gas price /tips for miners. Check GenericExtra for more info.
        extra: Bytes,
    },
    /// Request compatible with JSON-RPC API of receiving node
    Rpc {
        /// Encoded utf-8 string of module name that implements requested entrypoint
        module_name: Bytes,
        /// Encoded utf-8 string of method name that implements requested entrypoint
        method_name: Bytes,
    },
    /// Transaction (in substrate extrinics), signed offline and including dispatch call(s)
    TransactionDispatch {
        /// Encoded call bytes
        call_bytes: Bytes,
        /// Encoded tx signature
        signature: Bytes,
        /// Encoded extras to that transctions, like versions and gas price /tips for miners. Check GenericExtra for more info.
        extra: Bytes,
    },
    /// Custom transmission medium (like Substrate's XCMP)
    Custom {
        /// Custom message bytes, that would have to be decoded by the receiving end.
        payload: Bytes,
    },
}

/// Message signing types
///
/// Simple generic extra mirroring the SignedExtra currently used in extrinsics. Does not implement
/// the SignedExtension trait. It simply encodes to the same bytes as the real SignedExtra. The
/// Order is (CheckVersion, CheckGenesis, Check::Era, CheckNonce, CheckWeight, transactionPayment::ChargeTransactionPayment).
/// This can be locked up in the System module. Fields that are merely PhantomData are not encoded and are
/// therefore omitted here.
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
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
