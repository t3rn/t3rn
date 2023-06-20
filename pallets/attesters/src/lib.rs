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
    use sp_core::{hexdisplay::AsBytesRef, H256};
    use t3rn_abi::{recode::Codec, FilledAbi};
    pub use t3rn_primitives::portal::InclusionReceipt;

    use sp_runtime::{
        traits::{CheckedAdd, CheckedMul, Saturating, Zero},
        Percent,
    };
    use sp_std::{convert::TryInto, prelude::*};

    pub use t3rn_primitives::attesters::{
        AttesterInfo, AttestersChange, AttestersReadApi, AttestersWriteApi, BatchConfirmedSfxId,
        CommitteeTransitionIndices, LatencyStatus, PublicKeyEcdsa33b, Signature65b, COMMITTEE_SIZE,
        ECDSA_ATTESTER_KEY_TYPE_ID, ED25519_ATTESTER_KEY_TYPE_ID, SR25519_ATTESTER_KEY_TYPE_ID,
    };
    use t3rn_primitives::{
        attesters::{CommitteeRecoverable, CommitteeTransition},
        circuit::{Cause, CircuitStatus, ReadSFX},
        portal::Portal,
        rewards::RewardsWriteApi,
        xdns::Xdns,
        ExecutionVendor, GatewayVendor,
    };

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, PartialOrd)]
    pub enum BatchStatus {
        PendingMessage,
        PendingAttestation,
        ReadyForSubmissionByMajority,
        ReadyForSubmissionFullyApproved,
        Repatriated,
        Expired,
        Committed,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct BatchMessage<BlockNumber> {
        pub committed_sfx: Option<BatchConfirmedSfxId>,
        pub reverted_sfx: Option<BatchConfirmedSfxId>,
        pub next_committee: Option<CommitteeRecoverable>,
        pub banned_committee: Option<CommitteeRecoverable>,
        pub index: u32,
        // Below fields are not part of the message, but are used to track the state of the message
        pub signatures: Vec<(u32, Signature65b)>,
        pub created: BlockNumber,
        pub status: BatchStatus,
        pub latency: LatencyStatus,
    }

    impl<BlockNumber: Zero> Default for BatchMessage<BlockNumber> {
        fn default() -> Self {
            BatchMessage {
                committed_sfx: None,
                reverted_sfx: None,
                next_committee: None,
                banned_committee: None,
                signatures: Vec::new(),
                status: BatchStatus::PendingMessage,
                created: Zero::zero(),
                latency: LatencyStatus::OnTime,
                index: 0,
            }
        }
    }
    // Add the following method to `BatchMessage` struct
    impl<BlockNumber> BatchMessage<BlockNumber> {
        pub fn message(&self) -> Vec<u8> {
            let mut encoded_message = Vec::new();

            if let Some(ref committee) = self.next_committee {
                for recoverable in committee.iter() {
                    encoded_message.extend_from_slice(recoverable.as_bytes_ref());
                }
            }
            if let Some(ref committee) = self.banned_committee {
                for recoverable in committee.iter() {
                    encoded_message.extend_from_slice(recoverable.as_bytes_ref());
                }
            }
            if let Some(ref sfx) = self.committed_sfx {
                sfx.encode_to(&mut encoded_message);
                // Remove first 1 byte if the message is not empty to strip the SCALE-vector length prefix
                if !encoded_message.is_empty() {
                    encoded_message.remove(0);
                }
            }
            if let Some(ref sfx) = self.reverted_sfx {
                sfx.encode_to(&mut encoded_message);
                // Remove first 1 byte if the message is not empty to strip the SCALE-vector length prefix
                if !encoded_message.is_empty() {
                    encoded_message.remove(0);
                }
            }
            encoded_message.extend_from_slice(self.index.to_le_bytes().as_slice());
            encoded_message
        }

        pub fn message_hash(&self) -> H256 {
            let mut keccak = Keccak::v256();
            keccak.update(&self.message());
            let mut res: [u8; 32] = [0; 32];
            keccak.finalize(&mut res);
            H256::from(res)
        }

        pub fn is_empty(&self) -> bool {
            self.next_committee.is_none()
                && self.banned_committee.is_none()
                && self.committed_sfx.is_none()
                && self.reverted_sfx.is_none()
        }

        pub fn has_no_sfx(&self) -> bool {
            self.committed_sfx.is_none() && self.reverted_sfx.is_none()
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
        type RepatriationPeriod: Get<Self::BlockNumber>;
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
        type Rewards: RewardsWriteApi<Self::AccountId, BalanceOf<Self>, Self::BlockNumber>;
        type ReadSFX: ReadSFX<Self::Hash, Self::AccountId, BalanceOf<Self>, Self::BlockNumber>;
        type Xdns: Xdns<Self, BalanceOf<Self>>;
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
    pub type PermanentSlashes<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn attestation_targets)]
    pub type AttestationTargets<T: Config> = StorageValue<_, Vec<TargetId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_attestation_targets)]
    pub type PendingAttestationTargets<T: Config> = StorageValue<_, Vec<TargetId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn attesters_agreements)]
    pub type AttestersAgreements<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // Attester
        Blake2_128Concat,
        TargetId, // Target
        Vec<u8>,  // Recoverable pubkey/address from signature on target
    >;
    #[pallet::storage]
    #[pallet::getter(fn next_batches)]
    pub type NextBatch<T: Config> = StorageMap<_, Identity, TargetId, BatchMessage<T::BlockNumber>>;

    #[pallet::storage]
    #[pallet::getter(fn next_committee_on_target)]
    pub type NextCommitteeOnTarget<T: Config> =
        StorageMap<_, Identity, TargetId, CommitteeTransition>;

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
        NewAttestationMessageHash(TargetId, H256, ExecutionVendor),
        NewConfirmationBatch(TargetId, BatchMessage<T::BlockNumber>, Vec<u8>, H256),
        Nominated(T::AccountId, T::AccountId, BalanceOf<T>),
        NewTargetActivated(TargetId),
        NewTargetProposed(TargetId),
        AttesterAgreedToNewTarget(T::AccountId, TargetId, Vec<u8>),
        CurrentPendingAttestationBatches(TargetId, Vec<(u32, H256)>),
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
        AttesterDidNotAgreeToNewTarget,
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
        RejectingFromSlashedAttester,
        TargetAlreadyActive,
        TargetNotActive,
        XdnsTargetNotActive,
        XdnsGatewayDoesNotHaveEscrowAddressRegistered,
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
            PendingAttestationTargets::<T>::mutate(|pending| {
                if let Some(index) = pending.iter().position(|x| x == &target) {
                    pending.remove(index);
                }
            });

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn agree_to_new_attestation_target(
            origin: OriginFor<T>,
            target: TargetId,
            recoverable: Vec<u8>,
        ) -> DispatchResult {
            let attester = ensure_signed(origin)?;

            // Ensure the attester is registered
            ensure!(
                Attesters::<T>::contains_key(&attester),
                Error::<T>::NotRegistered
            );

            AttestersAgreements::<T>::insert(&attester, target, recoverable.clone());

            Self::deposit_event(Event::AttesterAgreedToNewTarget(
                attester,
                target,
                recoverable,
            ));

            if Self::try_activate_new_target(&target) {
                Self::deposit_event(Event::NewTargetActivated(target));
            }

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn force_activate_target(origin: OriginFor<T>, target: TargetId) -> DispatchResult {
            ensure_root(origin)?;

            // Ensure that gateway has the escrow address attached to it.
            ensure!(
                <T as Config>::Xdns::get_escrow_account(&target).is_ok(),
                Error::<T>::XdnsGatewayDoesNotHaveEscrowAddressRegistered
            );

            // Activate the new target
            PendingAttestationTargets::<T>::mutate(|pending| {
                if let Some(index) = pending.iter().position(|x| x == &target) {
                    pending.remove(index);
                }
            });
            AttestationTargets::<T>::mutate(|active| {
                if !active.contains(&target) {
                    active.push(target);
                }
            });

            Self::deposit_event(Event::NewTargetActivated(target));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn add_attestation_target(origin: OriginFor<T>, target: TargetId) -> DispatchResult {
            ensure_root(origin)?;

            // Ensure that gateway has the escrow address attached to it.
            ensure!(
                <T as Config>::Xdns::get_escrow_account(&target).is_ok(),
                Error::<T>::XdnsGatewayDoesNotHaveEscrowAddressRegistered
            );

            ensure!(
                !AttestationTargets::<T>::get().contains(&target),
                Error::<T>::TargetAlreadyActive
            );

            PendingAttestationTargets::<T>::mutate(|pending| {
                if !pending.contains(&target) {
                    pending.push(target);
                }
            });

            Self::deposit_event(Event::NewTargetProposed(target));

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

            // Ensure target is activated
            ensure!(
                AttestationTargets::<T>::get().contains(&target),
                Error::<T>::TargetNotActive
            );

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

            let attested_recoverable = AttestersAgreements::<T>::get(&account_id, target)
                .ok_or(Error::<T>::AttesterDidNotAgreeToNewTarget)?;

            let vendor = <T as Config>::Xdns::get_verification_vendor(&target)
                .map_err(|_| Error::<T>::XdnsTargetNotActive)?;

            // ToDo: Generalize attesters to work with multiple ExecutionVendor architecture.
            //  For now, assume Ethereum.
            //  let _target_verification_vendor = T::Xdns::get_verification_vendor(&target)?;
            let is_verified = attester
                .verify_attestation_signature(
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    &message.encode(),
                    &signature,
                    attested_recoverable,
                    &vendor,
                )
                .map_err(|_| Error::<T>::InvalidSignature)?;

            let signature_65b: [u8; 65] = signature
                .try_into()
                .map_err(|_| Error::<T>::InvalidSignature)?;

            if !is_verified {
                PermanentSlashes::<T>::append(account_id);
                return Err(Error::<T>::RejectingFromSlashedAttester.into())
            }

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
                let quorum = (T::CommitteeSize::get() * 2 / 3) as usize;
                let full_approval = T::CommitteeSize::get() as usize;
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
                    Self::deposit_event(Event::NewConfirmationBatch(
                        target,
                        batch.clone(),
                        batch.message(),
                        batch.message_hash(),
                    ));
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

    impl<T: Config> AttestersWriteApi<T::AccountId, DispatchError> for Pallet<T> {
        fn request_sfx_attestation_commit(
            target: TargetId,
            sfx_id: H256,
        ) -> Result<(), DispatchError> {
            NextBatch::<T>::try_mutate(target, |next_batch| {
                if let Some(ref mut next_batch) = next_batch {
                    if let Some(ref mut batch_sfx) = &mut next_batch.committed_sfx {
                        if batch_sfx.contains(&sfx_id) {
                            return Err("SfxAlreadyRequested".into())
                        } else {
                            batch_sfx.push(sfx_id);
                        }
                    } else {
                        next_batch.committed_sfx = Some(vec![sfx_id]);
                    }
                    Ok(())
                } else {
                    Err("BatchNotFound".into())
                }
            })
        }

        fn request_sfx_attestation_revert(
            target: TargetId,
            sfx_id: H256,
        ) -> Result<(), DispatchError> {
            NextBatch::<T>::try_mutate(target, |next_batch| {
                if let Some(ref mut next_batch) = next_batch {
                    if let Some(ref mut batch_sfx) = &mut next_batch.reverted_sfx {
                        if batch_sfx.contains(&sfx_id) {
                            return Err(Error::<T>::SfxAlreadyRequested.into())
                        } else {
                            batch_sfx.push(sfx_id);
                        }
                    } else {
                        next_batch.reverted_sfx = Some(vec![sfx_id]);
                    }
                    Ok(())
                } else {
                    Err(Error::<T>::BatchNotFound.into())
                }
            })
        }

        fn request_ban_attesters_attestation(
            ban_attester: &T::AccountId,
        ) -> Result<(), DispatchError> {
            for target in AttestationTargets::<T>::get() {
                let attester_recoverable = AttestersAgreements::<T>::get(ban_attester, target)
                    .ok_or(Error::<T>::AttesterDidNotAgreeToNewTarget)?;

                NextBatch::<T>::try_mutate(target, |next_batch| {
                    let next_batch = next_batch
                        .as_mut()
                        .ok_or::<DispatchError>(Error::<T>::BatchNotFound.into())?;

                    match &mut next_batch.banned_committee {
                        Some(attesters) => {
                            ensure!(
                                !attesters.contains(&attester_recoverable),
                                Error::<T>::BanAttesterAlreadyRequested
                            );
                            attesters.push(attester_recoverable);
                        },
                        None => {
                            next_batch.banned_committee = Some(vec![attester_recoverable]);
                        },
                    }
                    Ok::<(), DispatchError>(())
                })?;
            }

            Ok(())
        }

        fn request_next_committee_attestation() {
            for target in AttestationTargets::<T>::get() {
                if let Some(next_batch) = NextBatch::<T>::get(target) {
                    if next_batch.next_committee.is_none() {
                        let committee_transition_for_target =
                            Self::get_current_committee_transition_for_target(&target);
                        let committee_recoverable_on_target = committee_transition_for_target
                            .clone()
                            .into_iter()
                            .map(|(_index, recoverable)| recoverable)
                            .collect::<Vec<Vec<u8>>>();

                        let next_committee = match committee_recoverable_on_target.len() {
                            0 => None,
                            _ => Some(committee_recoverable_on_target),
                        };
                        // We only update the next_committee if it was None.
                        NextBatch::<T>::insert(
                            target,
                            BatchMessage {
                                next_committee,
                                ..next_batch
                            },
                        );
                        NextCommitteeOnTarget::<T>::insert(target, committee_transition_for_target);
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

        // Select the oldest batch with PendingAttestation and return the LatencyStatus
        fn read_attestation_latency(target: &TargetId) -> Option<LatencyStatus> {
            let mut batches = Self::get_batches(*target, BatchStatus::PendingAttestation);
            batches.sort_by(|a, b| a.created.cmp(&b.created));
            let oldest_batch = batches.first();
            match oldest_batch {
                Some(batch) => Some(batch.latency.clone()),
                None => None,
            }
        }

        fn active_set() -> Vec<T::AccountId> {
            ActiveSet::<T>::get()
        }

        fn honest_active_set() -> Vec<T::AccountId> {
            let active_set = ActiveSet::<T>::get();
            active_set
                .into_iter()
                .filter(|a| !Self::is_permanently_slashed(a))
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

        fn get_activated_targets() -> Vec<TargetId> {
            AttestationTargets::<T>::get()
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
                    PermanentSlashes::<T>::append(account_id);
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

        pub fn try_activate_new_target(target: &TargetId) -> bool {
            // Check if all members of the ActiveSet members have submitted their agreements
            let active_set = ActiveSet::<T>::get();
            let mut active_set_agreements = 0;
            for attester in active_set.iter() {
                if AttestersAgreements::<T>::contains_key(attester, target) {
                    active_set_agreements += 1;
                }
            }

            if active_set_agreements == active_set.len() {
                // Activate the new target
                PendingAttestationTargets::<T>::mutate(|pending| {
                    if let Some(index) = pending.iter().position(|x| x == target) {
                        pending.remove(index);
                    }
                });
                AttestationTargets::<T>::mutate(|active| {
                    if !active.contains(target) {
                        active.push(*target);
                    }
                });
                true
            } else {
                false
            }
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

        pub fn get_batches_to_commit(target: TargetId) -> Vec<BatchMessage<T::BlockNumber>> {
            // Get the batches to sign
            match Batches::<T>::get(target) {
                Some(batches) => batches
                    .iter()
                    .filter(|b| {
                        b.status == BatchStatus::ReadyForSubmissionByMajority
                            || b.status == BatchStatus::ReadyForSubmissionFullyApproved
                    })
                    .cloned()
                    .collect(),
                None => vec![],
            }
        }

        pub fn get_latest_batch_to_commit(
            target: TargetId,
        ) -> Option<BatchMessage<T::BlockNumber>> {
            // Get the batches to sign
            let mut batches = Self::get_batches_to_commit(target);
            batches.sort_by(|a, b| b.index.cmp(&a.index));
            batches.first().cloned()
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

        fn get_current_committee_transition_for_target(target: &TargetId) -> CommitteeTransition {
            let current_committee = CurrentCommittee::<T>::get();
            let mut committee_transition = Vec::new();

            for attester in &current_committee {
                if let Some(attester_info) = Attesters::<T>::get(attester) {
                    if let Some(checked_recoverable) =
                        AttestersAgreements::<T>::get(attester, target)
                    {
                        committee_transition.push((attester_info.index, checked_recoverable));
                    }
                }
            }

            committee_transition.sort_by(|(a, _), (b, _)| a.cmp(b));

            committee_transition
        }

        fn get_current_committee_indices() -> CommitteeTransitionIndices {
            let current_committee = CurrentCommittee::<T>::get();
            let mut committee_indices: CommitteeTransitionIndices = [0; COMMITTEE_SIZE]; // Initialize the committee_indices array

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

        pub fn process_repatriations(n: T::BlockNumber, aggregated_weight: Weight) -> Weight {
            for target in AttestationTargets::<T>::get() {
                Batches::<T>::mutate(target, |batches| {
                    let mut repatriated = false;
                    if let Some(batches) = batches {
                        batches
                            .iter_mut()
                            .filter(|batch| {
                                batch.status == BatchStatus::PendingAttestation
                                    && batch.created + T::RepatriationPeriod::get() <= n
                            })
                            .for_each(|batch| {
                                // Merge both Reverted and Committed SFX, including status flags
                                let mut sfx_to_repatriate: Vec<(CircuitStatus, H256)> = vec![];

                                if let Some(batch_sfx) = batch.committed_sfx.as_ref() {
                                    for sfx_id in batch_sfx.iter() {
                                        sfx_to_repatriate.push((CircuitStatus::Committed, *sfx_id));
                                    }
                                }
                                if let Some(batch_sfx) = batch.reverted_sfx.as_ref() {
                                    for sfx_id in batch_sfx.iter() {
                                        sfx_to_repatriate.push((
                                            CircuitStatus::Reverted(Cause::Timeout),
                                            *sfx_id,
                                        ));
                                    }
                                }

                                sfx_to_repatriate
                                    .iter()
                                    .filter_map(|(status, sfx_id)| {
                                        T::Hash::decode(&mut &sfx_id.as_bytes()[..])
                                            .map(|sfx_id_as_hash| (sfx_id, sfx_id_as_hash, status))
                                            .ok()
                                    })
                                    .for_each(|(sfx_id, sfx_id_as_hash, status)| {
                                        let requester: Option<T::AccountId> =
                                            match T::ReadSFX::get_fsx_requester(sfx_id_as_hash) {
                                                Ok(requester) => Some(requester),
                                                Err(_) => None,
                                            };

                                        if let Ok(fsx) = T::ReadSFX::get_fsx(sfx_id_as_hash) {
                                            if T::Rewards::repatriate_for_late_attestation(
                                                sfx_id, &fsx, status, requester,
                                            ) {
                                                repatriated = true;
                                            }
                                        } else {
                                            log::warn!(
                                                "SFX not found while processing repatriations"
                                            );
                                        }
                                    });

                                batch.latency = match batch.latency {
                                    LatencyStatus::OnTime =>
                                        if repatriated {
                                            LatencyStatus::Late(1, 1)
                                        } else {
                                            LatencyStatus::Late(1, 0)
                                        },
                                    LatencyStatus::Late(n, r) =>
                                        if repatriated {
                                            LatencyStatus::Late(
                                                n.saturating_add(1),
                                                r.saturating_add(1),
                                            )
                                        } else {
                                            LatencyStatus::Late(n.saturating_add(1), r)
                                        },
                                }
                            });
                    }
                });
            }
            aggregated_weight
        }

        pub fn process_next_batch_window(n: T::BlockNumber, aggregated_weight: Weight) -> Weight {
            let quorum = (T::CommitteeSize::get() * 2 / 3) as usize;

            for target in AttestationTargets::<T>::get() {
                let mut new_next_batch = BatchMessage::default();
                new_next_batch.created = n;
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
                                    batch.message(),
                                    batch.message_hash(),
                                ));
                            } else {
                                // Mark the batch as late if it has not been attested for. Skip if BatchingWindow overlaps with RepatriationPeriod
                                if !((n % T::RepatriationPeriod::get()).is_zero()
                                    && !batch.has_no_sfx())
                                {
                                    batch.latency = match batch.latency {
                                        LatencyStatus::OnTime => LatencyStatus::Late(1, 0),
                                        LatencyStatus::Late(n, r) =>
                                            LatencyStatus::Late(n.saturating_add(1), r),
                                    };
                                }
                            }
                        }
                    }
                });

                let batches_pending_attestation =
                    Self::get_batches(target, BatchStatus::PendingAttestation);
                if !batches_pending_attestation.is_empty() {
                    // Emit all pending attestation batches for the target with indexes and message hashes
                    Self::deposit_event(Event::CurrentPendingAttestationBatches(
                        target,
                        batches_pending_attestation
                            .iter()
                            .map(|batch| (batch.index, batch.message_hash()))
                            .collect::<Vec<(u32, H256)>>(),
                    ));
                }

                if let Some(mut next_batch) = NextBatch::<T>::get(target) {
                    // Check if batch has pending messages to attest for
                    // Leave the batch empty if it has no messages to attest for
                    if !next_batch.is_empty() {
                        let message_hash = next_batch.message_hash();
                        next_batch.status = BatchStatus::PendingAttestation;
                        // Push the batch to the batches vector
                        Batches::<T>::append(target, &next_batch);
                        // Create a new empty batch for the next window
                        new_next_batch.index = next_batch.index.saturating_add(1);
                        NextBatch::<T>::insert(target, new_next_batch);

                        Self::deposit_event(Event::NewAttestationBatch(target, next_batch));

                        let execution_vendor = match T::Xdns::get_verification_vendor(&target) {
                            Ok(gv) => match gv {
                                GatewayVendor::Ethereum => ExecutionVendor::EVM,
                                _ => ExecutionVendor::Substrate,
                            },
                            Err(_) => ExecutionVendor::EVM,
                        };

                        Self::deposit_event(Event::NewAttestationMessageHash(
                            target,
                            message_hash,
                            execution_vendor,
                        ));
                    }
                } else {
                    // Create a new !first! empty batch for the next window on the newly accepted target
                    NextBatch::<T>::insert(target, new_next_batch);
                    Self::request_next_committee_attestation();
                }
            }
            aggregated_weight
        }

        pub fn is_permanently_slashed(account: &T::AccountId) -> bool {
            PermanentSlashes::<T>::get().contains(account)
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
                        .filter(|(account_id, _)| !Self::is_permanently_slashed(&account_id))
                        .take(32)
                        .cloned()
                        .map(|(account_id, _balance)| account_id)
                        .collect::<Vec<T::AccountId>>(),
                );
                aggregated_weight += T::DbWeight::get().reads_writes(1, 1);

                // Call shuffle_committee
                Self::shuffle_committee();
                aggregated_weight += T::DbWeight::get().reads_writes(2, 2);
                Self::request_next_committee_attestation();
                aggregated_weight += T::DbWeight::get().reads_writes(2, 2);

                return aggregated_weight
            }

            if (n % T::BatchingWindow::get()).is_zero() {
                // Check if there any pending attestations to submit with the current batch
                aggregated_weight = Self::process_next_batch_window(n, aggregated_weight);
            }
            if (n % T::RepatriationPeriod::get()).is_zero() {
                aggregated_weight = Self::process_repatriations(n, aggregated_weight);
            }
            aggregated_weight
        }
    }

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub phantom: PhantomData<T>,
        pub attestation_targets: Vec<TargetId>,
    }

    // The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                phantom: Default::default(),
                attestation_targets: Default::default(),
            }
        }
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            // Extend the list of attestation targets
            for target in self.attestation_targets.iter() {
                AttestationTargets::<T>::append(target);
            }

            for target in AttestationTargets::<T>::get() {
                let mut new_next_batch = BatchMessage::default();
                new_next_batch.created = frame_system::Pallet::<T>::block_number();
                // Create new batch for next window
                NextBatch::<T>::insert(target, new_next_batch.clone());
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
    use std::ops::Index;

    use codec::Encode;
    use frame_support::{
        assert_err, assert_noop, assert_ok,
        traits::{Currency, Get, Hooks, Len},
        StorageValue,
    };
    use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, Pair, RuntimePublic};
    use sp_core::H256;
    use sp_runtime::traits::BlakeTwo256;

    use crate::TargetBatchDispatchEvent;
    use sp_std::convert::TryInto;
    use t3rn_mini_mock_runtime::{
        AccountId, ActiveSet, AttestationTargets, Attesters, AttestersError, AttestersStore,
        Balance, Balances, BatchMessage, BatchStatus, BlockNumber, ConfigAttesters, ConfigRewards,
        CurrentCommittee, ExtBuilder, FullSideEffects, LatencyStatus, MiniRuntime, NextBatch,
        NextCommitteeOnTarget, Nominations, Origin, PendingUnnominations, PermanentSlashes,
        PreviousCommittee, Rewards, SFX2XTXLinksMap, SortedNominatedAttesters, System,
        XExecSignals,
    };
    use t3rn_primitives::{
        attesters::{
            ecdsa_pubkey_to_eth_address, AttesterInfo, AttestersReadApi, AttestersWriteApi,
            CommitteeRecoverable, CommitteeTransitionIndices,
        },
        circuit::{CircuitStatus, FullSideEffect, SecurityLvl, SideEffect, XExecSignal},
        claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
        TreasuryAccount, TreasuryAccountProvider,
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

    pub fn select_new_committee() {
        // Run to the next shuffling window
        let shuffling_frequency = <MiniRuntime as ConfigAttesters>::ShufflingFrequency::get();
        let current_block = System::block_number();

        if current_block < shuffling_frequency {
            Attesters::on_initialize(shuffling_frequency);
            System::set_block_number(shuffling_frequency);
        } else {
            let shuffling_multiplier = current_block / shuffling_frequency;
            Attesters::on_initialize(shuffling_multiplier * shuffling_frequency);
            System::set_block_number(shuffling_multiplier * shuffling_frequency);
        }
        assert!(!ActiveSet::<MiniRuntime>::get().is_empty(),);
        assert!(!CurrentCommittee::<MiniRuntime>::get().is_empty(),);
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

    pub fn make_all_agree_to_new_target_or_force_if_no_active_set(target: &TargetId) {
        assert!(!Attesters::attestation_targets().contains(target));
        assert!(Attesters::pending_attestation_targets().contains(target));
        if AttestersStore::<MiniRuntime>::iter().count() == 0 {
            Attesters::force_activate_target(Origin::root(), *target);
        }
        for (attester, attester_info) in AttestersStore::<MiniRuntime>::iter() {
            // assume attester agrees to eth target: deriving eth address from ecdsa key
            let derived_eth_address = ecdsa_pubkey_to_eth_address(&attester_info.key_ec);
            assert_ok!(derived_eth_address);
            assert_ok!(Attesters::agree_to_new_attestation_target(
                Origin::signed(attester),
                *target,
                derived_eth_address.unwrap().encode(),
            ));
        }
        assert!(!Attesters::pending_attestation_targets().contains(target));
        assert!(Attesters::attestation_targets().contains(target));
    }

    pub fn add_target_and_transition_to_next_batch(target: TargetId, index: u32) -> BlockNumber {
        Attesters::add_attestation_target(Origin::root(), target);
        if !Attesters::attestation_targets().contains(&target) {
            // if active set is empty, select the next active set
            if !ActiveSet::<MiniRuntime>::get().is_empty() {
                select_new_committee();
            }
            assert_eq!(Attesters::pending_attestation_targets(), vec![target]);
            make_all_agree_to_new_target_or_force_if_no_active_set(&target);
        }

        transition_to_next_batch(target, index)
    }

    fn transition_to_next_batch(target: TargetId, index: u32) -> BlockNumber {
        let current_block: BlockNumber = System::block_number();
        let batching_window: BlockNumber = <MiniRuntime as ConfigAttesters>::BatchingWindow::get();

        // calculate the closest multiple of batching_window
        let closest_block = ((current_block / batching_window) + 1) * batching_window;

        System::set_block_number(closest_block);

        // Transition to the next batch
        System::set_block_number(closest_block);
        Attesters::on_initialize(closest_block);

        let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
        assert_eq!(next_batch.status, BatchStatus::PendingMessage);
        // Pending message doesn't update if it's empty, therefore difficult to predict the created block number without more context
        assert!(next_batch.created <= closest_block);
        assert_eq!(next_batch.index, index);
        assert_eq!(next_batch.committed_sfx, None);
        assert_eq!(next_batch.reverted_sfx, None);
        assert_eq!(next_batch.banned_committee, None);
        assert_eq!(next_batch.signatures, Vec::new());

        System::set_block_number(closest_block);
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
        if Attesters::get_latest_batch_to_sign_message(target).is_none() {
            let _current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let sfx_id_a = H256::from(message);
            Attesters::request_sfx_attestation_commit(target, sfx_id_a);

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);
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
    fn submitting_attestation_reads_as_on_time_latency_status() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
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

            let batch_latency = Attesters::read_attestation_latency(&[0u8; 4]);
            assert!(batch_latency.is_some());
            assert_eq!(batch_latency, Some(LatencyStatus::OnTime));
        });
    }

    #[test]
    fn register_and_submit_attestation_in_ecdsa() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
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
            assert_eq!(
                latest_batch_some.signatures,
                vec![(attester_info.index, signature.try_into().unwrap())]
            );
        });
    }

    #[test]
    fn double_attestation_is_not_allowed() {
        let target = [0u8; 4];
        let _mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();

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
                target,
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
                    target,
                ),
                AttestersError::<MiniRuntime>::AttestationDoubleSignAttempt
            );
        });
    }

    #[test]
    fn test_adding_sfx_moves_next_batch_to_pending_attestation() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation_commit(target, sfx_id_a));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            assert_eq!(
                Attesters::get_batches(target, BatchStatus::PendingAttestation),
                vec![BatchMessage {
                    committed_sfx: Some(vec![sfx_id_a]),
                    reverted_sfx: None,
                    next_committee: None,
                    banned_committee: None,
                    signatures: vec![],
                    status: BatchStatus::PendingAttestation,
                    created: current_block_1,
                    latency: LatencyStatus::OnTime,
                    index: 0,
                }]
            );
        });
    }

    #[test]
    fn test_successfull_process_repatriation_for_pending_attestation_with_one_fsx_reverted() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let repatriated_executor = AccountId::from([1u8; 32]);
            let mock_xtx_id = H256([2u8; 32]);
            let mock_fsx = FullSideEffect {
                input: SideEffect {
                    enforce_executor: Some(repatriated_executor.clone()),
                    target,
                    max_reward: 10,
                    insurance: 1,
                    action: [0u8; 4],
                    encoded_args: vec![],
                    signature: vec![],
                    reward_asset_id: None,
                },
                confirmed: None,
                security_lvl: SecurityLvl::Escrow,
                submission_target_height: 1,
                best_bid: None,
                index: 0,
            };

            let sfx_id = mock_fsx
                .input
                .generate_id::<BlakeTwo256>(mock_xtx_id.as_bytes(), 0u32);

            assert_ok!(Attesters::request_sfx_attestation_revert(target, sfx_id));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let pending_batches = Attesters::get_batches(target, BatchStatus::PendingAttestation);
            assert_eq!(pending_batches.len(), 1);
            let pending_batch = pending_batches[0].clone();
            assert_eq!(pending_batch.reverted_sfx, Some(vec![sfx_id]));
            assert_eq!(pending_batch.committed_sfx, None);
            assert_eq!(pending_batch.created, current_block_1);

            const SLASH_TREASURY_BALANCE: Balance = 100;
            Balances::deposit_creating(
                &MiniRuntime::get_treasury_account(TreasuryAccount::Slash),
                SLASH_TREASURY_BALANCE,
            );

            FullSideEffects::<MiniRuntime>::insert(mock_xtx_id, vec![vec![mock_fsx.clone()]]);
            SFX2XTXLinksMap::<MiniRuntime>::insert(sfx_id, mock_xtx_id);

            let repatriation_period: BlockNumber =
                <MiniRuntime as ConfigAttesters>::RepatriationPeriod::get();
            Attesters::on_initialize(2 * repatriation_period);

            // The batch should change the status to Repatriated
            let the_same_batch =
                Attesters::get_batch_by_message(target, pending_batch.message()).unwrap();

            assert_eq!(the_same_batch.status, BatchStatus::PendingAttestation);
            assert_eq!(the_same_batch.latency, LatencyStatus::Late(1, 1));

            assert_eq!(
                Rewards::get_pending_claims(repatriated_executor.clone()),
                Some(vec![ClaimableArtifacts {
                    beneficiary: repatriated_executor,
                    role: CircuitRole::Executor,
                    total_round_claim: mock_fsx.input.max_reward / 2,
                    benefit_source: BenefitSource::SlashTreasury,
                }])
            );
        });
    }

    #[test]
    fn test_successfull_process_repatriation_for_pending_attestation_with_one_fsx() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let repatriated_requester = AccountId::from([4u8; 32]);
            let repatriated_executor = AccountId::from([1u8; 32]);
            let mock_xtx_id = H256([2u8; 32]);
            let mock_fsx = FullSideEffect {
                input: SideEffect {
                    enforce_executor: Some(repatriated_executor),
                    target,
                    max_reward: 10,
                    insurance: 1,
                    action: [0u8; 4],
                    encoded_args: vec![],
                    signature: vec![],
                    reward_asset_id: None,
                },
                confirmed: None,
                security_lvl: SecurityLvl::Escrow,
                submission_target_height: 1,
                best_bid: None,
                index: 0,
            };

            let mock_xtx = XExecSignal {
                requester: repatriated_requester.clone(),
                requester_nonce: 1,
                timeouts_at: 1,
                speed_mode: Default::default(),
                delay_steps_at: None,
                status: CircuitStatus::Committed,
                steps_cnt: (0, 0),
            };

            let sfx_id = mock_fsx
                .input
                .generate_id::<BlakeTwo256>(mock_xtx_id.as_bytes(), 0u32);

            assert_ok!(Attesters::request_sfx_attestation_commit(target, sfx_id));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let pending_batches = Attesters::get_batches(target, BatchStatus::PendingAttestation);
            assert_eq!(pending_batches.len(), 1);
            let pending_batch = pending_batches[0].clone();
            assert_eq!(pending_batch.committed_sfx, Some(vec![sfx_id]));
            assert_eq!(pending_batch.reverted_sfx, None);
            assert_eq!(pending_batch.created, current_block_1);

            const SLASH_TREASURY_BALANCE: Balance = 100;
            Balances::deposit_creating(
                &MiniRuntime::get_treasury_account(TreasuryAccount::Slash),
                SLASH_TREASURY_BALANCE,
            );

            XExecSignals::<MiniRuntime>::insert(mock_xtx_id, mock_xtx);
            FullSideEffects::<MiniRuntime>::insert(mock_xtx_id, vec![vec![mock_fsx.clone()]]);
            SFX2XTXLinksMap::<MiniRuntime>::insert(sfx_id, mock_xtx_id);

            let repatriation_period: BlockNumber =
                <MiniRuntime as ConfigAttesters>::RepatriationPeriod::get();
            Attesters::on_initialize(2 * repatriation_period);

            // The batch should change the status to Repatriated
            let the_same_batch =
                Attesters::get_batch_by_message(target, pending_batch.message()).unwrap();

            assert_eq!(the_same_batch.status, BatchStatus::PendingAttestation);
            assert_eq!(the_same_batch.latency, LatencyStatus::Late(1, 1));
            assert_eq!(
                Rewards::get_pending_claims(repatriated_requester.clone()),
                Some(vec![ClaimableArtifacts {
                    beneficiary: repatriated_requester,
                    role: CircuitRole::Requester,
                    total_round_claim: mock_fsx.input.max_reward / 2,
                    benefit_source: BenefitSource::SlashTreasury,
                }])
            );
        });
    }

    #[test]
    fn test_process_repatriation_changes_status_to_expired_after_repatriation_period_when_fsx_not_found(
    ) {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let sfx_id = H256([3u8; 32]);

            assert_ok!(Attesters::request_sfx_attestation_commit(target, sfx_id));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let pending_batches = Attesters::get_batches(target, BatchStatus::PendingAttestation);
            assert_eq!(pending_batches.len(), 1);
            let pending_batch = pending_batches[0].clone();
            assert_eq!(pending_batch.committed_sfx, Some(vec![sfx_id]));
            assert_eq!(pending_batch.created, current_block_1);

            const SLASH_TREASURY_BALANCE: Balance = 100;
            Balances::deposit_creating(
                &MiniRuntime::get_treasury_account(TreasuryAccount::Slash),
                SLASH_TREASURY_BALANCE,
            );

            let repatriation_period: BlockNumber =
                <MiniRuntime as ConfigAttesters>::RepatriationPeriod::get();
            Attesters::on_initialize(2 * repatriation_period);

            // The batch should change the status to Expired
            let the_same_batch =
                Attesters::get_batch_by_message(target, pending_batch.message()).unwrap();
            assert_eq!(the_same_batch.status, BatchStatus::PendingAttestation);
            assert_eq!(the_same_batch.latency, LatencyStatus::Late(1, 0));
        });
    }

    #[test]
    fn test_process_repatriation_changes_status_to_expired_after_repatriation_period_when_no_batch_fsx_required(
    ) {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];

            for counter in 1..33u8 {
                // Register an attester
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            Attesters::request_next_committee_attestation();

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let pending_batches = Attesters::get_batches(target, BatchStatus::PendingAttestation);
            assert_eq!(pending_batches.len(), 1);
            let pending_batch = pending_batches[0].clone();
            assert_eq!(pending_batch.committed_sfx, None);
            assert_eq!(pending_batch.created, current_block_1);

            const SLASH_TREASURY_BALANCE: Balance = 100;
            Balances::deposit_creating(
                &MiniRuntime::get_treasury_account(TreasuryAccount::Slash),
                SLASH_TREASURY_BALANCE,
            );

            let repatriation_period: BlockNumber =
                <MiniRuntime as ConfigAttesters>::RepatriationPeriod::get();
            Attesters::on_initialize(2 * repatriation_period);

            // The batch should change the status to Expired
            let the_same_batch =
                Attesters::get_batch_by_message(target, pending_batch.message()).unwrap();

            assert_eq!(the_same_batch.status, BatchStatus::PendingAttestation);
            assert_eq!(the_same_batch.latency, LatencyStatus::Late(1, 0));
        });
    }

    #[test]
    fn test_pending_attestation_batch_with_single_sfx_yields_correct_message_hash() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            let _current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation_commit(target, sfx_id_a));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let (_message_hash, expected_message_bytes) =
                calculate_hash_for_sfx_message(sfx_id_a.into(), 0);

            assert_eq!(
                Attesters::get_latest_batch_to_sign_message(target),
                Some(expected_message_bytes.clone())
            );

            let mut keccak = Keccak::v256();
            keccak.update(&expected_message_bytes);
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
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            for counter in 1..33u8 {
                // Register an attester
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let _committee_transition: CommitteeTransitionIndices = [
                1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32,
                14u32, 15u32, 16u32, 17u32, 18u32, 19u32, 20u32, 21u32, 22u32, 23u32, 24u32, 25u32,
                26u32, 27u32, 28u32, 29u32, 30u32, 31u32, 32u32,
            ];

            Attesters::request_next_committee_attestation();

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation_commit(target, sfx_id_a));

            let ban_attester = AccountId::from([3; 32]);
            assert_ok!(Attesters::request_ban_attesters_attestation(&ban_attester));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let expected_transition: Option<CommitteeRecoverable> = Some(vec![
                vec![
                    26, 100, 47, 14, 60, 58, 245, 69, 231, 172, 189, 56, 176, 114, 81, 179, 153, 9,
                    20, 241,
                ],
                vec![
                    80, 80, 164, 244, 179, 249, 51, 140, 52, 114, 220, 192, 26, 135, 199, 106, 20,
                    75, 60, 156,
                ],
                vec![
                    51, 37, 167, 132, 37, 241, 122, 126, 72, 126, 181, 102, 107, 43, 253, 147, 171,
                    176, 108, 112,
                ],
                vec![
                    196, 139, 129, 43, 180, 52, 1, 57, 44, 3, 115, 129, 172, 169, 52, 244, 6, 156,
                    5, 23,
                ],
                vec![
                    208, 154, 209, 64, 128, 212, 178, 87, 168, 25, 164, 245, 121, 184, 72, 91, 232,
                    143, 8, 108,
                ],
                vec![
                    12, 176, 48, 209, 26, 139, 228, 139, 96, 65, 136, 87, 135, 77, 238, 230, 29,
                    16, 113, 224,
                ],
                vec![
                    74, 98, 49, 102, 35, 173, 69, 127, 2, 205, 197, 217, 151, 222, 214, 122, 56,
                    62, 197, 105,
                ],
                vec![
                    153, 200, 81, 234, 163, 195, 151, 105, 20, 214, 59, 130, 44, 103, 226, 1, 236,
                    11, 251, 184,
                ],
                vec![
                    88, 218, 153, 10, 143, 74, 58, 108, 167, 203, 99, 21, 214, 138, 20, 1, 5, 145,
                    115, 82,
                ],
                vec![
                    193, 113, 3, 61, 92, 191, 247, 23, 95, 41, 223, 211, 166, 61, 218, 61, 111,
                    143, 56, 94,
                ],
                vec![
                    242, 136, 236, 175, 21, 121, 14, 252, 172, 82, 137, 70, 150, 58, 109, 184, 195,
                    248, 33, 29,
                ],
                vec![
                    99, 70, 123, 2, 167, 56, 36, 8, 168, 69, 165, 235, 133, 181, 35, 139, 138, 77,
                    208, 237,
                ],
                vec![
                    34, 156, 120, 75, 147, 204, 180, 64, 249, 29, 197, 19, 44, 116, 169, 83, 25,
                    73, 125, 244,
                ],
                vec![
                    129, 161, 247, 202, 26, 64, 224, 4, 216, 227, 205, 205, 183, 38, 58, 173, 217,
                    206, 26, 243,
                ],
                vec![
                    105, 26, 141, 5, 103, 143, 201, 98, 255, 15, 33, 116, 19, 67, 121, 192, 5, 28,
                    182, 134,
                ],
                vec![
                    239, 4, 90, 85, 76, 187, 0, 22, 39, 94, 144, 227, 0, 47, 77, 33, 198, 242, 99,
                    225,
                ],
                vec![
                    25, 231, 227, 118, 231, 194, 19, 183, 231, 231, 228, 108, 199, 10, 93, 208,
                    134, 218, 255, 42,
                ],
                vec![
                    28, 90, 119, 217, 250, 126, 244, 102, 149, 27, 47, 1, 247, 36, 188, 163, 165,
                    130, 11, 99,
                ],
                vec![
                    3, 161, 187, 166, 11, 90, 163, 112, 148, 207, 22, 18, 58, 221, 103, 76, 1, 88,
                    148, 136,
                ],
                vec![
                    30, 50, 171, 207, 230, 219, 21, 193, 87, 7, 9, 227, 252, 2, 114, 83, 53, 245,
                    10, 71,
                ],
                vec![
                    51, 224, 245, 57, 227, 27, 53, 23, 15, 170, 160, 98, 175, 112, 59, 118, 168,
                    40, 43, 247,
                ],
                vec![
                    161, 67, 201, 108, 74, 81, 135, 115, 67, 196, 221, 119, 253, 98, 88, 79, 215,
                    242, 93, 180,
                ],
                vec![
                    126, 159, 180, 15, 102, 196, 225, 50, 250, 94, 100, 228, 159, 48, 126, 2, 183,
                    101, 64, 248,
                ],
                vec![
                    215, 231, 83, 158, 167, 75, 228, 250, 203, 88, 197, 12, 206, 191, 6, 96, 127,
                    241, 148, 205,
                ],
                vec![
                    4, 146, 155, 184, 96, 118, 224, 159, 36, 139, 37, 73, 49, 73, 30, 54, 29, 59,
                    78, 84,
                ],
                vec![
                    239, 75, 112, 1, 58, 93, 39, 218, 97, 215, 26, 215, 22, 254, 18, 148, 247, 72,
                    209, 82,
                ],
                vec![
                    177, 167, 217, 66, 140, 229, 200, 14, 37, 78, 101, 251, 225, 188, 248, 47, 100,
                    123, 93, 238,
                ],
                vec![
                    25, 80, 41, 10, 165, 39, 129, 221, 108, 66, 85, 70, 123, 187, 235, 159, 119,
                    60, 25, 54,
                ],
                vec![
                    116, 10, 58, 110, 64, 197, 45, 43, 126, 50, 236, 207, 254, 100, 60, 77, 157,
                    170, 187, 91,
                ],
                vec![
                    235, 230, 196, 217, 170, 160, 118, 159, 4, 105, 143, 152, 109, 26, 198, 229,
                    234, 199, 108, 28,
                ],
                vec![
                    168, 139, 113, 15, 175, 255, 104, 227, 215, 187, 75, 61, 215, 44, 53, 139, 91,
                    219, 154, 24,
                ],
                vec![
                    182, 230, 16, 146, 27, 10, 15, 111, 96, 140, 14, 31, 41, 168, 69, 85, 43, 198,
                    219, 44,
                ],
            ]);

            assert_eq!(
                Attesters::get_latest_batch_to_sign(target),
                Some(BatchMessage {
                    committed_sfx: Some(vec![sfx_id_a]),
                    reverted_sfx: None,
                    next_committee: expected_transition,
                    banned_committee: Some(vec![vec![
                        51, 37, 167, 132, 37, 241, 122, 126, 72, 126, 181, 102, 107, 43, 253, 147,
                        171, 176, 108, 112,
                    ]]),
                    signatures: vec![],
                    status: BatchStatus::PendingAttestation,
                    created: current_block_1,
                    latency: LatencyStatus::OnTime,
                    index: 0,
                })
            );

            assert_eq!(
                Attesters::get_latest_batch_to_sign_message(target),
                Some(vec![
                    100, 47, 14, 60, 58, 245, 69, 231, 172, 189, 56, 176, 114, 81, 179, 153, 9, 20,
                    241, 80, 80, 164, 244, 179, 249, 51, 140, 52, 114, 220, 192, 26, 135, 199, 106,
                    20, 75, 60, 156, 51, 37, 167, 132, 37, 241, 122, 126, 72, 126, 181, 102, 107,
                    43, 253, 147, 171, 176, 108, 112, 196, 139, 129, 43, 180, 52, 1, 57, 44, 3,
                    115, 129, 172, 169, 52, 244, 6, 156, 5, 23, 208, 154, 209, 64, 128, 212, 178,
                    87, 168, 25, 164, 245, 121, 184, 72, 91, 232, 143, 8, 108, 12, 176, 48, 209,
                    26, 139, 228, 139, 96, 65, 136, 87, 135, 77, 238, 230, 29, 16, 113, 224, 74,
                    98, 49, 102, 35, 173, 69, 127, 2, 205, 197, 217, 151, 222, 214, 122, 56, 62,
                    197, 105, 153, 200, 81, 234, 163, 195, 151, 105, 20, 214, 59, 130, 44, 103,
                    226, 1, 236, 11, 251, 184, 88, 218, 153, 10, 143, 74, 58, 108, 167, 203, 99,
                    21, 214, 138, 20, 1, 5, 145, 115, 82, 193, 113, 3, 61, 92, 191, 247, 23, 95,
                    41, 223, 211, 166, 61, 218, 61, 111, 143, 56, 94, 242, 136, 236, 175, 21, 121,
                    14, 252, 172, 82, 137, 70, 150, 58, 109, 184, 195, 248, 33, 29, 99, 70, 123, 2,
                    167, 56, 36, 8, 168, 69, 165, 235, 133, 181, 35, 139, 138, 77, 208, 237, 34,
                    156, 120, 75, 147, 204, 180, 64, 249, 29, 197, 19, 44, 116, 169, 83, 25, 73,
                    125, 244, 129, 161, 247, 202, 26, 64, 224, 4, 216, 227, 205, 205, 183, 38, 58,
                    173, 217, 206, 26, 243, 105, 26, 141, 5, 103, 143, 201, 98, 255, 15, 33, 116,
                    19, 67, 121, 192, 5, 28, 182, 134, 239, 4, 90, 85, 76, 187, 0, 22, 39, 94, 144,
                    227, 0, 47, 77, 33, 198, 242, 99, 225, 25, 231, 227, 118, 231, 194, 19, 183,
                    231, 231, 228, 108, 199, 10, 93, 208, 134, 218, 255, 42, 28, 90, 119, 217, 250,
                    126, 244, 102, 149, 27, 47, 1, 247, 36, 188, 163, 165, 130, 11, 99, 3, 161,
                    187, 166, 11, 90, 163, 112, 148, 207, 22, 18, 58, 221, 103, 76, 1, 88, 148,
                    136, 30, 50, 171, 207, 230, 219, 21, 193, 87, 7, 9, 227, 252, 2, 114, 83, 53,
                    245, 10, 71, 51, 224, 245, 57, 227, 27, 53, 23, 15, 170, 160, 98, 175, 112, 59,
                    118, 168, 40, 43, 247, 161, 67, 201, 108, 74, 81, 135, 115, 67, 196, 221, 119,
                    253, 98, 88, 79, 215, 242, 93, 180, 126, 159, 180, 15, 102, 196, 225, 50, 250,
                    94, 100, 228, 159, 48, 126, 2, 183, 101, 64, 248, 215, 231, 83, 158, 167, 75,
                    228, 250, 203, 88, 197, 12, 206, 191, 6, 96, 127, 241, 148, 205, 4, 146, 155,
                    184, 96, 118, 224, 159, 36, 139, 37, 73, 49, 73, 30, 54, 29, 59, 78, 84, 239,
                    75, 112, 1, 58, 93, 39, 218, 97, 215, 26, 215, 22, 254, 18, 148, 247, 72, 209,
                    82, 177, 167, 217, 66, 140, 229, 200, 14, 37, 78, 101, 251, 225, 188, 248, 47,
                    100, 123, 93, 238, 25, 80, 41, 10, 165, 39, 129, 221, 108, 66, 85, 70, 123,
                    187, 235, 159, 119, 60, 25, 54, 116, 10, 58, 110, 64, 197, 45, 43, 126, 50,
                    236, 207, 254, 100, 60, 77, 157, 170, 187, 91, 235, 230, 196, 217, 170, 160,
                    118, 159, 4, 105, 143, 152, 109, 26, 198, 229, 234, 199, 108, 28, 168, 139,
                    113, 15, 175, 255, 104, 227, 215, 187, 75, 61, 215, 44, 53, 139, 91, 219, 154,
                    24, 182, 230, 16, 146, 27, 10, 15, 111, 96, 140, 14, 31, 41, 168, 69, 85, 43,
                    198, 219, 44, 51, 37, 167, 132, 37, 241, 122, 126, 72, 126, 181, 102, 107, 43,
                    253, 147, 171, 176, 108, 112, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0
                ])
            );

            assert_eq!(
                Attesters::get_latest_batch_to_sign_hash(target),
                Some(
                    hex_literal::hex!(
                        "58cd0ea9f78f115b381b29bc7edaab46f214968c05ff24b6b14474e4e47cfcdd"
                    )
                    .into()
                )
            );
        });
    }

    #[test]
    fn test_pending_attestation_batch_with_all_attestations_ordered_yields_correct_message_hash() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];

            for counter in 1..33u8 {
                // Register an attester
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            let _current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let _committee_transition: CommitteeTransitionIndices = [
                1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32,
                14u32, 15u32, 16u32, 17u32, 18u32, 19u32, 20u32, 21u32, 22u32, 23u32, 24u32, 25u32,
                26u32, 27u32, 28u32, 29u32, 30u32, 31u32, 32u32,
            ];

            Attesters::request_next_committee_attestation();
            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let expected_message_for_next_committe_transition_to_eth: Vec<u8> = vec![
                26, 100, 47, 14, 60, 58, 245, 69, 231, 172, 189, 56, 176, 114, 81, 179, 153, 9, 20,
                241, 80, 80, 164, 244, 179, 249, 51, 140, 52, 114, 220, 192, 26, 135, 199, 106, 20,
                75, 60, 156, 51, 37, 167, 132, 37, 241, 122, 126, 72, 126, 181, 102, 107, 43, 253,
                147, 171, 176, 108, 112, 196, 139, 129, 43, 180, 52, 1, 57, 44, 3, 115, 129, 172,
                169, 52, 244, 6, 156, 5, 23, 208, 154, 209, 64, 128, 212, 178, 87, 168, 25, 164,
                245, 121, 184, 72, 91, 232, 143, 8, 108, 12, 176, 48, 209, 26, 139, 228, 139, 96,
                65, 136, 87, 135, 77, 238, 230, 29, 16, 113, 224, 74, 98, 49, 102, 35, 173, 69,
                127, 2, 205, 197, 217, 151, 222, 214, 122, 56, 62, 197, 105, 153, 200, 81, 234,
                163, 195, 151, 105, 20, 214, 59, 130, 44, 103, 226, 1, 236, 11, 251, 184, 88, 218,
                153, 10, 143, 74, 58, 108, 167, 203, 99, 21, 214, 138, 20, 1, 5, 145, 115, 82, 193,
                113, 3, 61, 92, 191, 247, 23, 95, 41, 223, 211, 166, 61, 218, 61, 111, 143, 56, 94,
                242, 136, 236, 175, 21, 121, 14, 252, 172, 82, 137, 70, 150, 58, 109, 184, 195,
                248, 33, 29, 99, 70, 123, 2, 167, 56, 36, 8, 168, 69, 165, 235, 133, 181, 35, 139,
                138, 77, 208, 237, 34, 156, 120, 75, 147, 204, 180, 64, 249, 29, 197, 19, 44, 116,
                169, 83, 25, 73, 125, 244, 129, 161, 247, 202, 26, 64, 224, 4, 216, 227, 205, 205,
                183, 38, 58, 173, 217, 206, 26, 243, 105, 26, 141, 5, 103, 143, 201, 98, 255, 15,
                33, 116, 19, 67, 121, 192, 5, 28, 182, 134, 239, 4, 90, 85, 76, 187, 0, 22, 39, 94,
                144, 227, 0, 47, 77, 33, 198, 242, 99, 225, 25, 231, 227, 118, 231, 194, 19, 183,
                231, 231, 228, 108, 199, 10, 93, 208, 134, 218, 255, 42, 28, 90, 119, 217, 250,
                126, 244, 102, 149, 27, 47, 1, 247, 36, 188, 163, 165, 130, 11, 99, 3, 161, 187,
                166, 11, 90, 163, 112, 148, 207, 22, 18, 58, 221, 103, 76, 1, 88, 148, 136, 30, 50,
                171, 207, 230, 219, 21, 193, 87, 7, 9, 227, 252, 2, 114, 83, 53, 245, 10, 71, 51,
                224, 245, 57, 227, 27, 53, 23, 15, 170, 160, 98, 175, 112, 59, 118, 168, 40, 43,
                247, 161, 67, 201, 108, 74, 81, 135, 115, 67, 196, 221, 119, 253, 98, 88, 79, 215,
                242, 93, 180, 126, 159, 180, 15, 102, 196, 225, 50, 250, 94, 100, 228, 159, 48,
                126, 2, 183, 101, 64, 248, 215, 231, 83, 158, 167, 75, 228, 250, 203, 88, 197, 12,
                206, 191, 6, 96, 127, 241, 148, 205, 4, 146, 155, 184, 96, 118, 224, 159, 36, 139,
                37, 73, 49, 73, 30, 54, 29, 59, 78, 84, 239, 75, 112, 1, 58, 93, 39, 218, 97, 215,
                26, 215, 22, 254, 18, 148, 247, 72, 209, 82, 177, 167, 217, 66, 140, 229, 200, 14,
                37, 78, 101, 251, 225, 188, 248, 47, 100, 123, 93, 238, 25, 80, 41, 10, 165, 39,
                129, 221, 108, 66, 85, 70, 123, 187, 235, 159, 119, 60, 25, 54, 116, 10, 58, 110,
                64, 197, 45, 43, 126, 50, 236, 207, 254, 100, 60, 77, 157, 170, 187, 91, 235, 230,
                196, 217, 170, 160, 118, 159, 4, 105, 143, 152, 109, 26, 198, 229, 234, 199, 108,
                28, 168, 139, 113, 15, 175, 255, 104, 227, 215, 187, 75, 61, 215, 44, 53, 139, 91,
                219, 154, 24, 182, 230, 16, 146, 27, 10, 15, 111, 96, 140, 14, 31, 41, 168, 69, 85,
                43, 198, 219, 44, 0, 0, 0, 0,
            ];

            assert_eq!(
                Attesters::get_latest_batch_to_sign_message(target),
                Some(expected_message_for_next_committe_transition_to_eth.clone())
            );

            let mut keccak = Keccak::v256();
            keccak.update(&expected_message_for_next_committe_transition_to_eth);
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
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];
            NextBatch::<MiniRuntime>::insert(target, BatchMessage::default());

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation_commit(target, sfx_id_a));

            assert_noop!(
                Attesters::request_sfx_attestation_commit(target, sfx_id_a),
                "SfxAlreadyRequested",
            );
        });
    }

    #[test]
    fn test_adding_2_sfx_to_next_batch_and_transition_to_pending_attestation() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = [0, 0, 0, 0];

            AttestationTargets::<MiniRuntime>::put(vec![target]);
            assert_eq!(NextBatch::<MiniRuntime>::get(target), None);
            NextBatch::<MiniRuntime>::insert(target, BatchMessage::default());

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation_commit(target, sfx_id_a));

            // Verify that the attestation is added to the next batch
            let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
            assert_eq!(next_batch.committed_sfx, Some(vec![sfx_id_a]));

            // Add another SFX to the next batch
            let sfx_id_b = H256::repeat_byte(2);
            assert_ok!(Attesters::request_sfx_attestation_commit(target, sfx_id_b));
            let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
            assert_eq!(next_batch.committed_sfx, Some(vec![sfx_id_a, sfx_id_b]));

            let mut empty_batch = BatchMessage {
                committed_sfx: None,
                reverted_sfx: None,
                next_committee: None,
                banned_committee: None,
                signatures: vec![],
                status: BatchStatus::PendingMessage,
                created: 0,
                latency: LatencyStatus::OnTime,
                index: 0,
            };
            let batching_window: BlockNumber =
                <MiniRuntime as ConfigAttesters>::BatchingWindow::get();

            // Transition to the next batch
            System::set_block_number(batching_window * 2);
            Attesters::on_initialize(batching_window * 2);
            let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
            assert_eq!(next_batch.committed_sfx, None);

            // Verify that batches by status are correct
            assert_eq!(
                Attesters::get_batches(target, BatchStatus::PendingMessage),
                vec![]
            );
            assert_eq!(
                Attesters::get_batches(target, BatchStatus::PendingAttestation),
                vec![BatchMessage {
                    committed_sfx: Some(vec![sfx_id_a, sfx_id_b]),
                    reverted_sfx: None,
                    next_committee: None,
                    banned_committee: None,
                    signatures: vec![],
                    status: BatchStatus::PendingAttestation,
                    created: 0,
                    latency: LatencyStatus::OnTime,
                    index: 0,
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
            empty_batch.index += 1;
            assert_eq!(NextBatch::<MiniRuntime>::get(target), Some(empty_batch));
        });
    }

    #[test]
    fn committee_transition_generates_next_3_batches_pending_attestations_when_late() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            // On initialization, the current committee should be empty and the previous committee should be None
            assert!(CurrentCommittee::<MiniRuntime>::get().is_empty());
            assert_eq!(PreviousCommittee::<MiniRuntime>::get(), vec![]);

            const TARGET_0: TargetId = [0u8; 4];

            // Register multiple attesters
            let attester_count = 100;
            for counter in 1..=attester_count {
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }
            // Trigger the committee first setup
            select_new_committee();
            add_target_and_transition_to_next_batch(TARGET_0, 0);

            // Check if the committee is set up and has the correct size
            let committee = CurrentCommittee::<MiniRuntime>::get();
            let committee_size: u32 = <MiniRuntime as ConfigAttesters>::CommitteeSize::get();
            assert_eq!(committee.len(), committee_size as usize);

            // Check that each member of the committee is in the registered attesters
            for member in &committee {
                assert!(AttestersStore::<MiniRuntime>::contains_key(member));
            }

            // Expect NextBatch message to have committee transition awaiting attestation on registered target
            let next_batch = NextBatch::<MiniRuntime>::get(TARGET_0).unwrap();
            assert_eq!(next_batch.status, BatchStatus::PendingMessage);
            assert_eq!(next_batch.latency, LatencyStatus::OnTime);
            assert_eq!(next_batch.index, 0);
            assert_eq!(next_batch.committed_sfx, None);
            assert_eq!(next_batch.reverted_sfx, None);
            assert!(next_batch.next_committee.is_some());
            assert!(!next_batch.next_committee.clone().unwrap().is_empty());
            assert_eq!(next_batch.banned_committee, None);
            assert_eq!(next_batch.signatures, Vec::new());

            let batch_0_hash = next_batch.message_hash();
            // If no attestations are received, the next batch should be empty, and the current batch should be pending attestation with indication of late submission
            add_target_and_transition_to_next_batch(TARGET_0, 1);

            let batch_0 = Attesters::get_batch_by_message_hash(TARGET_0, batch_0_hash).unwrap();
            assert_eq!(batch_0.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_0.latency, LatencyStatus::OnTime);

            // Next batch should mark the initial batch as late, but don't modify the batch_1 since there's no new messages to attest for
            add_target_and_transition_to_next_batch(TARGET_0, 1);
            let batch_0 = Attesters::get_batch_by_message_hash(TARGET_0, batch_0_hash).unwrap();
            assert_eq!(batch_0.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_0.latency, LatencyStatus::Late(1, 0));

            let committee_0_on_target =
                NextCommitteeOnTarget::<MiniRuntime>::get(TARGET_0).unwrap();

            add_target_and_transition_to_next_batch(TARGET_0, 1);
            let batch_0 = Attesters::get_batch_by_message_hash(TARGET_0, batch_0_hash).unwrap();
            assert_eq!(batch_0.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_0.latency, LatencyStatus::Late(2, 0));
            // Trigger the next committee transition
            select_new_committee();
            // Retreive next batch
            let batch_1 = NextBatch::<MiniRuntime>::get(TARGET_0).unwrap();
            assert!(batch_1.next_committee.is_some());
            assert_eq!(batch_1.index, 1);
            let committee_1_on_target =
                NextCommitteeOnTarget::<MiniRuntime>::get(TARGET_0).unwrap();
            let batch_1_hash = batch_1.message_hash();
            // todo: fix the randomness source on mini-mock (yields 0)
            // assert!(committee_0_on_target != committee_1_on_target);
            add_target_and_transition_to_next_batch(TARGET_0, 2);
            let batch_0 = Attesters::get_batch_by_message_hash(TARGET_0, batch_0_hash).unwrap();
            assert_eq!(batch_0.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_0.latency, LatencyStatus::Late(3, 0));
            let batch_1 = Attesters::get_batch_by_message_hash(TARGET_0, batch_1_hash).unwrap();
            assert_eq!(batch_1.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_1.latency, LatencyStatus::OnTime);
        });
    }

    #[test]
    fn committee_setup_and_transition() {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();

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
        let _target = [0u8; 4];
        let _mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            let sfx_id_to_sign_on: [u8; 32] = *b"message_that_needs_attestation32";
            let (_message_hash, expected_message_bytes) =
                calculate_hash_for_sfx_message(sfx_id_to_sign_on, 0);

            for counter in 1..33u8 {
                // Register an attester
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }
            select_new_committee();
            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    sfx_id_to_sign_on,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    [0u8; 4],
                    [counter; 32],
                );
            }
            assert_eq!(
                Attesters::get_batches([0u8; 4], BatchStatus::ReadyForSubmissionFullyApproved)
                    .len(),
                1
            );
        });
    }

    #[test]
    fn register_and_submit_21x_attestations_in_ecdsa_changes_status_to_approved_in_next_batching_window(
    ) {
        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            let message: [u8; 32] = *b"message_that_needs_attestation32";
            let (_message_hash, expected_message_bytes) =
                calculate_hash_for_sfx_message(message, 0);

            for counter in 1..22u8 {
                // Register an attester
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            select_new_committee();

            for counter in 1..22u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    message,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    [0u8; 4],
                    [counter; 32],
                );
            }

            let batch = Attesters::get_latest_batch_to_sign([0u8; 4])
                .expect("get_latest_batch_to_sign should return a batch");

            assert_eq!(batch.status, BatchStatus::PendingAttestation);

            // Trigger batching transition
            add_target_and_transition_to_next_batch([0u8; 4], 1);
            let batch = Attesters::get_batch_by_message([0u8; 4], batch.message())
                .expect("get_batch_by_message should return a batch");
            assert_eq!(batch.status, BatchStatus::ReadyForSubmissionByMajority);
        });
    }

    fn calculate_hash_for_sfx_message(message: [u8; 32], index: u32) -> ([u8; 32], Vec<u8>) {
        let mut message_bytes: Vec<u8> = Vec::new();
        message_bytes.extend_from_slice(message.as_ref());
        message_bytes.extend_from_slice(index.to_le_bytes().as_ref());

        let mut keccak = Keccak::v256();
        keccak.update(message_bytes.as_ref());
        let mut res: [u8; 32] = [0; 32];
        keccak.finalize(&mut res);
        (res, message_bytes)
    }

    #[test]
    fn register_and_submit_32x_attestations_in_ecdsa_with_batching_plus_confirmation_to_polka_target(
    ) {
        let target: TargetId = [1u8; 4];
        let _mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            let message: [u8; 32] = *b"message_that_needs_attestation32";
            let (_message_hash, message_bytes) = calculate_hash_for_sfx_message(message, 0);

            for counter in 1..33u8 {
                // Register an attester
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            select_new_committee();

            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                // Submit an attestation signed with the Ed25519 key
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    message,
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    target,
                    [counter; 32],
                );
            }

            let attested_batches =
                Attesters::get_batches(target, BatchStatus::ReadyForSubmissionFullyApproved);

            assert_eq!(attested_batches.len(), 1);
            let first_batch = attested_batches[0].clone();

            // Check if the attestations have been added to the batch
            let first_batch_hash = first_batch.message_hash();
            let first_batch_message = first_batch.message();

            assert_eq!(first_batch.signatures.len(), 32);
            assert_eq!(
                first_batch.status,
                BatchStatus::ReadyForSubmissionFullyApproved
            );

            let mock_valid_batch_confirmation = TargetBatchDispatchEvent {
                signatures: first_batch.signatures,
                hash: first_batch_hash,
                message: first_batch_message.clone(),
            };

            // Commit the batch
            assert_ok!(Attesters::commit_batch(
                Origin::signed(AccountId::from([1; 32])),
                target,
                mock_valid_batch_confirmation.encode(),
            ));

            // Check if the batch status has been updated to Committed
            let batch = Attesters::get_batch_by_message(target, first_batch_message)
                .expect("Batch by message should exist");
            assert_eq!(batch.status, BatchStatus::Committed);
        });
    }

    #[test]
    fn register_and_submit_32x_attestations_and_check_collusion_permanent_slash() {
        let target: TargetId = [1u8; 4];
        let _mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_polkadot_gateway_record()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let message: [u8; 32] = *b"message_that_needs_attestation32";
            let index_0: u32 = 0;
            let mut expected_message_bytes = Vec::new();
            expected_message_bytes.extend_from_slice(message.encode().as_slice());
            expected_message_bytes.extend_from_slice(index_0.to_le_bytes().as_slice());

            for counter in 1..33u8 {
                // Register an attester
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            select_new_committee();

            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
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

            let fist_batches =
                Attesters::get_batches(target, BatchStatus::ReadyForSubmissionFullyApproved);
            assert_eq!(fist_batches.len(), 1);
            let first_batch = fist_batches[0].clone();
            assert_eq!(first_batch.signatures.len(), 32);

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
            let batch = Attesters::get_batch_by_message_hash(target, latest_batch_hash)
                .expect("Batch by message should exist");

            assert_eq!(batch.status, BatchStatus::ReadyForSubmissionFullyApproved);

            let slashed_permanently = PermanentSlashes::<MiniRuntime>::get();

            // Check if all of the attesters have been slashed
            for counter in 1..33u8 {
                let attester = AccountId::from([counter; 32]);
                assert!(Attesters::is_permanently_slashed(&attester));
                assert!(slashed_permanently.contains(&attester));
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
