use frame_support::weights::Weight;

use crate::{MiniRuntime as Runtime, *};
use sp_runtime::traits::One;

pub struct GlobalOnInitQueues;

impl pallet_clock::traits::OnHookQueues<Runtime> for GlobalOnInitQueues {
    fn process(n: BlockNumber, on_init_weight_limit: Weight) -> Weight {
        let mut weights_consumed = vec![];
        const PROCESS_SIGNAL_SHARE: u32 = 15;
        const XTX_TICK_SHARE: u32 = 35;
        const REVERT_XTX_SHARE: u32 = 35;
        const BUMP_ROUND_SHARE: u32 = 5;
        const CALC_CLAIMABLE_SHARE: u32 = 10;
        if PROCESS_SIGNAL_SHARE
            + XTX_TICK_SHARE
            + REVERT_XTX_SHARE
            + BUMP_ROUND_SHARE
            + CALC_CLAIMABLE_SHARE
            > 100
        {
            log::error!(
                "GlobalOnInitQueues::Invalid shares exceed 100%, returning 0 - re-check the shares"
            );
            return 0
        }
        // Iterate over all pre-init hooks implemented by pallets and return aggregated weight
        weights_consumed.push(Circuit::process_signal_queue(
            n,
            BlockNumber::one(),
            Perbill::from_percent(PROCESS_SIGNAL_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Circuit::process_signal_queue consumed: {:?}",
            weights_consumed
                .last()
                .expect("Circuit::process_signal_queue consumed weight")
        );
        weights_consumed.push(Circuit::process_xtx_tick_queue(
            n,
            BlockNumber::one(),
            Perbill::from_percent(XTX_TICK_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Circuit::process_xtx_tick_queue consumed: {:?}",
            weights_consumed
                .last()
                .expect("Circuit::process_xtx_tick_queue consumed weight")
        );
        weights_consumed.push(Circuit::process_emergency_revert_xtx_queue(
            n,
            10u32,
            Perbill::from_percent(REVERT_XTX_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Circuit::process_emergency_revert_xtx_queue consumed: {:?}",
            weights_consumed
                .last()
                .expect("Circuit::process_emergency_revert_xtx_queue consumed weight")
        );
        weights_consumed.push(Clock::check_bump_round(
            n,
            BlockNumber::one(),
            Perbill::from_percent(BUMP_ROUND_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Clock::check_bump_round consumed: {:?}",
            weights_consumed
                .last()
                .expect("Clock::check_bump_round consumed weight")
        );
        weights_consumed.push(Clock::calculate_claimable_for_round(
            n,
            BlockNumber::one(),
            Perbill::from_percent(CALC_CLAIMABLE_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Clock::calculate_claimable_for_round consumed: {:?}",
            weights_consumed
                .last()
                .expect("Clock::calculate_claimable_for_round consumed weight")
        );
        let total_consumed: Weight = weights_consumed
            .iter()
            .fold(0, |acc: Weight, weight: &Weight| {
                acc.saturating_add(*weight)
            });

        log::debug!(
            "Total weight consumed by on init hook: {:?}",
            total_consumed
        );

        total_consumed
    }
}
