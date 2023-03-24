// This file contains all the migrations from old versions of the runtime to the latest version

// Storage Migration: FSX::SFX updates field "encoded_action: Vec<u8>" to "action: Action: [u8; 4]"
// Storage Migration Details: 16-03-2023; v1.3.0-rc -> v1.4.0-rc
pub mod v13 {
    pub use crate::{
        bid::SFXBid,
        sfx::{
            ConfirmationOutcome, ConfirmedSideEffect, Error, EventSignature, HardenedSideEffect,
            SecurityLvl, SideEffect, SideEffectName, TargetId,
        },
    };
    use crate::{fsx::FullSideEffect, types::Bytes};
    use codec::{Decode, Encode};
    use scale_info::TypeInfo;
    use sp_runtime::RuntimeDebug;
    use sp_std::prelude::*;

    // Deprecated versions of SideEffects and migrations to the latest SideEffect version
    #[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub struct SideEffectV13<AccountId, BalanceOf> {
        pub target: [u8; 4],
        pub max_reward: BalanceOf,
        pub insurance: BalanceOf,
        pub encoded_action: Vec<u8>,
        pub encoded_args: Vec<Bytes>,
        pub signature: Bytes,
        pub enforce_executor: Option<AccountId>,
        pub reward_asset_id: Option<u32>,
    }

    impl<AccountId, BalanceOf> From<SideEffectV13<AccountId, BalanceOf>>
        for SideEffect<AccountId, BalanceOf>
    {
        fn from(old: SideEffectV13<AccountId, BalanceOf>) -> Self {
            SideEffect {
                target: old.target,
                max_reward: old.max_reward,
                insurance: old.insurance,
                action: [
                    old.encoded_action[0],
                    old.encoded_action[1],
                    old.encoded_action[2],
                    old.encoded_action[3],
                ], // Assuming the first 4 bytes in `encoded_action` represent the `Sfx4bId`
                encoded_args: old.encoded_args,
                signature: old.signature,
                enforce_executor: old.enforce_executor,
                reward_asset_id: old.reward_asset_id,
            }
        }
    }

    #[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub struct FullSideEffectV13<AccountId, BlockNumber, BalanceOf> {
        pub input: SideEffectV13<AccountId, BalanceOf>,
        pub confirmed: Option<ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf>>,
        pub security_lvl: SecurityLvl,
        pub submission_target_height: Bytes,
        pub best_bid: Option<SFXBid<AccountId, BalanceOf, u32>>,
        pub index: u32,
    }

    impl<AccountId, BlockNumber: Encode + Clone + Decode + Default, BalanceOf>
        From<FullSideEffectV13<AccountId, BlockNumber, BalanceOf>>
        for FullSideEffect<AccountId, BlockNumber, BalanceOf>
    {
        fn from(old: FullSideEffectV13<AccountId, BlockNumber, BalanceOf>) -> Self {
            FullSideEffect {
                input: SideEffect::from(old.input),
                confirmed: old.confirmed,
                security_lvl: old.security_lvl,
                submission_target_height: BlockNumber::decode(
                    &mut &old.submission_target_height[..],
                )
                .unwrap_or_default(),
                best_bid: old.best_bid,
                index: old.index,
            }
        }
    }
}
