use t3rn_types::sfx::*;

use codec::{Decode, Encode};
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::vec::Vec;

pub type XtxId<T> = <T as frame_system::Config>::Hash;
pub use crate::volatile::{LocalState, Volatile};

use scale_info::TypeInfo;
use sp_std::fmt::Debug;

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Xtx<AccountId, BlockNumber, BalanceOf> {
    // todo: Add missing DFDs
    // pub contracts_dfd: InterExecSchedule -> ContractsDFD
    // pub side_effects_dfd: SideEffectsDFD
    // pub gateways_dfd: GatewaysDFD
    /// The owner of the bid
    pub requester: AccountId,

    /// Encoded content of composable tx
    pub initial_input: Vec<u8>,

    /// Expiry timeout
    pub timeouts_at: Option<BlockNumber>,

    /// Schedule execution of steps in the future intervals
    pub delay_steps_at: Option<Vec<BlockNumber>>,

    /// Has returned status already and what
    pub result_status: Option<Vec<u8>>,

    /// Total reward
    pub total_reward: Option<BalanceOf>,

    /// Local Xtx State
    pub local_state: LocalState,

    /// Vector of Steps that each can consist out of at least one FullSideEffect
    pub full_side_effects: Vec<Vec<FullSideEffect<AccountId, BlockNumber, BalanceOf>>>,
}

impl<
        AccountId: Encode + Clone + Debug,
        BlockNumber: Ord + Copy + Zero + Encode + Clone + Debug,
        BalanceOf: Copy + Zero + Encode + Decode + Clone + Debug,
    > Xtx<AccountId, BlockNumber, BalanceOf>
{
    pub fn new(
        // Requester of xtx
        requester: AccountId,
        // Encoded initial input set by a requester/SDK - base for the xtx state
        initial_input: Vec<u8>,
        // Expiry timeout
        timeouts_at: Option<BlockNumber>,
        // Schedule execution of steps in the future intervals
        delay_steps_at: Option<Vec<BlockNumber>>,
        // Total reward
        total_reward: Option<BalanceOf>,
        local_state: LocalState,
        full_side_effects: Vec<Vec<FullSideEffect<AccountId, BlockNumber, BalanceOf>>>,
    ) -> Self {
        Xtx {
            requester,
            initial_input,
            timeouts_at,
            delay_steps_at,
            result_status: None,
            total_reward,
            local_state,
            full_side_effects,
        }
    }

    pub fn generate_xtx_id<Hasher: sp_core::Hasher>(
        &self,
        requester_nonce: u32,
    ) -> <Hasher as sp_core::Hasher>::Out {
        let mut requester_on_32b_as_vec = self.requester.encode();

        let nonce_as_4b_word: [u8; 4] = requester_nonce.to_be_bytes();
        let mut nonce_as_32b_word: [u8; 32];
        nonce_as_32b_word = [0; 32];
        nonce_as_32b_word[28..32].copy_from_slice(&nonce_as_4b_word);
        requester_on_32b_as_vec.extend_from_slice(&nonce_as_32b_word);

        Hasher::hash(requester_on_32b_as_vec.as_slice())
    }

    pub fn is_completed(&self) -> bool {
        for step in self.full_side_effects.iter() {
            for full_side_effect in step.iter() {
                if full_side_effect.confirmed.is_none() {
                    return false
                }
            }
        }
        true
    }

    // Complete the full side effect of Xtx by assigning confirmed side effect/
    // This can only happen if the side effects is confirmed with respect to
    // the execution steps.
    //
    // Return true if the input side effect successfully confirmed.
    // Return false if no update has happened.
    // Throw an error if detected the confirmation ruleset has been violated.
    pub fn complete_side_effect<Hasher: sp_core::Hasher>(
        &mut self,
        confirmed: ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf>,
        input: SideEffect<AccountId, BalanceOf>,
        xtx_id: <Hasher as sp_core::Hasher>::Out,
        sfx_index: u32,
    ) -> Result<bool, &'static str> {
        let sfx_id = input.generate_id::<Hasher>(xtx_id.as_ref(), sfx_index);

        // Double check there are some side effects for that Xtx - should have been checked at API level tho already
        if self.full_side_effects.is_empty() {
            return Err("Xtx has no single side effect step to confirm")
        }

        let mut unconfirmed_step_no: Option<usize> = None;

        for (i, step) in self.full_side_effects.iter_mut().enumerate() {
            // Double check there are some side effects for that Xtx - should have been checked at API level tho already
            if step.is_empty() {
                return Err("Xtx has an empty single step.")
            }
            for full_side_effect in step.iter_mut() {
                if full_side_effect.confirmed.is_none() {
                    // Mark the first step no with encountered unconfirmed side effect
                    if unconfirmed_step_no.is_none() {
                        unconfirmed_step_no = Some(i);
                    }

                    // Recalculate the ID for each input side effect and compare with the input one.
                    // Check the current unconfirmed step before attempt to confirm the full side effect.
                    return if full_side_effect
                        .input
                        .generate_id::<Hasher>(xtx_id.as_ref(), full_side_effect.index)
                        == sfx_id
                        && unconfirmed_step_no == Some(i)
                    {
                        // We found the side effect to confirm from inside the unconfirmed step.
                        full_side_effect.confirmed = Some(confirmed);
                        Ok(true)
                    } else {
                        Err("Attempt to confirm side effect from the next step, \
                                but there still is at least one unfinished step")
                    }
                }
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type BlockNumber = u32;
    type BalanceOf = u64;
    type AccountId = u64;
    type Hashing = sp_runtime::traits::BlakeTwo256;

    #[test]
    fn successfully_creates_empty_xtx() {
        let empty_xtx = Xtx::<AccountId, BlockNumber, BalanceOf>::new(
            0,
            vec![],
            None,
            None,
            None,
            LocalState::new(),
            vec![],
        );

        assert_eq!(
            empty_xtx,
            Xtx {
                requester: 0,
                initial_input: vec![],
                timeouts_at: None,
                delay_steps_at: None,
                result_status: None,
                total_reward: None,
                local_state: LocalState::new(),
                full_side_effects: vec![],
            }
        );
    }

    #[test]
    fn successfully_confirms_1_side_effect_and_completes_xtx() {
        let input_side_effect_1 = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 1,
            action: [0, 0, 0, 0],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        let completing_side_effect_1 = ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            err: None,
            output: None,
            inclusion_data: vec![0],
            executioner: 1,
            received_at: 1,
            cost: None,
        };

        let mut xtx = Xtx::<AccountId, BlockNumber, BalanceOf>::new(
            0,
            vec![],
            None,
            None,
            None,
            LocalState::new(),
            vec![vec![FullSideEffect {
                input: input_side_effect_1.clone(),
                confirmed: None,
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 0,
            }]],
        );

        assert!(!xtx.is_completed());

        let res = xtx
            .complete_side_effect::<Hashing>(
                completing_side_effect_1.clone(),
                input_side_effect_1.clone(),
                xtx.generate_xtx_id::<Hashing>(0),
                0,
            )
            .unwrap();

        assert!(res);
        // Check that xtx.full_side_effects has been updated
        assert_eq!(
            xtx.full_side_effects[0][0],
            FullSideEffect {
                input: input_side_effect_1,
                confirmed: Some(completing_side_effect_1),
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 0
            }
        );

        assert!(xtx.is_completed());
    }

    #[test]
    fn successfully_confirms_2_side_effect_in_1_step_in_xtx() {
        let input_side_effect_1 = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 1,
            action: [0, 0, 0, 0],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        let input_side_effect_2 = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 1],
            max_reward: 1,
            action: [0, 0, 0, 0],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        let completing_side_effect_1 = ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            err: None,
            output: None,
            inclusion_data: vec![0],
            executioner: 1,
            received_at: 1,
            cost: None,
        };

        let completing_side_effect_2 = ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            err: None,
            output: None,
            inclusion_data: vec![0],
            executioner: 2,
            received_at: 1,
            cost: None,
        };

        let mut xtx = Xtx::<AccountId, BlockNumber, BalanceOf>::new(
            0,
            vec![],
            None,
            None,
            None,
            LocalState::new(),
            vec![vec![
                FullSideEffect {
                    input: input_side_effect_1.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 1,
                    best_bid: None,
                    index: 0,
                },
                FullSideEffect {
                    input: input_side_effect_2.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 1,
                    best_bid: None,
                    index: 1,
                },
            ]],
        );

        let res_1 = xtx
            .complete_side_effect::<Hashing>(
                completing_side_effect_1.clone(),
                input_side_effect_1.clone(),
                xtx.generate_xtx_id::<Hashing>(1),
                0,
            )
            .unwrap();

        assert!(res_1);
        // Check that the first xtx.full_side_effects has been updated
        assert_eq!(
            xtx.full_side_effects[0][0],
            FullSideEffect {
                input: input_side_effect_1,
                confirmed: Some(completing_side_effect_1),
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 0
            }
        );

        // Check that the second xtx.full_side_effects has NOT been updated
        assert_eq!(
            xtx.full_side_effects[0][1],
            FullSideEffect {
                input: input_side_effect_2.clone(),
                confirmed: None,
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 1
            }
        );

        assert!(!xtx.is_completed());

        let res_2 = xtx
            .complete_side_effect::<Hashing>(
                completing_side_effect_2.clone(),
                input_side_effect_2.clone(),
                xtx.generate_xtx_id::<Hashing>(2),
                1,
            )
            .unwrap();

        assert!(res_2);

        // Check that the second xtx.full_side_effects has now been updated
        assert_eq!(
            xtx.full_side_effects[0][1],
            FullSideEffect {
                input: input_side_effect_2,
                confirmed: Some(completing_side_effect_2),
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 1
            }
        );
        assert!(xtx.is_completed());
    }

    #[test]
    fn successfully_confirms_2_side_effect_in_2_steps_in_xtx() {
        let input_side_effect_1 = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 1,
            action: [0, 0, 0, 0],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        let input_side_effect_2 = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 1],
            max_reward: 1,
            action: [0, 0, 0, 0],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        let completing_side_effect_1 = ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            err: None,
            output: None,
            inclusion_data: vec![0],
            executioner: 1,
            received_at: 1,
            cost: None,
        };

        let completing_side_effect_2 = ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            err: None,
            output: None,
            inclusion_data: vec![0],
            executioner: 2,
            received_at: 1,
            cost: None,
        };

        // This time Xtx contains 2 steps each consisting from 1 side effect
        let mut xtx = Xtx::<AccountId, BlockNumber, BalanceOf>::new(
            0,
            vec![],
            None,
            None,
            None,
            LocalState::new(),
            vec![
                vec![FullSideEffect {
                    input: input_side_effect_1.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 1,
                    best_bid: None,
                    index: 0,
                }],
                vec![FullSideEffect {
                    input: input_side_effect_2.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 1,
                    best_bid: None,
                    index: 1,
                }],
            ],
        );

        let res_1 = xtx
            .complete_side_effect::<Hashing>(
                completing_side_effect_1.clone(),
                input_side_effect_1.clone(),
                xtx.generate_xtx_id::<Hashing>(3),
                0,
            )
            .unwrap();

        assert!(res_1);
        // Check that the first xtx.full_side_effects has been updated
        assert_eq!(
            xtx.full_side_effects[0][0],
            FullSideEffect {
                input: input_side_effect_1,
                confirmed: Some(completing_side_effect_1),
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 0
            }
        );

        // Check that the second xtx.full_side_effects has NOT been updated
        assert_eq!(
            xtx.full_side_effects[1][0],
            FullSideEffect {
                input: input_side_effect_2.clone(),
                confirmed: None,
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 1
            }
        );

        let res_2 = xtx
            .complete_side_effect::<Hashing>(
                completing_side_effect_2.clone(),
                input_side_effect_2.clone(),
                xtx.generate_xtx_id::<Hashing>(4),
                1,
            )
            .unwrap();

        assert!(res_2);

        // Check that the second xtx.full_side_effects has now been updated
        assert_eq!(
            xtx.full_side_effects[1][0],
            FullSideEffect {
                input: input_side_effect_2,
                confirmed: Some(completing_side_effect_2),
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 1
            }
        );
        assert!(xtx.is_completed());
    }

    #[test]
    fn throws_when_attempts_to_confirm_side_effect_from_2nd_step_without_1st_in_xtx() {
        let input_side_effect_1 = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 1,
            action: [0, 0, 0, 0],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        let input_side_effect_2 = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 1],
            max_reward: 1,
            action: [0, 0, 0, 0],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        let _completing_side_effect_1 = ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            err: None,
            output: None,
            inclusion_data: vec![0],
            executioner: 1,
            received_at: 1,
            cost: None,
        };

        let completing_side_effect_2 = ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            err: None,
            output: None,
            inclusion_data: vec![0],
            executioner: 2,
            received_at: 1,
            cost: None,
        };

        // This time Xtx contains 2 steps each consisting from 1 side effect
        let mut xtx = Xtx::<AccountId, BlockNumber, BalanceOf>::new(
            0,
            vec![],
            None,
            None,
            None,
            LocalState::new(),
            vec![
                vec![FullSideEffect {
                    input: input_side_effect_1.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 1,
                    best_bid: None,
                    index: 0,
                }],
                vec![FullSideEffect {
                    input: input_side_effect_2.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 1,
                    best_bid: None,
                    index: 1,
                }],
            ],
        );

        let res_2_err = xtx.complete_side_effect::<Hashing>(
            completing_side_effect_2,
            input_side_effect_2.clone(),
            xtx.generate_xtx_id::<Hashing>(5),
            1,
        );

        assert_eq!(res_2_err, Err("Attempt to confirm side effect from the next step, but there still is at least one unfinished step"));

        // Check that the firsts AND second xtx.full_side_effects has NOT been updated
        assert_eq!(
            xtx.full_side_effects[0][0],
            FullSideEffect {
                input: input_side_effect_1,
                confirmed: None,
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 0
            }
        );

        assert_eq!(
            xtx.full_side_effects[1][0],
            FullSideEffect {
                input: input_side_effect_2,
                confirmed: None,
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: 1,
                best_bid: None,
                index: 1
            }
        );

        assert!(!xtx.is_completed());
    }
}
