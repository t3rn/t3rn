mod threevm_mock {
    use crate::tests::*;
    use frame_support::parameter_types;
    use sp_runtime::traits::{BlakeTwo256, ConvertInto};

    parameter_types! {
        pub const CreateSideEffectsPrecompileDest: AccountId32 = AccountId32::new([51u8; 32]); // 0x333....3
        pub const CircuitTargetId: t3rn_primitives::ChainId = [3, 3, 3, 3];
        pub EscrowAccount: AccountId32 = AccountId32::new([15_u8; 32]);
    }

    impl pallet_3vm::Config for Test {
        type AccountManager = AccountManager;
        type AssetId = u32;
        type CircuitTargetId = CircuitTargetId;
        type ContractsRegistry = ContractsRegistry;
        type Currency = Balances;
        type EscrowAccount = EscrowAccount;
        type Event = Event;
        type OnLocalTrigger = Circuit;
        type SignalBounceThreshold = ConstU32<2>;
    }

    impl pallet_sudo::Config for Test {
        type Call = Call;
        type Event = Event;
    }

    impl pallet_contracts_registry::Config for Test {
        type Balances = Balances;
        type Currency = Balances;
        type Event = Event;
        type WeightInfo = ();
    }

    parameter_types! {
        pub const AssetDeposit: Balance = 1; // 1 UNIT deposit to create asset
        pub const ApprovalDeposit: Balance = 1;
        pub const AssetsStringLimit: u32 = 50;
        /// Key = 32 bytes, Value = 36 bytes (32+1+1+1+1)
        // https://github.com/paritytech/substrate/blob/069917b/frame/assets/src/lib.rs#L257L271
        pub const MetadataDepositBase: Balance = 0;
        pub const MetadataDepositPerByte: Balance = 0;
        pub const AssetAccountDeposit: Balance = 0;
    }

    impl pallet_assets::Config for Test {
        type ApprovalDeposit = ApprovalDeposit;
        type AssetAccountDeposit = AssetAccountDeposit;
        type AssetDeposit = AssetDeposit;
        type AssetId = u32;
        type Balance = Balance;
        type Currency = Balances;
        type Event = Event;
        type Extra = ();
        type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
        type Freezer = ();
        type MetadataDepositBase = MetadataDepositBase;
        type MetadataDepositPerByte = MetadataDepositPerByte;
        type StringLimit = AssetsStringLimit;
        type WeightInfo = ();
    }

    impl pallet_account_manager::Config for Test {
        type AssetBalanceOf = ConvertInto;
        type AssetId = u32;
        type Assets = Assets;
        type Clock = t3rn_primitives::clock::ClockMock<Self>;
        type Currency = Balances;
        type EscrowAccount = EscrowAccount;
        type Event = Event;
        type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
        type Time = Timestamp;
        type WeightInfo = ();
    }

    parameter_types! {
        pub const CircuitAccountId: AccountId32 = AccountId32::new([51u8; 32]); // 0x333....3
    }

    impl pallet_circuit::Config for Test {
        type AccountManager = AccountManager;
        type Balances = Balances;
        type Call = Call;
        type Currency = Balances;
        type DeletionQueueLimit = ConstU32<1024>;
        type Event = Event;
        type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
        type Portal = CircuitPortal;
        type SFXBiddingPeriod = ConstU64<3>;
        type SelfAccountId = CircuitAccountId;
        type SelfGatewayId = CircuitTargetId;
        type SelfParaId = ConstU32<3333u32>;
        type SignalQueueDepth = ConstU32<4>;
        type WeightInfo = ();
        type Xdns = Xdns;
        type XtxTimeoutCheckInterval = ConstU64<1024>;
        type XtxTimeoutDefault = ConstU64<1024>;
    }

    impl pallet_xdns::Config for Test {
        type Balances = Balances;
        type Currency = Balances;
        type Event = Event;
        type Time = Timestamp;
        type WeightInfo = ();
    }

    pub type CurrencyId = u32;
    pub type Balance = u64;
    pub type Amount = i64;

    parameter_types! {
        pub const HeadersToStore: u32 = 100;
    }

    type RococoBridgeInstance = ();

    #[derive(Debug)]
    pub struct Blake2ValU32Chain;

    use t3rn_primitives::portal::{KusamaLightClient, PolkadotLightClient, RococoLightClient};

    impl pallet_grandpa_finality_verifier::bridges::runtime::Chain for Blake2ValU32Chain {
        type BlockNumber = u32;
        type Hash = H256;
        type Hasher = BlakeTwo256;
        type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
    }

    impl pallet_grandpa_finality_verifier::Config<RococoLightClient> for Test {
        type BridgedChain = Blake2ValU32Chain;
        type HeadersToStore = HeadersToStore;
        type WeightInfo = ();
    }

    impl pallet_grandpa_finality_verifier::Config<PolkadotLightClient> for Test {
        type BridgedChain = Blake2ValU32Chain;
        type HeadersToStore = HeadersToStore;
        type WeightInfo = ();
    }

    impl pallet_grandpa_finality_verifier::Config<KusamaLightClient> for Test {
        type BridgedChain = Blake2ValU32Chain;
        type HeadersToStore = HeadersToStore;
        type WeightInfo = ();
    }

    parameter_types! {
        pub const ExecPalletId: frame_support::PalletId = frame_support::PalletId(*b"pal/exec");
    }

    impl pallet_portal::Config for Test {
        type Event = Event;
        type WeightInfo = pallet_portal::weights::SubstrateWeight<Test>;
        type Xdns = Xdns;
    }
}
