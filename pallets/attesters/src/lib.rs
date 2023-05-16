#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub use crate::pallet::*;

pub type TargetId = [u8; 4];

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    t3rn_primitives::reexport_currency_types!();
    use codec::{Decode, Encode};
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Randomness, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;

    use sp_runtime::{
        traits::{Saturating, Zero},
        Percent,
    };

    use sp_std::{convert::TryInto, prelude::*};

    pub use t3rn_primitives::attesters::{
        AttesterInfo, AttestersReadApi, ECDSA_ATTESTER_KEY_TYPE_ID, ED25519_ATTESTER_KEY_TYPE_ID,
        SR25519_ATTESTER_KEY_TYPE_ID,
    };

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub enum AttestationFor {
        SFX,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub enum AttestationStatus {
        Pending,
        Timeout,
        Approved,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub enum Slash<BlockNumber> {
        LateOrNoSubmissionAtBlocks(Vec<BlockNumber>),
        // Permanent slash
        Permanent,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub enum BatchStatus {
        Pending,
        ReadyForSubmission,
        Committed,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct Batch<Attester, Signature, BlockNumber> {
        pub attestations: Vec<Attestation<Attester, Signature>>,
        pub target: TargetId,
        pub created: BlockNumber,
        pub status: BatchStatus,
    }

    impl<Attester, Signature, BlockNumber: Zero> Default for Batch<Attester, Signature, BlockNumber> {
        fn default() -> Self {
            Self {
                attestations: Vec::new(),
                target: [0u8; 4],
                created: Zero::zero(),
                status: BatchStatus::Pending,
            }
        }
    }

    impl<Attester, Signature, BlockNumber> Batch<Attester, Signature, BlockNumber> {
        pub fn new(target: TargetId, now: BlockNumber) -> Self {
            Self {
                attestations: Vec::new(),
                target,
                created: now,
                status: BatchStatus::Pending,
            }
        }
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct Attestation<Attester, Signature> {
        pub for_: AttestationFor,
        pub status: AttestationStatus,
        pub signature: Vec<(Attester, Signature)>,
    }

    impl<Attester, Signature> Default for Attestation<Attester, Signature> {
        fn default() -> Self {
            Self {
                for_: AttestationFor::SFX,
                status: AttestationStatus::Pending,
                signature: Vec::new(),
            }
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type ActiveSetSize: Get<u32>;
        type CommitteeSize: Get<u32>;
        type BatchingWindow: Get<Self::BlockNumber>;
        type ShufflingFrequency: Get<Self::BlockNumber>;
        type MaxBatchSize: Get<u32>;
        type RewardMultiplier: Get<BalanceOf<Self>>;
        type CommitmentRewardSource: Get<Self::AccountId>;
        type SlashAccount: Get<Self::AccountId>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;
        type DefaultCommission: Get<Percent>;
        type MinNominatorBond: Get<BalanceOf<Self>>;
        type MinAttesterBond: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn attesters)]
    pub type Attesters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, AttesterInfo>;

    #[pallet::storage]
    pub type CurrentCommittee<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    pub type PreviousCommittee<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn sorted_nominated_attesters)]
    pub type SortedNominatedAttesters<T: Config> =
        StorageValue<_, Vec<(T::AccountId, BalanceOf<T>)>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn active_set)]
    pub type ActiveSet<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_slashes)]
    pub type PendingSlashes<T: Config> =
        StorageMap<_, Identity, T::AccountId, Vec<Slash<T::BlockNumber>>>;

    #[pallet::storage]
    #[pallet::getter(fn pending_unnominations)]
    pub type PendingUnnominations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<(T::AccountId, BalanceOf<T>, BlockNumberFor<T>)>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn attestations)]
    pub type Attestations<T: Config> =
        StorageMap<_, Identity, T::Hash, Attestation<T::AccountId, Vec<u8>>>;

    #[pallet::storage]
    #[pallet::getter(fn batches)]
    pub type Batches<T: Config> =
        StorageMap<_, Identity, TargetId, Vec<Batch<T::AccountId, Vec<u8>, T::BlockNumber>>>;

    #[pallet::storage]
    #[pallet::getter(fn nominations)]
    pub type Nominations<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // Attester
        Blake2_128Concat,
        T::AccountId, // Nominator
        BalanceOf<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn fast_confirmation_cost)]
    pub type FastConfirmationCost<T: Config> =
        StorageMap<_, Blake2_128Concat, TargetId, BalanceOf<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AttesterRegistered(T::AccountId),
        AttestationSubmitted(T::AccountId),
        Nominated(T::AccountId, T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidSignature,
        InvalidMessage,
        AlreadyRegistered,
        PublicKeyMissing,
        AttestationSignatureInvalid,
        AttestationDoubleSignAttempt,
        NotActiveSet,
        NotInCurrentCommittee,
        NotRegistered,
        NoNominationFound,
        AlreadyNominated,
        NominatorNotEnoughBalance,
        NominatorBondTooSmall,
        AttesterBondTooSmall,
        MissingNominations,
        BatchNotFound,
        BatchAlreadyCommitted,
        CommitteeSizeTooLarge,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_attester(
            origin: OriginFor<T>,
            self_nominate_amount: BalanceOf<T>,
            ecdsa_key: [u8; 33],
            ed25519_key: [u8; 32],
            sr25519_key: [u8; 32],
            custom_commission: Option<Percent>,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;

            // Check min. self-nomination bond
            ensure!(
                self_nominate_amount >= T::MinAttesterBond::get(),
                Error::<T>::AttesterBondTooSmall
            );

            // Ensure the attester is not already registered
            ensure!(
                !Attesters::<T>::contains_key(&account_id),
                Error::<T>::AlreadyRegistered
            );

            let commission = match custom_commission {
                Some(commission) => commission,
                None => T::DefaultCommission::get(),
            };

            Attesters::<T>::insert(
                &account_id,
                AttesterInfo {
                    key_ec: ecdsa_key,
                    key_ed: ed25519_key,
                    key_sr: sr25519_key,
                    commission,
                },
            );

            // Self nominate in order to be part of the active set selection
            Self::do_nominate(account_id.clone(), account_id, self_nominate_amount)?;

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn submit_attestation(
            origin: OriginFor<T>,
            message: Vec<u8>,
            signature: Vec<u8>,
            target: TargetId,
            attestation_for: AttestationFor,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;

            // Lookup the attester in the storage
            let attester = Attesters::<T>::get(&account_id).ok_or(Error::<T>::NotRegistered)?;

            // Check if active set
            ensure!(
                ActiveSet::<T>::get().contains(&account_id),
                Error::<T>::NotActiveSet
            );

            // Check if the attester is part of the current committee
            ensure!(
                CurrentCommittee::<T>::get().contains(&account_id),
                Error::<T>::NotInCurrentCommittee
            );

            let is_verified = attester
                .verify_attestation_signature(ECDSA_ATTESTER_KEY_TYPE_ID, &message, &signature)
                .map_err(|_| Error::<T>::InvalidSignature)?;

            // todo: slash the attester if the signature is invalid
            if !is_verified {
                let slash: Vec<Slash<T::BlockNumber>> = match PendingSlashes::<T>::get(&account_id)
                {
                    Some(already_pending) => {
                        let mut already_pending = already_pending;
                        already_pending.extend_from_slice(&[Slash::Permanent]);
                        already_pending
                    },
                    None => vec![Slash::Permanent],
                };
                // Apply permanent slash for colluding attestor
                PendingSlashes::<T>::insert(&account_id, slash);
            }

            ensure!(
                PendingSlashes::<T>::get(&account_id).is_none(),
                Error::<T>::AttestationSignatureInvalid
            );

            ensure!(is_verified, Error::<T>::AttestationSignatureInvalid);

            match attestation_for {
                AttestationFor::SFX => {
                    let sfx_hash: T::Hash = Decode::decode(&mut &message[..])
                        .map_err(|_| Error::<T>::InvalidMessage)?;
                    let mut sfx_attestation = Attestations::<T>::get(sfx_hash).unwrap_or_default();

                    // Check if the attester has already signed the attestation
                    ensure!(
                        !sfx_attestation
                            .signature
                            .iter()
                            .any(|(attester, _)| attester == &account_id),
                        Error::<T>::AttestationDoubleSignAttempt
                    );

                    sfx_attestation
                        .signature
                        .push((account_id.clone(), signature));

                    let required_signatures = (T::ActiveSetSize::get() * 2 / 3) as usize;
                    let status = if sfx_attestation.signature.len() < required_signatures {
                        AttestationStatus::Pending
                    } else {
                        AttestationStatus::Approved
                    };

                    sfx_attestation.for_ = attestation_for;
                    sfx_attestation.status = status;

                    Attestations::<T>::insert(sfx_hash, sfx_attestation.clone());

                    let mut target_batches = Batches::<T>::get(target).unwrap_or_default();
                    let current_block = frame_system::Pallet::<T>::block_number();

                    if target_batches.is_empty() {
                        // Create a new batch if there are no batches yet
                        let new_batch =
                            Batch::new(target, frame_system::Pallet::<T>::block_number());
                        target_batches.push(new_batch);
                    } else {
                        let is_full = target_batches.last().map_or(false, |b| {
                            b.attestations.len() >= T::MaxBatchSize::get() as usize
                        });
                        let interval_passed = target_batches.last().map_or(false, |b| {
                            current_block - b.created >= T::BatchingWindow::get()
                        });

                        if is_full || interval_passed {
                            // Create a new batch
                            let new_batch =
                                Batch::new(target, frame_system::Pallet::<T>::block_number());
                            target_batches.push(new_batch);
                        }
                    }

                    // Add the attestation to the last batch
                    if let Some(last_batch) = target_batches.last_mut() {
                        last_batch.attestations.push(sfx_attestation);

                        if last_batch.attestations.len() >= T::MaxBatchSize::get() as usize {
                            last_batch.status = BatchStatus::ReadyForSubmission;
                        }
                    }

                    // Save the updated batches
                    Batches::<T>::insert(target, target_batches);
                },
            }

            Self::deposit_event(Event::AttestationSubmitted(account_id));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn commit_batch(
            origin: OriginFor<T>,
            target: TargetId,
            batch_index: u32,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;

            Batches::<T>::try_mutate(target, |batches_option| {
                if let Some(batches) = batches_option {
                    let index = batch_index as usize;
                    if let Some(batch) = batches.get_mut(index) {
                        match batch.status {
                            BatchStatus::Pending | BatchStatus::ReadyForSubmission => {
                                batch.status = BatchStatus::Committed;
                                Ok(Some(batches.clone()))
                            },
                            _ => Err(Error::<T>::BatchAlreadyCommitted),
                        }
                    } else {
                        Err(Error::<T>::BatchNotFound)
                    }
                } else {
                    Err(Error::<T>::BatchNotFound)
                }
            })?;

            // Reward the submitter
            let fast_confirmation_cost =
                FastConfirmationCost::<T>::get(target).unwrap_or_else(Zero::zero);
            let total_reward = fast_confirmation_cost.saturating_mul(T::RewardMultiplier::get());

            T::Currency::transfer(
                &T::CommitmentRewardSource::get(),
                &submitter,
                total_reward,
                ExistenceRequirement::KeepAlive,
            )?;

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn nominate(
            origin: OriginFor<T>,
            attester: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let nominator = ensure_signed(origin)?;

            // Check min. nomination amount
            ensure!(
                amount >= T::MinNominatorBond::get(),
                Error::<T>::NominatorBondTooSmall
            );

            Self::do_nominate(nominator.clone(), attester.clone(), amount)?;
            Self::deposit_event(Event::Nominated(nominator, attester, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn unnominate(origin: OriginFor<T>, attester: T::AccountId) -> DispatchResult {
            let nominator = ensure_signed(origin)?;

            // Read the nominations for the given attester
            let nominations = Self::read_nominations(&attester);

            // Find the nomination for the current nominator
            let nomination = nominations
                .iter()
                .find(|(nominator_id, _)| nominator_id == &nominator)
                .ok_or(Error::<T>::NoNominationFound)?;

            // Check if the nominator has an existing nomination
            ensure!(nomination.0 == nominator, Error::<T>::NoNominationFound);

            let amount = nomination.1;

            // Calculate the block number when the unnomination can be processed
            let unlock_block = frame_system::Pallet::<T>::block_number()
                + T::ShufflingFrequency::get() * 2u32.into();

            // Store the pending unnomination
            PendingUnnominations::<T>::mutate(&nominator, |pending_unnominations| {
                let pending_unnominations = pending_unnominations.get_or_insert_with(Vec::new);
                pending_unnominations.push((attester.clone(), amount, unlock_block));
            });

            Ok(())
        }
    }

    impl<T: Config> AttestersReadApi<T::AccountId, BalanceOf<T>> for Pallet<T> {
        fn previous_committee() -> Vec<T::AccountId> {
            PreviousCommittee::<T>::get()
        }

        fn current_committee() -> Vec<T::AccountId> {
            CurrentCommittee::<T>::get()
        }

        fn active_set() -> Vec<T::AccountId> {
            ActiveSet::<T>::get()
        }

        fn honest_active_set() -> Vec<T::AccountId> {
            let active_set = ActiveSet::<T>::get();
            active_set
                .into_iter()
                .filter(|a| !PendingSlashes::<T>::contains_key(a))
                .collect()
        }

        fn read_attester_info(attester: &T::AccountId) -> Option<AttesterInfo> {
            Attesters::<T>::get(attester)
        }

        fn read_nominations(for_attester: &T::AccountId) -> Vec<(T::AccountId, BalanceOf<T>)> {
            Nominations::<T>::iter_prefix(for_attester)
                .map(|(nominator, balance)| (nominator, balance))
                .collect()
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn committee_size() -> usize {
            T::CommitteeSize::get() as usize
        }

        fn update_sorted_nominated_attesters(
            attester: &T::AccountId,
            amount: BalanceOf<T>,
        ) -> bool {
            let mut all_indices_match = true;
            SortedNominatedAttesters::<T>::mutate(|attesters| {
                if let Some(index) = attesters.iter().position(|(a, _n)| a == attester) {
                    let total_nomination = attesters[index].1 - amount;
                    if total_nomination.is_zero() {
                        attesters.remove(index);
                    } else {
                        attesters[index] = (attester.clone(), total_nomination);
                    }

                    // Sort the attesters by their nomination amount
                    attesters.sort_by(|(_a1, n1), (_a2, n2)| n2.cmp(n1));

                    // Keep only the top 32 attesters in the list
                    attesters.truncate(32);
                } else {
                    log::warn!("Attester not found while updating sorted nominated attesters");
                    all_indices_match = false
                }
            });
            all_indices_match
        }

        pub fn do_nominate(
            nominator: T::AccountId,
            attester: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            // Check if nominator has enough balance
            ensure!(
                T::Currency::free_balance(&nominator) >= amount,
                Error::<T>::NominatorNotEnoughBalance
            );

            let current_nomination =
                Nominations::<T>::get(&attester, &nominator).unwrap_or(Zero::zero());

            let new_nomination = current_nomination + amount;
            Nominations::<T>::insert(&attester, &nominator, new_nomination);

            // Update the sorted list of nominated attesters
            SortedNominatedAttesters::<T>::try_mutate(|attesters| {
                let total_nomination = Nominations::<T>::iter_prefix(&attester)
                    .map(|(_, balance)| balance)
                    .fold(Zero::zero(), |acc, balance| acc + balance);

                if let Some(index) = attesters.iter().position(|(a, _n)| a == &attester) {
                    // Update the existing nomination amount
                    attesters[index] = (attester.clone(), total_nomination);
                } else {
                    // Add the new attester to the list
                    attesters.push((attester.clone(), total_nomination));
                }

                // Sort the attesters by their nomination amount
                attesters.sort_by(|(_a1, n1), (_a2, n2)| n2.cmp(n1));

                // Keep only the top 32 attesters in the list
                attesters.truncate(32);

                Ok::<(), Error<T>>(())
            })?;

            // Lock the nomination amount in the nominator's account
            T::Currency::reserve(&nominator, amount)?;

            Ok(())
        }

        fn shuffle_committee() -> bool {
            let active_set = ActiveSet::<T>::get();
            let active_set_size = active_set.len();
            let mut committee_size = T::CommitteeSize::get() as usize;

            let full_shuffle = if committee_size > active_set_size {
                committee_size = active_set_size;
                false
            } else {
                true
            };

            let current_committee = CurrentCommittee::<T>::get();
            PreviousCommittee::<T>::put(current_committee);

            let _seed = T::RandomnessSource::random_seed();

            let mut shuffled_active_set = active_set;
            for i in (1..shuffled_active_set.len()).rev() {
                let random_value = T::RandomnessSource::random(&i.to_be_bytes());
                let random_index = random_value
                    .0
                    .as_ref()
                    .iter()
                    .fold(0usize, |acc, &val| (acc + val as usize) % (i + 1));

                if i != random_index {
                    shuffled_active_set.swap(i, random_index);
                }
            }

            let new_committee = shuffled_active_set
                .into_iter()
                .take(committee_size)
                .collect::<Vec<T::AccountId>>();

            CurrentCommittee::<T>::put(new_committee);

            full_shuffle
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: T::BlockNumber) -> Weight {
            // Check if a shuffling round has passed
            if (n % T::ShufflingFrequency::get()).is_zero() {
                let mut aggregated_weight: Weight = 0;

                // Process pending unnominations
                aggregated_weight += T::DbWeight::get().reads(1);
                for (nominator, pending_unnominations) in PendingUnnominations::<T>::iter() {
                    let mut pending_unnominations = pending_unnominations.clone();
                    let mut pending_unnominations_updated = false;
                    let mut indices_to_remove = Vec::new();

                    for (index, (attester, amount, unlock_block)) in
                        pending_unnominations.iter().enumerate()
                    {
                        if unlock_block <= &n {
                            // Save the index to be removed later
                            indices_to_remove.push(index);
                            pending_unnominations_updated = true;

                            // Unreserve the nominated amount in the nominator's account
                            T::Currency::unreserve(&nominator, *amount);
                            aggregated_weight += T::DbWeight::get().writes(1);

                            // Remove the nomination from storage
                            Nominations::<T>::remove(attester, &nominator);
                            aggregated_weight += T::DbWeight::get().writes(1);

                            // Update the sorted list of nominated attesters
                            let _ = Self::update_sorted_nominated_attesters(attester, *amount);
                            aggregated_weight += T::DbWeight::get().writes(1);
                        }
                    }

                    // Remove the pending unnomination from the list
                    for &index in indices_to_remove.iter().rev() {
                        pending_unnominations.remove(index);
                    }

                    // Update the pending unnominations storage item if necessary
                    if pending_unnominations_updated {
                        if pending_unnominations.is_empty() {
                            PendingUnnominations::<T>::remove(&nominator);
                        } else {
                            PendingUnnominations::<T>::insert(&nominator, pending_unnominations);
                        }
                        aggregated_weight += T::DbWeight::get().writes(1);
                    }
                }

                // Update the active set of attesters
                ActiveSet::<T>::put(
                    SortedNominatedAttesters::<T>::get()
                        .iter()
                        .filter(|(account_id, _)| PendingSlashes::<T>::get(account_id).is_none())
                        .take(32)
                        .cloned()
                        .map(|(account_id, _balance)| account_id)
                        .collect::<Vec<T::AccountId>>(),
                );
                aggregated_weight += T::DbWeight::get().reads_writes(1, 1);

                // Call shuffle_committee
                if !Self::shuffle_committee() {
                    log::error!("Failed to shuffle committee");
                    aggregated_weight += T::DbWeight::get().reads_writes(2, 2);
                }
                aggregated_weight += T::DbWeight::get().reads_writes(2, 2);

                return aggregated_weight
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
pub mod attesters_test {
    use super::{
        TargetId, ECDSA_ATTESTER_KEY_TYPE_ID, ED25519_ATTESTER_KEY_TYPE_ID,
        SR25519_ATTESTER_KEY_TYPE_ID,
    };
    use codec::Encode;
    use frame_support::{
        assert_ok,
        traits::{Currency, Get, Hooks},
        StorageValue,
    };
    use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, Pair, RuntimePublic};
    use sp_core::H256;
    use t3rn_primitives::attesters::AttestersReadApi;

    use frame_support::traits::Len;
    use sp_std::convert::TryInto;
    use t3rn_mini_mock_runtime::{
        AccountId, ActiveSet, AttestationFor, AttestationStatus, Attesters, AttestersError,
        AttestersStore, Balance, Balances, BatchStatus, Batches, BlockNumber, ConfigAttesters,
        ConfigRewards, CurrentCommittee, ExtBuilder, MiniRuntime, Origin, PendingUnnominations,
        PreviousCommittee, Rewards, SortedNominatedAttesters, System,
    };
    use t3rn_primitives::claimable::{BenefitSource, CircuitRole, ClaimableArtifacts};

    pub fn register_attester_with_single_private_key(secret_key: [u8; 32]) {
        // Register an attester
        let attester = AccountId::from(secret_key);

        let ecdsa_key = ecdsa::Pair::from_seed(&secret_key).public().to_raw_vec();
        let ed25519_key = ed25519::Pair::from_seed(&secret_key).public().to_raw_vec();
        let sr25519_key = sr25519::Pair::from_seed(&secret_key).public().to_raw_vec();

        let _ = Balances::deposit_creating(&attester, 100u128);

        assert_ok!(Attesters::register_attester(
            Origin::signed(attester),
            10u128,
            ecdsa_key.try_into().unwrap(),
            ed25519_key.try_into().unwrap(),
            sr25519_key.try_into().unwrap(),
            None,
        ));

        // Run to active set selection
        Attesters::on_initialize(400u32);
    }

    #[test]
    fn register_attester_from_single_private_key() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            register_attester_with_single_private_key([1u8; 32]);
        });
    }

    fn sign_and_submit_attestation(
        attester: AccountId,
        message: [u8; 32],
        key_type: KeyTypeId,
        target: TargetId,
        secret_key: [u8; 32],
    ) {
        let signature: Vec<u8> = match key_type {
            ECDSA_ATTESTER_KEY_TYPE_ID =>
                ecdsa::Pair::from_seed(&secret_key).sign(&message).encode(),
            ED25519_ATTESTER_KEY_TYPE_ID => ed25519::Pair::from_seed(&secret_key)
                .sign(&message)
                .encode(),
            SR25519_ATTESTER_KEY_TYPE_ID => sr25519::Pair::from_seed(&secret_key)
                .sign(&message)
                .encode(),
            _ => panic!("Invalid key type"),
        };

        assert_ok!(Attesters::submit_attestation(
            Origin::signed(attester),
            message.to_vec(),
            signature,
            target,
            AttestationFor::SFX,
        ));
    }

    #[test]
    fn register_and_submit_attestation_in_ecdsa() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register an attester
            let attester = AccountId::from([1; 32]);
            register_attester_with_single_private_key([1u8; 32]);
            // Submit an attestation signed with the Ed25519 key
            let message: [u8; 32] = *b"message_that_needs_attestation32";
            sign_and_submit_attestation(
                attester,
                message,
                ECDSA_ATTESTER_KEY_TYPE_ID,
                [0u8; 4],
                [1u8; 32],
            );

            let attestation =
                Attesters::attestations(H256::from(*b"message_that_needs_attestation32"))
                    .expect("Attestation should exist");
            assert_eq!(attestation.status, AttestationStatus::Pending);
        });
    }

    #[test]
    fn double_attestation_is_not_allowed() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register an attester
            let attester = AccountId::from([1; 32]);
            register_attester_with_single_private_key([1u8; 32]);
            // Submit an attestation signed with the Ed25519 key
            let message: [u8; 32] = *b"message_that_needs_attestation32";
            sign_and_submit_attestation(
                attester.clone(),
                message,
                ECDSA_ATTESTER_KEY_TYPE_ID,
                [0u8; 4],
                [1u8; 32],
            );

            let same_signature_again = ecdsa::Pair::from_seed(&[1u8; 32]).sign(&message).encode();

            frame_support::assert_err!(
                Attesters::submit_attestation(
                    Origin::signed(attester),
                    message.to_vec(),
                    same_signature_again,
                    [0, 0, 0, 0],
                    AttestationFor::SFX,
                ),
                AttestersError::<MiniRuntime>::AttestationDoubleSignAttempt
            );
        });
    }

    #[test]
    fn committee_setup_and_transition() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // On initialization, the current committee should be empty and the previous committee should be None
            assert!(CurrentCommittee::<MiniRuntime>::get().is_empty());
            assert_eq!(PreviousCommittee::<MiniRuntime>::get(), vec![]);

            // Register multiple attesters
            let attester_count = 100;
            for counter in 1..=attester_count {
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            // Trigger the first setup
            Attesters::on_initialize(400u32);

            // Check if the committee is set up and has the correct size
            let committee = CurrentCommittee::<MiniRuntime>::get();
            let committee_size: u32 = <MiniRuntime as ConfigAttesters>::CommitteeSize::get();
            assert_eq!(committee.len(), committee_size as usize);

            // Check that each member of the committee is in the registered attesters
            for member in &committee {
                assert!(AttestersStore::<MiniRuntime>::contains_key(member));
            }

            // Trigger the transition
            Attesters::on_initialize(800u32);

            // Check if the previous committee is now set to the old committee and the new committee is different
            let previous_committee = PreviousCommittee::<MiniRuntime>::get();
            assert_eq!(previous_committee, committee);

            let new_committee = CurrentCommittee::<MiniRuntime>::get();
            // todo: RandomnessCollectiveFlip always returns 0x0000...0000 as random value
            // assert_ne!(new_committee, committee);

            // Check if the new committee is set up and has the correct size
            assert_eq!(new_committee.len(), Attesters::committee_size());

            // Check that each member of the new committee is in the registered attesters
            for member in &new_committee {
                assert!(AttestersStore::<MiniRuntime>::contains_key(member));
            }
        });
    }

    #[test]
    fn register_and_submit_32x_attestations_in_ecdsa_changes_status_to_approved() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                // Submit an attestation signed with the Ed25519 key
                let message: [u8; 32] = *b"message_that_needs_attestation32";
                sign_and_submit_attestation(
                    attester,
                    message,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    [0u8; 4],
                    [counter; 32],
                );
            }

            let attestation =
                Attesters::attestations(H256::from(*b"message_that_needs_attestation32"))
                    .expect("Attestation should exist");
            assert_eq!(attestation.status, AttestationStatus::Approved);
        });
    }

    #[test]
    fn register_and_submit_32x_attestations_in_ecdsa_with_batching() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let target: TargetId = [1u8; 4];
            let message: [u8; 32] = *b"message_that_needs_attestation32";

            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                // Submit an attestation signed with the Ed25519 key
                sign_and_submit_attestation(
                    attester,
                    message,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    target,
                    [counter; 32],
                );
            }

            // Check if the attestations have been added to the batch
            let batches = Batches::<MiniRuntime>::get(target).expect("Batches should exist");
            let first_batch = batches.first().expect("First batch should exist");
            assert_eq!(first_batch.attestations.len(), 32);
            assert_eq!(first_batch.status, BatchStatus::Pending);

            // Commit the batch
            let batch_index = 0;
            let _ = Attesters::commit_batch(
                Origin::signed(AccountId::from([1; 32])),
                target,
                batch_index,
            );

            // Check if the batch status has been updated to Committed
            let batches = Batches::<MiniRuntime>::get(target).expect("Batches should exist");
            let first_batch = batches.first().expect("First batch should exist");
            assert_eq!(first_batch.status, BatchStatus::Committed);
        });
    }

    #[test]
    fn attester_nomination() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register 64 attesters
            let mut attesters = Vec::new();

            for counter in 1..65u8 {
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                attesters.push(attester);
            }

            // Nominate the attesters
            for counter in 1..65u128 {
                let nominator = AccountId::from([(counter + 1) as u8; 32]);
                let attester = attesters[(counter - 1) as usize].clone();
                let amount = 1000u128 + counter;
                let _ = Balances::deposit_creating(&nominator, amount);
                assert_ok!(Attesters::nominate(
                    Origin::signed(nominator.clone()),
                    attester.clone(),
                    amount
                ));
            }

            Attesters::on_initialize(400);

            // Check that the top 32 attesters are the ones with the most nominations
            let active_set = ActiveSet::<MiniRuntime>::get();

            assert_eq!(active_set.len(), 32);
            let top_nominated_attesters = SortedNominatedAttesters::<MiniRuntime>::get();
            for (i, (attester, _nominated_stake)) in top_nominated_attesters.iter().enumerate() {
                let nominations = Attesters::read_nominations(attester);
                let total_nomination: Balance =
                    nominations.iter().map(|(_nominator, amount)| *amount).sum();
                assert_eq!(
                    total_nomination,
                    1000u128 + 64 + 10 - i as u128, // where 10 is the self-bond for attesters
                    "attester: {attester:?}, total_nomination: {total_nomination}"
                );
            }
        });
    }

    #[test]
    fn attester_nomination_generates_equal_inflation_rewards_for_attesters_and_nominators() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register 64 attesters
            let mut attesters = Vec::new();

            for counter in 1..65u8 {
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                attesters.push(attester);
            }

            // Nominate the attesters from separate nominators accounts
            for counter in 1..65u128 {
                let nominator = AccountId::from([(64 + counter + 1) as u8; 32]);
                let attester = attesters[(counter - 1) as usize].clone();
                let amount = 1000u128 + counter;
                let _ = Balances::deposit_creating(&nominator, amount);
                assert_ok!(Attesters::nominate(
                    Origin::signed(nominator.clone()),
                    attester.clone(),
                    amount
                ));
            }

            Attesters::on_initialize(400);

            // Trigger inflation rewards distribution
            let distribution_period =
                <MiniRuntime as ConfigRewards>::InflationDistributionPeriod::get();
            System::set_block_number(distribution_period);
            let equal_distribution: Balance = 32 * 1000u128;
            // check consumed all available rewards
            assert_eq!(
                Rewards::distribute_attester_rewards(equal_distribution),
                equal_distribution
            );

            // Check claimable rewards for attesters - only the top 32 set should be able to claim
            for counter in 1..33u128 {
                let attester = attesters[(64 - counter) as usize].clone();
                let claimable_rewards = Rewards::get_pending_claims(&attester);
                // 10% default commission rate of 32 x 1000u128 available rewards to distribute across 32x active set attesters
                let _one_period_claimable_reward = 100u128;
                assert_eq!(
                    claimable_rewards,
                    Some(vec![
                        ClaimableArtifacts {
                            beneficiary: attester.clone(),
                            role: CircuitRole::Attester,
                            total_round_claim: 100, // that's reward as an attester with 10% commission of 1000
                            benefit_source: BenefitSource::Inflation
                        },
                        ClaimableArtifacts {
                            beneficiary: attester,
                            role: CircuitRole::Staker,
                            total_round_claim: 8, // that's reward as a self-bonded staker
                            benefit_source: BenefitSource::Inflation
                        },
                    ])
                );
            }

            // The attesters outside of top 32 should not be able to claim
            for counter in 33..65u128 {
                let attester = attesters[(64 - counter) as usize].clone();
                let claimable_rewards = Rewards::get_pending_claims(&attester);
                assert_eq!(claimable_rewards, None);
            }

            // Check claimable rewards for nominators that voted for the 32 top attesters
            for counter in 33..65u128 {
                let nominator = AccountId::from([(64 + counter + 1) as u8; 32]);
                let claimable_rewards = Rewards::get_pending_claims(&nominator);
                let one_period_claimable_reward = 1000u128 - 100 - 9; // 1000 - 100 (attester reward) - 9 (self-bonded staker reward)
                println!("checking for nominator: {nominator:?}");
                assert_eq!(
                    claimable_rewards,
                    Some(vec![ClaimableArtifacts {
                        beneficiary: nominator.clone(),
                        role: CircuitRole::Staker,
                        total_round_claim: one_period_claimable_reward,
                        benefit_source: BenefitSource::Inflation,
                    }])
                );
            }

            // Check nominators that not voted for the 32 top attesters - they should not be able to claim
            for counter in 1..33u128 {
                let nominator = AccountId::from([(64 + counter + 1) as u8; 32]);
                let claimable_rewards = Rewards::get_pending_claims(&nominator);
                println!("checking for nominator: {nominator:?}");
                assert_eq!(claimable_rewards, None,);
            }
        });
    }

    #[test]
    fn attester_unnomination() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register 3 attesters
            let mut attesters = Vec::new();

            for counter in 1..4u8 {
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                attesters.push(attester);
            }

            // Nominate the attesters
            let nominator = AccountId::from([250; 32]);
            let _ = Balances::deposit_creating(&nominator, 3000);

            for attester in &attesters {
                assert_ok!(Attesters::nominate(
                    Origin::signed(nominator.clone()),
                    attester.clone(),
                    1000
                ));
            }

            // Unnominate one attester
            let attester_to_unnominate = attesters[1].clone();
            assert_ok!(Attesters::unnominate(
                Origin::signed(nominator.clone()),
                attester_to_unnominate.clone()
            ));

            // Verify that the unnomination is pending and nominations are not updated yet
            let pending_unnominations = PendingUnnominations::<MiniRuntime>::get(&nominator);
            assert_eq!(pending_unnominations.len(), 1);
            let pending_unnominations = pending_unnominations.unwrap();
            assert_eq!(pending_unnominations[0].0, attester_to_unnominate);

            // Still 2 nominations - the unnomination is not yet processed
            let nominations = Attesters::read_nominations(&attester_to_unnominate);
            assert_eq!(nominations.len(), 2);
        });
    }

    #[test]
    fn on_initialize_logic_unnominate_larger_set() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register 64 attesters
            let mut attesters = Vec::new();

            for counter in 1..65u8 {
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                attesters.push(attester);
            }

            // Nominate the attesters with different stakes
            let nominator = AccountId::from([250; 32]);
            let _ = Balances::deposit_creating(&nominator, 128_000_000);

            for (i, attester) in attesters.iter().enumerate() {
                for _ in 0..2 {
                    assert_ok!(Attesters::nominate(
                        Origin::signed(nominator.clone()),
                        attester.clone(),
                        1000 + i as Balance
                    ));
                }
            }

            // Unnominate one attester
            let attester_to_unnominate = attesters[1].clone();
            assert_ok!(Attesters::unnominate(
                Origin::signed(nominator),
                attester_to_unnominate.clone()
            ));

            // Check if the attester_to_unnominate is in the active set
            assert!(ActiveSet::<MiniRuntime>::get().contains(&attester_to_unnominate));

            // Move to the block where unnomination is processed
            let unlock_block: BlockNumber = 3 * 400; // 1 is the current block number, 5 is the ShufflingFrequency
            System::set_block_number(unlock_block);

            // Call on_initialize
            Attesters::on_initialize(unlock_block);

            // Verify that the active set is updated correctly
            let active_set = ActiveSet::<MiniRuntime>::get();
            assert_eq!(active_set.len(), 32);

            // Check if the attester_to_unnominate is removed from the active set
            assert!(!active_set.contains(&attester_to_unnominate));
        });
    }
}
