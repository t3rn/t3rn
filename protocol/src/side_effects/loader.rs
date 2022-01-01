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

    // /// Based on the content of local state - check if it's necessary to commit the deposit
    // /// for a given side effect.
    // /// try_commit_insurance_deposit will parse the Opt. Insurance keys from the local state
    // /// and pass them to insurance protocol.
    // pub fn validate_many_full<
    //     AccountId: Encode + Clone,
    //     BlockNumber: Ord + Copy + Zero + Encode + Clone,
    //     BalanceOf: Copy + Zero + Encode + Decode + Clone,
    //     Hasher: sp_core::Hasher,
    //     OnRequestInsurance: FnOnce(SideEffect<AccountId, BlockNumber, BalanceOf>, BalanceOf, BalanceOf, AccountId, &mut LocalState) -> Result<(), &'static str>,
    // >(
    //     side_effects: Vec<SideEffect<AccountId, BlockNumber, BalanceOf>>,
    //     local_state: &mut LocalState,
    //     request_side_effect_insurance: OnRequestInsurance,
    //     requester: AccountId,
    // ) -> Result<Vec<FullSideEffect<AccountId, BlockNumber, BalanceOf>>> {
    //
    //     let mut full_side_effects_steps: Vec<
    //         Vec<FullSideEffect<AccountId, BlockNumber, BalanceOf>>,
    //     > = vec![];
    //
    //     for side_effect in side_effects.iter() {
    //         // ToDo: Generate Circuit's params as default ABI from let abi = pallet_xdns::get_abi(target_id)
    //         let gateway_abi = Default::default();
    //         use_protocol.notice_gateway(side_effect.target);
    //         use_protocol
    //             .validate_args::<T::AccountId, T::BlockNumber, BalanceOf<T>, SystemHashing<T>>(
    //                 side_effect.clone(),
    //                 gateway_abi,
    //                 &mut local_state,
    //             )?;
    //
    //         if let Some(insurance_and_reward) =
    //
    //         UniversalSideEffectsProtocol::check_if_insurance_required::<
    //             AccountId,
    //             BlockNumber,
    //             BalanceOf,
    //             Hasher,
    //         >(side_effect.clone(), &mut local_state)?
    //         {
    //             let (insurance, reward) = (insurance_and_reward[0], insurance_and_reward[1]);
    //             // UniversalSideEffectsProtocol::request_side_effect_insurance(
    //             request_side_effect_insurance(
    //                 Default::default(), // ToDo: Obtain XtxId before let x_tx_id: XtxId<T> = new_xtx.generate_xtx_id::<T>();
    //                 side_effect.clone(),
    //                 insurance,
    //                 reward,
    //                 &requester,
    //                 &mut local_state,
    //             )?;
    //         }
    //         full_side_effects.push(FullSideEffect {
    //             input: side_effect.clone(),
    //             confirmed: None,
    //         })
    //     }
    //
    //     full_side_effects_steps = match sequential {
    //         false => vec![full_side_effects],
    //         true => {
    //             let mut sequential_order: Vec<
    //                 Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
    //             > = vec![];
    //             for fse in full_side_effects.iter() {
    //                 sequential_order.push(vec![fse.clone()]);
    //             }
    //             sequential_order
    //         }
    //     };
    // }

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
                        let insurance_and_reward: [BalanceOf; 2] = Decode::decode(
                            &mut &maybe_insurance.to_vec()[..],
                        )
                        .expect(
                            "try_commit_insurance_deposit should always be called after validate \
                                side effects which checked the insurance value sanity in eval",
                        );
                        Ok(Some(insurance_and_reward))
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
