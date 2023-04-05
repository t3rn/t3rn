use crate::*;

use crate::{
    accounts_config::EscrowAccount, AccountId, AccountManager, Aura, Balance, Balances,
    BlockWeights, Call, Circuit, ContractsRegistry, Event, RandomnessCollectiveFlip, ThreeVm,
    Timestamp, Weight, AVERAGE_ON_INITIALIZE_RATIO,
};
use frame_support::{pallet_prelude::ConstU32, parameter_types, traits::FindAuthor};

use circuit_runtime_pallets::{
    evm_precompile_util, pallet_3vm, pallet_3vm_contracts, pallet_3vm_evm,
    pallet_3vm_evm_primitives, pallet_circuit::HeightResult,
};
use pallet_3vm_contracts::weights::WeightInfo;
use pallet_3vm_evm::{
    EnsureAddressTruncated, GasWeightMapping, StoredHashAddressMapping, SubstrateBlockHashMapping,
    ThreeVMCurrencyAdapter,
};
use pallet_3vm_evm_primitives::FeeCalculator;
use sp_core::{H160, U256};
use sp_runtime::{ConsensusEngineId, DispatchError, RuntimeAppPublic};

#[cfg(feature = "std")]
pub use pallet_3vm_evm_primitives::GenesisAccount as EvmGenesisAccount;
use t3rn_abi::{types::Bytes, Codec};
use t3rn_primitives::portal::{HeaderResult, Portal as PortalTrait, PortalReadApi};

// Unit = the base number of indivisible units for balances
const UNIT: Balance = 1_000_000_000_000;
const MILLIUNIT: Balance = 1_000_000_000;
const _EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

const fn deposit(items: u32, bytes: u32) -> Balance {
    (items as Balance * UNIT + (bytes as Balance) * (5 * MILLIUNIT / 100)) / 10
}

parameter_types! {
    pub const CreateSideEffectsPrecompileDest: AccountId = AccountId::new([51u8; 32]); // 0x333...3
    pub const CircuitTargetId: t3rn_primitives::ChainId = [3, 3, 3, 3];

    pub const MaxValueSize: u32 = 16_384;
    // The lazy deletion runs inside on_initialize.
    pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
        BlockWeights::get().max_block;
    // The weight needed for decoding the queue should be less or equal than a fifth
    // of the overall weight dedicated to the lazy deletion.
    pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get() / (
            <Runtime as pallet_3vm_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1) -
            <Runtime as pallet_3vm_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0)
        )) / 5) as u32;
    pub Schedule: pallet_3vm_contracts::Schedule<Runtime> = {
        let mut schedule = pallet_3vm_contracts::Schedule::<Runtime>::default();
        // We decided to **temporarily* increase the default allowed contract size here
        // (the default is `128 * 1024`).
        //
        // Our reasoning is that a number of people ran into `CodeTooLarge` when trying
        // to deploy their contracts. We are currently introducing a number of optimizations
        // into ink! which should bring the contract sizes lower. In the meantime we don't
        // want to pose additional friction on developers.
        schedule.limits.code_len = 256 * 1024;
        schedule
    };
    pub const MaxCodeSize: u32 = 2 * 1024;
    pub const DepositPerItem: Balance = deposit(1, 0);
    pub const DepositPerByte: Balance = deposit(0, 1);
}

impl pallet_3vm::Config for Runtime {
    type AccountManager = AccountManager;
    type AssetId = AssetId;
    type CircuitTargetId = CircuitTargetId;
    type ContractsRegistry = ContractsRegistry;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Event = Event;
    type OnLocalTrigger = Circuit;
    type SignalBounceThreshold = ConstU32<2>;
}

impl pallet_3vm_contracts::Config for Runtime {
    type AddressGenerator = pallet_3vm_contracts::DefaultAddressGenerator;
    type Call = Call;
    /// The safest default is to allow no calls at all.
    ///
    /// Runtimes should whitelist dispatchables that are allowed to be called from contracts
    /// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
    /// change because that would break already deployed contracts. The `Call` structure itself
    /// is not allowed to change the indices of existing pallets, too.
    type CallFilter = frame_support::traits::Nothing;
    type CallStack = [pallet_3vm_contracts::Frame<Self>; 31];
    type ChainExtension = ();
    type Currency = Balances;
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type DepositPerByte = DepositPerByte;
    type DepositPerItem = DepositPerItem;
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
    type Schedule = Schedule;
    type ThreeVm = ThreeVm;
    type Time = Timestamp;
    type WeightInfo = pallet_3vm_contracts::weights::SubstrateWeight<Self>;
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
}

pub struct FindAuthorTruncated<F>(sp_std::marker::PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
    fn find_author<'a, I>(digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        if let Some(author_index) = F::find_author(digests) {
            let authority_id = Aura::authorities()[author_index as usize].clone();
            return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]))
        }
        None
    }
}

const WEIGHT_PER_GAS: u64 = 20_000;

pub struct FixedGasWeightMapping;
impl GasWeightMapping for FixedGasWeightMapping {
    fn gas_to_weight(gas: u64) -> Weight {
        gas.saturating_mul(WEIGHT_PER_GAS)
    }

    fn weight_to_gas(weight: Weight) -> u64 {
        weight.wrapping_div(WEIGHT_PER_GAS)
    }
}

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> U256 {
        100.into() // TODO: do this right, this is about what pallet-contracts costs
    }
}

impl PortalReadApi<BlockNumber> for Runtime {
    fn get_latest_finalized_header(
        gateway_id: t3rn_primitives::ChainId,
    ) -> Result<HeaderResult, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::get_latest_finalized_header(gateway_id)
    }

    fn get_latest_finalized_height(
        gateway_id: t3rn_primitives::ChainId,
    ) -> Result<HeightResult<BlockNumber>, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::get_latest_finalized_height(gateway_id)
    }

    fn get_latest_updated_height(
        gateway_id: t3rn_primitives::ChainId,
    ) -> Result<HeightResult<BlockNumber>, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::get_latest_updated_height(gateway_id)
    }

    fn get_current_epoch(
        gateway_id: t3rn_primitives::ChainId,
    ) -> Result<HeightResult<BlockNumber>, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::get_current_epoch(gateway_id)
    }

    fn read_fast_confirmation_offset(
        gateway_id: t3rn_primitives::ChainId,
    ) -> Result<BlockNumber, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::read_fast_confirmation_offset(gateway_id)
    }

    fn read_rational_confirmation_offset(
        gateway_id: t3rn_primitives::ChainId,
    ) -> Result<BlockNumber, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::read_rational_confirmation_offset(gateway_id)
    }

    fn read_epoch_offset(
        gateway_id: t3rn_primitives::ChainId,
    ) -> Result<BlockNumber, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::read_epoch_offset(gateway_id)
    }

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::verify_event_inclusion(
            gateway_id,
            message,
            submission_target_height,
        )
    }

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::verify_state_inclusion(
            gateway_id,
            message,
            submission_target_height,
        )
    }

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::verify_tx_inclusion(
            gateway_id,
            message,
            submission_target_height,
        )
    }

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::verify_state_inclusion_and_recode(
            gateway_id,
            message,
            submission_target_height,
            abi_descriptor,
            out_codec,
        )
    }

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::verify_tx_inclusion_and_recode(
            gateway_id,
            message,
            submission_target_height,
            abi_descriptor,
            out_codec,
        )
    }

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError> {
        <Portal as PortalReadApi<BlockNumber>>::verify_event_inclusion_and_recode(
            gateway_id,
            message,
            submission_target_height,
            abi_descriptor,
            out_codec,
        )
    }
}

parameter_types! {
    pub const ChainId: u64 = 42;
    pub BlockGasLimit: U256 = U256::from(u32::max_value());
    pub PrecompilesValue: evm_precompile_util::Precompiles<Runtime, BlockNumber> = evm_precompile_util::Precompiles::new(sp_std::vec![
        (0_u64, evm_precompile_util::KnownPrecompile::ECRecover),
        (1_u64, evm_precompile_util::KnownPrecompile::Sha256),
        (2_u64, evm_precompile_util::KnownPrecompile::Ripemd160),
        (3_u64, evm_precompile_util::KnownPrecompile::Identity),
        (4_u64, evm_precompile_util::KnownPrecompile::Modexp),
        (5_u64, evm_precompile_util::KnownPrecompile::Sha3FIPS256),
        (6_u64, evm_precompile_util::KnownPrecompile::Sha3FIPS512),
        (7_u64, evm_precompile_util::KnownPrecompile::ECRecoverPublicKey),
    ].into_iter().collect(), sp_std::vec![
        (819_u64, evm_precompile_util::CustomPrecompile::<Runtime, BlockNumber>::Portal),
    ].into_iter().collect());
}

// TODO[https://github.com/t3rn/3vm/issues/102]: configure this appropriately
impl pallet_3vm_evm::Config for Runtime {
    type AddressMapping = StoredHashAddressMapping<Self>;
    type BlockGasLimit = BlockGasLimit;
    type BlockHashMapping = SubstrateBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressTruncated;
    type ChainId = ChainId;
    type Currency = Balances;
    type Event = Event;
    type FeeCalculator = FixedGasPrice;
    // BaseFee pallet may be better from frontier TODO
    type FindAuthor = FindAuthorTruncated<Aura>;
    type GasWeightMapping = FixedGasWeightMapping;
    type OnChargeTransaction = ThreeVMCurrencyAdapter<Balances, ()>;
    type PrecompilesType = evm_precompile_util::Precompiles<Self, BlockNumber>;
    type PrecompilesValue = PrecompilesValue;
    type Runner = pallet_3vm_evm::runner::stack::Runner<Self>;
    type ThreeVm = ThreeVm;
    type WithdrawOrigin = EnsureAddressTruncated;
}
