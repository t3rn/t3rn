use crate::{
    AccountId, AllPalletsWithSystem, AssetRegistry, Assets, Balance, Balances, Contracts, DmpQueue,
    EnsureRoot, Evm, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime, RuntimeCall,
    RuntimeEvent, RuntimeOrigin, WeightToFee, XcmpQueue, MAXIMUM_BLOCK_WEIGHT,
};
use cumulus_primitives_core::ParaId;
use frame_support::{
    match_types, parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, Everything, Nothing, PalletInfoAccess},
    weights::{ConstantMultiplier, IdentityFee, Weight},
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use polkadot_runtime_common::ToAuthor;
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
    ConvertedConcreteAssetId, ConvertedConcreteId, CurrencyAdapter, EnsureXcmOrigin,
    FixedWeightBounds, FungiblesAdapter, IsConcrete, LocalMint, NativeAsset, ParentIsPreset,
    RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
    SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
    UsingComponents,
};
use xcm_executor::{traits::JustTry, XcmExecutor};

parameter_types! {
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
    pub const XbiSovereign: AccountId = AccountId::new([68u8; 32]); // 0x444...4
    pub NotificationWeight: Weight = Weight::from_ref_time(1_000_000_000);
}

// ToDo: Implement
// impl codec::EncodeLike<pallet_xbi_portal::Call<Runtime>> for Call {}

impl pallet_xbi_portal::Config for Runtime {
    type AssetRegistry = AssetRegistry;
    type Assets = Assets;
    // type SelfAccountId = XbiSovereign;
    // type Callback = XBIPortalRuntimeEntry;
    type Callback = ();
    type CheckInLimit = ConstU32<100>;
    type CheckInterval = ConstU32<3>;
    type CheckOutLimit = ConstU32<100>;
    type Contracts = Contracts;
    type Currency = Balances;
    type DeFi = ();
    type Evm = Evm;
    type ExpectedBlockTimeMs = ConstU32<6000>;
    type FeeConversion = ConstantMultiplier<u128, ConstU128<10u128>>;
    type NotificationWeight = NotificationWeight;
    type ParachainId = ConstU32<3333>;
    type ReserveBalanceCustodian = XbiSovereign;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type TimeoutChecksLimit = ConstU32<3000>;
    type Xcm = XcmRouter;
    type XcmSovereignOrigin = XbiSovereign;
}

// pub struct XBIPortalRuntimeEntry {}

// impl pallet_xbi_portal::primitives::xbi::XBIPortal<Runtime> for XBIPortalRuntimeEntry {
//     fn do_check_in_xbi(xbi: XBIFormat) -> Result<(), Error<Runtime>> {
//         XBIPortal::do_check_in_xbi(xbi)
//     }

//     fn get_status(xbi_id: H256) -> XBIStatus {
//         XBIPortal::get_status(xbi_id)
//     }

//     fn get_check_in(
//         xbi_id: H256,
//     ) -> Result<XBICheckIn<<Runtime as frame_system::Config>::BlockNumber>, DispatchError> {
//         XBIPortal::get_check_in(xbi_id)
//     }

//     fn get_check_out(xbi_id: H256) -> Result<XBICheckOut, DispatchError> {
//         XBIPortal::get_check_out(xbi_id)
//     }
// }

// impl pallet_xbi_portal::primitives::xbi_callback::XBICallback<Runtime> for XBIPortalRuntimeEntry {
//     fn callback(xbi_checkin: XBICheckIn<BlockNumber>, xbi_checkout: XBICheckOut) {
//         Circuit::do_xbi_exit(xbi_checkin, xbi_checkout);
//     }
// }
parameter_types! {
    pub const RelayLocation: MultiLocation = MultiLocation::parent();
    // Our representation of the relay asset id
    pub const RelayAssetId: u32 = 1;
    pub const RelayNetwork: NetworkId = NetworkId::Rococo;
    pub const SelfLocation: MultiLocation = MultiLocation::here();
    pub UniversalLocation: InteriorMultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();

    pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
    pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
    pub CheckingAccount: AccountId = PolkadotXcm::check_account();
    pub AssetsPalletLocation: MultiLocation =
        PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the parent `AccountId`.
    ParentIsPreset<AccountId>,
    // Sibling parachain origins convert to AccountId via the `ParaId::into`.
    SiblingParachainConvertsVia<Sibling, AccountId>,
    // Straight up local `AccountId32` origins just alias directly to `AccountId`.
    AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type SovereignAccountOf = (
    SiblingParachainConvertsVia<ParaId, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type LocalAssetTransactor = CurrencyAdapter<
    // Use this currency:
    Balances,
    // Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<SelfLocation>,
    // We can convert the MultiLocations with our converter above:
    SovereignAccountOf,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    (),
>;

/// Means for transacting assets besides the native currency on this chain.
pub type FungiblesTransactor = FungiblesAdapter<
    Assets,
    // Use the asset registry for lookups
    ConvertedConcreteId<
        parachains_common::AssetIdForTrustBackedAssets,
        Balance,
        AssetRegistry,
        JustTry,
    >,
    // Convert an XCM MultiLocation into a local account id:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We only want to allow teleports of known assets. We use non-zero issuance as an indication
    // that this asset is known.
    LocalMint<parachains_common::impls::NonZeroIssuance<AccountId, Assets>>,
    // The account to use for tracking teleports.
    CheckingAccount,
>;

pub type AssetTransactors = (LocalAssetTransactor, FungiblesTransactor);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
    // Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
    // recognized.
    RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
    // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
    // recognized.
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
    // Native signed account converter; this just converts an `AccountId32` origin into a normal
    // `Origin::Signed` origin of the same 32-byte value.
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
    // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
    pub UnitWeightCost: Weight = Weight::from_ref_time(1_000_000_000);
    pub const MaxInstructions: u32 = 100;
    pub const MaxAssetsIntoHolding: u32 = 64;
}

match_types! {
    pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
        MultiLocation { parents: 1, interior: Here } |
        MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
    };
}

// FIXME: should be using asset_registry
pub type Barrier = (
    TakeWeightCredit,
    AllowTopLevelPaidExecutionFrom<Everything>,
    AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
    // ^^^ Parent and its exec plurality get free execution
    AssetRegistry,
);

parameter_types! {
    pub const Roc: MultiAssetFilter = Wild(AllOf { fun: WildFungible, id: Concrete(RelayLocation::get()) });
    pub const AllAssets: MultiAssetFilter = Wild(All);
    pub const RocForRococo: (MultiAssetFilter, MultiLocation) = (Roc::get(), RelayLocation::get());
    pub const RococoForSlim: (MultiAssetFilter, MultiLocation) = (AllAssets::get(), Parachain(1).into_location());
    pub const RococoForSlender: (MultiAssetFilter, MultiLocation) = (AllAssets::get(), Parachain(2).into_location());
    pub const RococoForLarge: (MultiAssetFilter, MultiLocation) = (AllAssets::get(), Parachain(3).into_location());
    pub const RococoForStatemine: (MultiAssetFilter, MultiLocation) = (Roc::get(), Parachain(4).into_location());
    pub const RococoForCanvas: (MultiAssetFilter, MultiLocation) = (Roc::get(), Parachain(5).into_location());
    pub const RococoForEncointer: (MultiAssetFilter, MultiLocation) = (Roc::get(), Parachain(6).into_location());
}

pub type TrustedTeleporters = (
    NativeAsset,
    xcm_builder::Case<RocForRococo>,
    xcm_builder::Case<RococoForSlim>,
    xcm_builder::Case<RococoForSlender>,
    xcm_builder::Case<RococoForLarge>,
    // xcm_builder::Case<RococoForStatemine>,
    // xcm_builder::Case<RococoForCanvas>,
    // xcm_builder::Case<RococoForEncointer>,
);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type AssetClaims = PolkadotXcm;
    type AssetExchanger = ();
    type AssetLocker = ();
    // How to withdraw and deposit an asset.
    type AssetTransactor = AssetTransactors;
    type AssetTrap = PolkadotXcm;
    type Barrier = Barrier;
    type CallDispatcher = RuntimeCall;
    type FeeManager = ();
    type IsReserve = NativeAsset;
    type IsTeleporter = TrustedTeleporters;
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type MessageExporter = ();
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type ResponseHandler = PolkadotXcm;
    type RuntimeCall = RuntimeCall;
    type SafeCallFilter = Everything;
    type SubscriptionService = PolkadotXcm;
    // FIXME: should be using asset_registry
    type Trader =
        UsingComponents<WeightToFee, RelayLocation, AccountId, Balances, ToAuthor<Runtime>>;
    type UniversalAliases = Nothing;
    type UniversalLocation = UniversalLocation;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type XcmSender = XcmRouter;
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.div(4);
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::AnyRelayNumber;
    type DmpMessageHandler = DmpQueue;
    type OnSystemEvent = ();
    type OutboundXcmpMessageSource = XcmpQueue;
    type ReservedDmpWeight = ReservedDmpWeight;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type RuntimeEvent = RuntimeEvent;
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type XcmpMessageHandler = XcmpQueue;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type ChannelInfo = ParachainSystem;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
    type PriceForSiblingDelivery = ();
    type RuntimeEvent = RuntimeEvent;
    type VersionWrapper = ();
    type WeightInfo = ();
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// TODO: this would probably be configured much like the asset registry, e.g basilisk might not allow XCMP but we do.
/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
    // Two routers - use UMP to communicate with the relay chain:
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem, (), ()>,
    // ..and XCMP to communicate with the sibling chains.
    XcmpQueue,
);

impl pallet_xcm::Config for Runtime {
    // ^ Override for AdvertisedXcmVersion default
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type MaxLockers = ConstU32<8>;
    #[cfg(feature = "runtime-benchmarks")]
    type ReachableDest = ReachableDest;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type SovereignAccountOf = LocationToAccountId;
    type TrustedLockers = ();
    type UniversalLocation = UniversalLocation;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    // TODO: make sure this is configured for production
    type WeightInfo = pallet_xcm::TestWeightInfo;
    type XcmExecuteFilter = Everything;
    // ^ Disable dispatchable execute on the XCM pallet.
    // Needs to be `Everything` for local testing.
    type XcmExecutor = XcmExecutor<XcmConfig>;
    // type XcmReserveTransferFilter = Everything;
    type XcmReserveTransferFilter = Nothing;
    type XcmRouter = XcmRouter;
    type XcmTeleportFilter = Everything;

    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
}
