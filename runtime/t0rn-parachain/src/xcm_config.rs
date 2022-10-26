use crate::{
    parachain_config::WeightToFee, utility::ChargeWeightInFungibles, AccountId, Assets, Authorship,
    Balance, Balances, Call, Event, Origin, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime,
    WeightToFeePolynomial, XcmpQueue,
};
use core::borrow::Borrow;
use frame_support::{
    match_types, parameter_types,
    traits::{
        tokens::{fungibles, fungibles::Inspect, BalanceConversion},
        Contains, Everything, Get, PalletInfoAccess,
    },
    weights::{Weight, WeightToFee as WeightToFeeExt},
};
use orml_traits::location::{RelativeReserveProvider, Reserve};
use orml_xcm_support::MultiNativeAsset;
use pallet_xcm::XcmPassthrough;
use parachains_common::xcm_config::{DenyReserveTransferToRelayChain, DenyThenTry};
use polkadot_parachain::primitives::Sibling;
use sp_runtime::traits::{ConvertInto, Zero};
use sp_std::marker::PhantomData;
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
    AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, AsPrefixedGeneralIndex, Case,
    ConvertedConcreteAssetId, CurrencyAdapter, EnsureXcmOrigin, FixedWeightBounds,
    FungiblesAdapter, IsConcrete, LocationInverter, NativeAsset, ParentIsPreset,
    RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
    SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
    UsingComponents,
};
use xcm_executor::{
    traits::{Convert, JustTry},
    XcmExecutor,
};

parameter_types! {
    pub const RelayLocation: MultiLocation = MultiLocation::parent();
    pub const T3rnLocation: MultiLocation = MultiLocation::here();
    pub const RelayNetwork: NetworkId = NetworkId::Any;
    pub RelayChainOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
    pub Ancestry: Junction = Parachain(ParachainInfo::parachain_id().into());


    pub RelativeAssetsPalletLocation: Junction =
        PalletInstance(<Assets as PalletInfoAccess>::index() as u8);

    pub NativeAssetPalletLocation: Junction =
        PalletInstance(<Balances as PalletInfoAccess>::index() as u8);

    // Self Reserve location, defines the multilocation identifiying the self-reserve currency
    // This is used to match it also against our Balances pallet when we receive such
    // a MultiLocation: (Self Balances pallet index)
    // We use the ABSOLUTE multilocation
    pub NativeAssetLocationAbsolute: MultiLocation = MultiLocation {
        parents: 1,
        interior: Junctions::X2(Ancestry::get(), NativeAssetPalletLocation::get())
    };

    // Assets pallet location, defines the multilocation where we have t3rn assets
    // This is used to match it also against our Balances pallet when we receive such
    // a MultiLocation: (Self Assets pallet index)
    // We use the ABSOLUTE multilocation
    pub AssetsPalletLocationAbsolute: MultiLocation = MultiLocation {
        parents: 1,
        interior: Junctions::X2(Ancestry::get(), RelativeAssetsPalletLocation::get())
    };

    pub CheckingAccount: AccountId = PolkadotXcm::check_account();

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

// Transacting native asset
pub type NativeCurrencyTransactor = CurrencyAdapter<
    // Use this currency:
    Balances,
    // Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<NativeAssetLocationAbsolute>,
    // Convert an XCM MultiLocation into a local account id:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We don't track any teleports of Native Assets.
    (),
>;

type FungibleAssetId = u32;

/// Means for transacting assets besides the native currency on this chain.
pub type FungiblesTransactor = FungiblesAdapter<
    // Use this fungibles implementation:
    Assets,
    // Use this currency when it is a fungible asset matching the given location or name:
    ConvertedConcreteAssetId<
        FungibleAssetId,
        Balance,
        //        AsPrefixedGeneralIndex<AssetsPalletLocation, FungibleAssetId, JustTry>, TODO: fix
        OneForOneAssetId<AssetsPalletLocationAbsolute, FungibleAssetId, JustTry>, // TODO: only supports assets
        JustTry,
    >,
    // Convert an XCM MultiLocation into a local account id:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We only want to allow teleports of known assets. We use non-zero issuance as an indication
    // that this asset is known.
    NonZeroIssuance<AccountId, Assets>,
    // The account to use for tracking teleports.
    CheckingAccount,
>;

pub type AssetTransactors = (NativeCurrencyTransactor, FungiblesTransactor);

pub struct NonZeroIssuance<AccountId, Assets>(PhantomData<(AccountId, Assets)>);
impl<AccountId, Assets> Contains<<Assets as fungibles::Inspect<AccountId>>::AssetId>
    for NonZeroIssuance<AccountId, Assets>
where
    Assets: fungibles::Inspect<AccountId>,
{
    fn contains(id: &<Assets as fungibles::Inspect<AccountId>>::AssetId) -> bool {
        !Assets::total_issuance(*id).is_zero()
    }
}

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<LocationToAccountId, Origin>,
    // Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
    // recognized.
    RelayChainAsNative<RelayChainOrigin, Origin>,
    // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
    // recognized.
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
    // Native signed account converter; this just converts an `AccountId32` origin into a normal
    // `Origin::Signed` origin of the same 32-byte value.
    SignedAccountId32AsNative<RelayNetwork, Origin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<Origin>,
);

parameter_types! {
    // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
    pub UnitWeightCost: Weight = 1_000_000_000;
    pub const MaxInstructions: u32 = 100;
    pub SelfLocationAbsolute: MultiLocation = MultiLocation {
        parents: 1,
        interior: Junctions::X1(
            Parachain(ParachainInfo::parachain_id().into())
        )
    };
    pub XcmAssetFeesReceiver: Option<AccountId> = Authorship::author();
}

match_types! {
    pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
        MultiLocation { parents: 1, interior: Here } |
        MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
    };
}

pub type Barrier = DenyThenTry<
    DenyReserveTransferToRelayChain,
    (
        TakeWeightCredit,
        AllowTopLevelPaidExecutionFrom<Everything>,
        AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
        // ^^^ Parent and its exec plurality get free execution
        AllowKnownQueryResponses<PolkadotXcm>,
        AllowSubscriptionsFrom<Everything>,
    ),
>;

// Copyright 2019-2022 PureStake Inc.
// This function is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

/// This struct offers uses RelativeReserveProvider to output relative views of multilocations
/// However, additionally accepts a MultiLocation that aims at representing the chain part
/// (parent: 1, Parachain(paraId)) of the absolute representation of our chain.
/// If a token reserve matches against this absolute view, we return  Some(MultiLocation::here())
/// This helps users by preventing errors when they try to transfer a token through xtokens
/// to our chain (either inserting the relative or the absolute value).
pub struct AbsoluteAndRelativeReserve<AbsoluteMultiLocation>(PhantomData<AbsoluteMultiLocation>);
impl<AbsoluteMultiLocation> Reserve for AbsoluteAndRelativeReserve<AbsoluteMultiLocation>
where
    AbsoluteMultiLocation: Get<MultiLocation>,
{
    fn reserve(asset: &MultiAsset) -> Option<MultiLocation> {
        RelativeReserveProvider::reserve(asset).map(|relative_reserve| {
            if relative_reserve == AbsoluteMultiLocation::get() {
                MultiLocation::here()
            } else {
                relative_reserve
            }
        })
    }
}

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// A `ChargeFeeInFungibles` implementation that converts the output of
/// a given WeightToFee implementation an amount charged in
/// a particular assetId from pallet-assets
pub struct AssetFeeAsExistentialDepositMultiplier<Runtime, WeightToFee, BalanceConverter>(
    PhantomData<(Runtime, WeightToFee, BalanceConverter)>,
);
impl<CurrencyBalance, Runtime, WeightToFee, BalanceConverter>
    crate::utility::ChargeWeightInFungibles<AccountIdOf<Runtime>, pallet_assets::Pallet<Runtime>>
    for AssetFeeAsExistentialDepositMultiplier<Runtime, WeightToFee, BalanceConverter>
where
    Runtime: pallet_assets::Config,
    WeightToFee: WeightToFeePolynomial<Balance = CurrencyBalance>,
    BalanceConverter: BalanceConversion<
        CurrencyBalance,
        <Runtime as pallet_assets::Config>::AssetId,
        <Runtime as pallet_assets::Config>::Balance,
    >,
    AccountIdOf<Runtime>: From<crate::AccountId> + Into<crate::AccountId>,
{
    fn charge_weight_in_fungibles(
        asset_id: <pallet_assets::Pallet<Runtime> as Inspect<AccountIdOf<Runtime>>>::AssetId,
        weight: Weight,
    ) -> Result<<pallet_assets::Pallet<Runtime> as Inspect<AccountIdOf<Runtime>>>::Balance, XcmError>
    {
        let amount = WeightToFee::weight_to_fee(&weight);
        // If the amount gotten is not at least the ED, then make it be the ED of the asset
        // This is to avoid burning assets and decreasing the supply
        let asset_amount = BalanceConverter::to_asset_balance(amount, asset_id)
            .map_err(|_| XcmError::TooExpensive)?;
        Ok(asset_amount)
    }
}

// Supports a one for one mapping from the last GeneralIndex to an AssetId, this means the sender can have duplicate GeneralIndexes for non-last in case of collissions
pub struct OneForOneAssetId<Prefix, AssetId, ConvertAssetId>(
    PhantomData<(Prefix, AssetId, ConvertAssetId)>,
);
impl<Prefix: Get<MultiLocation>, AssetId: Clone, ConvertAssetId: Convert<u128, AssetId>>
    Convert<MultiLocation, AssetId> for OneForOneAssetId<Prefix, AssetId, ConvertAssetId>
{
    // Extract the id from the index, ignoring the prefix
    fn convert_ref(id: impl Borrow<MultiLocation>) -> Result<AssetId, ()> {
        match id.borrow().last() {
            Some(Junction::GeneralIndex(id)) => ConvertAssetId::convert_ref(id),
            x => {
                log::debug!(target: "xcm::weight", "Failed Checking last id is a general index {:?}", x);
                Err(())
            },
        }
    }

    // push the asset back onto the prefix
    fn reverse_ref(what: impl Borrow<AssetId>) -> Result<MultiLocation, ()> {
        let mut location = Prefix::get();
        let id = ConvertAssetId::reverse_ref(what)?;
        location
            .push_interior(Junction::GeneralIndex(id))
            .map_err(|_| ())?;
        Ok(location)
    }
}

parameter_types! {
    pub FromSiblingCase: (MultiAssetFilter, MultiLocation) = (MultiAssetFilter::Wild(WildMultiAsset::All), MultiLocation {
        parents:1,
        interior: Junctions::X1(Parachain(2))
    });
}
pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type AssetClaims = PolkadotXcm;
    // How to withdraw and deposit an asset.
    type AssetTransactor = AssetTransactors;
    type AssetTrap = PolkadotXcm;
    type Barrier = Barrier;
    type Call = Call;
    // We are our only reserve
    type IsReserve = MultiNativeAsset<AbsoluteAndRelativeReserve<NativeAssetLocationAbsolute>>;
    type IsTeleporter = (NativeAsset, Case<FromSiblingCase>);
    type LocationInverter = LocationInverter<Ancestry>;
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type ResponseHandler = PolkadotXcm;
    type SubscriptionService = PolkadotXcm;
    // TODO: these traders need very careful attention
    // we probably want to introduce an asset manager which covers these and avoids
    // algorithmic lookups
    type Trader = (
        // Trade native currency
        UsingComponents<WeightToFee, T3rnLocation, AccountId, Balances, ()>,
        // Trade native absolute
        UsingComponents<WeightToFee, NativeAssetLocationAbsolute, AccountId, Balances, ()>,
        crate::utility::TakeFirstAssetTrader<
            AccountId,
            AssetFeeAsExistentialDepositMultiplier<
                Runtime,
                WeightToFee,
                pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto>,
            >,
            ConvertedConcreteAssetId<
                FungibleAssetId,
                Balance,
                OneForOneAssetId<AssetsPalletLocationAbsolute, FungibleAssetId, JustTry>, // TODO: this should just have some mapping register, like bsx,astar
                JustTry,
            >,
            Assets,
            crate::utility::XcmFeesTo32ByteAccount<
                FungiblesTransactor,
                AccountId,
                XcmAssetFeesReceiver,
            >,
        >,
    );
    type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
    type XcmSender = XcmRouter;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
    // Two routers - use UMP to communicate with the relay chain:
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem, ()>,
    // ..and XCMP to communicate with the sibling chains.
    XcmpQueue,
);

impl pallet_xcm::Config for Runtime {
    // ^ Override for AdvertisedXcmVersion default
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Call = Call;
    type Event = Event;
    type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
    type LocationInverter = LocationInverter<Ancestry>;
    type Origin = Origin;
    type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
    type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
    type XcmExecuteFilter = Everything;
    // ^ Disable dispatchable execute on the XCM pallet.
    // Needs to be `Everything` for local testing.
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmReserveTransferFilter = Everything;
    type XcmRouter = XcmRouter;
    type XcmTeleportFilter = Everything;

    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
}

impl cumulus_pallet_xcm::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}
