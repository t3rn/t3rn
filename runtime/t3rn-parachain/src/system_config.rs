use crate::{Hash as HashPrimitive, *};
use frame_support::{
    parameter_types,
    traits::{ConstU128, ConstU32, ConstU8},
    weights::IdentityFee,
};
use sp_runtime::traits::BlakeTwo256;

// Configure FRAME pallets to include in runtime.
impl frame_system::Config for Runtime {
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The maximum length of a block (in bytes).
    type BlockLength = circuit_runtime_types::BlockLength;
    /// The index type for blocks.
    type BlockNumber = circuit_runtime_types::BlockNumber;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = circuit_runtime_types::BlockWeights;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The ubiquitous event type.
    type Event = Event;
    /// The type for hashing blocks and tries.
    type Hash = HashPrimitive;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// Version of the runtime.
    type Version = Version;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    type MinimumPeriod = MinimumPeriod;
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type WeightInfo = ();
}

impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    /// The ubiquitous event type.
    type Event = Event;
    type ExistentialDeposit = ConstU128<500>;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type Event = Event;
    type FeeMultiplierUpdate = ();
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type OnChargeTransaction = CurrencyAdapter;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
}

impl pallet_sudo::Config for Runtime {
    type Call = Call;
    type Event = Event;
}

impl pallet_utility::Config for Runtime {
    type Call = Call;
    type Event = Event;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}
