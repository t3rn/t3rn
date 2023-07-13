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

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{ReservableCurrency, Time};
use scale_info::TypeInfo;

pub use t3rn_abi::recode::Codec as T3rnCodec;
pub use t3rn_types::{gateway, types::Bytes};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
use sp_runtime::{
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature,
};

pub use gateway_inbound_protocol::GatewayInboundProtocol;
// pub use orml_traits;

use num_traits::Zero;
use sp_std::{prelude::*, vec};
#[cfg(feature = "std")]
use std::fmt::Debug;

pub mod account_manager;
pub mod attesters;
pub mod circuit;
pub mod claimable;
pub mod clock;
pub mod common;
pub mod contract_metadata;
pub mod contracts_registry;
pub mod executors;
pub mod gateway_inbound_protocol;
pub mod light_client;
pub mod match_format;
pub mod monetary;
pub mod portal;
pub mod rewards;
pub mod signature_caster;
pub mod storage;
pub mod threevm;
pub mod transfers;
pub mod volatile;
pub mod xdns;
pub mod xtx;

use crate::attesters::LatencyStatus;
use t3rn_types::sfx::{SecurityLvl, TargetId};

pub type ChainId = [u8; 4];
pub type ExecutionSource = [u8; 32];
pub const EMPTY_EXECUTION_SOURCE: [u8; 32] = [0u8; 32];

pub fn execution_source_to_option(source: ExecutionSource) -> Option<ExecutionSource> {
    if source == EMPTY_EXECUTION_SOURCE {
        None
    } else {
        Some(source)
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayType {
    ProgrammableInternal(u32),
    ProgrammableExternal(u32),
    TxOnly(u32),
    OnCircuit(u32),
}

impl Default for GatewayType {
    fn default() -> GatewayType {
        GatewayType::ProgrammableExternal(0)
    }
}

impl GatewayType {
    pub fn fetch_nonce(self) -> u32 {
        match self {
            Self::ProgrammableInternal(nonce) => nonce,
            Self::ProgrammableExternal(nonce) => nonce,
            Self::OnCircuit(nonce) => nonce,
            Self::TxOnly(nonce) => nonce,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default)]
pub enum TreasuryAccount {
    #[default]
    Treasury,
    Escrow,
    Fee,
    Parachain,
    Slash,
}

pub trait TreasuryAccountProvider<Account> {
    fn get_treasury_account(treasury_account: TreasuryAccount) -> Account;
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default)]
pub enum GatewayVendor {
    Polkadot,
    Kusama,
    #[default]
    Rococo,
    Ethereum,
}
use sp_std::slice::Iter;
impl GatewayVendor {
    pub fn iterator() -> Iter<'static, GatewayVendor> {
        static VENDORS: [GatewayVendor; 4] = [
            GatewayVendor::Polkadot,
            GatewayVendor::Kusama,
            GatewayVendor::Rococo,
            GatewayVendor::Ethereum,
        ];
        VENDORS.iter()
    }

    pub fn eta_per_speed_mode_in_epochs<Epoch: From<u32>>(&self, speed_mode: &SpeedMode) -> Epoch {
        match self {
            GatewayVendor::Polkadot | GatewayVendor::Kusama | GatewayVendor::Rococo =>
                match speed_mode {
                    SpeedMode::Fast => 4u32.into(),
                    SpeedMode::Rational => 6u32.into(),
                    SpeedMode::Finalized => 8u32.into(),
                },
            GatewayVendor::Ethereum => match speed_mode {
                SpeedMode::Fast => 1u32.into(),
                SpeedMode::Rational => 2u32.into(),
                SpeedMode::Finalized => 3u32.into(),
            },
        }
    }

    pub fn calculate_offsets<BlockNumber: From<u32> + Clone + Saturating + Zero>(
        &self,
        speed_mode: &SpeedMode,
        emergency_offset: BlockNumber,
        epoch_history: Option<Vec<EpochEstimate<BlockNumber>>>,
    ) -> (BlockNumber, BlockNumber) {
        let eta_in_epochs = self.eta_per_speed_mode_in_epochs::<BlockNumber>(speed_mode);

        let (submit_by_local_offset, submit_by_remote_offset) = epoch_history
            .and_then(|history| history.last().cloned())
            .map(|record| {
                (
                    record
                        .moving_average_local
                        .clone()
                        .saturating_mul(eta_in_epochs.clone()),
                    record.moving_average_remote.saturating_mul(eta_in_epochs),
                )
            })
            .unwrap_or_else(|| (emergency_offset.clone(), emergency_offset.clone()));

        if submit_by_local_offset.is_zero() || submit_by_remote_offset.is_zero() {
            (emergency_offset.clone(), emergency_offset)
        } else {
            (submit_by_local_offset, submit_by_remote_offset)
        }
    }
}

#[cfg(test)]
mod tests_gateway_vendor {
    use super::*;

    #[test]
    fn test_calculate_offsets_no_history() {
        let vendor = GatewayVendor::Polkadot;
        let speed_mode = SpeedMode::Fast;
        let emergency_offset = 10;

        let (local_offset, remote_offset) =
            vendor.calculate_offsets::<u32>(&speed_mode, emergency_offset, None);

        assert_eq!(local_offset, emergency_offset);
        assert_eq!(remote_offset, emergency_offset);
    }

    #[test]
    fn test_calculate_offsets_zero_average() {
        let vendor = GatewayVendor::Polkadot;
        let speed_mode = SpeedMode::Fast;
        let emergency_offset = 10u32;

        let epoch_history = Some(vec![EpochEstimate {
            local: 20,
            remote: 30,
            moving_average_local: 0,
            moving_average_remote: 0,
        }]);

        let (local_offset, remote_offset) =
            vendor.calculate_offsets::<u32>(&speed_mode, emergency_offset, epoch_history);

        assert_eq!(local_offset, emergency_offset);
        assert_eq!(remote_offset, emergency_offset);
    }

    #[test]
    fn test_calculate_offsets_non_zero_average() {
        let vendor = GatewayVendor::Polkadot;
        let speed_mode = SpeedMode::Fast;
        let emergency_offset = 10u32;

        let epoch_history = Some(vec![EpochEstimate {
            local: 20,
            remote: 30,
            moving_average_local: 5,
            moving_average_remote: 7,
        }]);

        let (local_offset, remote_offset) =
            vendor.calculate_offsets::<u32>(&speed_mode, emergency_offset, epoch_history);

        assert_eq!(local_offset, 5 * 4); // vendor is Polkadot and speed_mode is Fast, so eta_per_speed_mode_in_epochs should return 4
        assert_eq!(remote_offset, 7 * 4); // same here
    }

    #[test]
    fn test_calculate_offsets_with_speed_modes() {
        let vendor = GatewayVendor::Polkadot;
        let emergency_offset = 10u32;

        let epoch_history = Some(vec![EpochEstimate {
            local: 20,
            remote: 30,
            moving_average_local: 5,
            moving_average_remote: 7,
        }]);

        for (expected_mul, speed_mode) in &[
            (4, SpeedMode::Fast),
            (6, SpeedMode::Rational),
            (8, SpeedMode::Finalized),
        ] {
            let (local_offset, remote_offset) = vendor.calculate_offsets::<u32>(
                speed_mode,
                emergency_offset,
                epoch_history.clone(),
            );

            assert_eq!(local_offset, 5 * expected_mul); // as eta_per_speed_mode_in_epochs should return the expected_mul for the given speed_mode
            assert_eq!(remote_offset, 7 * expected_mul); // same here
        }
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default)]
pub enum ExecutionVendor {
    #[default]
    Substrate,
    EVM,
}

impl TokenInfo {
    pub fn match_execution_vendor(&self) -> ExecutionVendor {
        match self {
            TokenInfo::Substrate(_) => ExecutionVendor::Substrate,
            TokenInfo::Ethereum(_) => ExecutionVendor::EVM,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Structure used at gateway registration as a starting point for multi-finality-verifier
pub struct GenericPrimitivesHeader {
    pub parent_hash: Option<sp_core::hash::H256>,
    pub number: u64,
    pub state_root: Option<sp_core::hash::H256>,
    pub extrinsics_root: Option<sp_core::hash::H256>,
    pub digest: Option<sp_runtime::generic::Digest>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayPointer {
    pub id: ChainId,
    pub vendor: GatewayVendor,
    pub gateway_type: GatewayType,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayGenesisConfig {
    /// SCALE-encoded modules following the format of selected frame_metadata::RuntimeMetadataVXX
    pub modules_encoded: Option<Vec<u8>>,
    /// Extrinsics version
    pub extrinsics_version: u8,
    /// Genesis hash - block id of the genesis block use to distinct the network and sign messages
    /// Length depending on parameter passed in abi::GatewayABIConfig
    pub genesis_hash: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenInfo {
    Substrate(SubstrateToken),
    Ethereum(EthereumToken),
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Default, Decode, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SpeedMode {
    Fast,
    Rational,
    #[default]
    Finalized,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SubstrateToken {
    pub id: u32,
    pub symbol: Vec<u8>,
    pub decimals: u8,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EthereumToken {
    pub symbol: Vec<u8>,
    pub decimals: u8,
    pub address: Option<[u8; 20]>,
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self::Substrate(SubstrateToken {
            symbol: Encode::encode("TKN"),
            decimals: 9,
            id: 42,
        })
    }
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Compose<Account, Balance> {
    pub name: Vec<u8>,
    pub code_txt: Vec<u8>,
    pub exec_type: Vec<u8>,
    pub dest: Account,
    pub value: Balance,
    pub bytes: Vec<u8>,
    pub input_data: Vec<u8>,
}

/// A result type of a get storage call.
pub type FetchContractsResult = Result<Vec<u8>, ContractAccessError>;

/// Read latest height of gateway known to a light client
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ReadLatestGatewayHeight {
    Success { encoded_height: Vec<u8> },
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

pub type GenericAddress = sp_runtime::MultiAddress<sp_runtime::AccountId32, ()>;

pub trait EscrowTrait<T: frame_system::Config> {
    type Currency: ReservableCurrency<T::AccountId>;
    type Time: Time;
}

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
/// Request message format that derivative of could be compatible with JSON-RPC API
/// with either signed or unsigned payload or custom transmission medium like XCMP protocol
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CircuitOutboundMessage {
    /// Message name/identifier
    pub name: Bytes,
    /// Module/pallet name
    pub module_name: Bytes,
    /// Method name
    pub method_name: Bytes,
    /// Encoded sender's public key
    pub sender: Option<Bytes>,
    /// Encoded target's public key
    pub target: Option<Bytes>,
    /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
    pub arguments: Vec<Bytes>,
    /// Expected results
    pub expected_output: Vec<GatewayExpectedOutput>,
    /// Extra payload in case the message is signed or uses custom delivery protocols like XCMP
    pub extra_payload: Option<ExtraMessagePayload>,
    /// type of gateway chain this message is intended for
    pub gateway_vendor: GatewayVendor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RpcPayloadUnsigned<'a> {
    pub method_name: &'a str,
    pub params: Vec<Bytes>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RpcPayloadSigned<'a> {
    pub method_name: &'a str,
    pub signed_extrinsic: Bytes,
}

impl CircuitOutboundMessage {
    pub fn to_jsonrpc_unsigned(&self) -> Result<RpcPayloadUnsigned, &'static str> {
        let method_name: &str = sp_std::str::from_utf8(&self.name[..])
            .map_err(|_| "`Can't decode method name to &str")?;

        Ok(RpcPayloadUnsigned {
            method_name,
            params: self.arguments.clone(),
        })
    }

    pub fn to_jsonrpc_signed(&self) -> Result<RpcPayloadSigned, &'static str> {
        let method_name: &str = sp_std::str::from_utf8(&self.name[..])
            .map_err(|_| "`Can't decode method name to &str")?;

        let signed_ext = self
            .extra_payload
            .as_ref()
            .map(|payload| payload.tx_signed.clone())
            .ok_or("no signed extrinsic provided")?;

        Ok(RpcPayloadSigned {
            method_name,
            signed_extrinsic: signed_ext,
        })
    }
}

/// Inclusion proofs of different tries
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
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
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CircuitInboundResult {
    pub result_format: Bytes,
    pub proof_type: ProofTriePointer,
}

/// Inbound Steps that specifie expected data deposited by relayers back to the Circuit after each step
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayExpectedOutput {
    /// Effect would be the modified storage key
    Storage {
        key: Vec<Vec<u8>>,
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
#[derive(Encode, Decode, Clone, Default, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewaysOverview<BlockNumber> {
    data: Vec<(TargetId, BlockNumber, Vec<GatewayActivity<BlockNumber>>)>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FinalityVerifierActivity<BlockNumber> {
    pub verifier: GatewayVendor,

    pub reported_at: BlockNumber,

    pub justified_height: BlockNumber,

    pub finalized_height: BlockNumber,

    pub updated_height: BlockNumber,

    pub epoch: BlockNumber,

    pub is_active: bool,
}
impl<BlockNumber: Zero + Clone + Saturating + Default + Ord> FinalityVerifierActivity<BlockNumber> {
    pub fn new_for_finalized_compare(
        reported_at: BlockNumber,
        finalized_height: BlockNumber,
    ) -> Self {
        FinalityVerifierActivity {
            verifier: Default::default(),
            reported_at,
            justified_height: Zero::zero(),
            finalized_height,
            updated_height: Zero::zero(),
            epoch: Zero::zero(),
            is_active: false,
        }
    }

    pub fn determine_finalized_reports_increase(
        activities: &[FinalityVerifierActivity<BlockNumber>],
    ) -> Option<(BlockNumber, BlockNumber)> {
        if activities.len() < 2 {
            return None
        }
        // Sort by reported_at to get the oldest and the latest reports
        let mut sorted_activities: Vec<_> = activities.iter().collect();
        sorted_activities.sort_by(|a, b| a.reported_at.cmp(&b.reported_at));
        let oldest_report = sorted_activities.first()?;
        let latest_report = sorted_activities.last()?;
        // if any of the reports contains zero finalized height or reported, we can't compare them
        if oldest_report.finalized_height.is_zero()
            || oldest_report.reported_at.is_zero()
            || latest_report.finalized_height.is_zero()
            || latest_report.reported_at.is_zero()
        {
            return None
        }
        let finalized_height_increase = latest_report
            .finalized_height
            .clone()
            .saturating_sub(oldest_report.finalized_height.clone());

        let reported_at_increase = latest_report
            .reported_at
            .clone()
            .saturating_sub(oldest_report.reported_at.clone());

        Some((finalized_height_increase, reported_at_increase))
    }
}

#[cfg(test)]
mod tests_finality_verifier_activity {
    use super::*;

    #[test]
    fn test_zero_cases_return_none() {
        let activity1 = FinalityVerifierActivity::<u32>::new_for_finalized_compare(0u32, 100u32);
        let activity2 = FinalityVerifierActivity::new_for_finalized_compare(100u32, 0u32);
        let activities = vec![activity1, activity2];

        assert_eq!(
            FinalityVerifierActivity::<u32>::determine_finalized_reports_increase(&activities),
            None
        );
    }

    #[test]
    fn test_length_below_two_returns_none() {
        let activity = FinalityVerifierActivity::<u32>::new_for_finalized_compare(50u32, 100u32);
        let activities = vec![activity];

        assert_eq!(
            FinalityVerifierActivity::<u32>::determine_finalized_reports_increase(&activities),
            None
        );
    }

    #[test]
    fn test_determines_increase_for_two_elements() {
        let activity1 = FinalityVerifierActivity::<u32>::new_for_finalized_compare(50u32, 100u32);
        let activity2 = FinalityVerifierActivity::<u32>::new_for_finalized_compare(100u32, 200u32);
        let activities = vec![activity1, activity2];

        let result =
            FinalityVerifierActivity::<u32>::determine_finalized_reports_increase(&activities);
        assert_eq!(result, Some((100u32, 50u32)));
    }

    #[test]
    fn test_determines_increase_for_three_elements() {
        let activity1 = FinalityVerifierActivity::<u32>::new_for_finalized_compare(50u32, 100u32);
        let activity2 = FinalityVerifierActivity::<u32>::new_for_finalized_compare(75u32, 150u32);
        let activity3 = FinalityVerifierActivity::<u32>::new_for_finalized_compare(100u32, 200u32);
        let activities = vec![activity1, activity2, activity3];

        let result =
            FinalityVerifierActivity::<u32>::determine_finalized_reports_increase(&activities);
        assert_eq!(result, Some((100u32, 50u32)));
    }
}

impl<BlockNumber: Zero> Default for FinalityVerifierActivity<BlockNumber> {
    fn default() -> Self {
        FinalityVerifierActivity {
            verifier: GatewayVendor::Rococo,
            reported_at: Zero::zero(),
            justified_height: Zero::zero(),
            finalized_height: Zero::zero(),
            updated_height: Zero::zero(),
            epoch: Zero::zero(),
            is_active: false,
        }
    }
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayActivity<BlockNumber> {
    pub gateway_id: TargetId,

    pub reported_at: BlockNumber,

    pub justified_height: BlockNumber,

    pub finalized_height: BlockNumber,

    pub updated_height: BlockNumber,

    pub attestation_latency: Option<LatencyStatus>,

    pub security_lvl: SecurityLvl,

    pub is_active: bool,
}

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
/// Extra payload in case the message is signed ro has other custom parameters required by linking protocol.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ExtraMessagePayload {
    pub signer: Bytes,
    /// Encoded utf-8 string of module name that implements requested entrypoint
    pub module_name: Bytes,
    /// Encoded utf-8 string of method name that implements requested entrypoint
    pub method_name: Bytes,
    /// Encoded call bytes
    pub call_bytes: Bytes,
    /// Encoded tx signature
    pub signature: Bytes,
    /// Encoded extras to that transctions, like versions and gas price /tips for miners. Check GenericExtra for more info.
    pub extra: Bytes,
    /// Encoded and signed transaction ready to send
    pub tx_signed: Bytes,
    /// Custom message bytes, that would have to be decoded by the receiving end.
    /// Could be utilized by custom transmission medium (like Substrate's XCMP)
    pub custom_payload: Option<Bytes>,
}

/// Retrieves all available gateways for a given ChainId.
/// Currently returns a vector with a single hardcoded result.
/// Eventually this will search all known gateways on pallet-xdns.
pub fn retrieve_gateway_pointers(gateway_id: ChainId) -> Result<Vec<GatewayPointer>, &'static str> {
    Ok(vec![GatewayPointer {
        id: gateway_id,
        gateway_type: GatewayType::ProgrammableExternal(0),
        vendor: GatewayVendor::Rococo,
    }])
}

pub type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Alias to the public key used for this chain, actually a `MultiSigner`. Like
/// the signature, this also isn't a fixed size when encoded, as different
/// cryptos have different size public keys.
pub type AccountPublic = <MultiSignature as Verify>::Signer;

/// Common types across all runtimes
pub type BlockNumber = u32;

use crate::xdns::EpochEstimate;
use sp_runtime::traits::Saturating;
pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;

pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;

/// Index of a transaction in the chain. 32-bit should be plenty.
pub type Nonce = u32;

/// Balance of an account.
pub type Balance = u128;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
