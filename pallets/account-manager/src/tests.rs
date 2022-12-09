use crate::tests::pallet_asset_tx_payment::ChargeAssetTxPayment;
use circuit_mock_runtime::{ExtBuilder, *};
use codec::{self, Encode};
use frame_support::{
    assert_ok,
    dispatch::{DispatchInfo, PostDispatchInfo},
    traits::fungibles::Mutate,
    weights::Weight,
};
use pallet_balances::Call as BalancesCall;
// use pallet_transaction_payment::CurrencyAdapter;
use sp_runtime::{traits::SignedExtension, AccountId32};

pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([1u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([2u8; 32]);

pub fn info_from_weight(w: Weight) -> DispatchInfo {
    // pays_fee: Pays::Yes -- class: DispatchClass::Normal
    DispatchInfo {
        weight: w,
        ..Default::default()
    }
}

fn default_post_info() -> PostDispatchInfo {
    PostDispatchInfo {
        actual_weight: None,
        pays_fee: Default::default(),
    }
}

#[test]
fn transaction_payment_in_asset_possible() {
    ExtBuilder::default().build().execute_with(|| {
        // setup
        let name = "AssetXyz";
        let symbol = "XYZ";
        let decimals = 12;
        let asset_id = 1_u32;
        let min_balance = 1;
        let caller = BOB;
        let weight: u64 = 5;
        let len = 10;
        let initial_balance = 10_000_000_000_000;
        let native_balance = Balances::free_balance(caller.clone());

        // create asset
        assert_ok!(Assets::force_create(
            Origin::root(),
            asset_id.into(),
            sp_runtime::MultiAddress::Id(ALICE),
            true, /* is_sufficient */
            min_balance
        ));

        assert_ok!(Assets::set_metadata(
            Origin::signed(ALICE),
            asset_id.into(),
            name.encode(),
            symbol.encode(),
            decimals,
        ));

        // mint into the caller account
        assert_ok!(Assets::mint_into(asset_id.into(), &caller, initial_balance));
        assert_eq!(Assets::balance(asset_id, caller.clone()), initial_balance);

        // charge a bogus transfer call
        let call: &<Runtime as frame_system::Config>::Call =
            &Call::Balances(BalancesCall::transfer {
                dest: sp_runtime::MultiAddress::Id(CHARLIE),
                value: 69,
            });

        let pre = ChargeAssetTxPayment::<Runtime>::from(0, Some(asset_id))
            .pre_dispatch(&caller, call, &info_from_weight(Weight::from(weight)), len)
            .unwrap();

        // assert that native balance is not used
        assert_eq!(Balances::free_balance(caller.clone()), native_balance);

        // check that fee was charged in the given asset
        let asset_balance = Assets::balance(asset_id, caller.clone());
        assert!(asset_balance < initial_balance);

        assert_ok!(ChargeAssetTxPayment::<Runtime>::post_dispatch(
            Some(pre),
            &info_from_weight(Weight::from(weight)),
            &default_post_info(),
            len,
            &Ok(())
        ));

        assert_eq!(Assets::balance(asset_id, caller), asset_balance);
    });
}
