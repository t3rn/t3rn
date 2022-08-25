use crate::Bytes;
use codec::{Decode, Encode};
use num_traits::Zero;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{
    convert::{TryFrom, TryInto},
    vec,
};

pub use interface::*;
pub use t3rn_types::side_effect::{
    ConfirmationOutcome, ConfirmedSideEffect, Error, EventSignature,
    FullSideEffect as HardenedSideEffect, SecurityLvl, SideEffect, SideEffectName, TargetId,
    ADD_LIQUIDITY_SIDE_EFFECT_ID, ASSETS_TRANSFER_SIDE_EFFECT_ID, CALL_SIDE_EFFECT_ID,
    COMPOSABLE_CALL_SIDE_EFFECT_ID, DATA_SIDE_EFFECT_ID, EVM_CALL_SIDE_EFFECT_ID,
    ORML_TRANSFER_SIDE_EFFECT_ID, SWAP_SIDE_EFFECT_ID, TRANSFER_SIDE_EFFECT_ID,
    WASM_CALL_SIDE_EFFECT_ID,
};

#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

pub mod interface;
pub mod parser;

pub type SideEffectId<T> = <T as frame_system::Config>::Hash;

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct FullSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub input: SideEffect<AccountId, BlockNumber, BalanceOf>,
    pub confirmed: Option<ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf>>,
    pub security_lvl: SecurityLvl,
    pub submission_target_height: Bytes,
}

impl<AccountId, BlockNumber, BalanceOf>
    TryInto<HardenedSideEffect<AccountId, BlockNumber, BalanceOf>>
    for FullSideEffect<AccountId, BlockNumber, BalanceOf>
where
    AccountId: Encode + Clone,
    BlockNumber: Encode + Clone,
    BalanceOf: Encode + Zero + Clone,
{
    type Error = Error;

    fn try_into(
        self,
    ) -> Result<HardenedSideEffect<AccountId, BlockNumber, BalanceOf>, Self::Error> {
        let confirmation_outcome = self.clone().confirmed.and_then(|c| c.err.clone());
        let confirmed_executioner = self.clone().confirmed.map(|c| c.executioner.clone());
        let confirmed_received_at = self.clone().confirmed.map(|c| c.received_at.clone());
        let confirmed_cost = self.clone().confirmed.and_then(|c| c.cost);
        Ok(HardenedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: self.input.target,
            prize: self.input.prize,
            encoded_action: TargetId::try_from(self.input.encoded_action.clone())
                .unwrap_or_default(),
            encoded_args: self.input.encoded_args,
            encoded_args_abi: vec![],
            security_lvl: self.security_lvl,
            confirmation_outcome,
            confirmed_executioner,
            confirmed_received_at,
            confirmed_cost,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bridges::chain_circuit::{
        Balance as CircuitBalance, BlockNumber as CircuitBlockNumber,
    };
    use hex_literal::hex;
    use sp_core::crypto::AccountId32;
    use sp_runtime::testing::H256;
    use std::convert::TryInto;

    type BlockNumber = CircuitBlockNumber;
    type BalanceOf = CircuitBalance;
    type AccountId = AccountId32;
    type Hashing = sp_runtime::traits::BlakeTwo256;

    #[test]
    fn successfully_creates_empty_side_effect() {
        let empty_side_effect = SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(
            empty_side_effect,
            SideEffect {
                target: [0, 0, 0, 0],
                prize: 0,
                ordered_at: 0,
                encoded_action: vec![],
                encoded_args: vec![],
                signature: vec![],
                enforce_executioner: None,
            }
        );
    }

    #[test]
    fn successfully_encodes_transfer_full_side_effect_with_confirmation() {
        let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let optional_insurance = 2u128;
        let optional_reward = 3u128;

        let tsfx_input = SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![
                from.encode(),
                to.encode(),
                value.encode(),
                [optional_insurance.encode(), optional_reward.encode()].concat(),
            ],
            signature: vec![],
            enforce_executioner: None,
        };

        let tfsfx = FullSideEffect::<AccountId, BlockNumber, BalanceOf> {
            input: tsfx_input.clone(),
            security_lvl: SecurityLvl::Dirty,
            submission_target_height: vec![1, 0, 0, 0, 0, 0, 0, 0],
            confirmed: Some(ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
                err: Some(ConfirmationOutcome::Success),
                output: Some(vec![]),
                encoded_effect: vec![],
                inclusion_proof: None,
                executioner: from,
                received_at: 1u64 as BlockNumber,
                cost: Some(2u64 as BalanceOf),
            }),
        };

        let hsfx: HardenedSideEffect<AccountId, BlockNumber, BalanceOf> = tfsfx.try_into().unwrap();

        assert_eq!(
            hsfx,
            HardenedSideEffect {
                target: [0, 0, 0, 0],
                prize: 0,
                encoded_action: [0, 0, 0, 0],
                encoded_args: vec![
                    vec![
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1
                    ],
                    vec![
                        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                        2, 2, 2, 2, 2, 2, 2
                    ],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![
                        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ]
                ],
                encoded_args_abi: vec![],
                security_lvl: SecurityLvl::Dirty,
                confirmation_outcome: Some(ConfirmationOutcome::Success),
                confirmed_executioner: Some(AccountId32::new(
                    hex!("0101010101010101010101010101010101010101010101010101010101010101").into()
                )),
                confirmed_received_at: Some(1),
                confirmed_cost: Some(2)
            },
        );

        assert_eq!(
            tsfx_input,
            SideEffect {
                target: [0, 0, 0, 0],
                prize: 0,
                ordered_at: 0,
                encoded_action: vec![],
                encoded_args: vec![
                    vec![
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1
                    ],
                    vec![
                        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                        2, 2, 2, 2, 2, 2, 2
                    ],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![
                        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ]
                ],
                signature: vec![],
                enforce_executioner: None,
            }
        );
    }

    #[test]
    fn successfully_generates_id_for_side_empty_effect() {
        let empty_side_effect = SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(
            empty_side_effect.generate_id::<Hashing>(),
            H256::from_slice(&hex!(
                "89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3"
            ))
        );
    }

    #[test]
    fn successfully_defaults_side_effect_to_an_empty_one() {
        let empty_side_effect = SideEffect::<u64, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(empty_side_effect, SideEffect::default(),);
    }
}
