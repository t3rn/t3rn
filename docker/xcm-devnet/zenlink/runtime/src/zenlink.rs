// Copyright 2020-2021 Zenlink
// Licensed under GPL-3.0.

use sp_std::{convert::TryInto, marker::PhantomData};

use frame_support::dispatch::{DispatchError, DispatchResult};
use orml_traits::MultiCurrency;

use super::{
	parameter_types, vec, AccountId, AccountId32, AccountId32Aliases, Balances, Currencies, Event, Get, MultiLocation,
	NetworkId, PalletId, Parachain, ParachainInfo, Runtime, ShouldExecute, Sibling, SiblingParachainConvertsVia, Vec,
	Weight, Xcm, XcmConfig, XcmExecutor, ZenlinkProtocol, X1,
};

pub use zenlink_protocol::{
	make_x2_location, AssetBalance, AssetId, LocalAssetHandler, MultiAssetsHandler, PairInfo, TransactorAdaptor,
	TrustedParas, ZenlinkMultiAssets, LIQUIDITY, LOCAL,
};

use dev_parachain_primitives::CurrencyId;

parameter_types! {
	pub const ZenlinkPalletId: PalletId = PalletId(*b"/zenlink");
	pub SelfParaId: u32 = ParachainInfo::get().into();

	// xcm
	pub const AnyNetwork: NetworkId = NetworkId::Any;
	pub ZenlinkRegistedParaChains: Vec<(MultiLocation, u128)> = vec![
		// Bifrost local and live, 0.01 BNC
		(make_x2_location(2001), 10_000_000_000),
		// Phala local and live, 1 PHA
		(make_x2_location(2004), 1_000_000_000_000),
		// Plasm local and live, 0.0000000000001 SDN
		(make_x2_location(2007), 1_000_000),
		// Sherpax live, 0 KSX
		(make_x2_location(2013), 0),

		// Zenlink local 1 for test
		(make_x2_location(200), 1_000_000),
		// Zenlink local 2 for test
		(make_x2_location(300), 1_000_000),
	];
}

pub struct ZenlinkAllowUnpaid<RegisteredChains>(PhantomData<RegisteredChains>);

impl<RegisteredChains> ShouldExecute for ZenlinkAllowUnpaid<RegisteredChains>
where
	RegisteredChains: Get<Vec<(MultiLocation, u128)>>,
{
	fn should_execute<Call>(
		origin: &MultiLocation,
		_top_level: bool,
		_message: &Xcm<Call>,
		_shallow_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		frame_support::log::info!("zenlink_protocol: ZenlinkAllowUnpaid = {:?}", origin);

		match &origin.interior {
			X1(AccountId32 { network, .. }) if *network == NetworkId::Any => Ok(()),
			X1(Parachain(id)) => {
				match RegisteredChains::get()
					.iter()
					.find(|(location, _)| *location == make_x2_location(*id))
				{
					Some(_) => Ok(()),
					None => Err(()),
				}
			}
			_ => Err(()),
		}
	}
}

pub type ZenlinkLocationToAccountId = (
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<AnyNetwork, AccountId>,
);

pub struct LocalAssetAdaptor<Local>(PhantomData<Local>);

// fn asset_id_to_currency_id(asset_id: AssetId) -> Result<CurrencyId, ()> {
// 	asset_id.try_into()
// }

impl<Local> LocalAssetHandler<AccountId> for LocalAssetAdaptor<Local>
where
	Local: MultiCurrency<AccountId, Balance = u128, CurrencyId = CurrencyId>,
{
	fn local_balance_of(asset_id: AssetId, who: &AccountId) -> AssetBalance {
		asset_id.try_into().map_or(AssetBalance::default(), |currency_id| {
			Local::free_balance(currency_id, who)
		})
	}

	fn local_total_supply(asset_id: AssetId) -> AssetBalance {
		asset_id.try_into().map_or(AssetBalance::default(), |currency_id| {
			Local::total_issuance(currency_id)
		})
	}

	fn local_is_exists(asset_id: AssetId) -> bool {
		asset_id.try_into().map_or(false, |currency_id| {
			Local::total_issuance(currency_id) > AssetBalance::default()
		})
	}

	fn local_transfer(
		asset_id: AssetId,
		origin: &AccountId,
		target: &AccountId,
		amount: AssetBalance,
	) -> DispatchResult {
		asset_id
			.try_into()
			.map_or(Err(DispatchError::CannotLookup), |currency_id| {
				Local::transfer(currency_id, origin, target, amount)
			})
	}

	fn local_deposit(
		asset_id: AssetId,
		origin: &AccountId,
		amount: AssetBalance,
	) -> Result<AssetBalance, DispatchError> {
		asset_id.try_into().map_or(Ok(AssetBalance::default()), |currency_id| {
			Local::deposit(currency_id, origin, amount).map(|_| amount)
		})
	}

	fn local_withdraw(
		asset_id: AssetId,
		origin: &AccountId,
		amount: AssetBalance,
	) -> Result<AssetBalance, DispatchError> {
		asset_id.try_into().map_or(Ok(AssetBalance::default()), |currency_id| {
			Local::withdraw(currency_id, origin, amount).map(|_| amount)
		})
	}
}

pub type MultiAssets = ZenlinkMultiAssets<ZenlinkProtocol, Balances, LocalAssetAdaptor<Currencies>>;

impl zenlink_protocol::Config for Runtime {
	type Event = Event;
	type MultiAssetsHandler = MultiAssets;
	type PalletId = ZenlinkPalletId;
	type SelfParaId = SelfParaId;

	type TargetChains = ZenlinkRegistedParaChains;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Conversion = ZenlinkLocationToAccountId;
}
