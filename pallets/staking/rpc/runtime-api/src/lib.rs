//! Runtime API definition required by Contracts Registry RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts Registry access methods.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use sp_runtime::codec::Codec;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    /// The API to interact with circuit portal
    pub trait StakingRuntimeApi<AccountId, Balance, BlockNumber> where
        AccountId: Codec,
        Balance: Codec,
        BlockNumber: Codec,
    {
        //TODO

        /*
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
    pub(crate) type CandidatePool<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    /// Get executor candidate info associated with an account.
    #[pallet::storage]
    #[pallet::getter(fn candidate_info)]
    pub(crate) type CandidateInfo<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, CandidateMetadata<BalanceOf<T>>, OptionQuery>;

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
    pub(crate) type ScheduledStakingRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<ScheduledStakingRequest<T::AccountId, BalanceOf<T>>>,
        ValueQuery,
    >;

    /// Outstanding configuration change per executor.
    #[pallet::storage]
    #[pallet::getter(fn scheduled_configration_requests)]
    pub(crate) type ScheduledConfigurationRequests<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, ScheduledConfigurationRequest, OptionQuery>;

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
    #[pallet::getter(fn total_value_locked)]
    pub(crate) type Total<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Total staked of a round's active set of executors.
    #[pallet::storage]
    #[pallet::getter(fn staked)]
    pub type Staked<T: Config> = StorageMap<_, Twox64Concat, RoundIndex, BalanceOf<T>, ValueQuery>;
        */
    }
}
