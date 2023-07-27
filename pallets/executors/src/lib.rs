//! <!-- markdown-link-check-disable -->
//! # Executor staking pallet
//! </pre></p></details>

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use codec::Decode;
pub use pallet::*;
use sp_runtime::DispatchError;
use xp_channel::{XbiFormat, XbiMetadata};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod stakes;
pub mod staking_actions;
pub mod subject_metadata;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use crate::SfxWithMetadataNewtype;

    use super::{
        stakes::Stakes,
        subject_metadata::{CandidateMetadata, StakerMetadata},
        weights,
    };
    use core::ops::Mul;
    use frame_support::{
        pallet_prelude::*,
        traits::{tokens::WithdrawReasons, Currency, LockableCurrency, ReservableCurrency},
    };
    use frame_system::{ensure_root, pallet_prelude::*};
    use pallet_circuit::XExecSignalId;
    use sp_runtime::{
        traits::{One, Saturating, Zero},
        AccountId32, Percent,
    };
    use sp_std::collections::btree_map::BTreeMap;
    pub use substrate_abi::{SubstrateAbiConverter as Sabi, TryConvert, Value256, ValueMorphism};
    use t3rn_primitives::{
        clock::Clock,
        common::{OrderedSet, Range, RoundIndex},
        executors::{
            Bond, CancelledScheduledStakingRequest, ExecutorInfo, ExecutorSnapshot,
            Fixtures as StakingFixtures, ScheduledConfigurationRequest, ScheduledStakingRequest,
            StakeAdjust, StakerAdded, StakingAction, EXECUTOR_LOCK_ID, STAKER_LOCK_ID,
        },
        monetary::DECIMALS,
    };
    use t3rn_types::sfx::SideEffect;
    use xp_channel::traits::XbiInstructionHandler;
    pub use xp_format::{Fees, XbiFormat, XbiInstruction, XbiMetadata};

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>
            + ReservableCurrency<Self::AccountId>
            + LockableCurrency<Self::AccountId>;

        // WIP
        // // /// Range for the target executor active set size.
        // // /// The ideal is applied during genesis as default.
        // // #[pallet::constant]
        // // type ActiveSetSize: Get<Range<u32>>;

        // // /// Protocol enforced maximum executor commission.
        // // #[pallet::constant]
        // // type MaxCommission: Get<Percent>;

        // // /// Protocol enforced maximum executor risk reward ratio.
        // // #[pallet::constant]
        // // type MaxRisk: Get<Percent>;

        // /// Minimum stake required for any candidate to be considered for the active set.
        // #[pallet::constant]
        // type MinExecutorBond: Get<BalanceOf<Self>>;

        // /// Minimum stake required for any candidate to be considered as candidate.
        // #[pallet::constant]
        // type MinCandidateBond: Get<BalanceOf<Self>>;

        // /// Minimum stake for any registered on-chain account to stake.
        // /// Requirement is checked on every staking action after the first.
        // #[pallet::constant]
        // type MinAtomicStake: Get<BalanceOf<Self>>;

        // /// Minimum stake for any registered on-chain account to be a staker.
        // /// Requirement checked at first staking action.
        // #[pallet::constant]
        // type MinTotalStake: Get<BalanceOf<Self>>;

        // /// Maximum top stakes per candidate.
        // #[pallet::constant]
        // type MaxTopStakesPerCandidate: Get<u32>;

        // /// Maximum bottom stakes per candidate.
        // #[pallet::constant]
        // type MaxBottomStakesPerCandidate: Get<u32>;

        // /// Maximum stakings per staker.
        // #[pallet::constant]
        // type MaxStakesPerStaker: Get<u32>;

        // /// Delay applied when changing an executor's configuration.
        // #[pallet::constant]
        // type ConfigureExecutorDelay: Get<u32>;

        // /// Leave candidates delay.
        // #[pallet::constant]
        // type LeaveCandidatesDelay: Get<u32>;

        // /// Leave stakers delay.
        // #[pallet::constant]
        // type LeaveStakersDelay: Get<u32>;

        // /// Candidate lower self bond delay.
        // #[pallet::constant]
        // type CandidateBondLessDelay: Get<u32>;

        // /// Revoke stake delay.
        // #[pallet::constant]
        // type RevokeStakeDelay: Get<u32>;

        /// Treasury round proveider.
        type Treasury: Clock<Self>;

        type WeightInfo: weights::WeightInfo;

        /// Allow executors to execute side effects on circuit
        type InstructionHandler: xp_channel::traits::XbiInstructionHandler<Self::Origin>;

        // TODO: We might not need this here, maybe we will just inject this into the pallets that need it, although here is a decent entrypoint for it.
        /// Allow other pallets in circuit to send messages over xbi
        type Xbi: xs_channel::Sender<xp_channel::Message>;
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

    /// Protocol enforced staking fixtures.
    #[pallet::storage]
    #[pallet::getter(fn fixtures)]
    pub type Fixtures<T: Config> = StorageValue<_, StakingFixtures<BalanceOf<T>>, ValueQuery>;

    /// Executors' commission and risk rates.
    #[pallet::storage]
    #[pallet::getter(fn executor_config)]
    pub type ExecutorConfig<T: Config> =
        StorageMap<_, Identity, T::AccountId, ExecutorInfo, OptionQuery>;

    /// The pool of executor candidates, each with their total backing stake.
    #[pallet::storage]
    #[pallet::getter(fn candidate_pool)]
    pub type CandidatePool<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    /// Get executor candidate info associated with an account.
    #[pallet::storage]
    #[pallet::getter(fn candidate_info)]
    pub type CandidateInfo<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, CandidateMetadata<BalanceOf<T>>, OptionQuery>;

    /// Active set of executors.
    #[pallet::storage]
    #[pallet::getter(fn active_set)]
    pub type ActiveSet<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// Get staker state associated with an account.
    #[pallet::storage]
    #[pallet::getter(fn staker_info)]
    pub type StakerInfo<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        StakerMetadata<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn at_stake)]
    /// Snapshot of executor delegation stake at the start of the round
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
    #[pallet::getter(fn scheduled_staking_requests)]
    pub type ScheduledStakingRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<ScheduledStakingRequest<T::AccountId, BalanceOf<T>>>,
        ValueQuery,
    >;

    /// Outstanding configuration change per executor.
    #[pallet::storage]
    #[pallet::getter(fn scheduled_configration_requests)]
    pub type ScheduledConfigurationRequests<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, ScheduledConfigurationRequest, OptionQuery>;

    /// Top stakes by executor candidate.
    #[pallet::storage]
    #[pallet::getter(fn top_stakes)]
    pub type TopStakes<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Stakes<T::AccountId, BalanceOf<T>>, OptionQuery>;

    /// Bottom stakes by executor candidate.
    #[pallet::storage]
    #[pallet::getter(fn bottom_stakes)]
    pub type BottomStakes<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Stakes<T::AccountId, BalanceOf<T>>, OptionQuery>;

    /// Total capital locked by this staking pallet.
    #[pallet::storage]
    #[pallet::getter(fn total_value_locked)]
    pub type Total<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Total staked of a round's active set of executors.
    #[pallet::storage]
    #[pallet::getter(fn staked)]
    pub type Staked<T: Config> = StorageMap<_, Twox64Concat, RoundIndex, BalanceOf<T>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Sets all protocol enforced staking fixtures.
        #[pallet::weight(10_000)] //TODO
        pub fn set_fixtures(
            origin: OriginFor<T>,
            fixtures: StakingFixtures<BalanceOf<T>>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(fixtures.are_valid(), <Error<T>>::FixturesCannotBeZero);

            <Fixtures<T>>::put(&fixtures);

            Self::deposit_event(Event::FixturesConfigured {
                active_set_size: fixtures.active_set_size,
                max_commission: fixtures.max_commission,
                max_risk: fixtures.max_risk,
                min_executor_bond: fixtures.min_executor_bond,
                min_candidate_bond: fixtures.min_candidate_bond,
                min_atomic_stake: fixtures.min_atomic_stake,
                min_total_stake: fixtures.min_total_stake,
                max_top_stakes_per_candidate: fixtures.max_top_stakes_per_candidate,
                max_bottom_stakes_per_candidate: fixtures.max_bottom_stakes_per_candidate,
                max_stakes_per_staker: fixtures.max_stakes_per_staker,
                configure_executor_delay: fixtures.configure_executor_delay,
                leave_candidates_delay: fixtures.leave_candidates_delay,
                leave_stakers_delay: fixtures.leave_stakers_delay,
                candidate_bond_less_delay: fixtures.candidate_bond_less_delay,
                revoke_stake_delay: fixtures.revoke_stake_delay,
            });

            Ok(())
        }

        /// Configures an executor's economics.
        /// The parameters must adhere to `T::MaxCommission` and `T::MaxRisk`.
        /// If this applies to an already configured executor `T::ConfigureExecutorDelay` is enforced,
        /// in case of first time configuration it will be effective immediately.
        #[pallet::weight(10_000)] //TODO
        pub fn schedule_configure_executor(
            origin: OriginFor<T>,
            commission: Percent,
            risk: Percent,
        ) -> DispatchResult {
            let executor = ensure_signed(origin)?;
            let fixtures = <Fixtures<T>>::get();

            ensure!(
                !commission.gt(&fixtures.max_commission),
                <Error<T>>::TooMuchCommission
            );
            ensure!(!risk.gt(&fixtures.max_risk), <Error<T>>::TooMuchRisk);

            // enforcing an executor config change delay to accomodate
            // a grace period allowing stakers to be notified and react
            if <ExecutorConfig<T>>::contains_key(&executor) {
                let when_executable = T::Treasury::current_round()
                    .index
                    .saturating_add(fixtures.configure_executor_delay);

                <ScheduledConfigurationRequests<T>>::insert(
                    executor,
                    ScheduledConfigurationRequest {
                        when_executable,
                        commission,
                        risk,
                    },
                )
            } else {
                <ExecutorConfig<T>>::insert(&executor, ExecutorInfo { commission, risk });

                Self::deposit_event(Event::ExecutorConfigured {
                    executor,
                    commission,
                    risk,
                });
            }

            Ok(())
        }

        /// Executes a scheduled excutor configuration request if due yet.
        #[pallet::weight(10_000)] //TODO
        pub fn execute_configure_executor(
            origin: OriginFor<T>,
            executor: T::AccountId,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            let current_round_index = T::Treasury::current_round().index;
            let req = <ScheduledConfigurationRequests<T>>::get(&executor)
                .ok_or(<Error<T>>::NoSuchConfigurationRequest)?;

            ensure!(
                req.when_executable <= current_round_index,
                <Error<T>>::ConfigurationRequestNotDueYet
            );

            <ScheduledConfigurationRequests<T>>::remove(&executor);

            // scheduled configuration request have been validated when persisted
            <ExecutorConfig<T>>::insert(
                &executor,
                ExecutorInfo {
                    commission: req.commission,
                    risk: req.risk,
                },
            );

            Self::deposit_event(Event::ExecutorConfigured {
                executor,
                commission: req.commission,
                risk: req.risk,
            });

            Ok(())
        }

        /// Cancels a scheduled executor configuration request if not due yet.
        /// The extrinsic must be signed by the executor itself.
        #[pallet::weight(10_000)] //TODO
        pub fn cancel_configure_executor(origin: OriginFor<T>) -> DispatchResult {
            let executor = ensure_signed(origin)?;

            if !<ScheduledConfigurationRequests<T>>::contains_key(&executor) {
                return Err(Error::<T>::NoSuchConfigurationRequest.into())
            }

            <ScheduledConfigurationRequests<T>>::remove(&executor);

            Ok(())
        }

        /// Increases an executor's self bond after having joined the candidate pool.
        #[pallet::weight(10_000)] //TODO
        pub fn candidate_bond_more(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let executor = ensure_signed(origin)?;

            let mut state =
                <CandidateInfo<T>>::get(&executor).ok_or(Error::<T>::NoSuchCandidate)?;

            state.bond_more::<T>(executor.clone(), amount)?;

            let (is_active, total_counted) = (state.is_active(), state.total_counted);

            <CandidateInfo<T>>::insert(&executor, state);

            if is_active {
                Self::update_active(executor, total_counted);
            }

            Ok(())
        }

        /// Request by an executor candidate to decrease its self bond.
        #[pallet::weight(10_000)] //TODO
        pub fn schedule_candidate_bond_less(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin)?;

            let mut state =
                <CandidateInfo<T>>::get(&executor).ok_or(Error::<T>::NoSuchCandidate)?;

            let when = state.schedule_bond_less::<T>(amount)?;

            <CandidateInfo<T>>::insert(&executor, state);

            Self::deposit_event(Event::CandidateBondLessRequested {
                candidate: executor,
                amount,
                execute_round: when,
            });

            Ok(().into())
        }

        /// Executes a pending request to adjust an executor's candidate self bond.
        /// Executable by anyone.
        #[pallet::weight(10_000)] //TODO
        pub fn execute_candidate_bond_less(
            origin: OriginFor<T>,
            candidate: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?; // could reward if not candidate self

            let mut state =
                <CandidateInfo<T>>::get(&candidate).ok_or(Error::<T>::NoSuchCandidate)?;

            state.execute_bond_less::<T>(candidate.clone())?;

            <CandidateInfo<T>>::insert(&candidate, state);

            Ok(().into())
        }

        /// Cancel pending request to adjust the executor candidate self bond WIP
        #[pallet::weight(10_000)] //TODO
        pub fn cancel_candidate_bond_less(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin)?;

            let mut state =
                <CandidateInfo<T>>::get(&executor).ok_or(Error::<T>::NoSuchCandidate)?;

            state.cancel_bond_less::<T>(executor.clone())?;

            <CandidateInfo<T>>::insert(&executor, state);

            Ok(().into())
        }

        /// Join the set of executor candidates.
        /// `candidate_count` must at least be the sie of the candidate pool.
        // #[pallet::weight(<T as Config>::WeightInfo::join_candidates(*candidate_count))]
        #[pallet::weight(10_000)] //TODO
        pub fn join_candidates(
            origin: OriginFor<T>,
            bond: BalanceOf<T>,
            candidate_count: u32,
        ) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin)?;

            ensure!(!Self::is_candidate(&executor), Error::<T>::CandidateExists);
            ensure!(!Self::is_staker(&executor), Error::<T>::StakerExists);

            let fixtures = <Fixtures<T>>::get();

            ensure!(
                bond >= fixtures.min_candidate_bond,
                Error::<T>::CandidateBondBelowMin
            );

            let mut candidates = <CandidatePool<T>>::get();
            let old_count = candidates.0.len() as u32;

            ensure!(
                candidate_count >= old_count,
                Error::<T>::TooLowCandidateCountWeightHintJoinCandidates
            );
            ensure!(
                candidates.insert(Bond {
                    owner: executor.clone(),
                    amount: bond
                }),
                Error::<T>::CandidateExists
            );
            ensure!(
                Self::get_executor_stakable_free_balance(&executor) >= bond,
                Error::<T>::InsufficientBalance,
            );

            T::Currency::set_lock(EXECUTOR_LOCK_ID, &executor, bond, WithdrawReasons::all());

            let candidate = CandidateMetadata::new(bond);

            <CandidateInfo<T>>::insert(&executor, candidate);

            let empty_stakes: Stakes<T::AccountId, BalanceOf<T>> = Default::default();

            // insert empty top stakes
            <TopStakes<T>>::insert(&executor, empty_stakes.clone());

            // insert empty bottom stakes
            <BottomStakes<T>>::insert(&executor, empty_stakes);

            <CandidatePool<T>>::put(candidates);

            let new_total = <Total<T>>::get().saturating_add(bond);

            <Total<T>>::put(new_total);

            Self::deposit_event(Event::CandidateJoined {
                account: executor,
                amount_locked: bond,
                total_locked: new_total,
            });

            Ok(().into())
        }

        // #[pallet::weight(<T as Config>::WeightInfo::schedule_leave_candidates(*candidate_count))]
        /// Request to leave the set of candidates. If successful, the account is immediately
        /// removed from the candidate pool to prevent selection as a executor.
        /// `candidate_count` must at least be the sie of the candidate pool.
        #[pallet::weight(10_000)] //TODO
        pub fn schedule_leave_candidates(
            origin: OriginFor<T>,
            candidate_count: u32,
        ) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin)?;

            let mut state =
                <CandidateInfo<T>>::get(&executor).ok_or(Error::<T>::NoSuchCandidate)?;

            let (now, when) = state.schedule_leave::<T>()?;

            let mut candidates = <CandidatePool<T>>::get();

            ensure!(
                candidate_count >= candidates.0.len() as u32,
                Error::<T>::TooLowCandidateCountToLeaveCandidates
            );

            if candidates.remove(&Bond::from_owner(executor.clone())) {
                <CandidatePool<T>>::put(candidates);
            }

            <CandidateInfo<T>>::insert(&executor, state);

            Self::deposit_event(Event::CandidateExitScheduled {
                exit_allowed_round: now,
                candidate: executor,
                scheduled_exit: when,
            });

            Ok(().into())
        }

        // #[pallet::weight(
        // 	<T as Config>::WeightInfo::execute_leave_candidates(*candidate_stake_count)
        // )]
        /// Execute leave candidates request.
        /// Executable by anyone.
        #[pallet::weight(10_000)] //TODO
        pub fn execute_leave_candidates(
            origin: OriginFor<T>,
            candidate: T::AccountId,
            candidate_stake_count: u32,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;

            let state = <CandidateInfo<T>>::get(&candidate).ok_or(Error::<T>::NoSuchCandidate)?;

            ensure!(
                state.stake_count <= candidate_stake_count,
                Error::<T>::TooLowCandidateStakeCountToLeaveCandidates
            );

            state.can_leave::<T>()?;

            let return_stake = |bond: Bond<T::AccountId, BalanceOf<T>>| -> DispatchResult {
                // remove stake from staker state
                let mut state = StakerInfo::<T>::get(&bond.owner).expect(
                    "Executor state and staker state are consistent. 
						Executor state has a record of this stake. Therefore, 
						staker state also has a record. qed.",
                );

                if let Some(remaining) = state.rm_stake(&candidate) {
                    Self::stake_remove_request_with_state(&candidate, &bond.owner, &mut state);

                    if remaining.is_zero() {
                        // we do not remove the scheduled stake requests from other executors
                        // since it is assumed that they were removed incrementally before only the
                        // last stake was left.
                        <StakerInfo<T>>::remove(&bond.owner);

                        T::Currency::remove_lock(STAKER_LOCK_ID, &bond.owner);
                    } else {
                        <StakerInfo<T>>::insert(&bond.owner, state);
                    }
                } else {
                    // TODO: review. we assume here that this staker has no remaining staked
                    // balance, so we ensure the lock is cleared
                    T::Currency::remove_lock(STAKER_LOCK_ID, &bond.owner);
                }

                Ok(())
            };

            // total backing stake is at least the candidate self bond
            let mut total_backing = state.bond;

            // return all top stakes
            let top_stakes =
                <TopStakes<T>>::take(&candidate).expect("CandidateInfo existence checked");

            for bond in top_stakes.stakes {
                return_stake(bond)?;
            }

            total_backing = total_backing.saturating_add(top_stakes.total);

            // return all bottom stakes
            let bottom_stakes =
                <BottomStakes<T>>::take(&candidate).expect("CandidateInfo existence checked");

            for bond in bottom_stakes.stakes {
                return_stake(bond)?;
            }

            total_backing = total_backing.saturating_add(bottom_stakes.total);

            T::Currency::remove_lock(EXECUTOR_LOCK_ID, &candidate);

            <CandidateInfo<T>>::remove(&candidate);

            <ScheduledStakingRequests<T>>::remove(&candidate);

            <TopStakes<T>>::remove(&candidate);

            <BottomStakes<T>>::remove(&candidate);

            let new_total_staked = <Total<T>>::get().saturating_sub(total_backing);

            <Total<T>>::put(new_total_staked);

            Self::deposit_event(Event::CandidateLeft {
                candidate,
                amount_unlocked: total_backing,
                total_locked: new_total_staked,
            });

            Ok(().into())
        }

        // #[pallet::weight(<T as Config>::WeightInfo::cancel_leave_candidates(*candidate_count))]
        /// Cancel open request to leave candidates.  WIP WIP WIP
        /// - only callable by executor account
        /// - result upon successful call is the candidate is active in the candidate pool
        #[pallet::weight(10_000)] //TODO
        pub fn cancel_leave_candidates(
            origin: OriginFor<T>,
            candidate_count: u32,
        ) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin)?;

            let mut state =
                <CandidateInfo<T>>::get(&executor).ok_or(Error::<T>::NoSuchCandidate)?;

            ensure!(state.is_leaving(), Error::<T>::CandidateNotLeaving);

            state.go_online();

            let mut candidates = <CandidatePool<T>>::get();

            ensure!(
                candidates.0.len() as u32 <= candidate_count,
                Error::<T>::TooLowCandidateCountWeightHintCancelLeaveCandidates
            );

            ensure!(
                candidates.insert(Bond {
                    owner: executor.clone(),
                    amount: state.total_counted
                }),
                Error::<T>::AlreadyActive
            );

            <CandidatePool<T>>::put(candidates);

            <CandidateInfo<T>>::insert(&executor, state);

            Self::deposit_event(Event::CandidateExitCancelled {
                candidate: executor,
            });

            Ok(().into())
        }

        /// Temporarily leave the set of executor candidates without unbonding.
        #[pallet::weight(10_000)] //TODO
        pub fn go_offline(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin)?;

            let mut state =
                <CandidateInfo<T>>::get(&executor).ok_or(Error::<T>::NoSuchCandidate)?;

            ensure!(state.is_active(), Error::<T>::AlreadyOffline);

            state.go_offline();

            let mut candidates = <CandidatePool<T>>::get();

            if candidates.remove(&Bond::from_owner(executor.clone())) {
                <CandidatePool<T>>::put(candidates);
            }

            <CandidateInfo<T>>::insert(&executor, state);

            Self::deposit_event(Event::CandidateWentOffline {
                candidate: executor,
            });

            Ok(().into())
        }

        /// Rejoin the set of executor candidates if previously had called `go_offline`.
        #[pallet::weight(10_000)] //TODO
        pub fn go_online(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin)?;

            let mut state =
                <CandidateInfo<T>>::get(&executor).ok_or(Error::<T>::NoSuchCandidate)?;

            ensure!(!state.is_active(), Error::<T>::AlreadyActive);

            ensure!(!state.is_leaving(), Error::<T>::CannotGoOnlineIfLeaving);

            state.go_online();

            let mut candidates = <CandidatePool<T>>::get();

            ensure!(
                candidates.insert(Bond {
                    owner: executor.clone(),
                    amount: state.total_counted
                }),
                Error::<T>::AlreadyActive
            );

            <CandidatePool<T>>::put(candidates);

            <CandidateInfo<T>>::insert(&executor, state);

            Self::deposit_event(Event::CandidateBackOnline {
                candidate: executor,
            });

            Ok(().into())
        }

        /////
        /// If caller is not a staker and not a cexecutor, then join the set of stakers.
        /// If caller is a staker, then makes stake to change their stake state.
        // #[pallet::weight(
        // 	<T as Config>::WeightInfo::delegate(
        // 		*candidate_stake_count,
        // 		*stake_count
        // 	)
        // )]
        #[pallet::weight(10_000)] //TODO
        pub fn stake(
            origin: OriginFor<T>,
            candidate: T::AccountId,
            amount: BalanceOf<T>,
            candidate_stake_count: u32,
            stake_count: u32,
        ) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;

            // check that caller can reserve the amount before any changes to storage
            ensure!(
                Self::get_staker_stakable_free_balance(&staker) >= amount,
                Error::<T>::InsufficientBalance
            );

            let fixtures = <Fixtures<T>>::get();

            let mut staker_state = if let Some(mut state) = <StakerInfo<T>>::get(&staker) {
                // stake after first
                ensure!(
                    amount >= fixtures.min_atomic_stake,
                    Error::<T>::StakeBelowMin
                );
                ensure!(
                    stake_count >= state.stakes.0.len() as u32,
                    Error::<T>::TooLowStakeCountToStake
                );

                ensure!(
                    (state.stakes.0.len() as u32) < fixtures.max_stakes_per_staker,
                    Error::<T>::MaxStakesExceeded
                );

                ensure!(
                    state.add_stake(Bond {
                        owner: candidate.clone(),
                        amount
                    }),
                    Error::<T>::AlreadyStakedCandidate
                );

                // Self::jit_ensure_staker_reserve_migrated(&staker)?;
                state
            } else {
                // first stake
                ensure!(
                    amount >= fixtures.min_atomic_stake,
                    Error::<T>::StakerBondBelowMin
                );

                ensure!(!Self::is_candidate(&staker), Error::<T>::CandidateExists);

                StakerMetadata::new(staker.clone(), candidate.clone(), amount)
            };

            let mut state =
                <CandidateInfo<T>>::get(&candidate).ok_or(Error::<T>::NoSuchCandidate)?;

            ensure!(
                candidate_stake_count >= state.stake_count,
                Error::<T>::TooLowCandidateStakeCountToStake
            );

            let (staker_position, less_total_staked) = state.add_stake::<T>(
                &candidate,
                Bond {
                    owner: staker.clone(),
                    amount,
                },
            )?;

            // TODO: causes redundant free_balance check
            staker_state.adjust_bond_lock::<T>(StakeAdjust::Increase(amount))?;

            // only is_some if kicked the lowest bottom as a consequence of this new stake
            let net_total_increase = if let Some(less) = less_total_staked {
                amount.saturating_sub(less)
            } else {
                amount
            };

            let new_total_locked = <Total<T>>::get().saturating_add(net_total_increase);

            <Total<T>>::put(new_total_locked);

            <CandidateInfo<T>>::insert(&candidate, state);

            <StakerInfo<T>>::insert(&staker, staker_state);

            // <DelegatorReserveToLockMigrations<T>>::insert(&staker, true);

            Self::deposit_event(Event::StakeAdded {
                staker,
                amount_locked: amount,
                candidate,
                staker_position,
            });

            Ok(().into())
        }

        /// Request to leave the set of stakers. If successful, the caller is scheduled to be
        /// allowed to exit via a [DelegationAction::Revoke] towards all existing stakes.
        /// Success forbids future stake requests until the request is invoked or cancelled. WIP WIP WIP
        // #[pallet::weight(<T as Config>::WeightInfo::schedule_leave_stakers())]
        #[pallet::weight(10_000)] //TODO
        pub fn schedule_leave_stakers(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;
            Self::staker_schedule_revoke_all(staker)
        }

        // #[pallet::weight(<T as Config>::WeightInfo::execute_leave_stakers(*stake_count))]
        /// Execute the right to exit the set of stakers and revoke all ongoing stakes.
        #[pallet::weight(10_000)] //TODO
        pub fn execute_leave_stakers(
            origin: OriginFor<T>,
            staker: T::AccountId,
            stake_count: u32,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            Self::staker_execute_scheduled_revoke_all(staker, stake_count)
        }

        // #[pallet::weight(<T as Config>::WeightInfo::cancel_leave_stakers())]
        /// Cancel a pending request to exit the set of stakers. Success clears the pending exit
        /// request (thereby resetting the delay upon another `leave_stakers` call).
        #[pallet::weight(10_000)] //TODO
        pub fn cancel_leave_stakers(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;
            Self::staker_cancel_scheduled_revoke_all(staker)
        }

        // #[pallet::weight(<T as Config>::WeightInfo::schedule_revoke_stake())]
        /// Request to revoke an existing stake. If successful, the stake is scheduled
        /// to be allowed to be revoked via the `execute_stake_request` extrinsic.
        #[pallet::weight(10_000)] //TODO
        pub fn schedule_revoke_stake(
            origin: OriginFor<T>,
            executor: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;
            Self::stake_schedule_revoke(executor, staker)
        }

        // #[pallet::weight(<T as Config>::WeightInfo::staker_bond_more())]
        /// Bond more for stakers wrt a specific executor candidate.
        #[pallet::weight(10_000)] //TODO
        pub fn staker_bond_more(
            origin: OriginFor<T>,
            candidate: T::AccountId,
            more: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;

            ensure!(
                !Self::stake_request_revoke_exists(&candidate, &staker),
                Error::<T>::PendingStakeRevoke
            );

            let mut state = <StakerInfo<T>>::get(&staker).ok_or(Error::<T>::NoSuchStaker)?;

            state.increase_stake::<T>(candidate, more)?;

            Ok(().into())
        }

        // #[pallet::weight(<T as Config>::WeightInfo::schedule_staker_bond_less())]
        /// Request bond less for stakers wrt a specific executor candidate.
        #[pallet::weight(10_000)] //TODO
        pub fn schedule_staker_bond_less(
            origin: OriginFor<T>,
            candidate: T::AccountId,
            less: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;
            Self::stake_schedule_bond_decrease(candidate, staker, less)
        }

        // #[pallet::weight(<T as Config>::WeightInfo::execute_staker_bond_less())]
        /// Execute pending request to change an existing stake
        #[pallet::weight(10_000)] //TODO
        pub fn execute_stake_request(
            origin: OriginFor<T>,
            staker: T::AccountId,
            candidate: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?; // we may want to reward caller if caller != staker
            Self::stake_execute_scheduled_request(candidate, staker)
        }

        // #[pallet::weight(<T as Config>::WeightInfo::cancel_staker_bond_less())]
        /// Cancel request to change an existing stake.
        #[pallet::weight(10_000)] //TODO
        pub fn cancel_stake_request(
            origin: OriginFor<T>,
            candidate: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;
            Self::stake_cancel_request(candidate, staker)
        }

        // Execute a side effect via XBI
        // `xbi` could change to side effects and do the conversions into Xbi messages
        #[pallet::weight(1_000_000_000)]
        // #[pallet::weight(<T as Config>::WeightInfo::execute_xbi())] TODO: benchmark me pls
        #[pallet::call_index(50)]
        pub fn execute_xbi(
            origin: OriginFor<T>, // Active relayer
            xtx_id: XExecSignalId<T>,
            side_effect: SideEffect<T::AccountId, BalanceOf<T>>,
            max_exec_cost: u128,
            max_notifications_cost: u128,
        ) -> DispatchResultWithPostInfo {
            Self::execute_side_effects_with_xbi(
                origin,
                xtx_id,
                side_effect,
                max_exec_cost,
                max_notifications_cost,
            )
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            //TODO listen2round updates then reselect active set + prep payouts
            //TODO//TODO//TODO//TODO//TODO//TODO//TODO//TODO//TODO//TODO//TODO

            419
        }
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
            amount_unlocked: BalanceOf<T>,
            total_locked: BalanceOf<T>,
        },
        /// Staker requested to decrease a bond for the executor candidate.
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
        /// Staker requested to leave the set of stakers.
        StakerExitScheduled {
            round: RoundIndex,
            staker: T::AccountId,
            scheduled_exit: RoundIndex,
        },
        /// Staker requested to revoke stake.
        StakeRevocationScheduled {
            round: RoundIndex,
            staker: T::AccountId,
            candidate: T::AccountId,
            scheduled_exit: RoundIndex,
        },
        /// Staker has left the set of stakers.
        StakerLeft {
            staker: T::AccountId,
            unstaked: BalanceOf<T>,
        },
        StakeAdded {
            staker: T::AccountId,
            amount_locked: BalanceOf<T>,
            candidate: T::AccountId,
            staker_position: StakerAdded<BalanceOf<T>>,
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
        /// Executor joined candidate pool.
        CandidateJoined {
            account: T::AccountId,
            amount_locked: BalanceOf<T>,
            total_locked: BalanceOf<T>,
        },
        /// An executor configured its terms of operations.
        ExecutorConfigured {
            executor: T::AccountId,
            commission: Percent,
            risk: Percent,
        },
        FixturesConfigured {
            active_set_size: Range<u32>,
            max_commission: Percent,
            max_risk: Percent,
            min_executor_bond: BalanceOf<T>,
            min_candidate_bond: BalanceOf<T>,
            min_atomic_stake: BalanceOf<T>,
            min_total_stake: BalanceOf<T>,
            max_top_stakes_per_candidate: u32,
            max_bottom_stakes_per_candidate: u32,
            max_stakes_per_staker: u32,
            configure_executor_delay: u32,
            leave_candidates_delay: u32,
            leave_stakers_delay: u32,
            candidate_bond_less_delay: u32,
            revoke_stake_delay: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoSuchCandidate,
        NoSuchStake,
        NoSuchPendingStakeRequest,
        NoSuchPendingCandidateRequest,
        NoSuchConfigurationRequest,
        CandidateAlreadyLeaving,
        CandidateCannotLeaveYet,
        PendingStakeRequestAlreadyExists,
        PendingStakeRequestNotDueYet,
        ConfigurationRequestNotDueYet,
        NoSuchStaker,
        StakeBelowMin,
        StakerBondBelowMin,
        StakerNotLeaving,
        StakerAlreadyLeaving,
        CandidateNotLeaving,
        StakerCannotLeaveYet,
        PendingCandidateRequestAlreadyExists,
        PendingStakeRevoke,
        CandidateBondBelowMin,
        PendingCandidateRequestNotDueYet,
        TooLowStakeCountToLeaveStakers,
        CannotDelegateLessThanOrEqualToLowestBottomWhenFull,
        TooMuchCommission,
        TooMuchRisk,
        FixturesCannotBeZero,
        AlreadyOffline,
        AlreadyActive,
        CannotGoOnlineIfLeaving,
        CandidateExists,
        StakerExists,
        TooLowCandidateCountWeightHintJoinCandidates,
        TooLowCandidateCountToLeaveCandidates,
        TooLowCandidateStakeCountToLeaveCandidates,
        TooLowCandidateCountWeightHintCancelLeaveCandidates,
        TooLowStakeCountToStake,
        TooLowCandidateStakeCountToStake,
        InsufficientBalance,
        MaxStakesExceeded,
        AlreadyStakedCandidate,
        FailedToCreateXBIMetadataDueToWrongAccountConversion,
        FailedToConvertSFX2XBI,
        SideEffectIsAlreadyScheduledToExecuteOverXBI,
        SfxNotRecognized,
        SfxDecodingValueErr,
        SfxDecodingAddressErr,
        SfxDecodingDataErr,
        DecodingAddressTo32Err,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub fixtures: StakingFixtures<BalanceOf<T>>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            let onethousand = BalanceOf::<T>::one()
                .mul((10 ^ DECIMALS).into())
                .mul(1000_u32.into());
            let fivehundred = BalanceOf::<T>::one()
                .mul((10 ^ DECIMALS).into())
                .mul(500_u32.into());

            Self {
                fixtures: StakingFixtures {
                    active_set_size: Range {
                        min: 1,   //TODO
                        ideal: 3, //TODO
                        max: 128, //TODO
                    },
                    max_commission: Percent::from_percent(50), //TODO
                    max_risk: Percent::from_percent(50),       //TODO
                    min_executor_bond: onethousand,            //TODO
                    min_candidate_bond: onethousand,           //TODO
                    min_atomic_stake: fivehundred,             //TODO
                    min_total_stake: fivehundred,              //TODO
                    max_top_stakes_per_candidate: 300,         //TODO
                    max_bottom_stakes_per_candidate: 50,       //TODO
                    max_stakes_per_staker: 100,                //TODO
                    // delays target a 14d term assuming a 6h round term
                    configure_executor_delay: 56,  //TODO
                    leave_candidates_delay: 56,    //TODO
                    leave_stakers_delay: 56,       //TODO
                    candidate_bond_less_delay: 56, //TODO
                    // revoke_stake_delay also used as decrease_stake_delay
                    revoke_stake_delay: 56, //TODO
                },
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <Fixtures<T>>::put(&self.fixtures);
            //TBC
        }
    }

    impl<T: Config> Pallet<T> {
        /// Whether given identity is a staker.
        pub fn is_staker(staker: &T::AccountId) -> bool {
            <StakerInfo<T>>::get(staker).is_some()
        }

        /// Whether given identity is an executor candidate.
        pub fn is_candidate(executor: &T::AccountId) -> bool {
            <CandidateInfo<T>>::get(executor).is_some()
        }

        /// Whether given identity is part of the eurrents executor active set.
        pub fn is_active(executor: &T::AccountId) -> bool {
            <ActiveSet<T>>::get().binary_search(executor).is_ok()
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

        /// Returns an account's free balance which is not locked in executor staking.
        pub fn get_executor_stakable_free_balance(executor: &T::AccountId) -> BalanceOf<T> {
            let mut balance = T::Currency::free_balance(executor);
            if let Some(info) = <CandidateInfo<T>>::get(executor) {
                balance = balance.saturating_sub(info.bond);
            }
            balance
        }

        /// Returns an account's free balance which is not locked in executor staking
        pub fn get_staker_stakable_free_balance(executor: &T::AccountId) -> BalanceOf<T> {
            let mut balance = T::Currency::free_balance(executor);

            if let Some(state) = <StakerInfo<T>>::get(executor) {
                balance = balance.saturating_sub(state.total());
            }

            balance
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
        // /// Returns an optional tuple of (Executor's AccountId, total paid)
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
            let fixtures = <Fixtures<T>>::get();
            let mut candidates = <CandidatePool<T>>::get().0;
            // order candidates by stake (least to greatest so requires `rev()`)
            candidates.sort_by(|a, b| a.amount.cmp(&b.amount));
            let top_n = fixtures.active_set_size.ideal as usize;
            // choose the top qualified candidates, ordered by stake
            let mut executors = candidates
                .into_iter()
                .rev()
                .take(top_n)
                .filter(|x| x.amount >= fixtures.min_executor_bond)
                .map(|x| x.owner)
                .collect::<Vec<T::AccountId>>();

            executors.sort();

            let (mut executor_count, mut stake_count, mut total) =
                (0u32, 0u32, BalanceOf::<T>::zero());

            if executors.is_empty()
                || executors.len() < <Fixtures<T>>::get().active_set_size.min as usize
            {
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
                let state =
                    <CandidateInfo<T>>::get(account).expect("all candidates must have info");

                executor_count = executor_count.saturating_add(1u32);
                stake_count = stake_count.saturating_add(state.stake_count);
                total = total.saturating_add(state.total_counted);
                let top_rewardable_stakes = Self::get_rewardable_stakers(account);

                <AtStake<T>>::insert(
                    current_round,
                    account,
                    ExecutorSnapshot {
                        bond: state.bond,
                        stakes: top_rewardable_stakes,
                        total: state.total_counted,
                    },
                );

                Self::deposit_event(Event::ExecutorChosen {
                    round: current_round,
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

        pub fn execute_side_effects_with_xbi(
            origin: OriginFor<T>, // Active relayer
            xtx_id: XExecSignalId<T>,
            side_effect: SideEffect<T::AccountId, BalanceOf<T>>,
            max_exec_cost: u128,
            max_notifications_cost: u128,
        ) -> DispatchResultWithPostInfo {
            let executor = ensure_signed(origin.clone())?;
            let account_to_32: AccountId32 = Decode::decode(&mut &executor.encode()[..])
                .map_err(|_| Error::<T>::DecodingAddressTo32Err)?;
            let nonce_always_0_because_we_use_seed = 0;
            let bypass_most_metadata_checks_default_para_id = Default::default();
            let sfx_id = side_effect.generate_id::<T::Hashing>(xtx_id.as_ref(), 0u32); // FIXME: index needs to be passed from

            let mut xbi: XbiFormat = SfxWithMetadataNewtype::<T>::new(
                side_effect,
                XbiMetadata::new(
                    bypass_most_metadata_checks_default_para_id, // pallet_circuit::bridges::chain_circuit::Circuit::self_para_id,
                    bypass_most_metadata_checks_default_para_id, // side_effect.target
                    Default::default(), // Since we are outside of the scope of XBI we dont need a timeout
                    Fees::new(
                        None, // TODO: although we can pay in non-native another time
                        Some(max_exec_cost),
                        Some(max_notifications_cost),
                    ),
                    Some(account_to_32),
                    nonce_always_0_because_we_use_seed,
                    Some(&sfx_id.encode()),
                ),
            )
            .try_into()?;

            T::InstructionHandler::handle(&origin, &mut xbi).map(Into::into)
        }
    }
}

#[derive(PartialEq)]
pub struct SfxWithMetadataNewtype<T: Config> {
    pub side_effect: t3rn_types::sfx::SideEffect<T::AccountId, BalanceOf<T>>,
    pub metadata: XbiMetadata,
}

impl<T: Config> SfxWithMetadataNewtype<T> {
    pub fn new(
        side_effect: t3rn_types::sfx::SideEffect<T::AccountId, BalanceOf<T>>,
        metadata: XbiMetadata,
    ) -> Self {
        Self {
            side_effect,
            metadata,
        }
    }
}

// Justification: We only need into for this, XBI doesn't need to know about SFX
#[allow(clippy::from_over_into)]
impl<T: Config> TryInto<XbiFormat> for SfxWithMetadataNewtype<T> {
    type Error = DispatchError;

    fn try_into(self) -> Result<XbiFormat, Self::Error> {
        let side_effect = self.side_effect;
        let metadata = self.metadata;

        match &side_effect.action {
            b"tran" => {
                Ok(XbiFormat {
                    instr: XbiInstruction::Transfer {
                        // Get dest as argument_1 of SFX::Transfer of Type::DynamicAddress
                        dest: Decode::decode(&mut &side_effect.encoded_args[0][..])
                            .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                        // Get dest as argument_2 of SFX::Transfer of Type::Value
                        value: Sabi::try_convert(ValueMorphism::<_, u128>::new(
                            &mut &side_effect.encoded_args[1][..],
                        ))
                        .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    },
                    metadata,
                })
            },
            b"mult" | b"tass" => Ok(XbiFormat {
                instr: XbiInstruction::TransferAssets {
                    // Get dest as argument_0 of SFX::TransferAssets of Type::DynamicBytes
                    currency_id: Decode::decode(&mut &side_effect.encoded_args[0][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    // Get dest as argument_1 of SFX::TransferAssets of Type::DynamicAddress
                    dest: Decode::decode(&mut &side_effect.encoded_args[1][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    // Get dest as argument_2 of SFX::TransferAssets of Type::Value
                    value: Sabi::try_convert(ValueMorphism::<_, u128>::new(
                        &mut &side_effect.encoded_args[2][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                },
                metadata,
            }),
            b"aliq" => Ok(XbiFormat {
                instr: XbiInstruction::AddLiquidity {
                    asset_a: Decode::decode(&mut &side_effect.encoded_args[2][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    asset_b: Decode::decode(&mut &side_effect.encoded_args[3][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    amount_a: Decode::decode(&mut &side_effect.encoded_args[5][..])
                        .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    amount_b_max_limit: Decode::decode(&mut &side_effect.encoded_args[6][..])
                        .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                },
                metadata,
            }),
            b"swap" => Ok(XbiFormat {
                instr: XbiInstruction::Swap {
                    asset_out: Decode::decode(&mut &side_effect.encoded_args[3][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    asset_in: Decode::decode(&mut &side_effect.encoded_args[4][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    amount: Decode::decode(&mut &side_effect.encoded_args[1][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    max_limit: Decode::decode(&mut &side_effect.encoded_args[2][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    discount: Default::default(),
                },
                metadata,
            }),
            b"cevm" => Ok(XbiFormat {
                instr: XbiInstruction::CallEvm {
                    // Get dest as argument_0 of SFX::CallEvm of Type::DynamicAddress
                    source: Decode::decode(&mut vec![0u8; 32].as_ref())
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    // source: Decode::decode(&mut &side_effect.encoded_args[0][..])
                    //     .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    // Get dest as argument_1 of SFX::CallEvm of Type::DynamicAddress
                    target: Decode::decode(&mut &side_effect.encoded_args[0][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    // Get dest as argument_2 of SFX::CallEvm of Type::Value
                    value: Sabi::try_convert(ValueMorphism::<_, Value256>::new(
                        &mut &side_effect.encoded_args[1][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_3 of SFX::CallEvm of Type::DynamicBytes
                    input: side_effect.encoded_args[2].clone(),
                    // Get dest as argument_4 of SFX::CallEvm of Type::Uint(64)
                    gas_limit: Sabi::try_convert(ValueMorphism::<_, u64>::new(
                        &mut &side_effect.encoded_args[3][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_5 of SFX::CallEvm of Type::Value
                    max_fee_per_gas: Sabi::try_convert(ValueMorphism::<_, Value256>::new(
                        &mut &side_effect.encoded_args[4][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_6 of SFX::CallEvm of Type::Option(Box::from(Type::Value))
                    max_priority_fee_per_gas: Sabi::try_convert(
                        ValueMorphism::<_, Option<Value256>>::new(
                            &mut &side_effect.encoded_args[5][..],
                        ),
                    )
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_7 of SFX::CallEvm of Type::Option(Box::from(Type::Value))
                    nonce: Sabi::try_convert(ValueMorphism::<_, Option<Value256>>::new(
                        &mut &side_effect.encoded_args[6][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_8 of SFX::CallEvm of Type::DynamicBytes
                    access_list: Decode::decode(&mut &side_effect.encoded_args[7][..])
                        .map_err(|_| Error::<T>::SfxDecodingDataErr)?,
                },
                metadata,
            }),
            b"wasm" => Ok(XbiFormat {
                instr: XbiInstruction::CallWasm {
                    // Get dest as argument_0 of SFX::CallWasm of Type::DynamicAddress
                    dest: Decode::decode(&mut &side_effect.encoded_args[0][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    // Get dest as argument_1 of SFX::CallWasm of Type::Value
                    value: Sabi::try_convert(ValueMorphism::<_, u128>::new(
                        &mut &side_effect.encoded_args[1][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_2 of SFX::CallWasm of Type::Value
                    gas_limit: Sabi::try_convert(ValueMorphism::<_, u64>::new(
                        &mut &side_effect.encoded_args[2][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_3 of SFX::CallEvm of Type::Option(Box::from(Type::Value))
                    storage_deposit_limit: Sabi::try_convert(
                        ValueMorphism::<_, Option<u128>>::new(
                            &mut &side_effect.encoded_args[3][..],
                        ),
                    )
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_4 of SFX::CallEvm of Type::DynamicBytes
                    data: side_effect.encoded_args[4].clone(),
                },
                metadata,
            }),
            b"call" => Ok(XbiFormat {
                instr: XbiInstruction::CallCustom {
                    // Get dest as argument_0 of SFX::CallWasm of Type::DynamicAddress
                    caller: Decode::decode(&mut vec![0u8; 32].as_ref())
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    // Get dest as argument_1 of SFX::CallWasm of Type::DynamicAddress
                    dest: Decode::decode(&mut &side_effect.encoded_args[0][..])
                        .map_err(|_| Error::<T>::SfxDecodingAddressErr)?,
                    // Get dest as argument_2 of SFX::CallWasm of Type::Value
                    value: Sabi::try_convert(ValueMorphism::<_, u128>::new(
                        &mut &side_effect.encoded_args[1][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_3 of SFX::CallEvm of Type::DynamicBytes
                    input: side_effect.encoded_args[2].clone(),
                    // Get dest as argument_4 of SFX::CallWasm of Type::Value
                    limit: Sabi::try_convert(ValueMorphism::<_, u64>::new(
                        &mut &side_effect.encoded_args[3][..],
                    ))
                    .map_err(|_| Error::<T>::SfxDecodingValueErr)?,
                    // Get dest as argument_5 of SFX::CallEvm of Type::DynamicBytes
                    additional_params: side_effect.encoded_args[4].clone(),
                },
                metadata,
            }),
            _ => Err(Error::<T>::SfxNotRecognized.into()),
        }
    }
}
