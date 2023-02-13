use crate::mock::*;
use codec::{Encode, MaxEncodedLen};
use frame_support::{storage::bounded_vec::BoundedVec, traits::ConstU32};
use sp_core::crypto::AccountId32;
use t3rn_sdk_primitives::xc::*;

#[test]
fn dummy_integration_test() {
    new_test_ext().execute_with(|| {});
}

#[test]
fn test_substrate_boundec_vec_compatibility() {
    let example_vector = vec![Some(1_u8), Some(2_u8), Some(3_u8)];
    let x = BoundedVec::<_, ConstU32<3>>::try_from(example_vector.clone()).unwrap();
    let y = t3rn_sdk_primitives::storage::BoundedVec::<_, 3>::from_iter(example_vector);
    assert_eq!(x.encode(), y.encode());

    let example_vector = vec![
        Chain::<_, _, [u8; 32]>::Polkadot(Operation::Transfer {
            caller: ALICE,
            to: CHARLIE,
            amount: 50,
            insurance: None,
        }),
        Chain::<_, _, [u8; 32]>::Kusama(Operation::Swap {
            caller: ALICE,
            to: CHARLIE,
            amount_from: 50,
            amount_to: 50,
            asset_from: [50_u8; 32],
            asset_to: [50_u8; 32],
            insurance: None,
        }),
    ];
    let x = BoundedVec::<_, ConstU32<3>>::try_from(example_vector.clone()).unwrap();
    let y = t3rn_sdk_primitives::storage::BoundedVec::<_, 3>::from_iter(example_vector);
    assert_eq!(x.encode(), y.encode());

    assert_eq!(
        <BoundedVec::<Chain::<AccountId32, u128, [u8; 32]>, ConstU32<3>> as MaxEncodedLen>::max_encoded_len(),
        <t3rn_sdk_primitives::storage::BoundedVec::<Chain::<AccountId32, u128, [u8; 32]>, 3> as MaxEncodedLen>::max_encoded_len()
    );
}
