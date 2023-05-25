use codec::{Decode, Encode};
use frame_support::{traits::FindAuthor, RuntimeDebug};
pub use pallet_attesters::{
    ActiveSet, AttestationTargets, Attesters as AttestersStore, BatchMessage, BatchStatus, Batches,
    Config as ConfigAttesters, CurrentCommittee, Error as AttestersError, NextBatch, Nominations,
    PendingSlashes, PendingUnnominations, PreviousCommittee, SortedNominatedAttesters,
};
pub use pallet_circuit::{Config as ConfigCircuit, FullSideEffects, SFX2XTXLinksMap, XExecSignals};
mod treasuries_config;

pub use pallet_account_manager::{
    Config as ConfigAccountManager, Error as AccountManagerError, Event as AccountManagerEvent,
    SettlementsPerRound,
};

use sp_runtime::ConsensusEngineId;

use pallet_attesters::TargetId;
use pallet_grandpa_finality_verifier::{
    bridges::runtime as bp_runtime,
    light_clients::{
        select_grandpa_light_client_instance, KusamaInstance, PolkadotInstance, RococoInstance,
    },
};
use pallet_portal::Error as PortalError;
pub use pallet_rewards::{
    Authors, AuthorsThisPeriod, Config as ConfigRewards, DistributionBlock, DistributionHistory,
    Error as RewardsError, PendingClaims,
};
use sp_core::H256;
use sp_runtime::{
    generic, parameter_types,
    traits::{BlakeTwo256, ConstU32, ConvertInto, IdentityLookup},
    Perbill, Percent,
};
use t3rn_primitives::{ExecutionVendor, GatewayVendor};
pub type AccountId = sp_runtime::AccountId32;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MiniRuntime>;
pub type Block = frame_system::mocking::MockBlock<MiniRuntime>;
pub type BlockNumber = u32;
pub type Balance = u128;
type Header = generic::Header<u32, BlakeTwo256>;

use pallet_grandpa_finality_verifier::light_clients::LightClient;

frame_support::construct_runtime!(
    pub enum MiniRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system = 1,
        Balances: pallet_balances = 2,
        Timestamp: pallet_timestamp = 3,
        RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip = 4,
        Assets: pallet_assets = 5,
        // Treasuries
        Treasury: pallet_treasury = 13, // Keep old treasury index for backwards compatibility
        EscrowTreasury: pallet_treasury::<Instance1> = 16,
        FeeTreasury: pallet_treasury::<Instance2> = 17,
        ParachainTreasury: pallet_treasury::<Instance3> = 18,
        SlashTreasury: pallet_treasury::<Instance4> = 19,
        // t3rn
        XDNS: pallet_xdns = 100,
        Attesters: pallet_attesters = 101,
        Rewards: pallet_rewards = 102,
        AccountManager: pallet_account_manager = 103,
        Clock: pallet_clock = 104,
        Circuit: pallet_circuit = 105,
        // Portal
        Portal: pallet_portal = 128,
        RococoBridge: pallet_grandpa_finality_verifier = 129,
        PolkadotBridge: pallet_grandpa_finality_verifier::<Instance1> = 130,
        KusamaBridge: pallet_grandpa_finality_verifier::<Instance2> = 131,

    }
);

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

impl pallet_assets::Config for MiniRuntime {
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

parameter_types! {
    pub const EscrowAccount: AccountId = AccountId::new([51u8; 32]);
}

impl pallet_account_manager::Config for MiniRuntime {
    type AssetBalanceOf = ConvertInto;
    type AssetId = u32;
    type Assets = Assets;
    type Clock = Clock;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Time = Timestamp;
    type WeightInfo = ();
}

impl pallet_clock::Config for MiniRuntime {
    type AccountManager = AccountManager;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type OnFinalizeQueues = pallet_clock::traits::EmptyOnHookQueues<Self>;
    type OnInitializeQueues = pallet_clock::traits::EmptyOnHookQueues<Self>;
    type RoundDuration = ConstU32<400>;
}
use t3rn_primitives::monetary::TRN;

parameter_types! {
    pub const TotalInflation: Perbill = Perbill::from_parts(44_000_000); // 4.4%
    pub const AttesterInflation: Perbill = Perbill::from_parts(11_000_000); // 1.1%
    pub const ExecutorInflation: Perbill = Perbill::from_parts(8_000_000); // 0.8%
    pub const CollatorInflation: Perbill = Perbill::from_parts(5_000_000); // 0.5%
    pub const TreasuryInflation: Perbill = Perbill::from_parts(20_000_000); // 2%
    pub const AttesterBootstrapRewards: Percent = Percent::from_parts(40); // 40%
    pub const CollatorBootstrapRewards: Percent = Percent::from_parts(20); // 20%
    pub const ExecutorBootstrapRewards: Percent = Percent::from_parts(40); // 40%
    pub const StartingRepatriationPercentage: Percent = Percent::from_parts(10); // 10%
    pub const OneYear: BlockNumber = 2_628_000; // (365.25 * 24 * 60 * 60) / 12; assuming 12s block time
    pub const InflationDistributionPeriod: BlockNumber = 100_800; // (14 * 24 * 60 * 60) / 12; assuming one distribution per two weeks
    pub const AvailableBootstrapSpenditure: Balance = 1_000_000 * (TRN as Balance); // 1 MLN UNIT
}

pub struct FindAuthorMockRoundRobinRotate32;

impl FindAuthor<AccountId> for FindAuthorMockRoundRobinRotate32 {
    fn find_author<'a, I>(_digests: I) -> Option<AccountId>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        // Get current block number
        let current_block_number = <frame_system::Module<MiniRuntime>>::block_number();

        let round_robin_rotate_32: u8 = (current_block_number % 32) as u8;

        let mock_rr_account = AccountId::new([round_robin_rotate_32; 32]);

        Some(mock_rr_account)
    }
}

impl pallet_rewards::Config for MiniRuntime {
    type AccountManager = AccountManager;
    type AttesterBootstrapRewards = AttesterBootstrapRewards;
    type AttesterInflation = AttesterInflation;
    type Attesters = Attesters;
    type AvailableBootstrapSpenditure = AvailableBootstrapSpenditure;
    type Clock = Clock;
    type CollatorBootstrapRewards = CollatorBootstrapRewards;
    type CollatorInflation = CollatorInflation;
    type Currency = Balances;
    type Event = Event;
    type ExecutorBootstrapRewards = ExecutorBootstrapRewards;
    type ExecutorInflation = ExecutorInflation;
    type FindAuthor = FindAuthorMockRoundRobinRotate32;
    type InflationDistributionPeriod = InflationDistributionPeriod;
    type OneYear = OneYear;
    type StartingRepatriationPercentage = StartingRepatriationPercentage;
    type TotalInflation = TotalInflation;
    type TreasuryAccounts = MiniRuntime;
    type TreasuryInflation = TreasuryInflation;
}

parameter_types! {
    pub const DefaultCommission: Percent = Percent::from_percent(10);
    pub const CommitmentRewardSource: AccountId = AccountId::new([51u8; 32]);
    pub const SlashAccount: AccountId = AccountId::new([51u8; 32]);
    pub const RewardMultiplier: Balance = 1;
    pub const MinAttesterBond: Balance = 1;
    pub const MinNominatorBond: Balance = 1;
}

impl pallet_attesters::Config for MiniRuntime {
    type ActiveSetSize = ConstU32<32>;
    type BatchingWindow = ConstU32<6>;
    type CommitmentRewardSource = CommitmentRewardSource;
    type CommitteeSize = ConstU32<32>;
    type Currency = Balances;
    type DefaultCommission = DefaultCommission;
    type Event = Event;
    type MaxBatchSize = ConstU32<128>;
    type MinAttesterBond = MinAttesterBond;
    type MinNominatorBond = MinNominatorBond;
    type Portal = Portal;
    type RandomnessSource = RandomnessCollectiveFlip;
    type ReadSFX = Circuit;
    type RepatriationPeriod = ConstU32<60>;
    type RewardMultiplier = RewardMultiplier;
    type Rewards = Rewards;
    type ShufflingFrequency = ConstU32<400>;
    type SlashAccount = SlashAccount;
    type Xdns = XDNS;
}

impl pallet_insecure_randomness_collective_flip::Config for MiniRuntime {}

impl pallet_balances::Config for MiniRuntime {
    type AccountStore = System;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    /// The ubiquitous event type.
    type Event = Event;
    type ExistentialDeposit = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl frame_system::Config for MiniRuntime {
    type AccountData = pallet_balances::AccountData<u128>;
    type AccountId = AccountId;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockHashCount = ();
    type BlockLength = ();
    type BlockNumber = BlockNumber;
    type BlockWeights = ();
    type Call = Call;
    type DbWeight = ();
    type Event = Event;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Index = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = ConstU32<16>;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type Origin = Origin;
    type PalletInfo = PalletInfo;
    type SS58Prefix = ();
    type SystemWeightInfo = ();
    type Version = ();
}

pub type AssetId = u32;

pub struct SelectLightClientRegistry;

impl pallet_portal::SelectLightClient<MiniRuntime> for SelectLightClientRegistry {
    fn select(
        vendor: GatewayVendor,
    ) -> Result<Box<dyn LightClient<MiniRuntime>>, PortalError<MiniRuntime>> {
        match vendor {
            GatewayVendor::Rococo =>
                select_grandpa_light_client_instance::<MiniRuntime, RococoInstance>(vendor)
                    .ok_or(PortalError::<MiniRuntime>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<MiniRuntime>>),
            GatewayVendor::Kusama =>
                select_grandpa_light_client_instance::<MiniRuntime, KusamaInstance>(vendor)
                    .ok_or(PortalError::<MiniRuntime>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<MiniRuntime>>),
            GatewayVendor::Polkadot =>
                select_grandpa_light_client_instance::<MiniRuntime, PolkadotInstance>(vendor)
                    .ok_or(PortalError::<MiniRuntime>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<MiniRuntime>>),
            _ => Err(PortalError::<MiniRuntime>::UnimplementedGatewayVendor),
        }
    }
}
const SLOT_DURATION: u64 = 12000;

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for MiniRuntime {
    type MinimumPeriod = MinimumPeriod;
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type WeightInfo = ();
}

impl pallet_xdns::Config for MiniRuntime {
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type Time = Timestamp;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<MiniRuntime>;
}

parameter_types! {
    pub const CircuitAccountId: AccountId = AccountId::new([51u8; 32]); // 0x333...3
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
}

impl pallet_circuit::Config for MiniRuntime {
    type AccountManager = AccountManager;
    type Balances = Balances;
    type Call = Call;
    type Currency = Balances;
    type DeletionQueueLimit = ConstU32<100u32>;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Portal = Portal;
    type SFXBiddingPeriod = ConstU32<3u32>;
    type SelfAccountId = CircuitAccountId;
    type SelfGatewayId = SelfGatewayId;
    type SelfParaId = ConstU32<3333u32>;
    type SignalQueueDepth = ConstU32<5u32>;
    type WeightInfo = ();
    type Xdns = XDNS;
    type XtxTimeoutCheckInterval = ConstU32<10u32>;
    type XtxTimeoutDefault = ConstU32<400u32>;
}

impl pallet_portal::Config for MiniRuntime {
    type Event = Event;
    type SelectLightClient = SelectLightClientRegistry;
    type WeightInfo = pallet_portal::weights::SubstrateWeight<MiniRuntime>;
    type Xdns = XDNS;
}

parameter_types! {
    pub const HeadersToStore: u32 = 100;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl bp_runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

impl pallet_grandpa_finality_verifier::Config<RococoInstance> for MiniRuntime {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type Event = Event;
    type FastConfirmationOffset = ConstU32<3u32>;
    type FinalizedConfirmationOffset = ConstU32<10u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<10u32>;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<PolkadotInstance> for MiniRuntime {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type Event = Event;
    type FastConfirmationOffset = ConstU32<3u32>;
    type FinalizedConfirmationOffset = ConstU32<10u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<10u32>;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<KusamaInstance> for MiniRuntime {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type Event = Event;
    type FastConfirmationOffset = ConstU32<3u32>;
    type FinalizedConfirmationOffset = ConstU32<10u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<10u32>;
    type WeightInfo = ();
}

// Mock from pallet events
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum MockedAssetEvent<T: frame_system::Config + pallet_balances::Config> {
    /// Some asset class was created.
    Created {
        asset_id: AssetId,
        creator: T::AccountId,
        owner: T::AccountId,
    },
    /// Some assets were issued.
    Issued {
        asset_id: AssetId,
        owner: T::AccountId,
        amount: T::Balance,
    },
    /// Some assets were transferred.
    Transferred {
        asset_id: AssetId,
        from: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    },
    /// Some assets were destroyed.
    Burned {
        asset_id: AssetId,
        owner: T::AccountId,
        balance: T::Balance,
    },
    /// The management team changed.
    TeamChanged {
        asset_id: AssetId,
        issuer: T::AccountId,
        admin: T::AccountId,
        freezer: T::AccountId,
    },
    /// The owner changed.
    OwnerChanged {
        asset_id: AssetId,
        owner: T::AccountId,
    },
    /// Some account `who` was frozen.
    Frozen {
        asset_id: AssetId,
        who: T::AccountId,
    },
    /// Some account `who` was thawed.
    Thawed {
        asset_id: AssetId,
        who: T::AccountId,
    },
    /// Some asset `asset_id` was frozen.
    AssetFrozen { asset_id: AssetId },
    /// Some asset `asset_id` was thawed.
    AssetThawed { asset_id: AssetId },
    /// Accounts were destroyed for given asset.
    AccountsDestroyed {
        asset_id: AssetId,
        accounts_destroyed: u32,
        accounts_remaining: u32,
    },
    /// Approvals were destroyed for given asset.
    ApprovalsDestroyed {
        asset_id: AssetId,
        approvals_destroyed: u32,
        approvals_remaining: u32,
    },
    /// An asset class is in the process of being destroyed.
    DestructionStarted { asset_id: AssetId },
    /// An asset class was destroyed.
    Destroyed { asset_id: AssetId },
    /// Some asset class was force-created.
    ForceCreated {
        asset_id: AssetId,
        owner: T::AccountId,
    },
    /// New metadata has been set for an asset.
    MetadataSet {
        asset_id: AssetId,
        name: Vec<u8>,
        symbol: Vec<u8>,
        decimals: u8,
        is_frozen: bool,
    },
    /// Metadata has been cleared for an asset.
    MetadataCleared { asset_id: AssetId },
    /// (Additional) funds have been approved for transfer to a destination account.
    ApprovedTransfer {
        asset_id: AssetId,
        source: T::AccountId,
        delegate: T::AccountId,
        amount: T::Balance,
    },
    /// An approval for account `delegate` was cancelled by `owner`.
    ApprovalCancelled {
        asset_id: AssetId,
        owner: T::AccountId,
        delegate: T::AccountId,
    },
    /// An `amount` was transferred in its entirety from `owner` to `destination` by
    /// the approved `delegate`.
    TransferredApproved {
        asset_id: AssetId,
        owner: T::AccountId,
        delegate: T::AccountId,
        destination: T::AccountId,
        amount: T::Balance,
    },
    /// An asset has had its attributes changed by the `Force` origin.
    AssetStatusChanged { asset_id: AssetId },
}

type CodeHash<T> = <T as frame_system::Config>::Hash;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum MockWasmContractsEvent<T: frame_system::Config + pallet_balances::Config> {
    /// Contract deployed by address at the specified address.
    Instantiated {
        deployer: T::AccountId,
        contract: T::AccountId,
    },

    /// Contract has been removed.
    ///
    /// # Note
    ///
    /// The only way for a contract to be removed and emitting this event is by calling
    /// `seal_terminate`.
    Terminated {
        /// The contract that was terminated.
        contract: T::AccountId,
        /// The account that received the contracts remaining balance
        beneficiary: T::AccountId,
    },

    /// Code with the specified hash has been stored.
    CodeStored { code_hash: T::Hash },

    /// A custom event emitted by the contract.
    ContractEmitted {
        /// The contract that emitted the event.
        contract: T::AccountId,
        /// Data supplied by the contract. Metadata generated during contract compilation
        /// is needed to decode it.
        data: Vec<u8>,
    },

    /// A code with the specified hash was removed.
    CodeRemoved { code_hash: T::Hash },

    /// A contract's code was updated.
    ContractCodeUpdated {
        /// The contract that has been updated.
        contract: T::AccountId,
        /// New code hash that was set for the contract.
        new_code_hash: T::Hash,
        /// Previous code hash of the contract.
        old_code_hash: T::Hash,
    },

    /// A contract was called either by a plain account or another contract.
    ///
    /// # Note
    ///
    /// Please keep in mind that like all events this is only emitted for successful
    /// calls. This is because on failure all storage changes including events are
    /// rolled back.
    Called {
        /// The account that called the `contract`.
        caller: T::AccountId,
        /// The contract that was called.
        contract: T::AccountId,
    },

    /// A contract delegate called a code hash.
    ///
    /// # Note
    ///
    /// Please keep in mind that like all events this is only emitted for successful
    /// calls. This is because on failure all storage changes including events are
    /// rolled back.
    DelegateCalled {
        /// The contract that performed the delegate call and hence in whose context
        /// the `code_hash` is executed.
        contract: T::AccountId,
        /// The code hash that was delegate called.
        code_hash: CodeHash<T>,
    },
}

use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;
use t3rn_abi::{types::Sfx4bId, SFXAbi};
use t3rn_primitives::{contracts_registry::RegistryContract, xdns::GatewayRecord};

#[derive(Default)]
pub struct ExtBuilder {
    known_gateway_records: Vec<GatewayRecord<AccountId>>,
    standard_sfx_abi: Vec<(Sfx4bId, SFXAbi)>,
    known_contracts: Vec<RegistryContract<H256, AccountId, Balance, BlockNumber>>,
    balances: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
    pub fn with_gateway_records(mut self, gateway_records: Vec<GatewayRecord<AccountId>>) -> Self {
        self.known_gateway_records = gateway_records;
        self
    }

    pub fn with_polkadot_gateway_record(mut self) -> Self {
        let target: TargetId = [1u8; 4];
        let mock_escrow_account: AccountId = AccountId::new([2u8; 32]);
        self.known_gateway_records.push(GatewayRecord {
            gateway_id: target,
            verification_vendor: GatewayVendor::Polkadot,
            execution_vendor: ExecutionVendor::Substrate,
            codec: t3rn_abi::Codec::Rlp,
            registrant: None,
            escrow_account: Some(mock_escrow_account),
            allowed_side_effects: vec![],
        });
        self
    }

    pub fn with_eth_gateway_record(mut self) -> Self {
        let target: TargetId = [0u8; 4];
        let mock_escrow_account: AccountId = AccountId::new([2u8; 32]);
        self.known_gateway_records.push(GatewayRecord {
            gateway_id: target,
            verification_vendor: GatewayVendor::Ethereum,
            execution_vendor: ExecutionVendor::EVM,
            codec: t3rn_abi::Codec::Rlp,
            registrant: None,
            escrow_account: Some(mock_escrow_account),
            allowed_side_effects: vec![],
        });
        self
    }

    pub fn with_standard_sfx_abi(mut self, standard_sfx_abi: Vec<(Sfx4bId, SFXAbi)>) -> Self {
        self.standard_sfx_abi = standard_sfx_abi;
        self
    }

    pub fn with_known_contracts(
        mut self,
        known_contracts: Vec<RegistryContract<H256, AccountId, Balance, BlockNumber>>,
    ) -> Self {
        self.known_contracts = known_contracts;
        self
    }

    pub fn balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<MiniRuntime>()
            .expect("Frame system builds valid default genesis config");

        const TRN: Balance = 1_000_000_000_000;
        const TOTAL_SUPPLY: Balance = TRN * 100_000_000; // 100 million TRN
        const GENESIS_ACCOUNT_ID: AccountId = AccountId::new([0u8; 32]);

        pallet_balances::GenesisConfig::<MiniRuntime> {
            balances: vec![(GENESIS_ACCOUNT_ID, TOTAL_SUPPLY)],
        }
        .assimilate_storage(&mut t)
        .expect("Pallet balances storage can be assimilated");

        pallet_xdns::GenesisConfig::<MiniRuntime> {
            known_gateway_records: self.known_gateway_records,
            standard_sfx_abi: self.standard_sfx_abi,
        }
        .assimilate_storage(&mut t)
        .expect("Pallet xdns can be assimilated");

        pallet_rewards::GenesisConfig::<MiniRuntime> {
            phantom: Default::default(),
        }
        .assimilate_storage(&mut t)
        .expect("Pallet xdns can be assimilated");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
