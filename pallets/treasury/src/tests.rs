use crate::{
    assert_last_event, assert_last_n_events,
    inflation::InflationInfo,
    mock::{Event as MockEvent, *},
    Beneficiaries, BeneficiaryRoundRewards, CurrentRound, Error, Event,
};
use frame_support::{assert_err, assert_noop, assert_ok};
use sp_runtime::Perbill;
use t3rn_primitives::{
    common::{Range, RoundIndex, RoundInfo, BLOCKS_PER_YEAR},
    monetary::{BeneficiaryRole, InflationAllocation},
};

#[test]
fn mint_for_round_requires_root() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Treasury::mint_for_round(Origin::signed(419), 1, 1_000_000_000),
            sp_runtime::DispatchError::BadOrigin
        );
    })
}

#[test]
fn genesis_inflation_config() {
    new_test_ext().execute_with(|| {
        fast_forward_to(3);

        assert_eq!(
            Treasury::current_round(),
            RoundInfo {
                index: 1 as RoundIndex,
                head: 0,
                term: <DefaultRoundTerm>::get()
            }
        );
        assert_eq!(
            Treasury::inflation_config(),
            InflationInfo {
                annual: Range {
                    min: Perbill::from_parts(75000000),   //TODO
                    ideal: Perbill::from_parts(80000000), //TODO
                    max: Perbill::from_parts(85000000),   //TODO
                },
                round: Range {
                    min: Perbill::from_parts(49503),   //TODO
                    ideal: Perbill::from_parts(52679), //TODO
                    max: Perbill::from_parts(55841),   //TODO
                },
            }
        );
        assert_eq!(
            Treasury::inflation_alloc(),
            InflationAllocation {
                executor: Perbill::from_percent(50),  //TODO
                developer: Perbill::from_percent(50), //TODO
            },
        )
    })
}

#[test] //FIXME
fn hook_mints_for_each_past_round() {
    new_test_ext().execute_with(|| {
        let dev = 1;
        let total_active_executors_stake = 50; //TODO unfix, also 50/50 split

        <CurrentRound<Test>>::put(RoundInfo {
            index: 1_u32,
            term: <DefaultRoundTerm>::get(),
            head: 1,
        });
        <Beneficiaries<Test>>::insert(dev, BeneficiaryRole::Developer, 0);
        <BeneficiaryRoundRewards<Test>>::insert(dev, 1, 5);
        <BeneficiaryRoundRewards<Test>>::insert(dev, 2, 5);

        // fast forward two round begins
        fast_forward_to(<DefaultRoundTerm>::get() as u64);
        let first_round_issuance = Treasury::compute_round_issuance(total_active_executors_stake);
        fast_forward_to(
            (2_u32
                .checked_mul(<DefaultRoundTerm>::get())
                .unwrap()
                .checked_add(1)
                .unwrap()) as u64,
        );

        assert_last_n_events!(
            6,
            vec![
                Event::NewRound {
                    round: 2,
                    head: (<DefaultRoundTerm>::get().checked_add(1).unwrap()) as u64,
                },
                Event::BeneficiaryTokensIssued(dev, 527), //TODO
                Event::RoundTokensIssued(1, first_round_issuance),
                Event::NewRound {
                    round: 3,
                    head: (2_u32
                        .checked_mul(<DefaultRoundTerm>::get())
                        .unwrap()
                        .checked_add(1)
                        .unwrap()) as u64,
                },
                Event::BeneficiaryTokensIssued(dev, 526), //TODO
                Event::RoundTokensIssued(
                    2,
                    Treasury::compute_round_issuance(total_active_executors_stake)
                )
            ]
        );
    })
}

//TODO refactor wen increment
#[test]
fn mint_for_round_splits_total_rewards_correctly_amongst_actors() {
    new_test_ext().execute_with(|| {
        let dev = 1;
        let exec = 2;
        let total_active_executors_stake = 10;

        <Beneficiaries<Test>>::insert(dev, BeneficiaryRole::Developer, 0);
        <Beneficiaries<Test>>::insert(exec, BeneficiaryRole::Executor, 0);

        assert_ok!(Treasury::mint_for_round(
            Origin::root(),
            1,
            total_active_executors_stake
        ));

        assert_last_n_events!(
            3,
            vec![
                Event::BeneficiaryTokensIssued(dev, 5),
                Event::BeneficiaryTokensIssued(exec, 5),
                Event::RoundTokensIssued(1, total_active_executors_stake)
            ]
        );
    })
}

#[test]
fn claim_rewards_fails_if_not_beneficiary() {
    new_test_ext().execute_with(|| {
        <Beneficiaries<Test>>::insert(1, BeneficiaryRole::Developer, 0);

        assert_err!(
            Treasury::claim_rewards(Origin::signed(1)),
            Error::<Test>::NoRewardsAvailable
        );
    })
}

#[test]
fn claim_rewards_fails_if_none_available() {
    new_test_ext().execute_with(|| {
        <Beneficiaries<Test>>::insert(1, BeneficiaryRole::Developer, 0);

        assert_err!(
            Treasury::claim_rewards(Origin::signed(1)),
            Error::<Test>::NoRewardsAvailable
        );
    })
}

#[test]
fn claim_rewards_accumulates_all_past_rounds_rewards() {
    new_test_ext().execute_with(|| {
        // initialize claimer with some balance
        let claimer = 419;
        Balances::set_balance(Origin::root(), claimer, 100, 0).expect("claimer account");

        // configure pallet storage to reward our claimer
        <Beneficiaries<Test>>::insert(claimer, BeneficiaryRole::Executor, 0);
        <BeneficiaryRoundRewards<Test>>::insert(claimer, 1, 1);
        <BeneficiaryRoundRewards<Test>>::insert(claimer, 2, 1);

        assert_ok!(Treasury::claim_rewards(Origin::signed(claimer)));

        // assert balance allocated
        assert_eq!(Balances::free_balance(&claimer), 102);

        // assert storage is empty for candidate
        let remaining_storage = <BeneficiaryRoundRewards<Test>>::iter_key_prefix(1).count();
        assert_eq!(remaining_storage, 0);
    })
}

#[test]
fn set_inflation_requires_root() {
    new_test_ext().execute_with(|| {
        let new_inflation = Range {
            min: Perbill::from_percent(0),
            ideal: Perbill::from_percent(2),
            max: Perbill::from_percent(1),
        };

        assert_noop!(
            Treasury::set_inflation(Origin::signed(419), new_inflation),
            sp_runtime::DispatchError::BadOrigin
        );
    })
}

#[test]
fn set_inflation_fails_if_invalid() {
    new_test_ext().execute_with(|| {
        let new_inflation = Range {
            min: Perbill::from_percent(0),
            ideal: Perbill::from_percent(2),
            max: Perbill::from_percent(1),
        };

        assert_err!(
            Treasury::set_inflation(Origin::root(), new_inflation),
            Error::<Test>::InvalidInflationConfig
        );
    })
}

#[test]
fn set_inflation_fails_if_not_changed() {
    new_test_ext().execute_with(|| {
        let existing_inflation = Treasury::inflation_config().annual;

        assert_err!(
            Treasury::set_inflation(Origin::root(), existing_inflation),
            Error::<Test>::ValueNotChanged
        );
    })
}

#[test]
fn set_inflation_derives_round_from_annual_inflation() {
    new_test_ext().execute_with(|| {
        // input annual inflation config
        let actual_annual_inflation = Range {
            min: Perbill::from_parts(80000000),   //TODO
            ideal: Perbill::from_parts(80000000), //TODO
            max: Perbill::from_parts(80000000),   //TODO
        };

        // what we expect to get auto derived as round config
        // derivation also depends upon Treasury::current_round().term = 20
        // which must be at least as big as the active colator set
        let expected_round_inflation = Range {
            min: Perbill::from_parts(52679),   //TODO
            ideal: Perbill::from_parts(52679), //TODO
            max: Perbill::from_parts(52679),   //TODO
        };

        assert_ok!(Treasury::set_inflation(
            Origin::root(),
            actual_annual_inflation
        ));

        // assert new inflation config got stored
        assert_eq!(
            Treasury::inflation_config(),
            InflationInfo {
                annual: actual_annual_inflation,
                round: expected_round_inflation,
            }
        );

        // assert new inflation config was emitted
        assert_last_event!(MockEvent::Treasury(Event::InflationConfigChanged {
            annual_min: actual_annual_inflation.min,
            annual_ideal: actual_annual_inflation.ideal,
            annual_max: actual_annual_inflation.max,
            round_min: expected_round_inflation.min,
            round_ideal: expected_round_inflation.ideal,
            round_max: expected_round_inflation.max,
        }));
    })
}

#[test]
fn set_inflation_alloc_requires_root() {
    new_test_ext().execute_with(|| {
        let inflation_alloc = InflationAllocation {
            developer: Perbill::from_percent(50),
            executor: Perbill::from_percent(50),
        };

        assert_noop!(
            Treasury::set_inflation_alloc(Origin::signed(419), inflation_alloc),
            sp_runtime::DispatchError::BadOrigin
        );
    })
}

#[test]
fn set_inflation_alloc_fails_if_invalid() {
    new_test_ext().execute_with(|| {
        let inflation_alloc = InflationAllocation {
            developer: Perbill::from_percent(50),
            executor: Perbill::from_percent(60),
        };

        assert_noop!(
            Treasury::set_inflation_alloc(Origin::root(), inflation_alloc),
            Error::<Test>::InvalidInflationAllocation
        );
    })
}

#[test]
fn set_inflation_alloc_fails_if_not_changed() {
    new_test_ext().execute_with(|| {
        let inflation_alloc = InflationAllocation {
            developer: Perbill::from_percent(50),
            executor: Perbill::from_percent(50),
        };

        assert_noop!(
            Treasury::set_inflation_alloc(Origin::root(), inflation_alloc),
            Error::<Test>::ValueNotChanged
        );
    })
}

#[test]
fn set_inflation_alloc_amongst_actors() {
    new_test_ext().execute_with(|| {
        let inflation_alloc = InflationAllocation {
            developer: Perbill::from_percent(49),
            executor: Perbill::from_percent(51),
        };

        assert_ok!(Treasury::set_inflation_alloc(
            Origin::root(),
            inflation_alloc.clone()
        ));

        assert_eq!(Treasury::inflation_alloc(), inflation_alloc)
    })
}

#[test]
fn set_round_term_requires_root() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Treasury::set_round_term(Origin::signed(419), 419),
            sp_runtime::DispatchError::BadOrigin
        );
    })
}

#[test]
fn set_round_term_fails_if_not_changed() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Treasury::set_round_term(Origin::root(), <CurrentRound<Test>>::get().term),
            Error::<Test>::ValueNotChanged
        );
    })
}

#[test]
fn set_round_term_fails_if_lower_than_min() {
    new_test_ext().execute_with(|| {
        new_test_ext().execute_with(|| {
            assert_noop!(
                Treasury::set_round_term(Origin::root(), 7),
                Error::<Test>::RoundTermTooShort
            );
        })
    })
}

#[test]
fn set_round_term_derives_round_inflation() {
    new_test_ext().execute_with(|| {
        let new = 500;
        assert_ok!(Treasury::set_round_term(Origin::root(), new));
        assert_last_event!(MockEvent::Treasury(Event::RoundTermChanged {
            old: <DefaultRoundTerm>::get(),
            new,
            round_min: Perbill::from_parts(13752),
            round_ideal: Perbill::from_parts(14635),
            round_max: Perbill::from_parts(15513)
        }));
        assert_eq!(
            Treasury::inflation_config().round,
            Range {
                min: Perbill::from_parts(13752),
                ideal: Perbill::from_parts(14635),
                max: Perbill::from_parts(15513)
            }
        )
    })
}

#[test]
fn add_beneficiary_requires_root() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Treasury::add_beneficiary(Origin::signed(419), 419, BeneficiaryRole::Developer),
            sp_runtime::DispatchError::BadOrigin
        );
    })
}

#[test]
fn remove_beneficiary_requires_root() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Treasury::remove_beneficiary(Origin::signed(419), 419),
            sp_runtime::DispatchError::BadOrigin
        );
    })
}

#[test]
#[ignore]
fn gradually_decreasing_to_perpetual_inflation() {
    new_test_ext().execute_with(|| {
        // at genesis
        assert_eq!(
            Treasury::inflation_config().annual,
            Range {
                min: Perbill::from_parts(75_000_000),
                ideal: Perbill::from_parts(80_000_000),
                max: Perbill::from_parts(85_000_000),
            }
        );

        // after 3 yrs
        fast_forward_to((BLOCKS_PER_YEAR.checked_mul(3).unwrap()) as u64);
        assert_eq!(
            Treasury::inflation_config().annual,
            Range {
                min: Perbill::from_parts(40_104_448),
                ideal: Perbill::from_parts(45_104_448),
                max: Perbill::from_parts(50_104_448),
            }
        );

        // after 6 yrs
        fast_forward_to((BLOCKS_PER_YEAR.checked_mul(6).unwrap()) as u64);
        assert_eq!(
            Treasury::inflation_config().annual,
            Range {
                min: Perbill::from_parts(5_000_000),
                ideal: Perbill::from_parts(10_000_000),
                max: Perbill::from_parts(15_000_000),
            }
        );
    })
}
