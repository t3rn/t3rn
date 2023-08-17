#[cfg(test)]
pub mod clock_test {
    use t3rn_mini_mock_runtime::{BlockNumber, Clock, ExtBuilder, MiniRuntime, System};
    use t3rn_primitives::common::RoundInfo;

    #[test]
    fn check_bump_round_doesnt_tick_below_300() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let current_round = Clock::current_round();
            let expected_round_0 = RoundInfo {
                index: 1,
                head: 1,
                term: 299,
            };
            assert_eq!(current_round, expected_round_0);

            System::set_block_number(299u32);
            Clock::check_bump_round(BlockNumber::from(299u32));
            let current_round = Clock::current_round();
            assert_eq!(current_round, expected_round_0);
        });
    }

    #[test]
    fn check_bump_round_ticks_above_300() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let current_round = Clock::current_round();
            let expected_round_0 = RoundInfo {
                index: 1,
                head: 1,
                term: 299,
            };
            assert_eq!(current_round, expected_round_0);

            System::set_block_number(300u32);
            Clock::check_bump_round(BlockNumber::from(300u32));

            let expected_round_next = RoundInfo {
                index: 2,
                head: 300,
                term: 300,
            };

            let current_round = Clock::current_round();
            assert_eq!(current_round, expected_round_next);
        });
    }
}
