use frame_support::weights::Weight;
use num_traits::AsPrimitive;

use crate::{Circuit, Runtime, XDNS, *};
use sp_runtime::{
    traits::{One, Zero},
    Percent,
};
pub struct GlobalOnInitQueues;

impl t3rn_primitives::clock::OnHookQueues<Runtime> for GlobalOnInitQueues {
    fn process(n: BlockNumber, on_init_weight_limit: Weight) -> Weight {
        const PROCESS_SIGNAL_SHARE: u8 = 5;
        const XTX_TICK_SHARE: u8 = 30;
        const REVERT_XTX_SHARE: u8 = 5;
        const WEEKLY_SHARE: u8 = 20;
        const BI_WEEKLY_SHARE: u8 = 10;
        const DAILY_SHARE: u8 = 10;
        const HOURLY_SHARE: u8 = 20;

        if PROCESS_SIGNAL_SHARE
            + XTX_TICK_SHARE
            + REVERT_XTX_SHARE
            + WEEKLY_SHARE
            + BI_WEEKLY_SHARE
            + DAILY_SHARE
            + HOURLY_SHARE
            > 100
        {
            log::error!(
                "GlobalOnInitQueues::Invalid shares exceed 100%, returning 0 - re-check the shares"
            );
            return Zero::zero()
        }

        const BLOCKS_PER_HOUR: BlockNumber = 60 * 5; // Assuming 12 second block time
        const BLOCKS_PER_DAY: BlockNumber = 24 * BLOCKS_PER_HOUR;
        const BLOCKS_PER_WEEK: BlockNumber = 7 * BLOCKS_PER_DAY;
        const BLOCKS_PER_2_WEEKS: BlockNumber = 2 * BLOCKS_PER_WEEK;

        let mut total_consumed: Weight = Zero::zero();
        // Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
        if (n % BLOCKS_PER_HOUR).is_zero() {
            let hourly_weight_limit: Weight =
                Percent::from_percent(HOURLY_SHARE) * on_init_weight_limit;
            // Percent::from_percent(HOURLY_SHARE) * on_init_weight_limit;
            total_consumed =
                total_consumed.saturating_add(Self::process_hourly(n, hourly_weight_limit));
        }

        if (n % BLOCKS_PER_DAY).is_zero() {
            let daily_weight_limit: Weight =
                Percent::from_percent(DAILY_SHARE) * on_init_weight_limit;
            total_consumed =
                total_consumed.saturating_add(Self::process_daily(n, daily_weight_limit));
        }

        if (n % BLOCKS_PER_WEEK).is_zero() {
            let weekly_weight_limit: Weight =
                Percent::from_percent(WEEKLY_SHARE) * on_init_weight_limit;
            total_consumed =
                total_consumed.saturating_add(Self::process_weekly(n, weekly_weight_limit));
        }

        if (n % BLOCKS_PER_2_WEEKS).is_zero() {
            let bi_weekly_weight_limit: Weight =
                Percent::from_percent(BI_WEEKLY_SHARE) * on_init_weight_limit;
            total_consumed =
                total_consumed.saturating_add(Self::process_bi_weekly(n, bi_weekly_weight_limit));
        }

        let weight = Circuit::process_signal_queue(
            n,
            BlockNumber::one(),
            Percent::from_percent(PROCESS_SIGNAL_SHARE) * on_init_weight_limit,
        );
        log::debug!("Circuit::process_signal_queue consumed: {:?}", weight);
        total_consumed = total_consumed.saturating_add(weight);

        let weight = Circuit::process_xtx_tick_queue(
            n,
            BlockNumber::one(),
            Percent::from_percent(XTX_TICK_SHARE) * on_init_weight_limit,
        );
        log::debug!("Circuit::process_xtx_tick_queue consumed: {:?}", weight);
        total_consumed = total_consumed.saturating_add(weight);

        let weight = Circuit::process_emergency_revert_xtx_queue(
            n,
            10u32,
            Percent::from_percent(REVERT_XTX_SHARE) * on_init_weight_limit,
        );
        log::debug!(
            "Circuit::process_emergency_revert_xtx_queue consumed: {:?}",
            weight
        );
        total_consumed = total_consumed.saturating_add(weight);

        let (_success, weight) = Rewards::process_author();

        log::debug!("Rewards::process_author consumed: {:?}", weight);
        total_consumed = total_consumed.saturating_add(weight);

        log::debug!(
            "Total weight consumed by on init hook: {:?}",
            total_consumed
        );

        total_consumed
    }

    fn process_bi_weekly(_n: BlockNumber, hook_weight_limit: Weight) -> Weight {
        let mut total_consumed: Weight = Zero::zero();

        let weight = Rewards::distribute_inflation();
        log::debug!("Rewards::distribute_inflation consumed: {:?}", weight);
        total_consumed = total_consumed.saturating_add(weight);

        let weight = Rewards::process_authors_this_period();

        log::debug!(
            "Rewards::process_authors_this_period consumed: {:?}",
            weight
        );
        total_consumed = total_consumed.saturating_add(weight);

        if total_consumed.all_gte(hook_weight_limit) {
            log::error!(
                "GlobalOnInitQueues::process_bi_weekly consumed more than the limit: {:?}",
                weight
            );
        }
        total_consumed
    }

    fn process_weekly(_n: BlockNumber, _hook_weight_limit: Weight) -> Weight {
        Zero::zero()
    }

    fn process_daily(_n: BlockNumber, _hook_weight_limit: Weight) -> Weight {
        Zero::zero()
    }

    fn process_hourly(n: BlockNumber, hook_weight_limit: Weight) -> Weight {
        let mut total_consumed: Weight = Zero::zero();

        let weight = XDNS::check_for_manual_verifier_overview_process(n);
        log::debug!(
            "XDNS::check_for_manual_verifier_overview_process consumed: {:?}",
            weight
        );
        total_consumed = total_consumed.saturating_add(weight);

        let weight = Rewards::process_accumulated_settlements();
        log::debug!(
            "Rewards::process_accumulated_settlements consumed: {:?}",
            weight
        );
        total_consumed = total_consumed.saturating_add(weight);

        let weight = Clock::check_bump_round(n);
        log::debug!("Clock::check_bump_round consumed: {:?}", weight);
        total_consumed = total_consumed.saturating_add(weight);

        if total_consumed.all_gte(hook_weight_limit) {
            log::error!(
                "GlobalOnInitQueues::process_hourly consumed more than the limit: {:?}",
                weight
            );
        }

        log::debug!(
            "GlobalOnInitQueues::process_hourly total_consumed: {:?}",
            total_consumed
        );

        total_consumed
    }
}
