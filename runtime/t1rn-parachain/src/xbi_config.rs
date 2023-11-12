use crate::{
    AccountId, AllPalletsWithSystem, AssetRegistry, Assets, Balance, Balances, DmpQueue,
    ParachainInfo, ParachainSystem, PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
    WeightToFee, XcmpQueue, MAXIMUM_BLOCK_WEIGHT,
};
use circuit_runtime_types::default_fee_per_second;
use cumulus_primitives_core::ParaId;

use cumulus_primitives_core::GetChannelInfo;
use frame_support::{
    match_types, parameter_types,
    traits::{ConstU32, Contains, ContainsPair, Currency, Everything, Get, Nothing, OnUnbalanced},
    weights::Weight,
};
use frame_system::EnsureRoot;
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;

use sp_std::{marker::PhantomData, vec::Vec};
use xcm::latest::prelude::*;

use parachains_common::AssetIdForTrustBackedAssets;
use xcm_builder::{
    AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
    AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, CurrencyAdapter, EnsureXcmOrigin,
    FixedRateOfFungible, FixedWeightBounds, FungiblesAdapter, IsConcrete, NativeAsset, NoChecking,
    ParentAsSuperuser, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
    SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
    SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId, UsingComponents,
    WithComputedOrigin,
};

use xcm_executor::{traits::JustTry, XcmExecutor};

use xcm_primitives::{AsAssetMultiLocation, ConvertedRegisteredAssetId};

parameter_types! {
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
    pub const XbiSovereign: AccountId = AccountId::new([68u8; 32]); // 0x444...4
}

parameter_types! {
    pub ReserveBalanceCustodian: AccountId = PolkadotXcm::check_account();
    pub NotificationWeight: Weight = Weight::from_parts(1, 0u64);
}

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
//     type Evm = Evm;
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
    pub UniversalLocation: InteriorMultiLocation = (
        GlobalConsensus(NetworkId::Rococo),
        Parachain(ParachainInfo::parachain_id().into()),
    ).into();
    pub CheckingAccount: AccountId = PolkadotXcm::check_account();
    pub AssetsPalletLocation: MultiLocation =
        PalletInstance(12u8).into();
    pub PlaceholderAccount: AccountId = PolkadotXcm::check_account();
}

pub type SovereignAccountOf = (
    SiblingParachainConvertsVia<ParaId, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);

parameter_types! {
    pub MaxAssetsIntoHolding: u32 = 64;
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

pub type TrustBackedAssetsConvertedConcreteId =
    assets_common::TrustBackedAssetsConvertedConcreteId<AssetsPalletLocation, Balance>;

/// Means for transacting assets besides the native currency on this chain.
pub type FungiblesTransactor = FungiblesAdapter<
    // Use this fungibles implementation:
    Assets,
    // Use this currency when it is a registered fungible asset matching the given location or name
    // Assets not found in AssetRegistry will not be used
    ConvertedRegisteredAssetId<
        AssetIdForTrustBackedAssets,
        Balance,
        AsAssetMultiLocation<AssetIdForTrustBackedAssets, AssetRegistry>,
        JustTry,
    >,
    // Convert an XCM MultiLocation into a local account id:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We don't track any teleports of `Assets`.
    NoChecking,
    // We don't track any teleports of `Assets`, but a placeholder account is provided due to trait
    // bounds.
    PlaceholderAccount,
>;

pub type AssetTransactors = (LocalAssetTransactor, FungiblesTransactor);
// pub type AssetTransactors = LocalAssetTransactor;

match_types! {
    pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
        MultiLocation { parents: 1, interior: Here } |
        MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
    };
}

// FIXME: should be using asset_registry
pub type Barrier = TrailingSetTopicAsId<(
    TakeWeightCredit,
    // Expected responses are OK.
    AllowKnownQueryResponses<PolkadotXcm>,
    // Allow XCMs with some computed origins to pass through.
    WithComputedOrigin<
        (
            // If the message is one that immediately attemps to pay for execution, then
            // allow it.
            AllowTopLevelPaidExecutionFrom<Everything>,
            // Parent, its pluralities (i.e. governance bodies), and the Fellows plurality
            // get free execution.
            AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
            // Subscriptions for version tracking are OK.
            AllowSubscriptionsFrom<ParentOrSiblings>,
        ),
        UniversalLocation,
        ConstU32<8>,
    >,
)>;

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

parameter_types! {
    pub AssetHubLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(1000)));
    pub const Native: MultiAssetFilter = Wild(AllOf { fun: WildFungible, id: Concrete(MultiLocation::here()) });
    pub AssetHubTrustedTeleporter: (MultiAssetFilter, MultiLocation) = (Native::get(), AssetHubLocation::get());
    pub RUsdtPerSecond: (xcm::v3::AssetId, u128, u128) = (
        MultiLocation::new(1, X3(Parachain(1000), PalletInstance(50), GeneralIndex(1984))).into(),
        default_fee_per_second() * 10,
        0u128
    );
    /// Roc = 7 Rococo USDT
    pub RocPerSecond: (xcm::v3::AssetId, u128,u128) = (MultiLocation::new(1,Here).into(), default_fee_per_second() * 70, 0u128);
    pub SelfReserve: MultiLocation = MultiLocation { parents:0, interior: Here };
}

pub struct ReserveAssetsFrom<T>(PhantomData<T>);
impl<T: Get<MultiLocation>> ContainsPair<MultiAsset, MultiLocation> for ReserveAssetsFrom<T> {
    fn contains(asset: &MultiAsset, origin: &MultiLocation) -> bool {
        let prefix = T::get();
        log::trace!(target: "xcm::AssetsFrom", "prefix: {:?}, origin: {:?}, asset: {:?}", prefix, origin, asset);
        &prefix == origin
    }
}

pub struct OnlyTeleportNative;
impl Contains<(MultiLocation, Vec<MultiAsset>)> for OnlyTeleportNative {
    fn contains(t: &(MultiLocation, Vec<MultiAsset>)) -> bool {
        t.1.iter().any(|asset| {
            log::trace!(target: "xcm::OnlyTeleportNative", "Asset to be teleported: {:?}", asset);

            if let MultiAsset {
                id: xcm::latest::AssetId::Concrete(asset_loc),
                fun: Fungible(_a),
            } = asset
            {
                match asset_loc {
                    MultiLocation {
                        parents: 0,
                        interior: Here,
                    } => true,
                    _ => false,
                }
            } else {
                false
            }
        })
    }
}

/// Type alias to conveniently refer to the `Currency::NegativeImbalance` associated type.
pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// Type alias to conveniently refer to `frame_system`'s `Config::AccountId`.
pub type AccountIdOf<R> = <R as frame_system::Config>::AccountId;

/// Logic for the author to get a portion of fees.
pub struct ToAuthor<R>(PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAuthor<R>
where
    R: pallet_balances::Config + pallet_collator_selection::Config,
    AccountIdOf<R>: From<polkadot_core_primitives::v2::AccountId>
        + Into<polkadot_core_primitives::v2::AccountId>,
    <R as frame_system::Config>::RuntimeEvent: From<pallet_balances::Event<R>>,
{
    fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
        let author = <pallet_collator_selection::Pallet<R>>::account_id();
        <pallet_balances::Pallet<R>>::resolve_creating(&author, amount);
    }
}

pub type Traders = (
    // Rococo USDT
    FixedRateOfFungible<RUsdtPerSecond, ()>,
    // ROC
    FixedRateOfFungible<RocPerSecond, ()>,
    // Everything else
    UsingComponents<WeightToFee, SelfReserve, AccountId, Balances, ToAuthor<Runtime>>,
);
pub type Reserves = (NativeAsset, ReserveAssetsFrom<AssetHubLocation>);
pub type TrustedTeleporters = (xcm_builder::Case<AssetHubTrustedTeleporter>,);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type Aliasers = Nothing;
    type AssetClaims = PolkadotXcm;
    type AssetExchanger = ();
    type AssetLocker = ();
    type AssetTransactor = AssetTransactors;
    type AssetTrap = PolkadotXcm;
    type Barrier = Barrier;
    type CallDispatcher = RuntimeCall;
    type FeeManager = ();
    type IsReserve = Reserves;
    type IsTeleporter = TrustedTeleporters;
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type MessageExporter = ();
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type ResponseHandler = PolkadotXcm;
    type RuntimeCall = RuntimeCall;
    type SafeCallFilter = Everything;
    type SubscriptionService = PolkadotXcm;
    type Trader = Traders;
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
    type AdminOrigin = EnsureRoot<crate::AccountId>;
    // type AdminOrigin = EnsureRoot<AccountId>;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    // We support local origins dispatching XCM executions in principle...
    type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type MaxLockers = ConstU32<8>;
    type MaxRemoteLockConsumers = frame_support::traits::ConstU32<0>;
    // type MaxRemoteLockConsumers = ConstU32<0>;
    #[cfg(feature = "runtime-benchmarks")]
    type ReachableDest = ReachableDest;
    type RemoteLockConsumerIdentifier = ();
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
    type XcmTeleportFilter = OnlyTeleportNative;

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
