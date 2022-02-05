#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_runtime::traits::Zero;
use sp_std::boxed::Box;
use sp_std::collections::btree_map::BTreeMap;

use sp_std::vec::*;

use crate::side_effects::protocol::SideEffectProtocol as SideEffectProtocolT;
use t3rn_primitives::abi::GatewayABIConfig;
use t3rn_primitives::side_effect::{SideEffect, TargetId};
use t3rn_primitives::volatile::Volatile;

use t3rn_primitives::volatile::LocalState;

pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

pub trait SideEffectsLazyLoader {
    fn notice_gateway(&mut self, gateway_id: TargetId);
    fn select_side_effect<
        AccountId,
        BlockNumber,
        BalanceOf,
        CB: FnOnce(&Box<dyn SideEffectProtocolT>) -> Result<(), &'static str>,
    >(
        &self,
        side_effect: SideEffect<AccountId, BlockNumber, BalanceOf>,
        matched_cb: CB,
    ) -> Result<(), &'static str>;
}

// F: FnOnce(&[(Vec<u8>, &[u8])]) -> R,
impl SideEffectsLazyLoader for UniversalSideEffectsProtocol {
    fn notice_gateway(&mut self, gateway_id: TargetId) {
        if !self.seen_side_effects_protocol.contains_key(&gateway_id) {
            // Load standard side effects protocol
            let mut known_std_side_effects: BTreeMap<[u8; 4], Box<dyn SideEffectProtocolT>> =
                BTreeMap::new();

            let standards = crate::side_effects::standards::get_all_standard_side_effects();
            // ToDo: Just load the std side effects for the gateway for now
            //    but missing implementation would:
            //     1. compare the "allowed_methods" with std side effects and load only selected ones
            //     2. load from XDNS memory / or receive already pre-fetched custom side effects for that gateway
            for standard in standards {
                known_std_side_effects.insert(standard.get_id(), standard);
            }
            self.seen_side_effects_protocol
                .insert(gateway_id, known_std_side_effects);
        }
    }

    fn select_side_effect<
        AccountId,
        BlockNumber,
        BalanceOf,
        CB: FnOnce(&Box<dyn SideEffectProtocolT>) -> Result<(), &'static str>,
    >(
        &self,
        side_effect: SideEffect<AccountId, BlockNumber, BalanceOf>,
        matched_cb: CB,
    ) -> Result<(), &'static str> {
        match self.seen_side_effects_protocol.get(&side_effect.target) {
            Some(available_side_effects) => {
                // ToDo: Change type of SideEffect::EncodedAction from Vec<u8> to [u8; 4]
                let mut action_id_4b: [u8; 4] = [0, 0, 0, 0];
                action_id_4b.copy_from_slice(&side_effect.encoded_action[0..4]);
                match available_side_effects.get(&action_id_4b) {
                    Some(side_effect_protocol) => {
                        matched_cb(side_effect_protocol)
                    }
                    _ => Err("UniversalSideEffectsProtocol::validate_args - side effect unsupported on chosen gateway"),
                }
            }
            _ => Err("UniversalSideEffectsProtocol::validate_args - unknown gateway"),
        }
    }
}

pub struct UniversalSideEffectsProtocol {
    pub seen_side_effects_protocol:
        BTreeMap<TargetId, BTreeMap<[u8; 4], Box<dyn SideEffectProtocolT>>>,
}

impl UniversalSideEffectsProtocol {
    pub fn new() -> Self {
        Self {
            seen_side_effects_protocol: BTreeMap::new(),
        }
    }

    pub fn validate_args<
        AccountId: Encode + Clone,
        BlockNumber: Ord + Copy + Zero + Encode + Clone,
        BalanceOf: Copy + Zero + Encode + Decode + Clone,
        Hasher: sp_core::Hasher,
    >(
        &self,
        side_effect: SideEffect<AccountId, BlockNumber, BalanceOf>,
        gateway_abi: GatewayABIConfig,
        local_state: &mut LocalState,
    ) -> Result<(), &'static str> {
        self.select_side_effect(
            side_effect.clone(),
            |side_effect_protocol: &Box<dyn SideEffectProtocolT>| {
                side_effect_protocol.validate_args(
                    side_effect.encoded_args.clone(),
                    &gateway_abi,
                    local_state,
                    Some(side_effect.generate_id::<Hasher>().as_ref().to_vec()), // or None
                )
            },
        )
    }

    /// Based on the content of local state - check if it's necessary to commit the deposit
    /// for a given side effect.
    /// try_commit_insurance_deposit will parse the Opt. Insurance keys from the local state
    /// and pass them to insurance protocol.
    pub fn check_if_insurance_required<
        AccountId: Encode + Clone,
        BlockNumber: Ord + Copy + Zero + Encode + Clone,
        BalanceOf: Copy + Zero + Encode + Decode + Clone,
        Hasher: sp_core::Hasher,
    >(
        side_effect: SideEffect<AccountId, BlockNumber, BalanceOf>,
        local_state: &mut LocalState,
    ) -> Result<Option<[BalanceOf; 2]>, &'static str> {
        const INSURANCE_SEARCH_KEY: &'static str = "insurance";
        let key = LocalState::stick_key_with_prefix(
            INSURANCE_SEARCH_KEY.encode(),
            side_effect.generate_id::<Hasher>().as_ref().to_vec(),
        );
        match local_state.get(key) {
            None => Err(
                "Critical Error - try_commit_insurance_deposit should always be \
                called after side effects state is populated with 'insurance' entries",
            ),
            Some(maybe_insurance) => {
                match maybe_insurance.to_vec().len() {
                    // If no insurance entry - insurance isn't required. Do nothing.
                    0 => Ok(None),
                    32 => {
                        // ToDo: Must be u128 as t3rn_protocol + insurances assume it's hardcoded u128 for value of insurance and reward
                        // FixMe: Please someone fix below
                        let insurance_and_reward_u128: [u128; 2] = Decode::decode(
                            &mut &maybe_insurance.to_vec()[..],
                        )
                        .expect(
                            "try_commit_insurance_deposit should always be called after validate \
                                side effects which checked the insurance value sanity in eval",
                        );

                        let insurance: BalanceOf = Decode::decode(
                            &mut &insurance_and_reward_u128[0].encode()[..],
                        )
                        .expect(
                            "try_commit_insurance_deposit should arrive from u128 to BalanceOf Circuit type",
                        );

                        let reward: BalanceOf = Decode::decode(
                            &mut &insurance_and_reward_u128[1].encode()[..],
                        )
                        .expect(
                            "try_commit_insurance_deposit should arrive from u128 to BalanceOf Circuit type",
                        );
                        Ok(Some([
                            insurance, reward
                        ]))
                    }
                    _ => Err(
                        "Critical Error - try_commit_insurance_deposit should always be \
                            called after side effects which checked the insurance value sanity in eval",
                    ),
                }
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use crate::side_effects::protocol::TransferSideEffectProtocol;
    use crate::side_effects::test_utils::*;

    use t3rn_primitives::abi::Type;

    #[test]
    fn loader_successfully_recognizes_insurance_required_for_transfer() {
        // Validate and populate state first
        let mut local_state = LocalState::new();
        let transfer_protocol_box = Box::new(TransferSideEffectProtocol {});
        let valid_transfer_side_effect = produce_and_validate_side_effect(
            vec![
                (Type::Address(32), ArgVariant::A),
                (Type::Address(32), ArgVariant::B),
                (Type::Uint(64), ArgVariant::A),
                (Type::OptionalInsurance, ArgVariant::A),
            ],
            &mut local_state,
            transfer_protocol_box.clone(),
        );

        let res = UniversalSideEffectsProtocol::check_if_insurance_required::<
            AccountId,
            BlockNumber,
            BalanceOf,
            Hashing,
        >(valid_transfer_side_effect, &mut local_state);

        assert_eq!(res, Ok(Some([1u64, 2u64])));
    }

    #[test]
    fn loader_successfully_recognizes_insurance_not_required_for_transfer() {
        // Validate and populate state first
        let mut local_state = LocalState::new();
        let transfer_protocol_box = Box::new(TransferSideEffectProtocol {});
        let valid_transfer_side_effect = produce_and_validate_side_effect(
            vec![
                (Type::Address(32), ArgVariant::A),
                (Type::Address(32), ArgVariant::B),
                (Type::Uint(64), ArgVariant::A),
                (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
            ],
            &mut local_state,
            transfer_protocol_box.clone(),
        );

        let res = UniversalSideEffectsProtocol::check_if_insurance_required::<
            AccountId,
            BlockNumber,
            BalanceOf,
            Hashing,
        >(valid_transfer_side_effect, &mut local_state);

        assert_eq!(res, Ok(None));
    }
}
