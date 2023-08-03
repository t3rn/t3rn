use circuit_mock_runtime::{ExtBuilder, RuntimeOrigin as Origin, *};
use circuit_runtime_pallets::{
    pallet_asset_tx_payment::ChargeAssetTxPayment, pallet_balances::Call as BalancesCall,
};
use codec::{self, Encode};
use frame_support::{
    assert_ok,
    dispatch::{DispatchInfo, PostDispatchInfo},
    traits::fungibles::Mutate,
    weights::Weight,
};
use sp_runtime::{traits::SignedExtension, AccountId32};

pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([1u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([2u8; 32]);
pub const CALL: &<Runtime as frame_system::Config>::RuntimeCall =
    &RuntimeCall::Balances(BalancesCall::transfer {
        dest: sp_runtime::MultiAddress::Id(CHARLIE),
        value: 69,
    });

pub fn info_from_weight(w: Weight) -> DispatchInfo {
    // pays_fee: Pays::Yes -- class: DispatchClass::Normal
    DispatchInfo {
        weight: w,
        ..Default::default()
    }
}

fn setup_asset() -> u32 {
    let name = "AssetXyz";
    let symbol = "XYZ";
    let decimals = 12;
    let asset_id = 1_u32;
    let min_balance = 1;

    assert_ok!(Assets::force_create(
        Origin::root(),
        asset_id,
        sp_runtime::MultiAddress::Id(ALICE),
        true, /* is_sufficient */
        min_balance
    ));

    assert_ok!(Assets::set_metadata(
        Origin::signed(ALICE),
        asset_id,
        name.encode(),
        symbol.encode(),
        decimals,
    ));

    asset_id
}

#[test]
fn transaction_payment_in_asset_possible() {
    ExtBuilder::default().build().execute_with(|| {
        let asset_id = setup_asset();
        let caller = BOB;
        let weight = 5_u64;
        let len = 10;
        let initial_balance = 10_000_000_000_000;
        let native_balance = Balances::free_balance(caller.clone());

        // mint into the caller account
        assert_ok!(Assets::mint_into(asset_id, &caller, initial_balance));
        assert_eq!(Assets::balance(asset_id, caller.clone()), initial_balance);

        // charge a bogus transfer call
        let pre = ChargeAssetTxPayment::<Runtime>::from(0, Some(asset_id))
            .pre_dispatch(&caller, CALL, &info_from_weight(weight.into()), len)
            .expect("asset transaction payment");

        // assert that native balance is not used
        assert_eq!(Balances::free_balance(caller.clone()), native_balance);

        // check that fee was charged in the given asset
        let asset_balance = Assets::balance(asset_id, caller.clone());
        assert!(asset_balance < initial_balance);

        assert_ok!(ChargeAssetTxPayment::<Runtime>::post_dispatch(
            Some(pre),
            &info_from_weight(weight.into()),
            &PostDispatchInfo {
                actual_weight: None,
                pays_fee: Default::default(),
            },
            len,
            &Ok(())
        ));

        assert_eq!(Assets::balance(asset_id, caller), asset_balance);
    });
}

#[test]
fn transaction_payment_in_asset_fails_if_insufficient_balance() {
    ExtBuilder::default().build().execute_with(|| {
        let asset_id = setup_asset();
        let caller = BOB;

        // try charge transaction fee in asset with no prior mint
        assert_eq!(Assets::balance(asset_id, caller.clone()), 0);
        assert!(ChargeAssetTxPayment::<Runtime>::from(0, Some(asset_id))
            .pre_dispatch(&caller, CALL, &info_from_weight(5_u64.into()), 10)
            .is_err());
    });
}
