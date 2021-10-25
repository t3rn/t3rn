// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! The Circuit runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// Runtime-generated enums
#![allow(clippy::large_enum_variant)]
// Runtime-generated DecodeLimit::decode_all_With_depth_limit
#![allow(clippy::unnecessary_mut_passed)]
// From construct_runtime macro
#![allow(clippy::from_over_into)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod gateway_messages;

use crate::gateway_messages::{ToGatewayMessagePayload, WithGatewayMessageBridge};
use beefy_primitives::{crypto::AuthorityId as BeefyId, ValidatorSet};
use bridge_runtime_common::messages::{
    source::estimate_message_dispatch_and_delivery_fee, MessageBridge,
};
use codec::Decode;
use pallet_beefy_mmr::mmr::MmrLeafVersion;
use pallet_grandpa::{
    fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use pallet_mmr_primitives as mmr;
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H160, H256, U256};
use sp_runtime::traits::{
    AccountIdLookup, BlakeTwo256, Block as BlockT, Keccak256, NumberFor, OpaqueKeys,
};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::Convert,
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, DispatchResult, MultiSignature, MultiSigner,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use pallet_contracts_primitives::RentProjection;
use t3rn_primitives::{transfers::BalanceOf, ComposableExecResult, Compose};

use ethereum_light_client::EthereumDifficultyConfig;
use volatile_vm::DispatchRuntimeCall;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{Currency, ExistenceRequirement, Imbalance, KeyOwnerProofSystem},
    weights::{constants::WEIGHT_PER_SECOND, DispatchClass, IdentityFee, RuntimeDbWeight, Weight},
    StorageValue,
};

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_bridge_grandpa::Call as BridgeGrandpaGatewayCall;
pub use pallet_bridge_messages::Call as MessagesCall;
pub use pallet_multi_finality_verifier::Call as BridgePolkadotLikeMultiFinalityVerifierCall;
pub use pallet_sudo::Call as SudoCall;
pub use pallet_timestamp::Call as TimestampCall;

use frame_support::weights::constants::RocksDbWeight;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

/// An index to a block.
pub type BlockNumber = bp_circuit::BlockNumber;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = bp_circuit::Signature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = bp_circuit::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = bp_circuit::Balance;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = bp_circuit::Hash;

/// Hashing algorithm used by the chain.
pub type Hashing = bp_circuit::Hasher;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, Hashing>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
        pub grandpa: Grandpa,
        pub beefy: Beefy,
    }
}

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("Circuit-runtime"),
    impl_name: create_runtime_str!("Circuit-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 250;
    pub const Version: RuntimeVersion = VERSION;
    pub const SS58Prefix: u8 = 60;
}

impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = ();
    /// Block and extrinsics weights: base values and limits.
    type BlockWeights = bp_circuit::BlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = bp_circuit::BlockLength;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = Hashing;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The header type.
    type Header = generic::Header<BlockNumber, Hashing>;
    /// The ubiquitous event type.
    type Event = Event;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Provides information about the pallet setup in the runtime.
    type PalletInfo = PalletInfo;
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    // TODO: update me (https://github.com/paritytech/parity-bridges-common/issues/78)
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// The designated SS58 prefix of this chain.
    type SS58Prefix = SS58Prefix;
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = ();
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
}

impl pallet_bridge_dispatch::Config for Runtime {
    type Event = Event;
    type MessageId = (bp_messages::LaneId, bp_messages::MessageNonce);
    type Call = Call;
    type CallFilter = ();
    type EncodedCall = crate::gateway_messages::FromGatewayEncodedCall;
    type SourceChainAccountId = bp_gateway::AccountId;
    type TargetChainAccountPublic = MultiSigner;
    type TargetChainSignature = MultiSignature;
    type AccountIdConverter = bp_circuit::AccountIdConverter;
}

impl pallet_grandpa::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type KeyOwnerProofSystem = ();
    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;
    type HandleEquivocation = ();
    // TODO: update me (https://github.com/paritytech/parity-bridges-common/issues/78)
    type WeightInfo = ();
}

impl pallet_beefy::Config for Runtime {
    type BeefyId = BeefyId;
}

parameter_types! {
    pub const MinimumPeriod: u64 = bp_circuit::SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    // TODO: update me (https://github.com/paritytech/parity-bridges-common/issues/78)
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: bp_circuit::Balance = 500;
    // For weight estimation, we assume that the most locks on an individual account will be 50.
    // This number may need to be adjusted in the future if this assumption no longer holds true.
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    // TODO: update me (https://github.com/paritytech/parity-bridges-common/issues/78)
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const TransactionBaseFee: Balance = 0;
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl pallet_randomness_collective_flip::Config for Runtime {}

impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}

type MmrHash = <Keccak256 as sp_runtime::traits::Hash>::Output;

/// A BEEFY consensus digest item with MMR root hash.
pub struct DepositLog;
impl pallet_mmr::primitives::OnNewRoot<MmrHash> for DepositLog {
    fn on_new_root(root: &Hash) {
        let digest = DigestItem::Consensus(
            beefy_primitives::BEEFY_ENGINE_ID,
            codec::Encode::encode(&beefy_primitives::ConsensusLog::<BeefyId>::MmrRoot(*root)),
        );
        <frame_system::Pallet<Runtime>>::deposit_log(digest);
    }
}

/// Configure Merkle Mountain Range pallet.
impl pallet_mmr::Config for Runtime {
    const INDEXING_PREFIX: &'static [u8] = b"mmr";
    type Hashing = Keccak256;
    type Hash = MmrHash;
    type LeafData = frame_system::Pallet<Self>;
    type OnNewRoot = DepositLog;
    type WeightInfo = ();
}

parameter_types! {
    /// Authorities are changing every 5 minutes.
    pub const Period: BlockNumber = bp_circuit::SESSION_LENGTH;
    pub const Offset: BlockNumber = 0;
}

// pub type PolkadotLikeGrandpaInstance = pallet_bridge_grandpa::Instance1;
// impl pallet_multi_finality_verifier::Config<PolkadotLikeGrandpaInstance> for Runtime {
//     type BridgedChain = bp_polkadot_core::PolkadotLike;
//     type MaxRequests = MaxRequests;
//     type WeightInfo = pallet_multi_finality_verifier::weights::GatewayWeight<Runtime>;
//     type HeadersToKeep = HeadersToKeep;
// }

impl pallet_session::Config for Runtime {
    type Event = Event;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = ();
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = pallet_shift_session_manager::Pallet<Runtime>;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type DisabledValidatorsThreshold = ();
    // TODO: update me (https://github.com/paritytech/parity-bridges-common/issues/78)
    type WeightInfo = ();
}

parameter_types! {
    // This is a pretty unscientific cap.
    //
    // Note that once this is hit the pallet will essentially throttle incoming requests down to one
    // call per block.
    pub const MaxRequests: u32 = 50;

    // Number of headers to keep.
    //
    // Assuming the worst case of every header being finalized, we will keep headers for at least a
    // week.
    pub const HeadersToKeep: u32 = 7 * bp_circuit::DAYS as u32;
}

pub type GatewayGrandpaInstance = ();
impl pallet_bridge_grandpa::Config for Runtime {
    type BridgedChain = bp_gateway::Gateway;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;

    // TODO [#391]: Use weights generated for the Circuit runtime instead of Gateway ones.
    type WeightInfo = ();
}

// start of contracts VMs
parameter_types! {
    pub const SignedClaimHandicap: u64 = 2;
    pub const TombstoneDeposit: u128 = 16;
    pub const DepositPerContract: u128 = 8 * DepositPerStorageByte::get();
    pub const DepositPerStorageByte: u128 = 10_000;
    pub const DepositPerStorageItem: u128 = 10_000;
    pub RentFraction: Perbill = Perbill::from_rational(4u32, 10_000u32);
    pub const SurchargeReward: u128 = 500_000;
    pub const MaxValueSize: u32 = 16_384;
    pub const DeletionQueueDepth: u32 = 1024;
    pub const DeletionWeightLimit: Weight = 500_000_000_000;
    pub const MaxCodeSize: u32 = 2 * 1024;
    pub MySchedule: pallet_contracts::Schedule<Runtime> = <pallet_contracts::Schedule<Runtime>>::default();
}

impl Convert<Weight, BalanceOf<Self>> for Runtime {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w.into()
    }
}

impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = Randomness;
    type Currency = Balances;
    type Event = Event;
    type RentPayment = ();
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type DepositPerContract = DepositPerContract;
    type DepositPerStorageByte = DepositPerStorageByte;
    type DepositPerStorageItem = DepositPerStorageItem;
    type RentFraction = RentFraction;
    type SurchargeReward = SurchargeReward;
    type CallStack = [pallet_contracts::Frame<Self>; 31];
    type WeightPrice = Self;
    type WeightInfo = ();
    type ChainExtension = ();
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type Schedule = MySchedule;
}

// EVM

parameter_types! {
    pub const ChainId: u64 = 33;
    pub const BlockGasLimit: U256 = U256::MAX;
}

pub struct FixedGasPrice;
impl pallet_evm::FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> U256 {
        1.into()
    }
}
pub struct HashedAddressMapping;

impl pallet_evm::AddressMapping<AccountId> for HashedAddressMapping {
    fn into_account_id(address: H160) -> AccountId {
        let mut data = [0u8; 32];
        data[0..20].copy_from_slice(&address[..]);
        AccountId::from(Into::<[u8; 32]>::into(data))
    }
}

impl t3rn_primitives::EscrowTrait for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

impl pallet_evm::Config for Runtime {
    type FeeCalculator = FixedGasPrice;
    type GasWeightMapping = ();
    type CallOrigin = pallet_evm::EnsureAddressTruncated;
    type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
    type AddressMapping = HashedAddressMapping;
    type Currency = Balances;
    type Event = Event;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type Precompiles = (
        pallet_evm_precompile_simple::ECRecover,
        pallet_evm_precompile_simple::Sha256,
        pallet_evm_precompile_simple::Ripemd160,
        pallet_evm_precompile_simple::Identity,
    );
    type ChainId = ChainId;
    type BlockGasLimit = BlockGasLimit;
    type OnChargeTransaction = ();
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping;
}

impl pallet_shift_session_manager::Config for Runtime {}

parameter_types! {
    pub const MaxMessagesToPruneAtOnce: bp_messages::MessageNonce = 8;
    pub const MaxUnrewardedRelayerEntriesAtInboundLane: bp_messages::MessageNonce =
        bp_circuit::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
    pub const MaxUnconfirmedMessagesAtInboundLane: bp_messages::MessageNonce =
        bp_circuit::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
    // `IdentityFee` is used by Circuit => we may use weight directly
    pub const GetDeliveryConfirmationTransactionFee: Balance =
        bp_circuit::MAX_SINGLE_MESSAGE_DELIVERY_CONFIRMATION_TX_WEIGHT as _;
    pub const RootAccountForPayments: Option<AccountId> = None;
}

/// Instance of the messages pallet used to relay messages to/from Gateway chain.
pub type WithGatewayMessagesInstance = pallet_bridge_messages::DefaultInstance;

impl pallet_bridge_messages::Config<WithGatewayMessagesInstance> for Runtime {
    type Event = Event;
    // TODO: https://github.com/paritytech/parity-bridges-common/issues/390
    type WeightInfo = ();
    type Parameter = gateway_messages::CircuitToGatewayMessagesParameter;
    type MaxMessagesToPruneAtOnce = MaxMessagesToPruneAtOnce;
    type MaxUnrewardedRelayerEntriesAtInboundLane = MaxUnrewardedRelayerEntriesAtInboundLane;
    type MaxUnconfirmedMessagesAtInboundLane = MaxUnconfirmedMessagesAtInboundLane;

    type OutboundPayload = crate::gateway_messages::ToGatewayMessagePayload;
    type OutboundMessageFee = Balance;

    type InboundPayload = crate::gateway_messages::FromGatewayMessagePayload;
    type InboundMessageFee = bp_gateway::Balance;
    type InboundRelayer = bp_gateway::AccountId;

    type AccountIdConverter = bp_circuit::AccountIdConverter;

    type TargetHeaderChain = crate::gateway_messages::Gateway;
    type LaneMessageVerifier = crate::gateway_messages::ToGatewayMessageVerifier;
    type MessageDeliveryAndDispatchPayment =
        pallet_bridge_messages::instant_payments::InstantCurrencyPayments<
            Runtime,
            pallet_balances::Pallet<Runtime>,
            GetDeliveryConfirmationTransactionFee,
            RootAccountForPayments,
        >;

    type SourceHeaderChain = crate::gateway_messages::Gateway;
    type MessageDispatch = crate::gateway_messages::FromGatewayMessageDispatch;
}

impl pallet_xdns::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<Runtime>;
}

impl pallet_contracts_registry::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_contracts_registry::weights::SubstrateWeight<Runtime>;
}

pub struct ExampleDispatchRuntimeCall;

impl DispatchRuntimeCall<Runtime> for ExampleDispatchRuntimeCall {
    fn dispatch_runtime_call(
        _module_name: &str,
        _fn_name: &str,
        _input: &[u8],
        _escrow_account: &<Runtime as frame_system::Config>::AccountId,
        _requested: &<Runtime as frame_system::Config>::AccountId,
        _callee: &<Runtime as frame_system::Config>::AccountId,
        _value: BalanceOf<Runtime>,
        _gas_meter: &mut volatile_vm::gas::GasMeter<Runtime>,
    ) -> DispatchResult {
        // match (module_name, fn_name) {
        //     ("Weights", "complex_calculations") => {
        //         let (_decoded_x, _decoded_y): (u32, u32) = match Decode::decode(&mut _input.clone())
        //         {
        //             Ok(dec) => dec,
        //             Err(_) => {
        //                 return Err(DispatchError::Other(
        //                     "Can't decode input for Weights::store_value. Expected u32.",
        //                 ));
        //             }
        //         };

        //         Ok(())
        //     }
        //     (_, _) => Err(DispatchError::Other(
        //         "Call to unrecognized runtime function",
        //     )),
        // }

        Ok(())
    }
}

parameter_types! {
    pub const UncleGenerations: u64 = 0;
    pub MyScheduleVVM: volatile_vm::Schedule<Runtime> = <volatile_vm::Schedule<Runtime>>::default();
}

parameter_types! {}

impl volatile_vm::VolatileVM for Runtime {
    type Randomness = Randomness;
    type Event = Event;
    type Call = Call;
    type DispatchRuntimeCall = ExampleDispatchRuntimeCall;
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type DepositPerContract = DepositPerContract;
    type DepositPerStorageByte = DepositPerStorageByte;
    type DepositPerStorageItem = DepositPerStorageItem;
    type RentFraction = RentFraction;
    type SurchargeReward = SurchargeReward;
    type CallStack = [volatile_vm::exec::Frame<Self>; 31];
    type ContractsLazyLoaded = [volatile_vm::wasm::PrefabWasmModule<Self>; 31];
    type WeightPrice = Self;
    type WeightInfo = ();
    type ChainExtension = ();
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type Schedule = MyScheduleVVM;
}

pub struct AccountId32Converter;
impl Convert<AccountId, [u8; 32]> for AccountId32Converter {
    fn convert(account_id: AccountId) -> [u8; 32] {
        account_id.into()
    }
}

pub struct CircuitToGateway;
impl Convert<Balance, u128> for CircuitToGateway {
    fn convert(val: Balance) -> u128 {
        val.into()
    }
}

impl pallet_circuit_execution_delivery::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type AccountId32Converter = AccountId32Converter;
    type ToStandardizedGatewayBalance = CircuitToGateway;
    type WeightInfo = pallet_circuit_execution_delivery::weights::SubstrateWeight<Runtime>;
}

type Blake2ValU64BridgeInstance = ();
type Blake2ValU32BridgeInstance = pallet_multi_finality_verifier::Instance1;
type Keccak256ValU64BridgeInstance = pallet_multi_finality_verifier::Instance2;
type Keccak256ValU32BridgeInstance = pallet_multi_finality_verifier::Instance3;

#[derive(Debug)]
pub struct Blake2ValU64Chain;
impl bp_runtime::Chain for Blake2ValU64Chain {
    type BlockNumber = <Runtime as frame_system::Config>::BlockNumber;
    type Hash = <Runtime as frame_system::Config>::Hash;
    type Hasher = <Runtime as frame_system::Config>::Hashing;
    type Header = <Runtime as frame_system::Config>::Header;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl bp_runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

#[derive(Debug)]
pub struct Keccak256ValU64Chain;
impl bp_runtime::Chain for Keccak256ValU64Chain {
    type BlockNumber = u64;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u64, Keccak256>;
}

#[derive(Debug)]
pub struct Keccak256ValU32Chain;
impl bp_runtime::Chain for Keccak256ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u32, Keccak256>;
}

impl pallet_multi_finality_verifier::Config<Blake2ValU64BridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU64Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_multi_finality_verifier::Config<Blake2ValU32BridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_multi_finality_verifier::Config<Keccak256ValU64BridgeInstance> for Runtime {
    type BridgedChain = Keccak256ValU64Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_multi_finality_verifier::Config<Keccak256ValU32BridgeInstance> for Runtime {
    type BridgedChain = Keccak256ValU32Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_utility::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const DescendantsUntilFinalized: u8 = 3;
    pub const DifficultyConfig: EthereumDifficultyConfig = EthereumDifficultyConfig::mainnet();
    pub const VerifyPoW: bool = true;
}

impl ethereum_light_client::Config for Runtime {
    type Event = Event;
    type DescendantsUntilFinalized = DescendantsUntilFinalized;
    type DifficultyConfig = DifficultyConfig;
    type VerifyPoW = VerifyPoW;
    // Todo: need to run benchmarks and set actual weights
    type WeightInfo = ();
}

parameter_types! {
    /// Version of the produced MMR leaf.
    ///
    /// The version consists of two parts;
    /// - `major` (3 bits)
    /// - `minor` (5 bits)
    ///
    /// `major` should be updated only if decoding the previous MMR Leaf format from the payload
    /// is not possible (i.e. backward incompatible change).
    /// `minor` should be updated if fields are added to the previous MMR Leaf, which given SCALE
    /// encoding does not prevent old leafs from being decoded.
    ///
    /// Hence we expect `major` to be changed really rarely (think never).
    /// See [`MmrLeafVersion`] type documentation for more details.
    pub LeafVersion: MmrLeafVersion = MmrLeafVersion::new(0, 0);
}

impl pallet_beefy_mmr::Config for Runtime {
    type LeafVersion = LeafVersion;
    type BeefyAuthorityToMerkleLeaf = pallet_beefy_mmr::BeefyEcdsaToEthereum;
    type ParachainHeads = ();
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        BridgeGatewayMessages: pallet_bridge_messages::{Pallet, Call, Storage, Event<T>},
        BridgeDispatch: pallet_bridge_dispatch::{Pallet, Event<T>},
        BridgeGatewayGrandpa: pallet_bridge_grandpa::{Pallet, Call, Storage},
        BridgePolkadotLikeMultiFinalityVerifier: pallet_multi_finality_verifier::<Instance1>::{Pallet, Call, Storage},
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Aura: pallet_aura::{Pallet, Config<T>},
        Grandpa: pallet_grandpa::{Pallet, Call, Storage, Config, Event},
        Beefy: pallet_beefy::{Pallet, Config<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
        Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>},
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
        ShiftSessionManager: pallet_shift_session_manager::{Pallet},

        Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
        Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>},
        EVM: pallet_evm::{Pallet, Config, Storage, Event<T>},
        XDNS: pallet_xdns::{Pallet, Call, Config<T>, Storage, Event<T>},
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Config<T>, Storage, Event<T>},
        VolatileVM: volatile_vm::{Pallet, Call, Event<T>, Storage},
        MultiFinalityVerifier: pallet_multi_finality_verifier::{Pallet, Call, Config<T>},
        ExecDelivery: pallet_circuit_execution_delivery::{Pallet, Call, Storage, Event<T>},
        Utility: pallet_utility::{Pallet, Call, Event},
        Mmr: pallet_mmr::{Pallet, Storage},
        EthereumLightClient: ethereum_light_client::{Pallet, Call, Storage, Event, Config},
        MmrLeaf: pallet_beefy_mmr::{Pallet, Storage},
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, Hashing>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPallets,
    (),
>;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            hash: <Block as BlockT>::Hash
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities()
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
        Block,
        Balance,
    > for Runtime {
        fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Grandpa::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
            )
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl beefy_primitives::BeefyApi<Block> for Runtime {
        fn validator_set() -> ValidatorSet<BeefyId> {
            Beefy::validator_set()
        }
    }

    impl pallet_mmr_primitives::MmrApi<Block, Hash> for Runtime {
        fn generate_proof(leaf_index: u64)
            -> Result<(mmr::EncodableOpaqueLeaf, mmr::Proof<Hash>), mmr::Error>
        {
            Mmr::generate_proof(leaf_index)
                .map(|(leaf, proof)| (mmr::EncodableOpaqueLeaf::from_leaf(&leaf), proof))
        }

        fn verify_proof(leaf: mmr::EncodableOpaqueLeaf, proof: mmr::Proof<Hash>)
            -> Result<(), mmr::Error>
        {
            pub type Leaf = <
                <Runtime as pallet_mmr::Config>::LeafData as mmr::LeafDataProvider
            >::LeafData;

            let leaf: Leaf = leaf
                .into_opaque_leaf()
                .try_decode()
                .ok_or(mmr::Error::Verify)?;
            Mmr::verify_leaf(leaf, proof)
        }

        fn verify_proof_stateless(
            root: Hash,
            leaf: mmr::EncodableOpaqueLeaf,
            proof: mmr::Proof<Hash>
        ) -> Result<(), mmr::Error> {
            type MmrHashing = <Runtime as pallet_mmr::Config>::Hashing;
            let node = mmr::DataOrHash::Data(leaf.into_opaque_leaf());
            pallet_mmr::verify_leaf_proof::<MmrHashing, _>(root, node, proof)
        }
    }

    // impl bp_gateway::GatewayFinalityApi<Block> for Runtime {
    // 	fn best_finalized() -> (bp_gateway::BlockNumber, bp_gateway::Hash) {
    // 		let header = BridgeGatewayGrandpa::best_finalized();
    // 		(header.number, header.hash())
    // 	}
    //
    // 	fn is_known_header(hash: bp_gateway::Hash) -> bool {
    // 		BridgeGatewayGrandpa::is_known_header(hash)
    // 	}
    // }

    impl bp_gateway::GatewayFinalityApi<Block> for Runtime {
        fn best_finalized() -> (bp_gateway::BlockNumber, bp_gateway::Hash) {
            // ToDo: Add argument and change call to pallet_multi_finality_verifier (gateway_id)
            // let header = BridgeGatewayGrandpa::best_finalized();
            let defa_gate: bp_runtime::ChainId = *b"gwes";
            let header = BridgePolkadotLikeMultiFinalityVerifier::best_finalized_map(defa_gate);
            (header.number, header.hash())
        }

        fn is_known_header(hash: bp_gateway::Hash) -> bool {
            // ToDo: Add argument and change call to pallet_multi_finality_verifier (gateway_id)
            // BridgeGatewayGrandpa::is_known_header(hash)
            let defa_gate: bp_runtime::ChainId = *b"gwes";
            BridgePolkadotLikeMultiFinalityVerifier::is_known_header(hash, defa_gate)
        }
    }

    impl bp_gateway::ToGatewayOutboundLaneApi<Block, Balance, ToGatewayMessagePayload> for Runtime {
        fn estimate_message_delivery_and_dispatch_fee(
            _lane_id: bp_messages::LaneId,
            payload: ToGatewayMessagePayload,
        ) -> Option<Balance> {
            estimate_message_dispatch_and_delivery_fee::<WithGatewayMessageBridge>(
                &payload,
                WithGatewayMessageBridge::RELAYER_FEE_PERCENT,
            ).ok()
        }

        fn messages_dispatch_weight(
            lane: bp_messages::LaneId,
            begin: bp_messages::MessageNonce,
            end: bp_messages::MessageNonce,
        ) -> Vec<(bp_messages::MessageNonce, Weight, u32)> {
            (begin..=end).filter_map(|nonce| {
                let encoded_payload = BridgeGatewayMessages::outbound_message_payload(lane, nonce)?;
                let decoded_payload = gateway_messages::ToGatewayMessagePayload::decode(
                    &mut &encoded_payload[..]
                ).ok()?;
                Some((nonce, decoded_payload.weight, encoded_payload.len() as _))
            })
            .collect()
        }

        fn latest_received_nonce(lane: bp_messages::LaneId) -> bp_messages::MessageNonce {
            BridgeGatewayMessages::outbound_latest_received_nonce(lane)
        }

        fn latest_generated_nonce(lane: bp_messages::LaneId) -> bp_messages::MessageNonce {
            BridgeGatewayMessages::outbound_latest_generated_nonce(lane)
        }
    }

    impl bp_gateway::FromGatewayInboundLaneApi<Block> for Runtime {
        fn latest_received_nonce(lane: bp_messages::LaneId) -> bp_messages::MessageNonce {
            BridgeGatewayMessages::inbound_latest_received_nonce(lane)
        }

        fn latest_confirmed_nonce(lane: bp_messages::LaneId) -> bp_messages::MessageNonce {
            BridgeGatewayMessages::inbound_latest_confirmed_nonce(lane)
        }

        fn unrewarded_relayers_state(lane: bp_messages::LaneId) -> bp_messages::UnrewardedRelayersState {
            BridgeGatewayMessages::inbound_unrewarded_relayers_state(lane)
        }
    }

    impl circuit_rpc_runtime_api::CircuitApi<Block, AccountId, Balance, BlockNumber> for Runtime
    {
        fn composable_exec(
            _origin: AccountId,
            _components: Vec<Compose<AccountId, Balance>>,
            _io: Vec<u8>,
            _gas_limit: u64,
            _input_data: Vec<u8>,
        ) -> ComposableExecResult { todo!() }
    }

    impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash>
        for Runtime
    {
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractExecResult {
            Contracts::bare_call(origin, dest, value, gas_limit, input_data, true)
        }

        fn instantiate(
            origin: AccountId,
            endowment: Balance,
            gas_limit: u64,
            code: pallet_contracts_primitives::Code<Hash>,
            data: Vec<u8>,
            salt: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, BlockNumber>
        {
            Contracts::bare_instantiate(origin, endowment, gas_limit, code, data, salt, false, true)
        }

        fn get_storage(
            address: AccountId,
            key: [u8; 32],
        ) -> pallet_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
        }

        fn rent_projection(
            _: AccountId
        ) -> Result<RentProjection<BlockNumber>, pallet_contracts_primitives::ContractAccessError> {
            unimplemented!();
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<
            Vec<frame_benchmarking::BenchmarkBatch>,
            sp_runtime::RuntimeString,
        > {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            impl frame_system_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, pallet_circuit_execution_delivery, ExecDelivery);
            add_benchmark!(params, batches, pallet_xdns, XDNS);
            add_benchmark!(params, batches, pallet_contracts_registry, ContractsRegistry);
            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

/// Gateway account ownership digest from Circuit.
///
/// The byte vector returned by this function should be signed with a Gateway account private key.
/// This way, the owner of `circuit_account_id` on Circuit proves that the Gateway account private key
/// is also under his control.
pub fn circuit_to_gateway_account_ownership_digest<Call, AccountId, SpecVersion>(
    gateway_call: &Call,
    circuit_account_id: AccountId,
    gateway_spec_version: SpecVersion,
) -> sp_std::vec::Vec<u8>
where
    Call: codec::Encode,
    AccountId: codec::Encode,
    SpecVersion: codec::Encode,
{
    pallet_bridge_dispatch::account_ownership_digest(
        gateway_call,
        circuit_account_id,
        gateway_spec_version,
        bp_runtime::CIRCUIT_CHAIN_ID,
        bp_runtime::GATEWAY_CHAIN_ID,
    )
}

#[cfg(test)]
mod tests {
    use bridge_runtime_common::messages;

    #[test]
    fn ensure_circuit_message_lane_weights_are_correct() {
        // TODO: https://github.com/paritytech/parity-bridges-common/issues/390
        type Weights = ();

        pallet_bridge_messages::ensure_weights_are_correct::<Weights>(
            bp_circuit::DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT,
            bp_circuit::ADDITIONAL_MESSAGE_BYTE_DELIVERY_WEIGHT,
            bp_circuit::MAX_SINGLE_MESSAGE_DELIVERY_CONFIRMATION_TX_WEIGHT,
        );

        let max_incoming_message_proof_size = bp_gateway::EXTRA_STORAGE_PROOF_SIZE.saturating_add(
            messages::target::maximal_incoming_message_size(bp_circuit::max_extrinsic_size()),
        );
        pallet_bridge_messages::ensure_able_to_receive_message::<Weights>(
            bp_circuit::max_extrinsic_size(),
            bp_circuit::max_extrinsic_weight(),
            max_incoming_message_proof_size,
            messages::target::maximal_incoming_message_dispatch_weight(
                bp_circuit::max_extrinsic_weight(),
            ),
        );

        let max_incoming_inbound_lane_data_proof_size =
            bp_messages::InboundLaneData::<()>::encoded_size_hint(
                bp_circuit::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE,
                bp_gateway::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE as _,
            )
            .unwrap_or(u32::MAX);
        pallet_bridge_messages::ensure_able_to_receive_confirmation::<Weights>(
            bp_circuit::max_extrinsic_size(),
            bp_circuit::max_extrinsic_weight(),
            max_incoming_inbound_lane_data_proof_size,
            bp_gateway::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
            bp_gateway::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE,
        );
    }
}
