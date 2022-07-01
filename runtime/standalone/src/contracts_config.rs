use super::{AccountId, Balance, BlockWeights, Weight, AVERAGE_ON_INITIALIZE_RATIO};
use crate::{
    accounts_config::EscrowAccount, AccountManager, Aura, Balances, Call, Circuit,
    ContractsRegistry, Event, RandomnessCollectiveFlip, Runtime, ThreeVm, Timestamp,
};
use frame_support::{pallet_prelude::ConstU32, parameter_types, traits::FindAuthor};
use pallet_3vm_contracts::weights::WeightInfo;
use pallet_3vm_evm::{
    EnsureAddressNever, EnsureAddressRoot, EnsureAddressTruncated, HashedAddressMapping,
    SubstrateBlockHashMapping, ThreeVMCurrencyAdapter,
};
use pallet_3vm_evm_primitives::FeeCalculator;
use sp_core::{H160, U256};
use sp_runtime::{traits::BlakeTwo256, ConsensusEngineId, RuntimeAppPublic};

// Unit = the base number of indivisible units for balances
const UNIT: Balance = 1_000_000_000_000;
const MILLIUNIT: Balance = 1_000_000_000;
const _EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

const fn deposit(items: u32, bytes: u32) -> Balance {
    (items as Balance * UNIT + (bytes as Balance) * (5 * MILLIUNIT / 100)) / 10
}

parameter_types! {
    pub const CreateSideEffectsPrecompileDest: AccountId = AccountId::new([33u8; 32]);
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
    type CircuitTargetId = CircuitTargetId;
    type ContractsRegistry = ContractsRegistry;
    type EscrowAccount = EscrowAccount;
    type Escrowed = AccountManager;
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
    type Escrowed = AccountManager;
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
    type Schedule = Schedule;
    type ThreeVm = ThreeVm;
    type Time = Timestamp;
    type WeightInfo = pallet_3vm_contracts::weights::SubstrateWeight<Self>;
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
}

// impl pallet_evm::Config for Runtime {
//     type FeeCalculator = FixedGasPrice;
//     type GasWeightMapping = MoonbeamGasWeightMapping;
//     type AddressMapping = moonbeam_runtime_common::IntoAddressMapping;
//     type PrecompilesType = MoonbeamPrecompiles<Self>;
//     type PrecompilesValue = PrecompilesValue;
//     type ChainId = EthereumChainId;
//     type FindAuthor = FindAuthorAdapter<AuthorInherent>;
//     type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
// }

// impl module_evm::Config for Runtime {
//     type Currency = Balances;
//     type TransferAll = Currencies;
//     type NewContractExtraBytes = NewContractExtraBytes;
//     type StorageDepositPerByte = StorageDepositPerByte;
//     type TxFeePerGas = TxFeePerGas;
//     type Event = Event;
//     type PrecompilesType = AllPrecompiles<Self>;
//     type PrecompilesValue = PrecompilesValue;
//     type GasToWeight = GasToWeight;
//     type ChargeTransactionPayment = module_transaction_payment::ChargeTransactionPayment<Runtime>;
//     type NetworkContractOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
//     type NetworkContractSource = NetworkContractSource;
//     type DeveloperDeposit = DeveloperDeposit;
//     type PublicationFee = PublicationFee;
//     type TreasuryAccount = AcalaTreasuryAccount;
//     type FreePublicationOrigin = EnsureRootOrHalfGeneralCouncil;
//     type Runner = module_evm::runner::stack::Runner<Self>;
//     type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
//     type Task = ScheduledTasks;
//     type IdleScheduler = IdleScheduler;
//     type WeightInfo = weights::module_evm::WeightInfo<Runtime>;
// }

// parameter_types! {
// 	pub const ChainId: u64 = 42;
// 	pub BlockGasLimit: U256 = U256::from(u32::max_value());
// 	pub PrecompilesValue: FrontierPrecompiles<Runtime> = FrontierPrecompiles::<_>::new();
// }
//
// impl pallet_evm::Config for Runtime {
//     type FeeCalculator = BaseFee;
//     type GasWeightMapping = ();
//     type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
//     type CallOrigin = EnsureAddressTruncated;
//     type WithdrawOrigin = EnsureAddressTruncated;
//     type AddressMapping = HashedAddressMapping<BlakeTwo256>;
//     type Currency = Balances;
//     type Event = Event;
//     type Runner = pallet_evm::runner::stack::Runner<Self>;
//     type PrecompilesType = FrontierPrecompiles<Self>;
//     type PrecompilesValue = PrecompilesValue;
//     type ChainId = ChainId;
//     type BlockGasLimit = BlockGasLimit;
//     type OnChargeTransaction = ();
//     type FindAuthor = FindAuthorTruncated<Aura>;
// }

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

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> U256 {
        1.into() // TODO: do this right
    }
}

parameter_types! {
    pub const ChainId: u64 = 42;
    pub BlockGasLimit: U256 = U256::from(u32::max_value());
}

impl pallet_3vm_evm::Config for Runtime {
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type BlockGasLimit = BlockGasLimit;
    type BlockHashMapping = SubstrateBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressTruncated;
    type ChainId = ChainId;
    type Currency = Balances;
    type Escrowed = AccountManager;
    type Event = Event;
    type FeeCalculator = FixedGasPrice;
    type FindAuthor = FindAuthorTruncated<Aura>;
    // TODO: this probably screws up something
    type GasWeightMapping = ();
    type OnChargeTransaction = ThreeVMCurrencyAdapter<Balances, ()>;
    type PrecompilesType = ();
    // TODO: add precompiles
    type PrecompilesValue = ();
    type Runner = pallet_3vm_evm::runner::stack::Runner<Self>;
    type ThreeVm = ThreeVm;
    type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
}
