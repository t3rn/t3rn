use crate::{
    accounts_config::EscrowAccount, AccountId, AccountManager, Aura, Balance, Balances,
    BlockWeights, Circuit, ContractsRegistry, RandomnessCollectiveFlip, Runtime, RuntimeCall,
    RuntimeEvent, ThreeVm, Timestamp, Weight,
};
use circuit_runtime_types::AssetId;
use frame_support::{
    pallet_prelude::{ConstU32, DispatchClass},
    parameter_types,
    traits::FindAuthor,
};
use pallet_3vm_evm::{
    EnsureAddressTruncated, FixedGasWeightMapping, StoredHashAddressMapping,
    SubstrateBlockHashMapping, ThreeVMCurrencyAdapter,
};
use pallet_3vm_evm_primitives::FeeCalculator;
use sp_core::{ConstBool, H160, U256};
use sp_runtime::{ConsensusEngineId, RuntimeAppPublic};

#[cfg(feature = "std")]
pub use pallet_3vm_evm_primitives::GenesisAccount as EvmGenesisAccount;

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
    // The weight needed for decoding the queue should be less or equal than a fifth
    // of the overall weight dedicated to the lazy deletion.
    pub DeletionWeightLimit: Weight = BlockWeights::get()
        .per_class
        .get(DispatchClass::Normal)
        .max_total
        .unwrap_or(BlockWeights::get().max_block);
    pub Schedule: pallet_3vm_contracts::Schedule<Runtime> = Default::default();

    pub const MaxCodeSize: u32 = 128 * 1024;
    pub const DepositPerItem: Balance = deposit(1, 0);
    pub const DepositPerByte: Balance = deposit(0, 1);
    pub const DeletionQueueDepth: u32 = 128;
}

impl pallet_3vm::Config for Runtime {
    type AccountManager = AccountManager;
    type AssetId = AssetId;
    type CircuitTargetId = CircuitTargetId;
    type ContractsRegistry = ContractsRegistry;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type OnLocalTrigger = Circuit;
    type RuntimeEvent = RuntimeEvent;
    type SignalBounceThreshold = ConstU32<2>;
}

impl pallet_3vm_contracts::Config for Runtime {
    type AddressGenerator = pallet_3vm_contracts::DefaultAddressGenerator;
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
    // This node is geared towards development and testing of contracts.
    // We decided to increase the default allowed contract size for this
    // reason (the default is `128 * 1024`).
    //
    // Our reasoning is that the error code `CodeTooLarge` is thrown
    // if a too-large contract is uploaded. We noticed that it poses
    // less friction during development when the requirement here is
    // just more lax.
    type MaxCodeLen = ConstU32<{ 256 * 1024 }>;
    type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
    type MaxStorageKeyLen = ConstU32<128>;
    type Randomness = RandomnessCollectiveFlip;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Schedule = Schedule;
    type ThreeVm = ThreeVm;
    type Time = Timestamp;
    type UnsafeUnstableInterface = ConstBool<true>;
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

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> (U256, Weight) {
        (100.into(), Weight::from_ref_time(1))
    }
}

parameter_types! {
    pub const ChainId: u64 = 42;
    pub BlockGasLimit: U256 = U256::from(u32::max_value());
    pub PrecompilesValue: evm_precompile_util::Precompiles = evm_precompile_util::Precompiles::new(sp_std::vec![
        (0_u64, evm_precompile_util::KnownPrecompile::ECRecover),
        (1_u64, evm_precompile_util::KnownPrecompile::Sha256),
        (2_u64, evm_precompile_util::KnownPrecompile::Ripemd160),
        (3_u64, evm_precompile_util::KnownPrecompile::Identity),
        (4_u64, evm_precompile_util::KnownPrecompile::Modexp),
        (5_u64, evm_precompile_util::KnownPrecompile::Sha3FIPS256),
        (6_u64, evm_precompile_util::KnownPrecompile::Sha3FIPS512),
        (7_u64, evm_precompile_util::KnownPrecompile::ECRecoverPublicKey),
    ].into_iter().collect());
    pub WeightPerGas: Weight = Weight::from_ref_time(20_000);
}

// TODO[https://github.com/t3rn/3vm/issues/102]: configure this appropriately
impl pallet_3vm_evm::Config for Runtime {
    type AddressMapping = StoredHashAddressMapping<Self>;
    type BlockGasLimit = BlockGasLimit;
    type BlockHashMapping = SubstrateBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressTruncated;
    type ChainId = ChainId;
    type Currency = Balances;
    type FeeCalculator = FixedGasPrice;
    // BaseFee pallet may be better from frontier TODO
    type FindAuthor = FindAuthorTruncated<Aura>;
    type GasWeightMapping = FixedGasWeightMapping<Runtime>;
    type OnChargeTransaction = ThreeVMCurrencyAdapter<Balances, ()>;
    type OnCreate = ();
    type PrecompilesType = evm_precompile_util::Precompiles;
    type PrecompilesValue = PrecompilesValue;
    type Runner = pallet_3vm_evm::runner::stack::Runner<Self>;
    type RuntimeEvent = RuntimeEvent;
    type ThreeVm = ThreeVm;
    type Timestamp = Timestamp;
    type WeightInfo = ();
    type WeightPerGas = WeightPerGas;
    type WithdrawOrigin = EnsureAddressTruncated;
}
