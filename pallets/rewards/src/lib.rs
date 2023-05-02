#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    t3rn_primitives::reexport_currency_types!();
    use codec::Encode;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        traits::{CheckedAdd, CheckedDiv, CheckedMul, Zero},
        Perbill,
    };
    use t3rn_primitives::{
        account_manager::{AccountManager, Settlement},
        claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
        clock::Clock,
        common::RoundInfo,
    };

    use sp_std::{convert::TryInto, prelude::*};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type AttesterInflation: Get<u32>;
        type CollatorInflation: Get<u32>;
        type AttesterBootstrapRewards: Get<u32>;
        type CollatorBootstrapRewards: Get<u32>;
        type ExecutorBootstrapRewards: Get<u32>;
        type DistributionPeriod: Get<Self::BlockNumber>;
        type Clock: Clock<Self>;
        type AccountManager: AccountManager<
            Self::AccountId,
            BalanceOf<Self>,
            Self::Hash,
            Self::BlockNumber,
            u32,
        >;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Attesters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    pub type Collators<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    pub type DistributionBlock<T: Config> = StorageValue<_, T::BlockNumber>;

    #[pallet::storage]
    pub type DistributionHistory<T: Config> =
        StorageValue<_, Vec<(T::BlockNumber, RoundInfo<T::BlockNumber>)>>;

    #[pallet::storage]
    pub type PendingClaims<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>>,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AttesterRewarded(T::AccountId, BalanceOf<T>),
        CollatorRewarded(T::AccountId, BalanceOf<T>),
        ExecutorRewarded(T::AccountId, BalanceOf<T>),
        Claimed(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        DistributionPeriodNotElapsed,
        NoPendingClaims,
        ArithmeticOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn trigger_distribution(origin: OriginFor<T>) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            Self::distribute_inflation()?;
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn claim(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let pending_claims =
                PendingClaims::<T>::get(&who).ok_or(Error::<T>::NoPendingClaims)?;

            let mut total_claim = BalanceOf::<T>::zero();
            for claim in &pending_claims {
                total_claim = total_claim
                    .checked_add(&claim.total_round_claim)
                    .ok_or(Error::<T>::ArithmeticOverflow)?;
            }

            ensure!(
                total_claim > BalanceOf::<T>::zero(),
                Error::<T>::NoPendingClaims
            );

            T::Currency::deposit_into_existing(&who, total_claim)?;
            PendingClaims::<T>::remove(&who);

            Self::deposit_event(Event::Claimed(who, total_claim));
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn distribute_inflation() -> DispatchResult {
            // Distribute rewards to attesters
            Self::distribute_attester_rewards()?;

            // Distribute rewards to collators
            Self::distribute_collator_rewards()?;

            // Distribute rewards to executors
            Self::distribute_executor_rewards()?;

            // Distribute bootstrap rewards from the treasury account
            Self::distribute_bootstrap_rewards()?;

            // Update the distribution block
            DistributionBlock::<T>::put(frame_system::Pallet::<T>::block_number());

            // Update the distribution history
            let current_round = T::Clock::current_round();
            let current_block = frame_system::Pallet::<T>::block_number();
            let mut history = DistributionHistory::<T>::get().unwrap_or_default();
            history.push((current_block, current_round));
            DistributionHistory::<T>::put(history);

            Ok(())
        }

        fn distribute_attester_rewards() -> DispatchResult {
            let inflation = T::AttesterInflation::get();
            let total_attesters = Attesters::<T>::iter().count() as u32;

            if total_attesters == 0 {
                return Ok(())
            }

            let reward_per_attester = BalanceOf::<T>::from(inflation)
                .checked_div(&total_attesters.into())
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            for (attester, _) in Attesters::<T>::iter() {
                Self::update_pending_claims(
                    &attester,
                    CircuitRole::Attester,
                    reward_per_attester,
                    BenefitSource::Inflation,
                );
            }

            Ok(())
        }

        fn distribute_collator_rewards() -> DispatchResult {
            let inflation = T::CollatorInflation::get();
            let total_collators = Collators::<T>::iter().count() as u32;

            if total_collators == 0 {
                return Ok(())
            }

            let reward_per_collator = BalanceOf::<T>::from(inflation)
                .checked_div(&total_collators.into())
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            for (collator, _) in Collators::<T>::iter() {
                Self::update_pending_claims(
                    &collator,
                    CircuitRole::Collator,
                    reward_per_collator,
                    BenefitSource::Inflation,
                );
            }

            Ok(())
        }

        fn distribute_executor_rewards() -> DispatchResult {
            for (executor, settlement) in Self::executions_this_round() {
                let total_reward = BalanceOf::<T>::from(settlement.settlement_amount)
                    .checked_mul(&2u32.into())
                    .ok_or(Error::<T>::ArithmeticOverflow)?;
                Self::update_pending_claims(
                    &executor,
                    CircuitRole::Executor,
                    total_reward,
                    BenefitSource::Inflation,
                );
            }

            Ok(())
        }

        pub fn executions_this_round() -> Vec<(T::AccountId, Settlement<T::AccountId, BalanceOf<T>>)>
        {
            T::AccountManager::get_settlements_by_role(CircuitRole::Executor)
        }

        pub fn executions_proportionally_of_total_this_round(
            executions_this_round: Vec<(T::AccountId, Settlement<T::AccountId, BalanceOf<T>>)>,
            total_settled_executions_this_round: BalanceOf<T>,
        ) -> Vec<(
            T::AccountId,
            Settlement<T::AccountId, BalanceOf<T>>,
            Perbill,
        )> {
            let mut executions_proportionally_of_total_this_round = Vec::new();
            for (executor, settlement) in executions_this_round {
                let proportion = Perbill::from_rational_approximation(
                    settlement.settlement_amount,
                    total_settled_executions_this_round,
                );
                executions_proportionally_of_total_this_round
                    .push((executor, settlement, proportion));
            }
            executions_proportionally_of_total_this_round
        }

        pub fn total_settled_executions_this_round(
            executions_this_round: Vec<(T::AccountId, Settlement<T::AccountId, BalanceOf<T>>)>,
        ) -> Result<BalanceOf<T>, DispatchError> {
            let mut total_settlement = BalanceOf::<T>::zero();
            for (_, settlement) in executions_this_round {
                total_settlement = total_settlement
                    .checked_add(&settlement.settlement_amount)
                    .ok_or(Error::<T>::ArithmeticOverflow)?;
            }
            Ok(total_settlement)
        }

        fn distribute_bootstrap_rewards() -> DispatchResult {
            let attester_bootstrap_rewards = T::AttesterBootstrapRewards::get();
            let collator_bootstrap_rewards = T::CollatorBootstrapRewards::get();
            let executor_bootstrap_rewards = T::ExecutorBootstrapRewards::get();

            for (attester, _) in Attesters::<T>::iter() {
                Self::update_pending_claims(
                    &attester,
                    CircuitRole::Attester,
                    attester_bootstrap_rewards.into(),
                    BenefitSource::BootstrapPool,
                );
            }

            for (collator, _) in Collators::<T>::iter() {
                Self::update_pending_claims(
                    &collator,
                    CircuitRole::Collator,
                    collator_bootstrap_rewards.into(),
                    BenefitSource::BootstrapPool,
                );
            }

            // Keep max 2x of the total executor rewards for bootstrap

            for (executor, _) in Self::executions_this_round() {
                Self::update_pending_claims(
                    &executor,
                    CircuitRole::Executor,
                    executor_bootstrap_rewards.into(),
                    BenefitSource::BootstrapPool,
                );
            }

            Ok(())
        }

        fn update_pending_claims(
            account: &T::AccountId,
            role: CircuitRole,
            reward: BalanceOf<T>,
            benefit_source: BenefitSource,
        ) {
            let claim = ClaimableArtifacts {
                beneficiary: account.clone(),
                role,
                total_round_claim: reward,
                benefit_source,
            };

            let mut pending_claims = PendingClaims::<T>::get(account).unwrap_or_default();
            pending_claims.push(claim);
            PendingClaims::<T>::insert(account, pending_claims);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: T::BlockNumber) -> Weight {
            if n % T::DistributionPeriod::get() == Zero::zero() {
                let _ = Self::distribute_inflation();
            }
            0
        }
    }

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        phantom: PhantomData<T>,
    }

    // The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                phantom: Default::default(),
            }
        }
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }
}

#[cfg(test)]
pub mod rewards_test {}
