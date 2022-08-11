use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn base_on_initialize() -> Weight;

    #[rustfmt::skip]
    fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight;

    // #[rustfmt::skip]
    // fn set_fixtures() -> Weight;
    // #[rustfmt::skip]
    // fn schedule_configure_executor() -> Weight;
    // #[rustfmt::skip]
    // fn execute_configure_executor() -> Weight;
    // #[rustfmt::skip]
    // cancel_configure_executor() -> Weight;
    // #[rustfmt::skip]
    // fn join_candidates(x: u32, ) -> Weight;
    // #[rustfmt::skip]
    // fn schedule_leave_candidates(x: u32, ) -> Weight;
    // #[rustfmt::skip]
    // fn execute_leave_candidates(x: u32, ) -> Weight;
    // #[rustfmt::skip]
    // fn cancel_leave_candidates(x: u32, ) -> Weight;
    // #[rustfmt::skip]
    // fn go_offline() -> Weight;
    // #[rustfmt::skip]
    // fn go_online() -> Weight;
    // #[rustfmt::skip]
    // fn candidate_bond_more() -> Weight;
    // #[rustfmt::skip]
    // fn schedule_candidate_bond_less() -> Weight;
    // #[rustfmt::skip]
    // fn execute_candidate_bond_less() -> Weight;
    // #[rustfmt::skip]
    // fn cancel_candidate_bond_less() -> Weight;
    // #[rustfmt::skip]
    // fn stake(x: u32, y: u32, ) -> Weight;
    // #[rustfmt::skip]
    // fn schedule_leave_stakers() -> Weight;
    // #[rustfmt::skip]
    // fn execute_leave_stakers(x: u32, ) -> Weight;
    // #[rustfmt::skip]
    // fn cancel_leave_stakers() -> Weight;
    // #[rustfmt::skip]
    // fn schedule_revoke_stake() -> Weight;
    // #[rustfmt::skip]
    // fn staker_bond_more() -> Weight;
    // #[rustfmt::skip]
    // fn schedule_staker_bond_less() -> Weight;
    // #[rustfmt::skip]
    // fn execute_revoke_stake() -> Weight;
    // #[rustfmt::skip]
    // fn execute_staker_bond_less() -> Weight;
    // #[rustfmt::skip]
    // fn cancel_revoke_stake() -> Weight;
    // #[rustfmt::skip]
    // fn cancel_staker_bond_less() -> Weight;
    // #[rustfmt::skip]
    // fn pay_one_executor_reward(y: u32, ) -> Weight;
    // #[rustfmt::skip]
    // fn cancel_stake_request() -> Weight;
    // #[rustfmt::skip]
    // fn execute_stake_request() -> Weight;
}

pub struct TreasuryWeight<T>(PhantomData<T>);

// TODO
impl<T: frame_system::Config> WeightInfo for TreasuryWeight<T> {
    #[rustfmt::skip]
    fn base_on_initialize() -> Weight {
		419 as Weight //TODO
	}

    #[rustfmt::skip]
    fn round_transition_on_initialize(_x: u32, _y: u32, ) -> Weight {
		419 as Weight //TODO
    }
}

// TODO
impl WeightInfo for () {
    #[rustfmt::skip]
    fn base_on_initialize() -> Weight {
		419 as Weight //TODO
	}

    #[rustfmt::skip]
    fn round_transition_on_initialize(_x: u32, _y: u32, ) -> Weight {
		419 as Weight //TODO
    }
}
