mod threevm_mock {
    use crate::tests::*;
    use frame_support::parameter_types;
    use sp_std::boxed::Box;

    use pallet_grandpa_finality_verifier::light_clients::{
        select_grandpa_light_client_instance, KusamaInstance, LightClient, PolkadotInstance,
        RococoInstance,
    };
    use pallet_portal::Error as PortalError;
    use t3rn_primitives::GatewayVendor;

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
        type OnLocalTrigger = Circuit;
        type RuntimeEvent = RuntimeEvent;
        type SignalBounceThreshold = ConstU32<2>;
    }

    impl pallet_sudo::Config for Test {
        type RuntimeCall = RuntimeCall;
        type RuntimeEvent = RuntimeEvent;
    }

    impl pallet_contracts_registry::Config for Test {
        type Balances = Balances;
        type Currency = Balances;
        type RuntimeEvent = RuntimeEvent;
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
        type Extra = ();
        type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
        type Freezer = ();
        type MetadataDepositBase = MetadataDepositBase;
        type MetadataDepositPerByte = MetadataDepositPerByte;
        type RuntimeEvent = RuntimeEvent;
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
        type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
        type RuntimeEvent = RuntimeEvent;
        type Time = Timestamp;
        type WeightInfo = ();
    }

    parameter_types! {
        pub const CircuitAccountId: AccountId32 = AccountId32::new([51u8; 32]); // 0x333....3
    }

    impl pallet_circuit::Config for Test {
        type AccountManager = AccountManager;
        type Balances = Balances;
        type Currency = Balances;
        type DeletionQueueLimit = ConstU32<1024>;
        type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
        type Portal = CircuitPortal;
        type RuntimeCall = RuntimeCall;
        type RuntimeEvent = RuntimeEvent;
        type SFXBiddingPeriod = ConstU32<3>;
        type SelfAccountId = CircuitAccountId;
        type SelfGatewayId = CircuitTargetId;
        type SelfParaId = ConstU32<3333u32>;
        type SignalQueueDepth = ConstU32<4>;
        type WeightInfo = ();
        type Xdns = Xdns;
        type XtxTimeoutCheckInterval = ConstU32<1024>;
        type XtxTimeoutDefault = ConstU32<1024>;
    }

    impl pallet_xdns::Config for Test {
        type Balances = Balances;
        type Currency = Balances;
        type RuntimeEvent = RuntimeEvent;
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

    impl pallet_grandpa_finality_verifier::bridges::runtime::Chain for Blake2ValU32Chain {
        type BlockNumber = u32;
        type Hash = H256;
        type Hasher = BlakeTwo256;
        type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
    }

    impl pallet_grandpa_finality_verifier::Config<RococoInstance> for Test {
        type BridgedChain = Blake2ValU32Chain;
        type EpochOffset = ConstU32<2_400u32>;
        type FastConfirmationOffset = ConstU32<3u32>;
        type FinalizedConfirmationOffset = ConstU32<10u32>;
        type HeadersToStore = HeadersToStore;
        type RationalConfirmationOffset = ConstU32<10u32>;
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
    }

    impl pallet_grandpa_finality_verifier::Config<PolkadotInstance> for Test {
        type BridgedChain = Blake2ValU32Chain;
        type EpochOffset = ConstU32<2_400u32>;
        type FastConfirmationOffset = ConstU32<3u32>;
        type FinalizedConfirmationOffset = ConstU32<10u32>;
        type HeadersToStore = HeadersToStore;
        type RationalConfirmationOffset = ConstU32<10u32>;
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
    }

    impl pallet_grandpa_finality_verifier::Config<KusamaInstance> for Test {
        type BridgedChain = Blake2ValU32Chain;
        type EpochOffset = ConstU32<2_400u32>;
        type FastConfirmationOffset = ConstU32<3u32>;
        type FinalizedConfirmationOffset = ConstU32<10u32>;
        type HeadersToStore = HeadersToStore;
        type RationalConfirmationOffset = ConstU32<10u32>;
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
    }

    parameter_types! {
        pub const ExecPalletId: frame_support::PalletId = frame_support::PalletId(*b"pal/exec");
    }

    pub struct SelectLightClientRegistry;

    impl pallet_portal::SelectLightClient<Test> for SelectLightClientRegistry {
        fn select(vendor: GatewayVendor) -> Result<Box<dyn LightClient<Test>>, PortalError<Test>> {
            match vendor {
                GatewayVendor::Rococo =>
                    select_grandpa_light_client_instance::<Test, RococoInstance>(vendor)
                        .ok_or(PortalError::<Test>::LightClientNotFoundByVendor)
                        .map(|lc| Box::new(lc) as Box<dyn LightClient<Test>>),
                GatewayVendor::Kusama =>
                    select_grandpa_light_client_instance::<Test, KusamaInstance>(vendor)
                        .ok_or(PortalError::<Test>::LightClientNotFoundByVendor)
                        .map(|lc| Box::new(lc) as Box<dyn LightClient<Test>>),
                GatewayVendor::Polkadot =>
                    select_grandpa_light_client_instance::<Test, PolkadotInstance>(vendor)
                        .ok_or(PortalError::<Test>::LightClientNotFoundByVendor)
                        .map(|lc| Box::new(lc) as Box<dyn LightClient<Test>>),
                _ => Err(PortalError::<Test>::UnimplementedGatewayVendor),
            }
        }
    }

    impl pallet_portal::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type SelectLightClient = SelectLightClientRegistry;
        type WeightInfo = pallet_portal::weights::SubstrateWeight<Test>;
        type Xdns = Xdns;
    }
}
