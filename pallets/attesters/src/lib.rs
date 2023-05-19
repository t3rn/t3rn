#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub use crate::pallet::*;

pub type TargetId = [u8; 4];

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    t3rn_primitives::reexport_currency_types!();
    use tiny_keccak::{Hasher, Keccak};

    use codec::{Decode, Encode};
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Randomness, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_core::H256;
    use t3rn_abi::{recode::Codec, FilledAbi};
    pub use t3rn_primitives::portal::InclusionReceipt;

    use sp_runtime::{
        traits::{CheckedAdd, CheckedMul, Saturating, Zero},
        Percent,
    };
    use sp_std::{convert::TryInto, prelude::*};

    pub use t3rn_primitives::attesters::{
        AttesterInfo, AttestersChange, AttestersReadApi, AttestersWriteApi, BatchConfirmedSfxId,
        CommitteeTransition, PublicKeyEcdsa33b, Signature65b, COMMITTEE_SIZE,
        ECDSA_ATTESTER_KEY_TYPE_ID, ED25519_ATTESTER_KEY_TYPE_ID, SR25519_ATTESTER_KEY_TYPE_ID,
    };
    use t3rn_primitives::{portal::Portal, xdns::Xdns};

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, PartialOrd)]
    pub enum BatchStatus {
        PendingMessage,
        PendingAttestation,
        ReadyForSubmissionByMajority,
        ReadyForSubmissionFullyApproved,
        Repatriated,
        Committed,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct BatchMessage<BlockNumber> {
        pub batch_sfx: Option<BatchConfirmedSfxId>,
        pub next_committee: Option<CommitteeTransition>,
        pub new_attesters: Option<AttestersChange>,
        pub ban_attesters: Option<AttestersChange>,
        pub remove_attesters: Option<AttestersChange>,
        pub signatures: Vec<(u32, Signature65b)>,
        pub status: BatchStatus,
        pub created: BlockNumber,
    }

    // Add the following method to `BatchMessage` struct
    impl<BlockNumber> BatchMessage<BlockNumber> {
        pub fn message(&self) -> Vec<u8> {
            let mut encoded_message = Vec::new();
            if let Some(ref sfx) = self.batch_sfx {
                sfx.encode_to(&mut encoded_message);
                // Remove first 1 byte if the message is not empty to strip the SCALE-vector length prefix
                if !encoded_message.is_empty() {
                    encoded_message.remove(0);
                }
            }
            if let Some(ref committee) = self.next_committee {
                committee.encode_to(&mut encoded_message);
            }
            if let Some(ref attestors) = self.new_attesters {
                attestors.encode_to(&mut encoded_message);
            }
            if let Some(ref attestors) = self.ban_attesters {
                attestors.encode_to(&mut encoded_message);
            }
            if let Some(ref attestors) = self.remove_attesters {
                attestors.encode_to(&mut encoded_message);
            }

            encoded_message
        }

        pub fn message_hash(&self) -> H256 {
            let mut keccak = Keccak::v256();
            keccak.update(&self.message());
            let mut res: [u8; 32] = [0; 32];
            keccak.finalize(&mut res);
            H256::from(res)
        }
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub enum Slash<BlockNumber> {
        // Slash for not submitting attestations
        LateOrNoSubmissionAtBlocks(Vec<BlockNumber>),
        // Permanent Slash for submitting invalid attestations
        Permanent,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct TargetBatchInclusionProof {
        // The batch message that was included in the block
        pub target_batch_message: Vec<u8>,
        // Signatures received on target
        pub signatures: Vec<(u32, Signature65b)>,
        // Inclusion merkle proof of the batch message
        pub inclusion_proof: Vec<u8>,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct TargetBatchDispatchEvent {
        // Signatures received on target
        pub signatures: Vec<(u32, Signature65b)>,
        // Message hash as H256 (32b)
        pub hash: H256,
        // The batch message that was included in the block
        pub message: Vec<u8>,
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
        type Portal: Portal<Self>;
        type Xdns: Xdns<Self>;
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
    pub type CurrentRetributionPerSFXPercentage<T: Config> = StorageValue<_, Percent, ValueQuery>;

    #[pallet::storage]
    pub type CurrentSlashTreasuryBalance<T: Config> = StorageValue<_, Percent, ValueQuery>;

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
    #[pallet::getter(fn attestation_targets)]
    pub type AttestationTargets<T: Config> = StorageValue<_, Vec<TargetId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_batches)]
    pub type NextBatch<T: Config> = StorageMap<_, Identity, TargetId, BatchMessage<T::BlockNumber>>;

    #[pallet::storage]
    #[pallet::getter(fn batches_to_sign)]
    pub type BatchesToSign<T: Config> =
        StorageMap<_, Identity, TargetId, Vec<BatchMessage<T::BlockNumber>>>;

    #[pallet::storage]
    #[pallet::getter(fn batches)]
    pub type Batches<T: Config> =
        StorageMap<_, Identity, TargetId, Vec<BatchMessage<T::BlockNumber>>>;

    #[pallet::storage]
    #[pallet::getter(fn pending_unnominations)]
    pub type PendingUnnominations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<(T::AccountId, BalanceOf<T>, BlockNumberFor<T>)>,
    >;

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
        AttesterDeregistrationScheduled(T::AccountId, T::BlockNumber),
        AttesterDeregistered(T::AccountId),
        AttestationSubmitted(T::AccountId),
        NewAttestationBatch(TargetId, BatchMessage<T::BlockNumber>),
        NewConfirmationBatch(TargetId, BatchMessage<T::BlockNumber>),
        Nominated(T::AccountId, T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        AttesterNotFound,
        ArithmeticOverflow,
        InvalidSignature,
        InvalidMessage,
        InvalidTargetInclusionProof,
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
        BatchHashMismatch,
        BatchNotFound,
        CollusionWithPermanentSlashDetected,
        BatchFoundWithUnsignableStatus,
        SfxAlreadyRequested,
        AddAttesterAlreadyRequested,
        RemoveAttesterAlreadyRequested,
        NextCommitteeAlreadyRequested,
        BanAttesterAlreadyRequested,
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

            let next_index = Attesters::<T>::iter().count() as u32;

            Attesters::<T>::insert(
                &account_id,
                AttesterInfo {
                    key_ec: ecdsa_key,
                    key_ed: ed25519_key,
                    key_sr: sr25519_key,
                    commission,
                    index: next_index,
                },
            );

            // Self nominate in order to be part of the active set selection
            Self::do_nominate(&account_id, &account_id, self_nominate_amount)?;

            Self::request_add_attesters_attestation(&account_id)?;

            Self::deposit_event(Event::AttesterRegistered(account_id));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn deregister_attester(origin: OriginFor<T>) -> DispatchResult {
            let attester = ensure_signed(origin)?;

            // Ensure the attester is registered
            ensure!(
                Attesters::<T>::contains_key(&attester),
                Error::<T>::NotRegistered
            );

            // Retreive the self-nomination amount
            let self_nomination =
                Nominations::<T>::get(&attester, &attester).unwrap_or(Zero::zero());

            // Schedule self-denomination
            // Calculate the block number when the unnomination can be processed after 2 x shuffling frequency
            let unlock_block = frame_system::Pallet::<T>::block_number()
                .checked_add(
                    &T::ShufflingFrequency::get()
                        .checked_mul(&T::BlockNumber::from(2u32))
                        .ok_or(Error::<T>::ArithmeticOverflow)?,
                )
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            // Store the pending unnomination
            PendingUnnominations::<T>::mutate(&attester, |pending_unnominations| {
                let pending_unnominations = pending_unnominations.get_or_insert_with(Vec::new);
                pending_unnominations.push((attester.clone(), self_nomination, unlock_block));
            });

            Self::deposit_event(Event::AttesterDeregistrationScheduled(
                attester,
                unlock_block,
            ));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn remove_attestation_target(origin: OriginFor<T>, target: TargetId) -> DispatchResult {
            ensure_root(origin)?;

            let mut targets = AttestationTargets::<T>::get();

            // Remove target if exists
            if !targets.contains(&target) {
                targets.retain(|&x| x != target);
            }

            AttestationTargets::<T>::put(targets);
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn add_attestation_target(origin: OriginFor<T>, target: TargetId) -> DispatchResult {
            ensure_root(origin)?;

            let mut targets = AttestationTargets::<T>::get();

            // Add target if doesn't exist yet
            if !targets.contains(&target) {
                targets.push(target);
            }

            AttestationTargets::<T>::put(targets);
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn submit_attestation(
            // Must be signed by the attester in current Committee
            origin: OriginFor<T>,
            // Message being a hash of the batch of attestations to sign
            message: H256,
            // Signature of the message
            signature: Vec<u8>,
            // Target of the attestation
            target: TargetId,
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
                .verify_attestation_signature(
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    &message.encode(),
                    &signature,
                )
                .map_err(|_| Error::<T>::InvalidSignature)?;

            let signature_65b: [u8; 65] = signature
                .try_into()
                .map_err(|_| Error::<T>::InvalidSignature)?;

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

            Batches::<T>::try_mutate(target, |batches_option| {
                let batches = batches_option.as_mut().ok_or(Error::<T>::BatchNotFound)?;

                // Find the batch with the status PendingAttestation and the same message
                let batch = batches
                    .iter_mut()
                    .find(|batch| batch.message_hash() == message)
                    .ok_or(Error::<T>::BatchNotFound)?;

                ensure!(
                    batch.status == BatchStatus::PendingAttestation
                        || batch.status == BatchStatus::ReadyForSubmissionByMajority,
                    Error::<T>::BatchFoundWithUnsignableStatus
                );

                // Check if the attester has already signed the batch
                ensure!(
                    !batch
                        .signatures
                        .iter()
                        .any(|(attester_index, _)| *attester_index == attester.index),
                    Error::<T>::AttestationDoubleSignAttempt
                );

                // Add signature to the batch
                batch.signatures.push((attester.index, signature_65b));

                // Update the status of the batch
                let quorum = (T::ActiveSetSize::get() * 2 / 3) as usize;
                let full_approval = T::ActiveSetSize::get() as usize;
                if batch.signatures.len() >= quorum {
                    log::debug!(
                        "Batch {:?} is ready for submission by majority",
                        batch.message_hash()
                    );
                }
                if batch.signatures.len() >= full_approval {
                    batch.status = BatchStatus::ReadyForSubmissionFullyApproved;
                    log::debug!(
                        "Batch {:?} is ready for submission by full approval",
                        batch.message_hash()
                    );
                    Self::deposit_event(Event::NewConfirmationBatch(target, batch.clone()));
                }

                Self::deposit_event(Event::AttestationSubmitted(account_id));

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn commit_batch(
            origin: OriginFor<T>,
            target: TargetId,
            target_inclusion_proof_encoded: Vec<u8>, // Add this parameter to accept Ethereum batch hash
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;

            let target_codec = T::Xdns::get_target_codec(&target)?;

            // ToDo: Check the source address of the batch ensuring matches Escrow contract address.
            let _target_escrow_address = T::Xdns::get_escrow_account(&target)?;

            let escrow_batch_success_descriptor = b"EscrowBatchSuccess:Struct(\
                Signatures:Vec(Tuple(Value32,Bytes)),\
                MessageHash:H256,\
                Message:Bytes\
            )"
            .to_vec();

            #[cfg(not(feature = "test-skip-verification"))]
            let escrow_inclusion_receipt =
                T::Portal::verify_event_inclusion(target, target_inclusion_proof_encoded, None)?;
            #[cfg(feature = "test-skip-verification")]
            let escrow_inclusion_receipt = InclusionReceipt::<T::BlockNumber> {
                height: Zero::zero(),
                message: target_inclusion_proof_encoded,
                including_header: [0u8; 32].encode(),
            };

            #[cfg(not(feature = "test-skip-verification"))]
            let recoded_batch_event_bytes = FilledAbi::try_fill_abi(
                escrow_batch_success_descriptor.try_into()?,
                escrow_inclusion_receipt.message,
                target_codec.clone(),
            )?
            .recode_as(&target_codec, &Codec::Scale)?;

            #[cfg(feature = "test-skip-verification")]
            let recoded_batch_event_bytes = escrow_inclusion_receipt.message;

            let on_target_batch_event =
                TargetBatchDispatchEvent::decode(&mut &recoded_batch_event_bytes[..])
                    .map_err(|_| Error::<T>::InvalidTargetInclusionProof)?;

            let message = on_target_batch_event.message.clone();

            match Self::find_and_update_batch(target, &message) {
                Err(_e) => {
                    // At this point we know the valid message has been recorded on target Escrow Smart Contract
                    // If we can't find the corresponding batch by the message - we have a problem - attesters are colluding.
                    log::error!(
                        "Collusion detected on target: {:?} for message: {:?} with hash {:?}",
                        target,
                        &message,
                        on_target_batch_event.hash
                    );
                    Self::apply_permanent_attesters_slash(
                        on_target_batch_event
                            .signatures
                            .iter()
                            .map(|(attester_index, _)| *attester_index)
                            .collect(),
                    );

                    Err(Error::<T>::CollusionWithPermanentSlashDetected.into())
                },
                Ok(()) => Self::reward_submitter(submitter, target),
            }
        }

        #[pallet::weight(10_000)]
        pub fn set_confirmation_cost(
            origin: OriginFor<T>,
            target: TargetId,
            cost: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            FastConfirmationCost::<T>::insert(target, cost);

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

            Self::do_nominate(&nominator, &attester, amount)?;
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

            // Calculate the block number when the unnomination can be processed after 2 x shuffling frequency
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

    impl<T: Config> AttestersWriteApi<T::AccountId, Error<T>> for Pallet<T> {
        fn request_sfx_attestation(target: TargetId, sfx_id: H256) -> Result<(), Error<T>> {
            NextBatch::<T>::try_mutate(target, |next_batch| {
                if let Some(ref mut next_batch) = next_batch {
                    if let Some(ref mut batch_sfx) = &mut next_batch.batch_sfx {
                        if batch_sfx.contains(&sfx_id) {
                            return Err(Error::<T>::SfxAlreadyRequested)
                        } else {
                            batch_sfx.push(sfx_id);
                        }
                    } else {
                        next_batch.batch_sfx = Some(vec![sfx_id]);
                    }
                    Ok(())
                } else {
                    Err(Error::<T>::BatchNotFound)
                }
            })
        }

        fn request_add_attesters_attestation(new_attester: &T::AccountId) -> Result<(), Error<T>> {
            let attester_info =
                Attesters::<T>::get(new_attester).ok_or(Error::<T>::AttesterNotFound)?;

            let attester_key_index = (attester_info.key_ec, attester_info.index);

            for target in AttestationTargets::<T>::get() {
                NextBatch::<T>::try_mutate(target, |next_batch| {
                    let next_batch = next_batch.as_mut().ok_or(Error::<T>::BatchNotFound)?;

                    match &mut next_batch.new_attesters {
                        Some(attesters) => {
                            ensure!(
                                !attesters.contains(&attester_key_index),
                                Error::<T>::AddAttesterAlreadyRequested
                            );
                            attesters.push(attester_key_index);
                        },
                        None => {
                            next_batch.new_attesters = Some(vec![attester_key_index]);
                        },
                    }
                    Ok(())
                })?;
            }

            Ok(())
        }

        fn request_ban_attesters_attestation(ban_attester: &T::AccountId) -> Result<(), Error<T>> {
            let attester_info =
                Attesters::<T>::get(ban_attester).ok_or(Error::<T>::AttesterNotFound)?;

            let attester_key_index = (attester_info.key_ec, attester_info.index);

            for target in AttestationTargets::<T>::get() {
                NextBatch::<T>::try_mutate(target, |next_batch| {
                    let next_batch = next_batch.as_mut().ok_or(Error::<T>::BatchNotFound)?;

                    match &mut next_batch.ban_attesters {
                        Some(attesters) => {
                            ensure!(
                                !attesters.contains(&attester_key_index),
                                Error::<T>::BanAttesterAlreadyRequested
                            );
                            attesters.push(attester_key_index);
                        },
                        None => {
                            next_batch.ban_attesters = Some(vec![attester_key_index]);
                        },
                    }
                    Ok(())
                })?;
            }

            Ok(())
        }

        fn request_remove_attesters_attestation(
            remove_attesters: &T::AccountId,
        ) -> Result<(), Error<T>> {
            let attester_info =
                Attesters::<T>::get(remove_attesters).ok_or(Error::<T>::AttesterNotFound)?;

            let attester_key_index = (attester_info.key_ec, attester_info.index);

            for target in AttestationTargets::<T>::get() {
                NextBatch::<T>::try_mutate(target, |next_batch| {
                    let next_batch = next_batch.as_mut().ok_or(Error::<T>::BatchNotFound)?;

                    match &mut next_batch.remove_attesters {
                        Some(attesters) => {
                            ensure!(
                                !attesters.contains(&attester_key_index),
                                Error::<T>::RemoveAttesterAlreadyRequested
                            );
                            attesters.push(attester_key_index);
                        },
                        None => {
                            next_batch.remove_attesters = Some(vec![attester_key_index]);
                        },
                    }
                    Ok(())
                })?;
            }

            Ok(())
        }

        fn request_next_committee_attestation(next_committee: CommitteeTransition) {
            for target in AttestationTargets::<T>::get() {
                if let Some(next_batch) = NextBatch::<T>::get(target) {
                    if next_batch.next_committee.is_none() {
                        // We only update the next_committee if it was None.
                        NextBatch::<T>::insert(
                            target,
                            BatchMessage {
                                next_committee: Some(next_committee),
                                ..next_batch
                            },
                        );
                    }
                }
            }
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

        /// # apply_partial_stake_slash
        ///
        /// This function applies a partial slash to the stakes of an attester and its nominators.
        /// It returns the self-nomination balance of the given attester and the updated nomination balances
        /// of the nominators with an applied grace percent.
        ///
        /// ## Parameters
        ///
        /// - `attester`: The account ID of the attester whose stake is being slashed.
        /// - `nominations`: A vector of tuples where each tuple represents a nominator and its balance.
        /// - `percent_slash`: The percent of stake to slash from the attester.
        /// - `percent_nominator_grace`: The percent of stake to slash from the nominators.
        ///
        /// ## Returns
        ///
        /// This function returns a tuple containing two elements:
        ///
        /// - The first element is the self-nomination balance of the attester after the slash has been applied.
        /// - The second element is a vector of tuples where each tuple represents a nominator and its balance after the slash has been applied.
        ///
        /// If the attester is not found in the nominations, the function returns zero as the self-nomination balance
        /// and an empty vector as the list of nominators.
        pub fn apply_partial_stake_slash(
            attester: T::AccountId,
            nominations: Vec<(T::AccountId, BalanceOf<T>)>,
            percent_slash: Percent,
            percent_nominator_grace: Percent,
        ) -> (BalanceOf<T>, Vec<(T::AccountId, BalanceOf<T>)>) {
            // Find the attester's self nomination balance or return zero if not found
            let self_nomination_balance: BalanceOf<T> = nominations
                .iter()
                .find_map(|(nominator, balance)| {
                    if nominator == &attester {
                        Some(*balance)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| {
                    log::warn!("Attester not found in nominations");
                    Zero::zero()
                });

            if self_nomination_balance.is_zero() {
                return (Zero::zero(), vec![])
            }

            // Calculate the amount to slash from the attester
            let slash_amount = percent_slash.mul_ceil(self_nomination_balance);

            // Update the nominations after slashing
            let nominators_after_slash = nominations
                .into_iter()
                .map(|(nominator, balance)| {
                    if nominator == attester {
                        // Subtract the slash amount from the attester's self nomination balance
                        (nominator, balance.saturating_sub(slash_amount))
                    } else {
                        // Subtract the nominator's grace amount from their nomination balance
                        let nominator_slash_amount = percent_nominator_grace.mul_ceil(balance);
                        (nominator, balance.saturating_sub(nominator_slash_amount))
                    }
                })
                .collect();

            (
                self_nomination_balance.saturating_sub(slash_amount),
                nominators_after_slash,
            )
        }

        /// Applies permanent slashes to colluding attesters.
        fn apply_permanent_attesters_slash(attester_indices: Vec<u32>) {
            for attester_index in attester_indices {
                if let Some((account_id, _attester_info)) =
                    Attesters::<T>::iter().find(|(_, info)| info.index == attester_index)
                {
                    PendingSlashes::<T>::insert(&account_id, vec![Slash::Permanent]);
                } else {
                    log::error!("Colluding attester index: {:?} not found", attester_index);
                }
            }
        }

        pub fn find_and_update_batch(target: TargetId, message: &Vec<u8>) -> DispatchResult {
            Batches::<T>::try_mutate(target, |batches_option| {
                let batches = batches_option.as_mut().ok_or(Error::<T>::BatchNotFound)?;
                let batch_by_message = batches
                    .iter_mut()
                    .find(|batch| &batch.message() == message)
                    .ok_or(Error::<T>::BatchNotFound)?;

                batch_by_message.status = BatchStatus::Committed;
                Ok(())
            })
        }

        pub fn reward_submitter(submitter: T::AccountId, target: TargetId) -> DispatchResult {
            let fast_confirmation_cost =
                FastConfirmationCost::<T>::get(target).unwrap_or(Zero::zero());
            let total_reward = fast_confirmation_cost.saturating_mul(T::RewardMultiplier::get());

            if total_reward > Zero::zero() {
                T::Currency::transfer(
                    &T::CommitmentRewardSource::get(),
                    &submitter,
                    total_reward,
                    ExistenceRequirement::KeepAlive,
                )?;
            }

            Ok(())
        }

        pub fn get_batches(
            target: TargetId,
            by_status: BatchStatus,
        ) -> Vec<BatchMessage<T::BlockNumber>> {
            // Get the batches to sign
            match Batches::<T>::get(target) {
                Some(batches) => batches
                    .iter()
                    .filter(|b| b.status == by_status)
                    .cloned()
                    .collect(),
                None => vec![],
            }
        }

        pub fn get_batch_by_message(
            target: TargetId,
            message: Vec<u8>,
        ) -> Option<BatchMessage<T::BlockNumber>> {
            match Batches::<T>::get(target) {
                Some(batches) => batches.iter().find(|&b| b.message() == message).cloned(),
                None => None,
            }
        }

        pub fn get_batch_by_message_hash(
            target: TargetId,
            message_hash: H256,
        ) -> Option<BatchMessage<T::BlockNumber>> {
            match Batches::<T>::get(target) {
                Some(batches) => batches
                    .iter()
                    .find(|&b| b.message_hash() == message_hash)
                    .cloned(),
                None => None,
            }
        }

        pub fn get_latest_batch_to_sign(target: TargetId) -> Option<BatchMessage<T::BlockNumber>> {
            let mut batches = Self::get_batches(target, BatchStatus::PendingAttestation);
            batches.sort_by(|a, b| b.created.cmp(&a.created));
            batches.first().cloned()
        }

        pub fn get_latest_batch_to_sign_hash(target: TargetId) -> Option<H256> {
            let mut batches = Self::get_batches(target, BatchStatus::PendingAttestation);
            batches.sort_by(|a, b| b.created.cmp(&a.created));
            batches.iter().map(|b| b.message_hash()).next()
        }

        pub fn get_latest_batch_to_sign_message(target: TargetId) -> Option<Vec<u8>> {
            let mut batches = Self::get_batches(target, BatchStatus::PendingAttestation);
            batches.sort_by(|a, b| b.created.cmp(&a.created));
            batches.iter().map(|b| b.message()).next()
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
            nominator: &T::AccountId,
            attester: &T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            // Check if nominator has enough balance
            ensure!(
                T::Currency::free_balance(nominator) >= amount,
                Error::<T>::NominatorNotEnoughBalance
            );

            let current_nomination =
                Nominations::<T>::get(attester, nominator).unwrap_or(Zero::zero());

            let new_nomination = current_nomination + amount;
            Nominations::<T>::insert(attester, nominator, new_nomination);

            // Update the sorted list of nominated attesters
            SortedNominatedAttesters::<T>::try_mutate(|attesters| {
                let total_nomination = Nominations::<T>::iter_prefix(attester)
                    .map(|(_, balance)| balance)
                    .fold(Zero::zero(), |acc, balance| acc + balance);

                if let Some(index) = attesters.iter().position(|(a, _n)| a == attester) {
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
            T::Currency::reserve(nominator, amount)?;

            Ok(())
        }

        fn get_current_committee_indices() -> CommitteeTransition {
            let current_committee = CurrentCommittee::<T>::get();
            let mut committee_indices: CommitteeTransition = [0; COMMITTEE_SIZE]; // Initialize the committee_indices array

            for (i, attester) in current_committee.iter().enumerate() {
                if let Some(attester_info) = Attesters::<T>::get(attester) {
                    committee_indices[i] = attester_info.index;
                }
            }

            committee_indices.sort(); // Sorting the indices in ascending order

            committee_indices
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

        pub fn process_next_batch_window(
            n: T::BlockNumber,
            mut aggregated_weight: Weight,
        ) -> Weight {
            let quorum = (T::ActiveSetSize::get() * 2 / 3) as usize;

            for target in AttestationTargets::<T>::get() {
                let new_next_batch = BatchMessage {
                    batch_sfx: None,
                    next_committee: None,
                    new_attesters: None,
                    remove_attesters: None,
                    ban_attesters: None,
                    signatures: Vec::new(),
                    status: BatchStatus::PendingMessage,
                    created: n,
                };
                if let Some(mut next_batch) = NextBatch::<T>::get(target) {
                    // Check if batch has pending messages to attest for
                    if next_batch.message().len().is_zero() {
                        // Leave the batch empty if it has no messages
                    } else {
                        next_batch.status = BatchStatus::PendingAttestation;
                        // Push the batch to the batches vector
                        Batches::<T>::append(target, &next_batch);
                        // Create a new empty batch for the next window
                        NextBatch::<T>::insert(target, new_next_batch);

                        Self::deposit_event(Event::NewAttestationBatch(target, next_batch));
                    }
                } else {
                    // Create a new empty batch for the next window
                    NextBatch::<T>::insert(target, new_next_batch);
                }

                // If a batch exists, update its status
                Batches::<T>::mutate(target, |batches| {
                    if let Some(batches) = batches {
                        for mut batch in batches.iter_mut() {
                            if batch.status == BatchStatus::PendingAttestation
                                && batch.signatures.len() >= quorum
                            {
                                batch.status = BatchStatus::ReadyForSubmissionByMajority;
                                Self::deposit_event(Event::NewConfirmationBatch(
                                    target,
                                    batch.clone(),
                                ));
                            }
                        }
                    }
                });
            }
            aggregated_weight
        }

        pub fn process_pending_unnominations(
            n: T::BlockNumber,
            mut aggregated_weight: Weight,
        ) -> Weight {
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

                        // Check if this is self-deregistration
                        if &nominator == attester {
                            // Retreive the self-nomination amount
                            let self_nomination =
                                Nominations::<T>::get(attester, attester).unwrap_or(Zero::zero());

                            if self_nomination.saturating_sub(*amount) < T::MinAttesterBond::get() {
                                // Handle full self-deregistration with releasing all the nominator's funds
                                for nomination in Nominations::<T>::iter_prefix(attester) {
                                    let (nominator, amount) = nomination;
                                    // Remove the nomination from storage
                                    Nominations::<T>::remove(attester, &nominator);
                                    // Unreserve the nominated amount in the nominator's account
                                    T::Currency::unreserve(&nominator, amount);
                                    aggregated_weight += T::DbWeight::get().writes(2);
                                }
                                // Remove the attester from the list of attesters
                                Attesters::<T>::remove(attester);
                                aggregated_weight += T::DbWeight::get().writes(1);
                                SortedNominatedAttesters::<T>::mutate(|attesters| {
                                    if let Some(index) =
                                        attesters.iter().position(|(a, _n)| a == attester)
                                    {
                                        attesters.remove(index);
                                    }
                                });
                                aggregated_weight += T::DbWeight::get().writes(1);

                                PendingUnnominations::<T>::remove(attester);
                                aggregated_weight += T::DbWeight::get().writes(1);

                                Self::deposit_event(Event::AttesterDeregistered(attester.clone()));

                                continue
                            }
                        }

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

            aggregated_weight
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: T::BlockNumber) -> Weight {
            let mut aggregated_weight: Weight = 0;
            // Check if a shuffling round has passed
            if (n % T::ShufflingFrequency::get()).is_zero() {
                // Process pending unnominations
                aggregated_weight = Self::process_pending_unnominations(n, aggregated_weight);

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
                } else {
                    Self::request_next_committee_attestation(Self::get_current_committee_indices());
                    aggregated_weight += T::DbWeight::get().reads_writes(2, 2);
                }

                return aggregated_weight
            }

            if (n % T::BatchingWindow::get()).is_zero() {
                // Check if there any pending attestations to submit with the current batch
                aggregated_weight = Self::process_next_batch_window(n, aggregated_weight);
            }
            aggregated_weight
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
        fn build(&self) {
            for target in AttestationTargets::<T>::get() {
                let new_batch_message = BatchMessage {
                    batch_sfx: None,
                    next_committee: None,
                    new_attesters: None,
                    remove_attesters: None,
                    ban_attesters: None,
                    signatures: Vec::new(),
                    status: BatchStatus::PendingMessage,
                    created: frame_system::Pallet::<T>::block_number(),
                };
                // Create new batch for next window
                NextBatch::<T>::insert(target, new_batch_message.clone());
            }
        }
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
        assert_err, assert_ok,
        traits::{Currency, Get, Hooks, Len},
        StorageValue,
    };
    use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, Pair, RuntimePublic};
    use sp_core::H256;

    use crate::TargetBatchDispatchEvent;

    use sp_std::convert::TryInto;
    use t3rn_mini_mock_runtime::{
        AccountId, ActiveSet, Attesters, AttestersError, AttestersStore, Balance, Balances,
        BatchMessage, BatchStatus, BlockNumber, ConfigAttesters, ConfigRewards, CurrentCommittee,
        ExtBuilder, MiniRuntime, NextBatch, Nominations, Origin, PendingSlashes,
        PendingUnnominations, PreviousCommittee, Rewards, SortedNominatedAttesters, System,
    };
    use t3rn_primitives::{
        attesters::{AttesterInfo, AttestersReadApi, AttestersWriteApi, CommitteeTransition},
        claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
        xdns::GatewayRecord,
        ExecutionVendor, GatewayVendor,
    };
    use tiny_keccak::{Hasher, Keccak};

    pub fn deregister_attester(attester: AccountId) {
        // Assert that attester is register prior to deregistration
        assert!(AttestersStore::<MiniRuntime>::get(&attester).is_some(),);

        let self_nomination_amount = Nominations::<MiniRuntime>::get(&attester, &attester).unwrap();

        assert!(self_nomination_amount > <MiniRuntime as ConfigAttesters>::MinAttesterBond::get());

        let attester_balance_prior = Balances::free_balance(&attester);

        let nominations_state_prior = Nominations::<MiniRuntime>::iter_prefix(&attester)
            .map(|(nominator, nomination)| {
                (
                    nominator.clone(),
                    nomination,
                    Balances::free_balance(&nominator),
                )
            })
            .collect::<Vec<(AccountId, Balance, Balance)>>();

        assert_ok!(Attesters::deregister_attester(Origin::signed(
            attester.clone()
        ),));

        // Check Pending Unnomination is created
        assert!(PendingUnnominations::<MiniRuntime>::get(&attester).is_some());

        // Check Pending Unnomination is created with entire self-nomination amount
        assert_eq!(
            PendingUnnominations::<MiniRuntime>::get(&attester).unwrap(),
            vec![(attester.clone(), self_nomination_amount, 801u32)],
        );

        // Run to active to unlock block = 2 x shuffling frequency + next window
        Attesters::on_initialize(1200u32);

        // Assert that attester is deregistered
        assert!(AttestersStore::<MiniRuntime>::get(&attester).is_none(),);

        // Assume not in active set
        assert!(!ActiveSet::<MiniRuntime>::get()
            .iter()
            .any(|x| x == &attester));

        // Assume deposit is returned to attester
        assert_eq!(
            Balances::free_balance(&attester),
            attester_balance_prior + self_nomination_amount
        );

        // Assume nominators are refunded
        for (nominator, nomination, nominator_balance_prior) in nominations_state_prior {
            assert_eq!(
                Balances::free_balance(&nominator),
                nominator_balance_prior + nomination
            )
        }
    }

    pub fn register_attester_with_single_private_key(secret_key: [u8; 32]) -> AttesterInfo {
        // Register an attester
        let attester = AccountId::from(secret_key);

        let ecdsa_key = ecdsa::Pair::from_seed(&secret_key).public().to_raw_vec();
        let ed25519_key = ed25519::Pair::from_seed(&secret_key).public().to_raw_vec();
        let sr25519_key = sr25519::Pair::from_seed(&secret_key).public().to_raw_vec();

        let _ = Balances::deposit_creating(&attester, 100u128);

        assert_ok!(Attesters::register_attester(
            Origin::signed(attester.clone()),
            10u128,
            ecdsa_key.clone().try_into().unwrap(),
            ed25519_key.clone().try_into().unwrap(),
            sr25519_key.clone().try_into().unwrap(),
            None,
        ));

        // Run to active set selection
        Attesters::on_initialize(400u32);

        let attester_info: AttesterInfo = AttestersStore::<MiniRuntime>::get(&attester).unwrap();
        assert_eq!(attester_info.key_ed.encode(), ed25519_key);
        assert_eq!(attester_info.key_ec.encode(), ecdsa_key);
        assert_eq!(attester_info.key_sr.encode(), sr25519_key);
        attester_info
    }

    pub fn add_target_and_transition_to_next_batch(target: TargetId) -> BlockNumber {
        Attesters::add_attestation_target(Origin::root(), target);
        let current_block: BlockNumber = System::block_number();
        let batching_window: BlockNumber = <MiniRuntime as ConfigAttesters>::BatchingWindow::get();

        // calculate the closest multiple of batching_window
        let closest_block = ((current_block / batching_window) + 1) * batching_window;

        System::set_block_number(closest_block);

        // assert_eq!(NextBatch::<MiniRuntime>::get(target), None);
        // Transition to the next batch
        System::set_block_number(closest_block);
        Attesters::on_initialize(closest_block);
        assert_eq!(
            NextBatch::<MiniRuntime>::get(target),
            Some(BatchMessage {
                batch_sfx: None,
                next_committee: None,
                new_attesters: None,
                remove_attesters: None,
                ban_attesters: None,
                signatures: Vec::new(),
                status: BatchStatus::PendingMessage,
                created: closest_block,
            })
        );

        closest_block
    }

    #[test]
    fn register_attester_from_single_private_key() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            register_attester_with_single_private_key([1u8; 32]);
        });
    }

    #[test]
    fn deregister_attester_releases_all_funds() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let _attester_info = register_attester_with_single_private_key([1u8; 32]);
            let attester_account_id = AccountId::from([1u8; 32]);

            deregister_attester(attester_account_id);
        });
    }

    // Returns H256 message hash + signature as Vec<u8>
    fn sign_and_submit_sfx_to_latest_attestation(
        attester: AccountId,
        message: [u8; 32],
        key_type: KeyTypeId,
        target: TargetId,
        secret_key: [u8; 32],
    ) -> (H256, Vec<u8>) {
        // Check if batch with message exists and if not create one
        if Attesters::get_latest_batch_to_sign_message(target).is_none()
            || Attesters::get_latest_batch_to_sign_message(target).unwrap() != message
        {
            let _current_block_1 = add_target_and_transition_to_next_batch(target);

            let sfx_id_a = H256::from(message);
            assert_ok!(Attesters::request_sfx_attestation(target, sfx_id_a));

            let _current_block_2 = add_target_and_transition_to_next_batch(target);
        }
        let latest_batch_hash = Attesters::get_latest_batch_to_sign_hash(target).unwrap();

        let signature: Vec<u8> = match key_type {
            ECDSA_ATTESTER_KEY_TYPE_ID => ecdsa::Pair::from_seed(&secret_key)
                .sign(latest_batch_hash.as_ref())
                .encode(),
            ED25519_ATTESTER_KEY_TYPE_ID => ed25519::Pair::from_seed(&secret_key)
                .sign(latest_batch_hash.as_ref())
                .encode(),
            SR25519_ATTESTER_KEY_TYPE_ID => sr25519::Pair::from_seed(&secret_key)
                .sign(latest_batch_hash.as_ref())
                .encode(),
            _ => panic!("Invalid key type"),
        };

        assert_ok!(Attesters::submit_attestation(
            Origin::signed(attester),
            latest_batch_hash,
            signature.clone(),
            target,
        ));

        (latest_batch_hash, signature)
    }

    #[test]
    fn register_and_submit_attestation_in_ecdsa() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register an attester
            let attester = AccountId::from([1; 32]);
            let attester_info = register_attester_with_single_private_key([1u8; 32]);
            // Submit an attestation signed with the Ed25519 key
            let sfx_id_to_sign_on: [u8; 32] = *b"message_that_needs_attestation32";
            let (_hash, signature) = sign_and_submit_sfx_to_latest_attestation(
                attester,
                sfx_id_to_sign_on,
                ECDSA_ATTESTER_KEY_TYPE_ID,
                [0u8; 4],
                [1u8; 32],
            );

            let latest_batch = Attesters::get_latest_batch_to_sign([0u8; 4]);
            assert!(latest_batch.is_some());

            let latest_batch_some = latest_batch.unwrap();
            assert_eq!(latest_batch_some.status, BatchStatus::PendingAttestation);

            // Attesters::attestations(H256::from(*b"message_that_needs_attestation32"))
            //     .expect("Attestation should exist");
            assert_eq!(
                latest_batch_some.signatures,
                vec![(attester_info.index, signature.try_into().unwrap())]
            );
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
            let sfx_id_to_sign_on: [u8; 32] = *b"message_that_needs_attestation32";
            let (message_hash, _signature) = sign_and_submit_sfx_to_latest_attestation(
                attester.clone(),
                sfx_id_to_sign_on,
                ECDSA_ATTESTER_KEY_TYPE_ID,
                [0u8; 4],
                [1u8; 32],
            );

            let same_signature_again = ecdsa::Pair::from_seed(&[1u8; 32])
                .sign(message_hash.as_ref())
                .encode();

            assert_err!(
                Attesters::submit_attestation(
                    Origin::signed(attester),
                    message_hash,
                    same_signature_again,
                    [0, 0, 0, 0],
                ),
                AttestersError::<MiniRuntime>::AttestationDoubleSignAttempt
            );
        });
    }

    #[test]
    fn test_adding_sfx_moves_next_batch_to_pending_attestation() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let current_block_1 = add_target_and_transition_to_next_batch(target);

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation(target, sfx_id_a));

            let _current_block_2 = add_target_and_transition_to_next_batch(target);

            assert_eq!(
                Attesters::get_batches(target, BatchStatus::PendingAttestation),
                vec![BatchMessage {
                    batch_sfx: Some(vec![sfx_id_a]),
                    next_committee: None,
                    new_attesters: None,
                    ban_attesters: None,
                    remove_attesters: None,
                    signatures: vec![],
                    status: BatchStatus::PendingAttestation,
                    created: current_block_1,
                }]
            );
        });
    }

    #[test]
    fn test_pending_attestation_batch_with_single_sfx_yields_correct_message_hash() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let _current_block_1 = add_target_and_transition_to_next_batch(target);

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation(target, sfx_id_a));

            let _current_block_2 = add_target_and_transition_to_next_batch(target);

            assert_eq!(
                Attesters::get_latest_batch_to_sign_message(target),
                Some(sfx_id_a.encode())
            );

            let mut keccak = Keccak::v256();
            keccak.update(&sfx_id_a.encode());
            let mut res: [u8; 32] = [0; 32];
            keccak.finalize(&mut res);
            let expected_keccak_sfx_hash = H256::from(res);
            assert_eq!(
                Attesters::get_latest_batch_to_sign_hash(target),
                Some(expected_keccak_sfx_hash)
            );
        });
    }

    #[test]
    fn test_pending_attestation_batch_with_committee_transition_yields_correct_message_hash() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let current_block_1 = add_target_and_transition_to_next_batch(target);

            let committee_transition: CommitteeTransition = [
                1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32,
                14u32, 15u32, 16u32, 17u32, 18u32, 19u32, 20u32, 21u32, 22u32, 23u32, 24u32, 25u32,
                26u32, 27u32, 28u32, 29u32, 30u32, 31u32, 32u32,
            ];

            Attesters::request_next_committee_attestation(committee_transition);

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation(target, sfx_id_a));

            let add_attester = AccountId::from([1; 32]);
            register_attester_with_single_private_key([1u8; 32]);
            assert_eq!(
                Attesters::request_add_attesters_attestation(&add_attester)
                    .unwrap_err()
                    .encode(),
                AttestersError::<MiniRuntime>::AddAttesterAlreadyRequested.encode()
            );

            let remove_attester = AccountId::from([2; 32]);
            register_attester_with_single_private_key([2u8; 32]);
            assert_ok!(Attesters::request_remove_attesters_attestation(
                &remove_attester
            ));
            let ban_attester = AccountId::from([3; 32]);
            register_attester_with_single_private_key([3u8; 32]);
            assert_ok!(Attesters::request_ban_attesters_attestation(&ban_attester));

            let _current_block_2 = add_target_and_transition_to_next_batch(target);

            assert_eq!(
                Attesters::get_latest_batch_to_sign(target),
                Some(BatchMessage {
                    batch_sfx: Some(vec![sfx_id_a]),
                    next_committee: Some(committee_transition),
                    new_attesters: Some(vec![
                        (
                            [
                                3, 27, 132, 197, 86, 123, 18, 100, 64, 153, 93, 62, 213, 170, 186,
                                5, 101, 215, 30, 24, 52, 96, 72, 25, 255, 156, 23, 245, 233, 213,
                                221, 7, 143
                            ],
                            0
                        ),
                        (
                            [
                                2, 77, 75, 108, 209, 54, 16, 50, 202, 155, 210, 174, 185, 217, 0,
                                170, 77, 69, 217, 234, 216, 10, 201, 66, 51, 116, 196, 81, 167, 37,
                                77, 7, 102
                            ],
                            1
                        ),
                        (
                            [
                                2, 83, 31, 230, 6, 129, 52, 80, 61, 39, 35, 19, 50, 39, 200, 103,
                                172, 143, 166, 200, 60, 83, 126, 154, 68, 195, 197, 189, 189, 203,
                                31, 227, 55
                            ],
                            2
                        )
                    ]),
                    remove_attesters: Some(vec![(
                        [
                            2, 77, 75, 108, 209, 54, 16, 50, 202, 155, 210, 174, 185, 217, 0, 170,
                            77, 69, 217, 234, 216, 10, 201, 66, 51, 116, 196, 81, 167, 37, 77, 7,
                            102
                        ],
                        1
                    ),]),
                    ban_attesters: Some(vec![(
                        [
                            2, 83, 31, 230, 6, 129, 52, 80, 61, 39, 35, 19, 50, 39, 200, 103, 172,
                            143, 166, 200, 60, 83, 126, 154, 68, 195, 197, 189, 189, 203, 31, 227,
                            55
                        ],
                        2
                    )]),
                    signatures: vec![],
                    status: BatchStatus::PendingAttestation,
                    created: current_block_1,
                })
            );

            assert_eq!(
                Attesters::get_latest_batch_to_sign_message(target),
                Some(vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0,
                    6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 12,
                    0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 18,
                    0, 0, 0, 19, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24,
                    0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0, 28, 0, 0, 0, 29, 0, 0, 0, 30,
                    0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 12, 3, 27, 132, 197, 86, 123, 18, 100, 64,
                    153, 93, 62, 213, 170, 186, 5, 101, 215, 30, 24, 52, 96, 72, 25, 255, 156, 23,
                    245, 233, 213, 221, 7, 143, 0, 0, 0, 0, 2, 77, 75, 108, 209, 54, 16, 50, 202,
                    155, 210, 174, 185, 217, 0, 170, 77, 69, 217, 234, 216, 10, 201, 66, 51, 116,
                    196, 81, 167, 37, 77, 7, 102, 1, 0, 0, 0, 2, 83, 31, 230, 6, 129, 52, 80, 61,
                    39, 35, 19, 50, 39, 200, 103, 172, 143, 166, 200, 60, 83, 126, 154, 68, 195,
                    197, 189, 189, 203, 31, 227, 55, 2, 0, 0, 0, 4, 2, 83, 31, 230, 6, 129, 52, 80,
                    61, 39, 35, 19, 50, 39, 200, 103, 172, 143, 166, 200, 60, 83, 126, 154, 68,
                    195, 197, 189, 189, 203, 31, 227, 55, 2, 0, 0, 0, 4, 2, 77, 75, 108, 209, 54,
                    16, 50, 202, 155, 210, 174, 185, 217, 0, 170, 77, 69, 217, 234, 216, 10, 201,
                    66, 51, 116, 196, 81, 167, 37, 77, 7, 102, 1, 0, 0, 0
                ])
            );

            assert_eq!(
                Attesters::get_latest_batch_to_sign_hash(target),
                Some(
                    hex_literal::hex!(
                        "b6b73fb1c00b1d498da0d140eddebd31ff25d0cf04e1af88f935462c292c74e7"
                    )
                    .into()
                )
            );
        });
    }

    #[test]
    fn test_pending_attestation_batch_with_all_attestations_ordered_yields_correct_message_hash() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let _current_block_1 = add_target_and_transition_to_next_batch(target);

            let committee_transition: CommitteeTransition = [
                1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32,
                14u32, 15u32, 16u32, 17u32, 18u32, 19u32, 20u32, 21u32, 22u32, 23u32, 24u32, 25u32,
                26u32, 27u32, 28u32, 29u32, 30u32, 31u32, 32u32,
            ];

            Attesters::request_next_committee_attestation(committee_transition);

            let _current_block_2 = add_target_and_transition_to_next_batch(target);

            assert_eq!(
                Attesters::get_latest_batch_to_sign_message(target),
                Some(committee_transition.encode())
            );

            let mut keccak = Keccak::v256();
            keccak.update(&committee_transition.encode());
            let mut res: [u8; 32] = [0; 32];
            keccak.finalize(&mut res);
            let expected_keccak_hash = H256::from(res);
            assert_eq!(
                Attesters::get_latest_batch_to_sign_hash(target),
                Some(expected_keccak_hash)
            );
        });
    }

    #[test]
    fn test_adding_2_same_sfx_to_next_batch_is_impossible() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            add_target_and_transition_to_next_batch(target);

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation(target, sfx_id_a));

            assert_eq!(
                Attesters::request_sfx_attestation(target, sfx_id_a)
                    .unwrap_err()
                    .encode(),
                AttestersError::<MiniRuntime>::SfxAlreadyRequested.encode(),
            );
        });
    }

    #[test]
    fn test_adding_2_sfx_to_next_batch_and_transition_to_pending_attestation() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            assert_eq!(NextBatch::<MiniRuntime>::get(target), None);
            let current_block = add_target_and_transition_to_next_batch(target);

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation(target, sfx_id_a));

            // Verify that the attestation is added to the next batch
            let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
            assert_eq!(next_batch.batch_sfx, Some(vec![sfx_id_a]));

            // Add another SFX to the next batch
            let sfx_id_b = H256::repeat_byte(2);
            assert_ok!(Attesters::request_sfx_attestation(target, sfx_id_b));
            let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
            assert_eq!(next_batch.batch_sfx, Some(vec![sfx_id_a, sfx_id_b]));

            let mut empty_batch = BatchMessage {
                batch_sfx: None,
                next_committee: None,
                new_attesters: None,
                ban_attesters: None,
                remove_attesters: None,
                signatures: vec![],
                status: BatchStatus::PendingMessage,
                created: current_block,
            };
            let _current_block: BlockNumber = System::block_number();
            let batching_window: BlockNumber =
                <MiniRuntime as ConfigAttesters>::BatchingWindow::get();

            // Transition to the next batch
            System::set_block_number(batching_window * 2);
            Attesters::on_initialize(batching_window * 2);
            let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
            assert_eq!(next_batch.batch_sfx, None);

            // Verify that batches by status are correct
            assert_eq!(
                Attesters::get_batches(target, BatchStatus::PendingMessage),
                vec![]
            );
            assert_eq!(
                Attesters::get_batches(target, BatchStatus::PendingAttestation),
                vec![BatchMessage {
                    batch_sfx: Some(vec![sfx_id_a, sfx_id_b]),
                    next_committee: None,
                    new_attesters: None,
                    ban_attesters: None,
                    remove_attesters: None,
                    signatures: vec![],
                    status: BatchStatus::PendingAttestation,
                    created: batching_window,
                }]
            );
            assert_eq!(
                Attesters::get_batches(target, BatchStatus::ReadyForSubmissionFullyApproved),
                vec![]
            );
            assert_eq!(
                Attesters::get_batches(target, BatchStatus::ReadyForSubmissionByMajority),
                vec![]
            );
            empty_batch.created = batching_window * 2;
            assert_eq!(NextBatch::<MiniRuntime>::get(target), Some(empty_batch));
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
            let sfx_id_to_sign_on: [u8; 32] = *b"message_that_needs_attestation32";

            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                // Submit an attestation signed with the Ed25519 key
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    sfx_id_to_sign_on,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    [0u8; 4],
                    [counter; 32],
                );
            }

            let batch = Attesters::get_batch_by_message([0u8; 4], sfx_id_to_sign_on.encode())
                .expect("get_batch_by_message should return a batch");
            assert_eq!(batch.status, BatchStatus::ReadyForSubmissionFullyApproved);
        });
    }

    #[test]
    fn register_and_submit_21x_attestations_in_ecdsa_changes_status_to_approved_in_next_batching_window(
    ) {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let message: [u8; 32] = *b"message_that_needs_attestation32";

            for counter in 1..22u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                // Submit an attestation signed with the Ed25519 key
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    message,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    [0u8; 4],
                    [counter; 32],
                );
            }

            let batch = Attesters::get_batch_by_message([0u8; 4], message.encode())
                .expect("get_batch_by_message should return a batch");
            assert_eq!(batch.status, BatchStatus::PendingAttestation);

            // Trigger batching transition
            add_target_and_transition_to_next_batch([0u8; 4]);
            let batch = Attesters::get_batch_by_message([0u8; 4], message.encode())
                .expect("get_batch_by_message should return a batch");
            assert_eq!(batch.status, BatchStatus::ReadyForSubmissionByMajority);
        });
    }

    #[test]
    fn register_and_submit_32x_attestations_in_ecdsa_with_batching_plus_confirmation_to_polka_target(
    ) {
        let target: TargetId = [1u8; 4];
        let mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_gateway_records(vec![GatewayRecord {
                gateway_id: target,
                verification_vendor: GatewayVendor::Polkadot,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Rlp,
                registrant: None,
                escrow_account: Some(mock_escrow_account),
                allowed_side_effects: vec![],
            }])
            .build();
        ext.execute_with(|| {
            let message: [u8; 32] = *b"message_that_needs_attestation32";

            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                // Submit an attestation signed with the Ed25519 key
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    message,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    target,
                    [counter; 32],
                );
            }

            // Check if the attestations have been added to the batch
            let first_batch = Attesters::get_batch_by_message(target, message.encode())
                .expect("Batch by message should exist");

            let first_batch_hash = first_batch.message_hash();

            assert_eq!(first_batch.signatures.len(), 32);
            assert_eq!(
                first_batch.status,
                BatchStatus::ReadyForSubmissionFullyApproved
            );

            let mock_valid_batch_confirmation = TargetBatchDispatchEvent {
                signatures: first_batch.signatures,
                hash: first_batch_hash,
                message: message.encode(),
            };

            // Commit the batch
            assert_ok!(Attesters::commit_batch(
                Origin::signed(AccountId::from([1; 32])),
                target,
                mock_valid_batch_confirmation.encode(),
            ));

            // Check if the batch status has been updated to Committed
            let batch = Attesters::get_batch_by_message(target, message.encode())
                .expect("Batch by message should exist");
            assert_eq!(batch.status, BatchStatus::Committed);
        });
    }

    #[test]
    fn register_and_submit_32x_attestations_and_check_collusion_permanent_slash() {
        let target: TargetId = [1u8; 4];
        let mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_gateway_records(vec![GatewayRecord {
                gateway_id: target,
                verification_vendor: GatewayVendor::Polkadot,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Rlp,
                registrant: None,
                escrow_account: Some(mock_escrow_account),
                allowed_side_effects: vec![],
            }])
            .build();
        ext.execute_with(|| {
            let message: [u8; 32] = *b"message_that_needs_attestation32";

            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
                // Submit an attestation signed with the Ed25519 key
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    message,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    target,
                    [counter; 32],
                );
            }

            // Check if the attestations have been added to the batch
            let first_batch = Attesters::get_batch_by_message(target, message.encode())
                .expect("Batch by message should exist");
            assert_eq!(first_batch.signatures.len(), 32);
            assert_eq!(
                first_batch.status,
                BatchStatus::ReadyForSubmissionFullyApproved
            );

            let colluded_message: [u8; 32] = *b"_message_that_was_colluded_by_32";

            let latest_batch_hash = first_batch.message_hash();

            let colluded_batch_confirmation = TargetBatchDispatchEvent {
                signatures: first_batch.signatures,
                hash: latest_batch_hash,
                message: colluded_message.encode(),
            };

            assert_err!(
                Attesters::commit_batch(
                    Origin::signed(AccountId::from([1; 32])),
                    target,
                    colluded_batch_confirmation.encode(),
                ),
                AttestersError::<MiniRuntime>::CollusionWithPermanentSlashDetected
            );

            // Check if the batch status has not been updated to Committed
            let batch = Attesters::get_batch_by_message(target, message.encode())
                .expect("Batch by message should exist");

            assert_eq!(batch.status, BatchStatus::ReadyForSubmissionFullyApproved);

            // Check if all of the attesters have been slashed
            for counter in 1..33u8 {
                let attester = AccountId::from([counter; 32]);
                assert!(PendingSlashes::<MiniRuntime>::get(&attester).is_some());
            }
        });
    }

    #[test]
    fn attester_deregistration_refunds_to_nominators() {
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

            deregister_attester(AccountId::from([1; 32]));
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
