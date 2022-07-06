//! <!-- markdown-link-check-disable -->
//! # Executor staking pallet
//! </pre></p></details>

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

#[cfg(test)]
mod mock;
// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod stakes;
pub mod staking_actions;
pub mod subject_metadata;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use super::{
        stakes::Stakes,
        subject_metadata::{CandidateMetadata, StakerMetadata},
        weights,
    };
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::{ensure_root, pallet_prelude::*};
    use sp_runtime::{
        traits::{Saturating, Zero},
        Percent,
    };
    use sp_std::collections::btree_map::BTreeMap;
    use t3rn_primitives::{
        common::{OrderedSet, Range, RoundIndex},
        staking::{
            Bond, CancelledScheduledStakingRequest, ExecutorSnapshot, ScheduledStakingRequest, StakerAdded,
            StakingAction, ExecutorInfo
        },
        treasury::Treasury as TTreasury,
    };

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Range for the target executor active set size.
        /// The ideal is applied during genesis as default.
        #[pallet::constant]
        type ActiveSetSize: Get<Range<u32>>;

        /// Protocol enforced maximum executor commission.
        #[pallet::constant]
        type MaxCommission: Get<Percent>;

        /// Protocol enforced maximum executor risk reward ratio.
        #[pallet::constant]
        type MaxRisk: Get<Percent>;

        /// Minimum stake required for any candidate to be considered for the active set.
        #[pallet::constant]
        type MinExecutorBond: Get<BalanceOf<Self>>;

        /// Minimum stake required for any candidate to be considered as candidate.
        #[pallet::constant]
        type MinCandidateBond: Get<BalanceOf<Self>>;

        /// Minimum stake for any registered on-chain account to stake.
        /// Requirement is checked on every staking action after the first.
        #[pallet::constant]
        type MinAtomicStake: Get<BalanceOf<Self>>;

        /// Minimum stake for any registered on-chain account to be a staker.
        /// Requirement checked at first staking action.
        #[pallet::constant]
        type MinTotalStake: Get<BalanceOf<Self>>;

        /// Maximum top stakes per candidate.
        #[pallet::constant]
        type MaxTopStakesPerCandidate: Get<u32>;

        /// Maximum bottom stakes per candidate.
        #[pallet::constant]
        type MaxBottomStakesPerCandidate: Get<u32>;

        /// Maximum stakings per staker.
        #[pallet::constant]
        type MaxStakesPerStaker: Get<u32>;

        /// Delay applied when changing an executor's configuration.
        #[pallet::constant]
        type ConfigureExecutorDelay: Get<u32>;

        /// Leave candidates delay.
        #[pallet::constant]
        type LeaveCandidatesDelay: Get<u32>;

        /// Leave stakers delay.
        #[pallet::constant]
        type LeaveStakersDelay: Get<u32>;

        /// Candidate lower self bond delay.
        #[pallet::constant]
        type CandidateBondLessDelay: Get<u32>;

        /// Revoke stake delay.
        #[pallet::constant]
        type RevokeStakeDelay: Get<u32>;

        /// Treasury round proveider.
        type Treasury: TTreasury<Self>;

        type WeightInfo: weights::WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    /// All stakes by executor and staker.
    #[pallet::storage]
    #[pallet::getter(fn stakes)]
    pub type AllStakes<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::AccountId,
        Twox64Concat,
        T::AccountId,
        BalanceOf<T>,
        OptionQuery,
    >;

    /// Executors' commissions.
    #[pallet::storage]
    #[pallet::getter(fn executor_config)]
    pub type ExecutorConfig<T: Config> = StorageMap<_, Identity, T::AccountId, ExecutorInfo, OptionQuery>;

    /// The pool of executor candidates, each with their total backing stake.
    #[pallet::storage]
    #[pallet::getter(fn candidate_pool)]
    pub(crate) type CandidatePool<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    /// Get executor candidate info associated with an account.
    #[pallet::storage]
    #[pallet::getter(fn candidate_info)]
    pub(crate) type CandidateInfo<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, CandidateMetadata<BalanceOf<T>>, OptionQuery>;

    /// Effective size of the executor active set.
    #[pallet::storage]
    #[pallet::getter(fn active_set_size)]
    pub type ActiveSetSize<T: Config> = StorageValue<_, Range<u32>, ValueQuery>;

    /// Active set of executors.
    #[pallet::storage]
    #[pallet::getter(fn active_set)]
    pub type ActiveSet<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// Get staker state associated with an account.
    #[pallet::storage]
    #[pallet::getter(fn staker_info)]
    pub(crate) type StakerInfo<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        StakerMetadata<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn at_stake)]
    /// Snapshot of collator delegation stake at the start of the round
    pub type AtStake<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        RoundIndex,
        Twox64Concat,
        T::AccountId,
        ExecutorSnapshot<T::AccountId, BalanceOf<T>>,
        ValueQuery,
    >;

    /// Outstanding staking requests per executor.
    #[pallet::storage]
    #[pallet::getter(fn stake_scheduled_requests)]
    pub(crate) type ScheduledStakingRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<ScheduledStakingRequest<T::AccountId, BalanceOf<T>>>,
        ValueQuery,
    >;

    /// Outstanding configuration change per executor.
    #[pallet::storage]
    #[pallet::getter(fn stake_scheduled_requests)]
    pub(crate) type ScheduledConfigurationRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        ScheduledConfigurationRequest,
        ValueQuery,
    >;

    /// Top stakes by executor candidate.
    #[pallet::storage]
    #[pallet::getter(fn top_stakes)]
    pub(crate) type TopStakes<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Stakes<T::AccountId, BalanceOf<T>>, OptionQuery>;

    /// Bottom stakes by executor candidate.
    #[pallet::storage]
    #[pallet::getter(fn bottom_stakes)]
    pub(crate) type BottomStakes<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Stakes<T::AccountId, BalanceOf<T>>, OptionQuery>;

    /// Total capital locked by this staking pallet.
    #[pallet::storage]
    #[pallet::getter(fn tvl)]
    pub(crate) type Total<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Total staked of a round's active set of executors.
	#[pallet::storage]
	#[pallet::getter(fn staked)]
	pub type Staked<T: Config> = StorageMap<_, Twox64Concat, RoundIndex, BalanceOf<T>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Sets the executor active set's size.
        #[pallet::weight(10_000)] //TODO
        pub fn set_active_set_size(origin: OriginFor<T>, size: u32) -> DispatchResult {
            ensure_root(origin)?;

            let old = <ActiveSetSize<T>>::get();

            <ActiveSetSize<T>>::put(size);

            Self::deposit_event(Event::ActiveSetSizeSet { old, new: size, });

            Ok(())
        }
        
        ///////////////////////////////////////////////////////////////////////
        // ExposesT::MinBond via pallet-executor-staking Config trait ✅
        // implements call fn bond(...) & fn unbond(...) 
        // implements call fn setup_executor( params: { commission_rate, !maybe! nominators_risk_ratio })
        // if executor already registered schedule update for T::ScheduleDelay time (by default 14 days)
        // implements updates call fn schedule_params_update( params ) ->
        // implements fn join_candidate() which makes executor being consider for an active set
        // implements fn schedule_leave_candidate() which makes executor not being consider for an active set anymore after T::ScheduleDelay (14 days)
        // implements fn self::active_set() that selects the top T::ActiveExecutors (make it 128)
        // Stakers for Executors
        // implements call fn stake(executor: AccountId, stake: Balance)
        // implements call fn schedule_unstake(executor: AccountId, stake: Balance) after T::ScheduleDelay (14 days)
        ///////////////////////////////////////////////////////////////////////


        /// Increases an executor's bond.
        #[pallet::weight(10_000)] //TODO
        pub fn bond(origin: OriginFor<T>, amount:  BalanceOf<T>,) -> DispatchResult {
            todo!();
        }

        /// Decreases an executor's bond.
        #[pallet::weight(10_000)] //TODO
        pub fn unbond(origin: OriginFor<T>, amount: BalanceOf<T>,) -> DispatchResult {
            todo!();
        }

        /// Configures an executor's economics.
        /// The parameters must adhere to `T::MaxCommission` and `T::MaxRisk`.
        /// If this applies to an already configured executor `T::ConfigureExecutorDelay` is enforced.
        #[pallet::weight(10_000)] //TODO
        pub fn configure_executor(origin: OriginFor<T>, commission: Percent, risk: Percent) {
            let executor = ensure_signed(origin);

            ensure!(commmission.lte(<MaxCommission<T>>::get()), <Error<T>>::TooMuchCommission);

            ensure!(risk.lte(<MaxRisk<T>>::get()), <Error<T>>::TooMuchRisk);

            // enforcing an executor config change delay to accomodate 
            // a grace period allowing stakers to be notified and react
            if <ExecutorConfig<T>>::contains_key(executor) {
                let when_executable = <ConfigureExecutorDelay<T>>::get().saturating_add(<frame_system::Pallet<T>>::block_number().into());

                <ScheduledConfigurationRequests<T>>::insert(executor, ScheduledConfigurationRequest {
                    when_executable,
                    commission,
                    risk,
                })
            } else {
                <ExecutorConfig<T>>::insert(executor, ExecutorInfo { commission, risk });

                Self::deposit_event(Event::ExecutorConfigured {executor, commission, risk });
            }

            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            let executable_requests = <ScheduledConfigurationRequests<T>>::iter()
            .filter(|executor,req| req.when_executable == n)

            // scheduled configuration request have been validated when persisted
            for (executor, req) in executable_requests {
                <ScheduledConfigurationRequests<T>>::remove(executor);

                <ExecutorConfig<T>>::insert(executor, ExecutorInfo { commission: req.commission, risk: req.risk });

                Self::deposit_event(Event::ExecutorConfigured {executor, commission, risk });
            }

            419 // TODO
        }

        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(_n: T::BlockNumber) {}
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Account joined the set of executor candidates.
        ExecutorCandidateJoined {
            candidate: T::AccountId,
            value_locked: BalanceOf<T>,
            total_value_locked: BalanceOf<T>,
        },
        /// Candidate selected for executors. Total stake includes all stakings.
        ExecutorChosen {
            executor: T::AccountId,
            round: RoundIndex,
            total_counted: BalanceOf<T>,
        },
        /// Candidate requested to decrease a self bond.
        CandidateBondLessRequested {
            candidate: T::AccountId,
            amount: BalanceOf<T>,
            execute_round: RoundIndex,
        },
        /// Candidate has increased a self bond.
        CandidateBondedMore {
            candidate: T::AccountId,
            amount: BalanceOf<T>,
            total_bond: BalanceOf<T>,
        },
        /// Candidate has decreased a self bond.
        CandidateBondedLess {
            candidate: T::AccountId,
            amount: BalanceOf<T>,
            total_bond: BalanceOf<T>,
        },
        /// Candidate temporarily leave the set of executor candidates without unbonding.
        CandidateWentOffline { candidate: T::AccountId },
        /// Candidate rejoins the set of executor candidates.
        CandidateBackOnline { candidate: T::AccountId },
        /// Candidate has requested to leave the set of candidates.
        CandidateExitScheduled {
            candidate: T::AccountId,
            exit_allowed_round: RoundIndex,
            scheduled_exit: RoundIndex,
        },
        /// Cancelled request to leave the set of candidates.
        CandidateExitCancelled { candidate: T::AccountId },
        /// Cancelled request to decrease candidate's bond.
        CandidateBondLessCancelled {
            candidate: T::AccountId,
            amount: BalanceOf<T>,
            execute_round: RoundIndex,
        },
        /// Candidate has left the set of candidates.
        CandidateLeft {
            candidate: T::AccountId,
            unlocked_amount: BalanceOf<T>,
            total_value_locked: BalanceOf<T>,
        },
        /// Delegator requested to decrease a bond for the executor candidate.
        StakeDecreaseScheduled {
            staker: T::AccountId,
            candidate: T::AccountId,
            amount: BalanceOf<T>,
            execute_round: RoundIndex,
        },
        // Delegation increased.
        StakeIncreased {
            staker: T::AccountId,
            candidate: T::AccountId,
            amount: BalanceOf<T>,
            in_top: bool,
        },
        // Delegation decreased.
        StakeDecreased {
            staker: T::AccountId,
            candidate: T::AccountId,
            amount: BalanceOf<T>,
            in_top: bool,
        },
        /// Delegator requested to leave the set of stakers.
        StakerExitScheduled {
            round: RoundIndex,
            staker: T::AccountId,
            scheduled_exit: RoundIndex,
        },
        /// Delegator requested to revoke stake.
        StakeRevocationScheduled {
            round: RoundIndex,
            staker: T::AccountId,
            candidate: T::AccountId,
            scheduled_exit: RoundIndex,
        },
        /// Delegator has left the set of stakers.
        StakerLeft {
            staker: T::AccountId,
            unstaked: BalanceOf<T>,
        },
        /// Delegation revoked.
        StakeRevoked {
            staker: T::AccountId,
            candidate: T::AccountId,
            unstaked: BalanceOf<T>,
        },
        /// Delegation kicked.
        StakeKicked {
            staker: T::AccountId,
            candidate: T::AccountId,
            unstaked: BalanceOf<T>,
        },
        /// Cancelled a pending request to exit the set of stakers.
        StakerExitCancelled { staker: T::AccountId },
        /// Cancelled request to change an existing stake.
        StakeRequestCancelled {
            staker: T::AccountId,
            cancelled_request: CancelledScheduledStakingRequest<BalanceOf<T>>,
            executor: T::AccountId,
        },
        /// New stake (increase of the existing one).
        Delegation {
            staker: T::AccountId,
            candidate: T::AccountId,
            locked_amount: BalanceOf<T>,
            staker_position: StakerAdded<BalanceOf<T>>,
        },
        /// Delegation from candidate state has been removed.
        StakerLeftCandidate {
            staker: T::AccountId,
            candidate: T::AccountId,
            unstaked: BalanceOf<T>,
            total_candidate_staked: BalanceOf<T>,
        },
        /// Paid the account (staker or executor) the balance as liquid rewards.
        Rewarded {
            account: T::AccountId,
            rewards: BalanceOf<T>,
        },
        /// Set total selected candidates to this value.
        ActiveSetSizeSet { old: u32, new: u32 },
        /// An executor configured its terms of operations.
        ExecutorConfigured {
            executor: T::AccountId,
            commission: Percent,
            risk_ratio: Percent,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoSuchCandidate,
        NoSuchStake,
        NoSuchPendingStakeRequest,
        NoSuchPendingCandidateRequest,
        CandidateAlreadyLeaving,
        CandidateCannotLeaveYet,
        PendingStakeRequestAlreadyExists,
        PendingStakeRequestNotDueYet,
        NoSuchStaker,
        StakeBelowMin,
        StakerBondBelowMin,
        StakerNotLeaving,
        StakerAlreadyLeaving,
        CandidateNotLeaving,
        StakerCannotLeaveYet,
        PendingCandidateRequestAlreadyExists,
        CandidateBondBelowMin,
        PendingCandidateRequestNotDueYet,
        TooLowStakeCountToLeaveStakers,
        CannotDelegateLessThanOrEqualToLowestBottomWhenFull,
        TooMuchCommission,
        TooMuchRisk
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub active_set_size: u32,
        pub max_commission: Percent,
        pub max_risk: Percent,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                active_set_size: 3, // TODO
                max_commission: Percent::from_percent(50), // TODO
                max_risk: Percent::from_percent(50), // TODO
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <ActiveSetSize<T>>::put(self.active_set_size);
            <MaxCommission<T>>::put(self.max_commission);
            <MaxRisk<T>>::put(self.max_risk);
        }
    }

    impl<T: Config> Pallet<T> {
        // /// Select the top `active_set_size()` candidates from the pool.
        // /// a vec of their AccountIds (in the order of selection)
        // pub fn select_active_set() -> Vec<T::AccountId> {
        // 	let mut candidates = <CandidatePool<T>>::get().0;
        // 	// order candidates by stake (least to greatest so requires `rev()`)
        // 	candidates.sort_by(|a, b| a.amount.cmp(&b.amount));
        // 	let top_n = <ActiveSetSize<T>>::get().ideal as usize;
        // 	// choose the top `active_set_size()` qualified candidates, ordered by stake
        // 	let mut executors = candidates
        // 		.into_iter()
        // 		.rev()
        // 		.take(top_n)
        // 		.filter(|x| x.amount >= T::MinExecutorBond::get())
        // 		.map(|x| x.owner)
        // 		.collect::<Vec<T::AccountId>>();

        //     if executors.len() < <ActiveSetSize<T>>::get().min as usize {
        //         //TODO handle to few eligible candidates for active set
        //     }

        // 	executors.sort();
        // 	executors
        // }

        /// Whether given identity is a staker.
        pub fn is_staker(acc: &T::AccountId) -> bool {
            <StakerInfo<T>>::get(acc).is_some()
        }

        /// Whether given identity is an executor candidate.
        pub fn is_candidate(acc: &T::AccountId) -> bool {
            <CandidateInfo<T>>::get(acc).is_some()
        }

        /// Whether given identity is part of the eurrents executor active set.
        pub fn is_active(acc: &T::AccountId) -> bool {
            <ActiveSet<T>>::get().binary_search(acc).is_ok()
        }

        /// Caller must ensure candidate is active before calling.
        pub(crate) fn update_active(candidate: T::AccountId, total: BalanceOf<T>) {
            let mut candidates = <CandidatePool<T>>::get();
            candidates.remove(&Bond::from_owner(candidate.clone()));
            candidates.insert(Bond {
                owner: candidate,
                amount: total,
            });
            <CandidatePool<T>>::put(candidates);
        }

        /// Remove stake from candidate state
        /// Amount input should be retrieved from staker and it informs the storage lookups
        pub(crate) fn staker_leaves_candidate(
            candidate: T::AccountId,
            staker: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let mut candidate_info =
                <CandidateInfo<T>>::get(&candidate).ok_or(Error::<T>::NoSuchCandidate)?;
            candidate_info.rm_stake_if_exists::<T>(&candidate, staker.clone(), amount)?;
            T::Currency::unreserve(&staker, amount);
            let new_total_locked = <Total<T>>::get().saturating_sub(amount);
            <Total<T>>::put(new_total_locked);
            let new_total = candidate_info.total_counted;
            <CandidateInfo<T>>::insert(&candidate, candidate_info);
            Self::deposit_event(Event::StakerLeftCandidate {
                staker,
                candidate,
                unstaked: amount,
                total_candidate_staked: new_total,
            });
            Ok(())
        }

        // fn prepare_staking_payouts(now: RoundIndex) {
        // 	// payout is now - delay rounds ago => now - delay > 0 else return early
        // 	let delay = T::RewardPaymentDelay::get();
        // 	if now <= delay {
        // 		return;
        // 	}
        // 	let round_to_payout = now.saturating_sub(delay);
        // 	let total_points = <Points<T>>::get(round_to_payout);
        // 	if total_points.is_zero() {
        // 		return;
        // 	}
        // 	let total_staked = <Staked<T>>::take(round_to_payout);
        // 	let total_issuance = Self::compute_issuance(total_staked);
        // 	let mut left_issuance = total_issuance;
        // 	// reserve portion of issuance for parachain bond account
        // 	let bond_config = <ParachainBondInfo<T>>::get();
        // 	let parachain_bond_reserve = bond_config.percent * total_issuance;
        // 	if let Ok(imb) =
        // 		T::Currency::deposit_into_existing(&bond_config.account, parachain_bond_reserve)
        // 	{
        // 		// update round issuance iff transfer succeeds
        // 		left_issuance = left_issuance.saturating_sub(imb.peek());
        // 		Self::deposit_event(Event::ReservedForParachainBond {
        // 			account: bond_config.account,
        // 			value: imb.peek(),
        // 		});
        // 	}

        // 	let payout = DelayedPayout {
        // 		round_issuance: total_issuance,
        // 		total_staking_reward: left_issuance,
        // 		executor_commission: <Commissions<T>>::get(),
        // 	};

        // 	<DelayedPayouts<T>>::insert(round_to_payout, payout);
        // }

        // /// Wrapper around pay_one_executor_reward which handles the following logic:
        // /// * whether or not a payout needs to be made
        // /// * cleaning up when payouts are done
        // /// * returns the weight consumed by pay_one_executor_reward if applicable
        // fn handle_delayed_payouts(now: RoundIndex) -> Weight {
        // 	let delay = T::RewardPaymentDelay::get();

        // 	// don't underflow uint
        // 	if now < delay {
        // 		return 0u64.into();
        // 	}

        // 	let paid_for_round = now.saturating_sub(delay);

        // 	if let Some(payout_info) = <DelayedPayouts<T>>::get(paid_for_round) {
        // 		let result = Self::pay_one_executor_reward(paid_for_round, payout_info);
        // 		if result.0.is_none() {
        // 			// result.0 indicates whether or not a payout was made
        // 			// clean up storage items that we no longer need
        // 			<DelayedPayouts<T>>::remove(paid_for_round);
        // 			<Points<T>>::remove(paid_for_round);
        // 		}
        // 		result.1 // weight consumed by pay_one_executor_reward
        // 	} else {
        // 		0u64.into()
        // 	}
        // }

        // /// Payout a single executor from the given round.
        // ///
        // /// Returns an optional tuple of (Collator's AccountId, total paid)
        // /// or None if there were no more payouts to be made for the round.
        // pub(crate) fn pay_one_executor_reward(
        // 	paid_for_round: RoundIndex,
        // 	payout_info: DelayedPayout<BalanceOf<T>>,
        // ) -> (Option<(T::AccountId, BalanceOf<T>)>, Weight) {
        // 	// TODO: it would probably be optimal to roll Points into the DelayedPayouts storage
        // 	// item so that we do fewer reads each block
        // 	let total_points = <Points<T>>::get(paid_for_round);
        // 	if total_points.is_zero() {
        // 		// TODO: this case is obnoxious... it's a value query, so it could mean one of two
        // 		// different logic errors:
        // 		// 1. we removed it before we should have
        // 		// 2. we called pay_one_executor_reward when we were actually done with deferred
        // 		//    payouts
        // 		log::warn!("pay_one_executor_reward called with no <Points<T>> for the round!");
        // 		return (None, 0u64.into());
        // 	}

        // 	let mint = |amt: BalanceOf<T>, to: T::AccountId| {
        // 		if let Ok(amount_transferred) = T::Currency::deposit_into_existing(&to, amt) {
        // 			Self::deposit_event(Event::Rewarded {
        // 				account: to.clone(),
        // 				rewards: amount_transferred.peek(),
        // 			});
        // 		}
        // 	};

        // 	let executor_fee = payout_info.executor_commission;
        // 	let executor_issuance = executor_fee * payout_info.round_issuance;

        // 	if let Some((executor, pts)) =
        // 		<AwardedPts<T>>::iter_prefix(paid_for_round).drain().next()
        // 	{
        // 		let mut extra_weight = 0;
        // 		let pct_due = Perbill::from_rational(pts, total_points);
        // 		let total_paid = pct_due * payout_info.total_staking_reward;
        // 		let mut amt_due = total_paid;
        // 		// Take the snapshot of block author and stakes
        // 		let state = <AtStake<T>>::take(paid_for_round, &executor);
        // 		let num_stakers = state.stakes.len();
        // 		if state.stakes.is_empty() {
        // 			// solo executor with no stakers
        // 			mint(amt_due, executor.clone());
        // 			extra_weight += T::OnCollatorPayout::on_executor_payout(
        // 				paid_for_round,
        // 				executor.clone(),
        // 				amt_due,
        // 			);
        // 		} else {
        // 			// pay executor first; commission + due_portion
        // 			let executor_pct = Perbill::from_rational(state.bond, state.total);
        // 			let commission = pct_due * executor_issuance;
        // 			amt_due = amt_due.saturating_sub(commission);
        // 			let executor_reward = (executor_pct * amt_due).saturating_add(commission);
        // 			mint(executor_reward, executor.clone());
        // 			extra_weight += T::OnCollatorPayout::on_executor_payout(
        // 				paid_for_round,
        // 				executor.clone(),
        // 				executor_reward,
        // 			);
        // 			// pay stakers due portion
        // 			for Bond { owner, amount } in state.stakes {
        // 				let percent = Perbill::from_rational(amount, state.total);
        // 				let due = percent * amt_due;
        // 				if !due.is_zero() {
        // 					mint(due, owner.clone());
        // 				}
        // 			}
        // 		}

        // 		(
        // 			Some((executor, total_paid)),
        // 			T::WeightInfo::pay_one_executor_reward(num_stakers as u32) + extra_weight,
        // 		)
        // 	} else {
        // 		// Note that we don't clean up storage here; it is cleaned up in
        // 		// handle_delayed_payouts()
        // 		(None, 0u64.into())
        // 	}
        // }

        /// Selects executors into the active set.
        /// Best as in most cumulatively supported in terms of stake.
        /// Returns [executor_count, stake_count, total staked].
        pub fn select_active_set(current_round: RoundIndex) -> (u32, u32, BalanceOf<T>) {
        	let mut candidates = <CandidatePool<T>>::get().0;
        	// order candidates by stake (least to greatest so requires `rev()`)
        	candidates.sort_by(|a, b| a.amount.cmp(&b.amount));
        	let top_n = <ActiveSetSize<T>>::get().ideal as usize;
        	// choose the top qualified candidates, ordered by stake
        	let mut executors = candidates
        		.into_iter()
        		.rev()
        		.take(top_n)
        		.filter(|x| x.amount >= T::MinExecutorBond::get())
        		.map(|x| x.owner)
        		.collect::<Vec<T::AccountId>>();

        	executors.sort();
        	// executors


            ////////// TBC

            let (mut executor_count, mut stake_count, mut total) =
            (0u32, 0u32, BalanceOf::<T>::zero());

        if executors.is_empty() || executors.len() < <ActiveSetSize<T>>::get().min as usize {
            // failed to select the minimum number of executors
            // => select executors from previous round
            let last_round = current_round.saturating_sub(1u32);
            let mut total_per_candidate: BTreeMap<T::AccountId, BalanceOf<T>> = BTreeMap::new();
            // set this round AtStake to last round AtStake
            for (account, snapshot) in <AtStake<T>>::iter_prefix(last_round) {
                executor_count = executor_count.saturating_add(1u32);
                stake_count = stake_count.saturating_add(snapshot.stakes.len() as u32);
                total = total.saturating_add(snapshot.total);
                total_per_candidate.insert(account.clone(), snapshot.total);
                <AtStake<T>>::insert(current_round, account, snapshot);
            }
            // `ActiveSet` remains unchanged from last round
            // emit ExecutorChosen event for tools that use this event
            for candidate in <ActiveSet<T>>::get() {
                let snapshot_total = total_per_candidate
                    .get(&candidate)
                    .expect("all selected candidates have snapshots");
            
                Self::deposit_event(Event::ExecutorChosen {
                    round: current_round,
                    executor: candidate,
                    total_counted: *snapshot_total,
                })
            }
            return (executor_count, stake_count, total)
        }

        // snapshot exposure for round for weighting reward distribution
        for account in executors.iter() {
            let state = <CandidateInfo<T>>::get(account)
                .expect("all candidates must have info");

            executor_count = executor_count.saturating_add(1u32);
            stake_count = stake_count.saturating_add(state.stake_count);
            total = total.saturating_add(state.total_counted);
            let top_rewardable_stakes = Self::get_rewardable_stakers(&account);

            <AtStake<T>>::insert(now, account, ExecutorSnapshot {
                bond: state.bond,
                stakes: top_rewardable_stakes,
                total: state.total_counted,
            });

            Self::deposit_event(Event::ExecutorChosen {
                round: now,
                executor: account.clone(),
                total_counted: state.total_counted,
            });
        }

        // insert canonical executor set
        <ActiveSet<T>>::put(executors);

        (executor_count, stake_count, total)
        }

        /// Apply the staker intent for revoke and decrease in order to build the
        /// effective list of stakers with their intended bond amount.
        ///
        /// This will:
        /// - if [DelegationChange::Revoke] is outstanding, set the bond amount to 0.
        /// - if [DelegationChange::Decrease] is outstanding, subtract the bond by specified amount.
        /// - else, do nothing
        ///
        /// The intended bond amounts will be used while calculating rewards.
        fn get_rewardable_stakers(
            executor: &T::AccountId,
        ) -> Vec<Bond<T::AccountId, BalanceOf<T>>> {
            let requests = <ScheduledStakingRequests<T>>::get(executor)
                .into_iter()
                .map(|x| (x.staker, x.action))
                .collect::<BTreeMap<_, _>>();

            <TopStakes<T>>::get(executor)
                .expect("all members of CandidateQ must be candidates")
                .stakes
                .into_iter()
                .map(|mut bond| {
                    bond.amount = match requests.get(&bond.owner) {
                        None => bond.amount,
                        Some(StakingAction::Revoke(_)) => {
                            log::warn!(
                                "reward for staker '{:?}' set to zero due to pending \
								revoke request",
                                bond.owner
                            );
                            BalanceOf::<T>::zero()
                        },
                        Some(StakingAction::Decrease(amount)) => {
                            log::warn!(
                                "reward for staker '{:?}' reduced by set amount due to pending \
								decrease request",
                                bond.owner
                            );
                            bond.amount.saturating_sub(*amount)
                        },
                    };

                    bond
                })
                .collect()
        }
    }
}
