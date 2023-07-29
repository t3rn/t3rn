use crate::{
    AccountId, AllPalletsWithSystem, AssetId, AssetRegistry, Assets, Balance, Balances, Contracts,
    DmpQueue, Evm, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent,
    RuntimeOrigin, WeightToFee, XcmpQueue, MAXIMUM_BLOCK_WEIGHT,
};
use circuit_runtime_pallets::{cumulus_primitives_core::GetChannelInfo, *};
use cumulus_primitives_core::ParaId;

use frame_support::{
    match_types, parameter_types,
    traits::{ConstU32, ConstU64, EitherOfDiverse, Everything, Nothing},
    weights::{IdentityFee, Weight},
};
use frame_system::EnsureRoot;
use pallet_xcm::{EnsureXcm, IsMajorityOfBody, XcmPassthrough};
use polkadot_parachain::primitives::Sibling;
use sp_runtime::traits::Zero;
use xcm::latest::prelude::*;
// use xcm_builder::{
//     AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
//     ConvertedConcreteAssetId, CurrencyAdapter, EnsureXcmOrigin, FixedWeightBounds,
//     FungiblesAdapter, IsConcrete, LocationInverter, NativeAsset, ParentIsPreset,
//     RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
//     SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
//     UsingComponents,
// };
use parachains_common::AssetIdForTrustBackedAssets;
use xcm_builder::{
    AccountId32Aliases,
    AllowKnownQueryResponses,
    AllowSubscriptionsFrom,
    AllowTopLevelPaidExecutionFrom,
    AllowUnpaidExecutionFrom,
    ConvertedConcreteAssetId, // WithComputedOrigin, WithUniqueTopic, TrailingSetTopicAsId, AllowExplicitUnpaidExecutionFrom
    ConvertedConcreteId,
    CurrencyAdapter,
    EnsureXcmOrigin,
    FixedWeightBounds,
    FungiblesAdapter,
    IsConcrete,
    LocalMint,
    // MatchedConvertedConcreteId,
    NativeAsset,
    ParentAsSuperuser,
    ParentIsPreset,
    RelayChainAsNative,
    SiblingParachainAsNative,
    SiblingParachainConvertsVia,
    SignedAccountId32AsNative,
    SignedToAccountId32,
    SovereignSignedViaLocation,
    TakeWeightCredit,
    UsingComponents,
};

use crate::{
    parachains_common::{
        impls::NonZeroIssuance,
        xcm_config::{DenyReserveTransferToRelayChain, DenyThenTry},
    },
    xcm_builder::AsPrefixedGeneralIndex,
};
use xcm_executor::{traits::JustTry, XcmExecutor};

parameter_types! {
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
    pub const XbiSovereign: AccountId = AccountId::new([68u8; 32]); // 0x444...4
}

parameter_types! {
    pub ReserveBalanceCustodian: AccountId = PolkadotXcm::check_account();
    pub NotificationWeight: Weight = Weight::from_ref_time(1);
}
//
// pub struct NonsenseNoopEvm;
// impl pallet_3vm_evm_primitives::traits::Evm<RuntimeOrigin> for NonsenseNoopEvm {
//     type Outcome = Result<
//         (
//             pallet_3vm_evm_primitives::CallInfo,
//             frame_support::pallet_prelude::Weight,
//         ),
//         sp_runtime::DispatchError,
//     >;
//
//     fn call(
//         _origin: RuntimeOrigin,
//         _target: sp_core::H160,
//         _input: Vec<u8>,
//         _value: sp_core::U256,
//         _gas_limit: u64,
//         _max_fee_per_gas: sp_core::U256,
//         _max_priority_fee_per_gas: Option<sp_core::U256>,
//         _nonce: Option<sp_core::U256>,
//         _access_list: Vec<(sp_core::H160, Vec<sp_core::H256>)>,
//     ) -> Self::Outcome {
//         Ok((
//             pallet_3vm_evm_primitives::CallInfo {
//                 exit_reason: pallet_3vm_evm_primitives::ExitReason::Succeed(
//                     pallet_3vm_evm_primitives::ExitSucceed::Stopped,
//                 ),
//                 value: vec![],
//                 used_gas: Default::default(),
//                 logs: vec![],
//             },
//             Zero::zero(),
//         ))
//     }
// }
//
// impl pallet_xbi_portal::Config for Runtime {
//     type AssetRegistry = AssetRegistry;
//     type Assets = Assets;
//     type Callback = ();
//     type CheckInLimit = ConstU32<100>;
//     type CheckInterval = ConstU32<3>;
//     type CheckOutLimit = ConstU32<100>;
//     type Contracts = Contracts;
//     type Currency = Balances;
//     type DeFi = ();
//     type Evm = NonsenseNoopEvm;
//     type ExpectedBlockTimeMs = ConstU32<6000>;
//     type FeeConversion = IdentityFee<Balance>;
//     type NotificationWeight = NotificationWeight;
//     type ParachainId = ConstU32<3333>;
//     type ReserveBalanceCustodian = ReserveBalanceCustodian;
//     type RuntimeCall = RuntimeCall;
//     type RuntimeEvent = RuntimeEvent;
//     type TimeoutChecksLimit = ConstU32<3000>;
//     type Xcm = XcmRouter;
//     type XcmSovereignOrigin = XbiSovereign;
// }

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
    pub RelayNetwork: Option<NetworkId> = Some(NetworkId::Rococo);
    pub const SelfLocation: MultiLocation = MultiLocation::here();

    pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
    pub Ancestry: MultiLocation = Parachain(3333).into();
    pub UniversalLocation: InteriorMultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
    pub CheckingAccount: AccountId = PolkadotXcm::check_account();
    pub AssetsPalletLocation: MultiLocation =
        PalletInstance(12u8).into();
}

pub type SovereignAccountOf = (
    SiblingParachainConvertsVia<ParaId, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);

parameter_types! {
    pub MaxAssetsIntoHolding: u32 = 64;
    pub SystemAssetHubLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(1000)));
    // ALWAYS ensure that the index in PalletInstance stays up-to-date with
    // the Relay Chain's Asset Hub's Assets pallet index
    pub SystemAssetHubAssetsPalletLocation: MultiLocation =
        MultiLocation::new(1, X2(Parachain(1000), PalletInstance(50)));
}
/// Means for transacting assets on this chain.
pub type LocalAssetTransactor = CurrencyAdapter<
    // Use this currency:
    Balances,
    // Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<SelfLocation>,
    // Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We don't track any teleports.
    (),
>;

// /// Means for transacting assets besides the native currency on this chain.
// pub type FungiblesTransactor = FungiblesAdapter<
//     // Use this fungibles implementation:
//     Assets,
//     // Use the asset registry for lookups
//     ConvertedConcreteAssetId<AssetId, Balance, Assets, JustTry>,
//     // Convert an XCM MultiLocation into a local account id:
//     LocationToAccountId,
//     // Our chain's account ID type (we can't get away without mentioning it explicitly):
//     AccountId,
//     // We only want to allow teleports of known assets. We use non-zero issuance as an indication
//     // that this asset is known.
//     LocalMint<NonZeroIssuance<AccountId, Assets>>,
//     // The account to use for tracking teleports.
//     CheckingAccount,
// >;
/// Means for transacting assets besides the native currency on this chain.
pub type FungiblesTransactor = FungiblesAdapter<
    // Use this fungibles implementation:
    Assets,
    // Use the asset registry for lookups
    ConvertedConcreteAssetId<AssetIdForTrustBackedAssets, Balance, AssetRegistry, JustTry>,
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
    // pub const RocForRococo: (MultiAssetFilter, MultiLocation) = (Roc::get(), RelayLocation::get());
    // pub const RococoForSlim: (MultiAssetFilter, MultiLocation) = (AllAssets::get(), Parachain(1).into());
    // pub const RococoForSlender: (MultiAssetFilter, MultiLocation) = (AllAssets::get(), Parachain(2).into());
    // pub const RococoForLarge: (MultiAssetFilter, MultiLocation) = (AllAssets::get(), Parachain(3).into());
    // pub const RococoForStatemine: (MultiAssetFilter, MultiLocation) = (Roc::get(), Parachain(4).into());
    // pub const RococoForCanvas: (MultiAssetFilter, MultiLocation) = (Roc::get(), Parachain(5).into());
    // pub const RococoForEncointer: (MultiAssetFilter, MultiLocation) = (Roc::get(), Parachain(6).into());
}
//
// pub type TrustedTeleporters = (
//     xcm_builder::Case<RocForRococo>,
//     xcm_builder::Case<RococoForSlim>,
//     xcm_builder::Case<RococoForSlender>,
//     xcm_builder::Case<RococoForLarge>,
//     // xcm_builder::Case<RococoForStatemine>,
//     // xcm_builder::Case<RococoForCanvas>,
//     // xcm_builder::Case<RococoForEncointer>,
// );

// pub struct XcmConfig;
// impl xcm_executor::Config for XcmConfig {
//     type AssetClaims = PolkadotXcm;
//     // How to withdraw and deposit an asset.
//     type AssetTransactor = AssetTransactors;
//     type AssetTrap = PolkadotXcm;
//     type Barrier = Barrier;
//     type IsReserve = NativeAsset;
//     type IsTeleporter = TrustedTeleporters;
//     type LocationInverter = LocationInverter<Ancestry>;
//     type OriginConverter = XcmOriginToTransactDispatchOrigin;
//     type ResponseHandler = PolkadotXcm;
//     type RuntimeCall = RuntimeCall;
//     type SubscriptionService = PolkadotXcm;
//     // FIXME: should be using asset_registry
//     type Trader = UsingComponents<IdentityFee<Balance>, RelayLocation, AccountId, Balances, ()>;
//     type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
//     type XcmSender = XcmRouter;
// }

parameter_types! {
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub SelfParaId: ParaId = ParaId::from(3333);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::AnyRelayNumber;
    type DmpMessageHandler = DmpQueue;
    type OnSystemEvent = ();
    type OutboundXcmpMessageSource = XcmpQueue;
    type ReservedDmpWeight = ReservedDmpWeight;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type RuntimeEvent = RuntimeEvent;
    type SelfParaId = SelfParaId;
    type XcmpMessageHandler = XcmpQueue;
}
//
// impl cumulus_pallet_xcmp_queue::Config for Runtime {
//     type ChannelInfo = ParachainSystem;
//     type ControllerOrigin = EnsureRoot<AccountId>;
//     type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
//     type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
//     type RuntimeEvent = RuntimeEvent;
//     type VersionWrapper = ();
//     type WeightInfo = ();
//     type XcmExecutor = XcmExecutor<XcmConfig>;
// }

impl GetChannelInfo for Runtime {
    fn get_channel_max(_id: ParaId) -> Option<usize> {
        None
    }

    fn get_channel_status(_id: ParaId) -> cumulus_primitives_core::ChannelStatus {
        cumulus_primitives_core::ChannelStatus::Ready(200, 200)
    }
}

impl parachain_info::Config for Runtime {}

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

// impl pallet_xcm::Config for Runtime {
//     // ^ Override for AdvertisedXcmVersion default
//     type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
//     type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
//     type LocationInverter = LocationInverter<Ancestry>;
//     type RuntimeCall = RuntimeCall;
//     type RuntimeEvent = RuntimeEvent;
//     type RuntimeOrigin = RuntimeOrigin;
//     type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
//     type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
//     // TODO: make sure this is configured for production
//     type XcmExecuteFilter = Everything;
//     // ^ Disable dispatchable execute on the XCM pallet.
//     // Needs to be `Everything` for local testing.
//     type XcmExecutor = XcmExecutor<XcmConfig>;
//     type XcmReserveTransferFilter = Everything;
//     type XcmRouter = XcmRouter;
//     type XcmTeleportFilter = Everything;
//
//     const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
// }

// /// We allow root and the Relay Chain council to execute privileged collator selection operations.
// pub type CollatorSelectionUpdateOrigin = EitherOfDiverse<
//     EnsureRoot<AccountId>,
//     EnsureXcm<IsMajorityOfBody<RelayLocation, ExecutiveBody>>,
// >;

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

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
    // Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
    // recognised.
    RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
    // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
    // recognised.
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
    // Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
    // transaction from the Root origin.
    ParentAsSuperuser<RuntimeOrigin>,
    // Native signed account converter; this just converts an `AccountId32` origin into a normal
    // `RuntimeOrigin::Signed` origin of the same 32-byte value.
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
    // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
    pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
    pub const MaxInstructions: u32 = 100;
}

match_types! {
    pub type ParentOrParentsPlurality: impl Contains<MultiLocation> = {
        MultiLocation { parents: 1, interior: Here } |
        MultiLocation { parents: 1, interior: X1(Plurality { .. }) }
    };
    pub type ParentOrSiblings: impl Contains<MultiLocation> = {
        MultiLocation { parents: 1, interior: Here } |
        MultiLocation { parents: 1, interior: X1(_) }
    };
}

// pub type Barrier = TrailingSetTopicAsId<
//     DenyThenTry<
//         DenyReserveTransferToRelayChain,
//         (
//             TakeWeightCredit,
//             // Expected responses are OK.
//             AllowKnownQueryResponses<PolkadotXcm>,
//             // Allow XCMs with some computed origins to pass through.
//             WithComputedOrigin<
//                 (
//                     // If the message is one that immediately attemps to pay for execution, then allow it.
//                     AllowTopLevelPaidExecutionFrom<Everything>,
//                     // Parent and its pluralities (i.e. governance bodies) get free execution.
//                     AllowExplicitUnpaidExecutionFrom<ParentOrParentsPlurality>,
//                     // Subscriptions for version tracking are OK.
//                     AllowSubscriptionsFrom<ParentOrSiblings>,
//                 ),
//                 UniversalLocation,
//                 ConstU32<8>,
//             >,
//         ),
//     >,
// >;

// use assets_common::{
//     local_and_foreign_assets::MatchesLocalAndForeignAssetsMultiLocation,
//     matching::{
//         FromSiblingParachain, IsForeignConcreteAsset, StartsWith, StartsWithExplicitGlobalConsensus,
//     },
// };

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    // type Aliasers = Nothing;
    type AssetClaims = PolkadotXcm;
    type AssetExchanger = ();
    type AssetLocker = ();
    type AssetTransactor = AssetTransactors;
    type AssetTrap = PolkadotXcm;
    type Barrier = Barrier;
    type CallDispatcher = RuntimeCall;
    type FeeManager = ();
    type IsReserve = NativeAsset;
    type IsTeleporter = (
        NativeAsset,
        // IsForeignConcreteAsset<FromSiblingParachain<parachain_info::Pallet<Runtime>>>,
    );
    type MaxAssetsIntoHolding = ConstU32<8>;
    type MessageExporter = ();
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type ResponseHandler = PolkadotXcm;
    type RuntimeCall = RuntimeCall;
    type SafeCallFilter = Everything;
    type SubscriptionService = PolkadotXcm;
    type Trader = UsingComponents<WeightToFee, RelayLocation, AccountId, Balances, ()>;
    type UniversalAliases = Nothing;
    type UniversalLocation = UniversalLocation;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type XcmSender = XcmRouter;
}

pub type XcmRouter = (
    // Two routers - use UMP to communicate with the relay chain:
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
    // ..and XCMP to communicate with the sibling chains.
    XcmpQueue,
);

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
// pub type XcmRouter = WithUniqueTopic<(
//     // Two routers - use UMP to communicate with the relay chain:
//     cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
//     // ..and XCMP to communicate with the sibling chains.
//     XcmpQueue,
// )>;

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
    pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
}

impl pallet_xcm::Config for Runtime {
    // type AdminOrigin = EnsureRoot<AccountId>;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    // We support local origins dispatching XCM executions in principle...
    type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type MaxLockers = ConstU32<8>;
    // type MaxRemoteLockConsumers = ConstU32<0>;
    #[cfg(feature = "runtime-benchmarks")]
    type ReachableDest = ReachableDest;
    // type RemoteLockConsumerIdentifier = ();
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    // We want to disallow users sending (arbitrary) XCMs from this chain.
    type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, ()>;
    type SovereignAccountOf = LocationToAccountId;
    type TrustedLockers = ();
    type UniversalLocation = UniversalLocation;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    // FIXME: Replace with benchmarked weight info
    type WeightInfo = pallet_xcm::TestWeightInfo;
    // ... but disallow generic XCM execution. As a result only teleports and reserve transfers are allowed.
    type XcmExecuteFilter = Nothing;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmReserveTransferFilter = Everything;
    type XcmRouter = XcmRouter;
    type XcmTeleportFilter = Everything;

    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type ChannelInfo = ParachainSystem;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
    type PriceForSiblingDelivery = ();
    type RuntimeEvent = RuntimeEvent;
    type VersionWrapper = PolkadotXcm;
    type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Runtime>;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}
