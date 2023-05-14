use crate::replay::*;
use circuit_mock_runtime::{ExtBuilder, *};
use codec::Encode;
use frame_support::{assert_ok, dispatch::DispatchErrorWithPostInfo};
use hex_literal::hex;
use sp_runtime::DispatchError;

#[test]
fn advance_block_works_correctly() {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| {
        assert_eq!(System::block_number(), 1);
        advance_to_block(Some(10));
        assert_eq!(System::block_number(), 10);
        assert_eq!(System::events().len(), 0);
        advance_to_block(None);
        assert_eq!(System::block_number(), 10); // only cleared events
        assert_eq!(System::events().len(), 0);
    });
}

#[test]
fn decodes_speed_mode_correctly() {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| {
        let fast = hex!("00").to_vec();
        let rational = hex!("01").to_vec();

        assert_eq!(decode_speed_mode(&fast).unwrap(), SpeedMode::Fast);
        assert_eq!(decode_speed_mode(&rational).unwrap(), SpeedMode::Rational);
    })
}

#[test]
fn decodes_side_effects_correctly() {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| {
        let sfx = hex!("04726f636f0080ca3961240000000000000000000000e876481700000000000000000000007472616e0880fc68ae55f42dcfd8060f1f67ec3c68a7dc3bce702f1ddb3d3551baf4e52f1a7d4000e40b54020000000000000000000000000000").to_vec();

        assert_eq!(decode_side_effect(&sfx).unwrap(), vec![SideEffect {
            target: *b"roco",
            max_reward: 40000000000000,
            insurance: 100000000000,
            action: *b"tran",
            encoded_args: vec![hex!("fc68ae55f42dcfd8060f1f67ec3c68a7dc3bce702f1ddb3d3551baf4e52f1a7d").to_vec(),hex!("00e40b54020000000000000000000000").to_vec()],
            signature: vec![],
            enforce_executor: None,
            reward_asset_id: None,
        }]);
    })
}

#[test]
fn handles_error_verification_correctly() {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| {
        let error = DispatchError::BadOrigin;
        let mut param = ExtrinsicParam {
            signer: "".to_string(),
            section: "".to_string(),
            method: "".to_string(),
            args: vec![],
            submission_height: None,
            events: vec![],
            error: error.clone().encode(),
        };

        // Dispatch works
        assert_ok!(verify_extrinsic_error(
            ErrorWrapper::Dispatch(error),
            &param
        ));

        let _error = DispatchErrorWithPostInfo {
            error: DispatchError::BadOrigin,
            post_info: Default::default(),
        };

        let error = DispatchError::BadOrigin;
        // DispatchWithPostInfo works
        assert_ok!(verify_extrinsic_error(
            ErrorWrapper::Dispatch(error),
            &param
        ));

        param.error = DispatchError::BadOrigin.encode();
        let error = DispatchError::ConsumerRemaining;

        // Mismatching errors fails
        assert!(verify_extrinsic_error(ErrorWrapper::Dispatch(error), &param).is_err());
    })
}
