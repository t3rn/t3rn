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
    use sp_application_crypto::RuntimePublic;
    use sp_core::{crypto::KeyTypeId, ecdsa, ed25519, sr25519};
    use sp_runtime::{
        traits::{Saturating, Zero},
        RuntimeAppPublic,
    };

    use sp_std::{convert::TryInto, prelude::*};

    use sp_runtime::traits::Verify;

    // Key types for attester crypto
    pub const ECDSA_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"ecat");
    pub const ED25519_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"edat");
    pub const SR25519_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"srat");

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
    pub enum BatchStatus {
        Pending,
        ReadyForSubmission,
        Committed,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct Batch<Attester, Signature, BlockNumber> {
        pub attestations: Vec<Attestation<Attester, Signature>>,
        pub target: [u8; 4],
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
        pub fn new(target: [u8; 4], now: BlockNumber) -> Self {
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
        type Currency: ReservableCurrency<Self::AccountId>;
        type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;
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
    #[pallet::getter(fn attestations)]
    pub type Attestations<T: Config> =
        StorageMap<_, Identity, T::Hash, Attestation<T::AccountId, Vec<u8>>>;

    #[pallet::storage]
    #[pallet::getter(fn batches)]
    pub type Batches<T: Config> =
        StorageMap<_, Identity, [u8; 4], Vec<Batch<T::AccountId, Vec<u8>, T::BlockNumber>>>;

    #[pallet::storage]
    #[pallet::getter(fn nominations)]
    pub type Nominations<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, (T::AccountId, BalanceOf<T>)>;

    #[pallet::storage]
    #[pallet::getter(fn fast_confirmation_cost)]
    pub type FastConfirmationCost<T: Config> =
        StorageMap<_, Blake2_128Concat, [u8; 4], BalanceOf<T>>;

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
        AlreadyNominated,
        NominatorNotEnoughBalance,
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
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;

            // Ensure the attester is not already registered
            ensure!(
                !Attesters::<T>::contains_key(&account_id),
                Error::<T>::AlreadyRegistered
            );

            Attesters::<T>::insert(
                &account_id,
                AttesterInfo {
                    key_ec: ecdsa_key,
                    key_ed: ed25519_key,
                    key_sr: sr25519_key,
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
            target: [u8; 4],
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
            target: [u8; 4],
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
            Self::do_nominate(nominator.clone(), attester.clone(), amount);
            Self::deposit_event(Event::Nominated(nominator, attester, amount));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn committee_size() -> usize {
            T::CommitteeSize::get() as usize
        }

        pub fn do_nominate(
            nominator: T::AccountId,
            attester: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let mut nomination = match Nominations::<T>::get(&attester) {
                Some(nomination) => nomination,
                None => (attester.clone(), Zero::zero()),
            };

            // Check if nominator has enough balance
            ensure!(
                T::Currency::free_balance(&nominator) >= amount,
                Error::<T>::NominatorNotEnoughBalance
            );

            nomination.1 += amount;

            let nomination_amount = nomination.1;

            // Update the nomination storage item
            Nominations::<T>::insert(&attester, nomination);

            // Update the sorted list of nominated attesters
            SortedNominatedAttesters::<T>::try_mutate(|attesters| {
                if let Some(index) = attesters.iter().position(|(a, _n)| a == &attester) {
                    // Update the existing nomination amount
                    attesters[index] = (attester.clone(), nomination_amount);
                } else {
                    // Add the new attester to the list
                    attesters.push((attester.clone(), nomination_amount));
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
                // Update the active set of attesters
                ActiveSet::<T>::put(
                    SortedNominatedAttesters::<T>::get()
                        .iter()
                        .take(32)
                        .cloned()
                        .map(|(account_id, _balance)| account_id)
                        .collect::<Vec<T::AccountId>>(),
                );

                // Call shuffle_committee
                if !Self::shuffle_committee() {
                    log::error!("Failed to shuffle committee");
                    return T::DbWeight::get().reads_writes(2, 2)
                }

                return T::DbWeight::get().reads_writes(5, 5)
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

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct AttesterInfo {
        pub key_ed: [u8; 32],
        pub key_ec: [u8; 33],
        pub key_sr: [u8; 32],
    }

    impl AttesterInfo {
        pub fn verify_attestation_signature(
            &self,
            key_type: KeyTypeId,
            message: &Vec<u8>,
            signature: &[u8],
        ) -> Result<bool, DispatchError> {
            match key_type {
                ECDSA_ATTESTER_KEY_TYPE_ID => {
                    let ecdsa_sig = ecdsa::Signature::from_slice(signature)
                        .ok_or::<DispatchError>("InvalidSignature".into())?;
                    let ecdsa_public = ecdsa::Public::from_raw(self.key_ec);
                    Ok(ecdsa_public.verify(message, &ecdsa_sig))
                },
                ED25519_ATTESTER_KEY_TYPE_ID => {
                    let ed25519_sig = ed25519::Signature::from_slice(signature)
                        .ok_or::<DispatchError>("InvalidSignature".into())?;
                    let ed25519_public = ed25519::Public::from_raw(self.key_ed);
                    Ok(ed25519_public.verify(message, &ed25519_sig))
                },
                SR25519_ATTESTER_KEY_TYPE_ID => {
                    let sr25519_sig = sr25519::Signature::from_slice(signature)
                        .ok_or::<DispatchError>("InvalidSignature".into())?;
                    let sr25519_public = sr25519::Public::from_raw(self.key_sr);
                    Ok(sr25519_public.verify(message, &sr25519_sig))
                },
                _ => Err("InvalidKeyTypeId".into()),
            }
        }
    }
}

#[cfg(test)]
pub mod attesters_test {
    use super::{
        ECDSA_ATTESTER_KEY_TYPE_ID, ED25519_ATTESTER_KEY_TYPE_ID, SR25519_ATTESTER_KEY_TYPE_ID,
    };
    use codec::Encode;
    use frame_support::{
        assert_ok,
        traits::{Currency, Get, Hooks},
        StorageValue,
    };
    use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, Pair, RuntimePublic};
    use sp_core::H256;

    use sp_std::convert::TryInto;
    use t3rn_mini_mock_runtime::{
        AccountId, ActiveSet, AttestationFor, AttestationStatus, Attesters, AttestersConfig,
        AttestersError, AttestersStore, Balances, BatchStatus, Batches, CurrentCommittee,
        ExtBuilder, MiniRuntime, Nominations, Origin, PreviousCommittee, SortedNominatedAttesters,
    };

    fn register_attester_with_single_private_key(secret_key: [u8; 32]) {
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
        target: [u8; 4],
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
            let committee_size: u32 = <MiniRuntime as AttestersConfig>::CommitteeSize::get();
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
            let target: [u8; 4] = [1u8; 4];
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
            println!("Batches: {batches:?}");
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
                    Origin::signed(nominator),
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
                let nomination = Nominations::<MiniRuntime>::get(attester).unwrap();
                assert_eq!(nomination.0, *attester);
                assert_eq!(
                    nomination.1,
                    1000u128 + 64 + 10 - i as u128, // where 10 is the self-bond for attesters
                    "attester: {:?}, nomination: {}",
                    attester,
                    nomination.1
                );
            }
        });
    }
}
