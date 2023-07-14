use frame_support::weights::Weight;

use crate::{MiniRuntime as Runtime, *};
use sp_runtime::traits::{One, Zero};

pub struct GlobalOnInitQueues;

impl t3rn_primitives::clock::OnHookQueues<Runtime> for GlobalOnInitQueues {
    fn process(n: BlockNumber, on_init_weight_limit: Weight) -> Weight {
        let mut weights_consumed = vec![];
        const PROCESS_SIGNAL_SHARE: u32 = 5;
        const XTX_TICK_SHARE: u32 = 30;
        const REVERT_XTX_SHARE: u32 = 5;
        const BUMP_ROUND_SHARE: u32 = 5;
        const CALC_CLAIMABLE_SHARE: u32 = 5;
        const WEEKLY_SHARE: u32 = 20;
        const DAILY_SHARE: u32 = 10;
        const HOURLY_SHARE: u32 = 20;

        if PROCESS_SIGNAL_SHARE
            + XTX_TICK_SHARE
            + REVERT_XTX_SHARE
            + BUMP_ROUND_SHARE
            + CALC_CLAIMABLE_SHARE
            + WEEKLY_SHARE
            + DAILY_SHARE
            + HOURLY_SHARE
            > 100
        {
            log::error!(
                "GlobalOnInitQueues::Invalid shares exceed 100%, returning 0 - re-check the shares"
            );
            return 0
        }
        const BLOCKS_PER_WEEK: BlockNumber = 7 * 24 * 60 * 60;
        const BLOCKS_PER_DAY: BlockNumber = 24 * 60 * 60;
        const BLOCKS_PER_HOUR: BlockNumber = 60 * 60;

        if (n % BLOCKS_PER_HOUR).is_zero() {
            let hourly_weight_limit: Weight =
                Perbill::from_percent(HOURLY_SHARE).mul_ceil(on_init_weight_limit);
            weights_consumed.push(Self::process_hourly(n, hourly_weight_limit))
        }

        if (n % BLOCKS_PER_DAY).is_zero() {
            let daily_weight_limit: Weight =
                Perbill::from_percent(DAILY_SHARE).mul_ceil(on_init_weight_limit);
            weights_consumed.push(Self::process_daily(n, daily_weight_limit))
        }

        if (n % BLOCKS_PER_WEEK).is_zero() {
            let weekly_weight_limit: Weight =
                Perbill::from_percent(WEEKLY_SHARE).mul_ceil(on_init_weight_limit);
            weights_consumed.push(Self::process_weekly(n, weekly_weight_limit))
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

    fn process_weekly(_n: BlockNumber, _hook_weight_limit: Weight) -> Weight {
        0
    }

    fn process_daily(_n: BlockNumber, _hook_weight_limit: Weight) -> Weight {
        0
    }

    fn process_hourly(n: BlockNumber, hook_weight_limit: Weight) -> Weight {
        let mut weights_consumed: Vec<Weight> = Vec::new();
        weights_consumed.push(XDNS::check_for_manual_verifier_overview_process(n));

        let total = weights_consumed
            .iter()
            .fold(0, |acc: Weight, weight: &Weight| {
                acc.saturating_add(*weight)
            });

        if total > hook_weight_limit {
            log::error!(
                "GlobalOnInitQueues::process_hourly consumed more than the limit: {:?}",
                total
            );
        }

        total
    }
}
