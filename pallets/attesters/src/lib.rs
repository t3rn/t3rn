#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub use crate::pallet::*;

pub type TargetId = [u8; 4];

#[frame_support::pallet]
pub mod pallet {

    // Overcharge factor as a constant.
    const OVERCHARGE_FACTOR: Percent = Percent::from_percent(32);
    const SIX_EPOCHS_IN_LOCAL_BLOCKS_U8: u8 = 6 * 32;
    // const THREE_EPOCHS_IN_LOCAL_BLOCKS_U8: u8 = 3 * 32;
    const TWO_EPOCHS_IN_LOCAL_BLOCKS_U8: u8 = 2 * 32;
    const ONE_EPOCHS_IN_LOCAL_BLOCKS_U8: u8 = 32;
    pub const REWARD_ADJUSTMENT: Percent = Percent::from_percent(25);

    use super::*;
    t3rn_primitives::reexport_currency_types!();
    use tiny_keccak::{Hasher, Keccak};

    use codec::{Decode, Encode};
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, GenesisBuild, Randomness, ReservableCurrency},
    };
    use frame_system::pallet_prelude::{BlockNumberFor, *};
    use sp_core::{hexdisplay::AsBytesRef, H160, H256, H512};
    pub use t3rn_primitives::portal::InclusionReceipt;
    use t3rn_primitives::{
        light_client::{LightClientAsyncAPI, LightClientHeartbeat},
        TreasuryAccount,
    };

    use sp_runtime::{
        traits::{CheckedAdd, CheckedDiv, CheckedMul, Saturating, Zero},
        Percent,
    };
    use sp_std::{convert::TryInto, prelude::*};

    use t3rn_abi::{Codec, FilledAbi};

    pub use t3rn_primitives::attesters::{
        AttesterInfo, AttestersChange, AttestersReadApi, AttestersWriteApi,
        BatchConfirmedSfxWithGMPPayload, BatchRevertedSfxId, BatchingFactor,
        CommitteeTransitionIndices, LatencyStatus, PublicKeyEcdsa33b, Signature65b, COMMITTEE_SIZE,
        ECDSA_ATTESTER_KEY_TYPE_ID, ED25519_ATTESTER_KEY_TYPE_ID, SR25519_ATTESTER_KEY_TYPE_ID,
    };
    use t3rn_primitives::{
        attesters::{CommitteeRecoverable, CommitteeTransition},
        circuit::{Cause, CircuitStatus, ReadSFX},
        portal::Portal,
        rewards::RewardsWriteApi,
        xdns::Xdns,
        ExecutionVendor, GatewayVendor, SpeedMode, TreasuryAccountProvider,
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
        pub available_to_commit_at: BlockNumber,
        pub committed_sfx: Option<BatchConfirmedSfxWithGMPPayload>,
        pub reverted_sfx: Option<BatchRevertedSfxId>,
        pub next_committee: Option<CommitteeRecoverable>,
        pub banned_committee: Option<CommitteeRecoverable>,
        pub index: u32,
        // Below fields are not part of the message, but are used to track the state of the message
        pub signatures: Vec<(u32, Signature65b)>,
        pub created: BlockNumber,
        pub status: BatchStatus,
        pub latency: LatencyStatus,
        pub halt: bool,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub struct InfluxMessage<BlockNumber> {
        pub message_hash: H256,
        pub message: H256,
        pub height_there: BlockNumber,
        pub gateway: TargetId,
        pub signatures: Vec<Vec<u8>>,
        pub created: BlockNumber,
        pub status: BatchStatus,
    }

    impl<BlockNumber: Zero> Default for BatchMessage<BlockNumber> {
        fn default() -> Self {
            BatchMessage {
                available_to_commit_at: Zero::zero(),
                committed_sfx: None,
                reverted_sfx: None,
                next_committee: None,
                banned_committee: None,
                signatures: Vec::new(),
                status: BatchStatus::PendingMessage,
                created: Zero::zero(),
                latency: LatencyStatus::OnTime,
                halt: false,
                index: 0,
            }
        }
    }
    // Add the following method to `BatchMessage` struct
    impl<BlockNumber> BatchMessage<BlockNumber> {
        pub fn message(&self) -> Vec<u8> {
            let mut encoded_message = Vec::new();

            let mut encode_eth_committee_addresses_into_message =
                |committee: &CommitteeRecoverable| {
                    for recoverable in committee.iter() {
                        // Ensure recoverable address is 20 bytes long
                        if recoverable.as_bytes_ref().len() != 20 {
                            log::warn!(
                                "Recoverable address in BatchMessage::message() is not 20 bytes long: {:?}", recoverable.as_bytes_ref()
                            );
                            continue
                        }
                        // Encoding of Ethereum address will extend the length of the encoded message by 12 bytes to fill entire 32b word
                        // Extend the encoded message with 12 bytes of zeros to keep the length of the encoded message constant
                        const ETH_ADDRESS_LEN: usize = 20;
                        const ETH_ADDRESS_PADDING: usize = 12;
                        let mut eth_address_as_32b_word =
                            [0u8; ETH_ADDRESS_LEN + ETH_ADDRESS_PADDING];
                        eth_address_as_32b_word
                            [ETH_ADDRESS_PADDING..ETH_ADDRESS_LEN + ETH_ADDRESS_PADDING]
                            .copy_from_slice(recoverable.as_bytes_ref());
                        encoded_message.extend_from_slice(eth_address_as_32b_word.as_slice());
                    }
                };

            if let Some(ref committee) = self.next_committee {
                encode_eth_committee_addresses_into_message(committee);
            }
            if let Some(ref committee) = self.banned_committee {
                encode_eth_committee_addresses_into_message(committee);
            }
            if let Some(ref sfx_vec) = self.committed_sfx {
                for sfx in sfx_vec.iter() {
                    encoded_message.extend_from_slice(sfx.as_bytes());
                }
            }
            if let Some(ref sfx_vec) = self.reverted_sfx {
                for sfx in sfx_vec.iter() {
                    encoded_message.extend_from_slice(sfx.as_bytes());
                }
            }
            encoded_message.extend_from_slice(self.index.to_be_bytes().as_slice());
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

        pub fn read_batching_factor(&self) -> u16 {
            let mut batching_factor = 0u16;
            if let Some(ref sfx) = self.committed_sfx {
                batching_factor = sfx.len() as u16;
            }
            if let Some(ref sfx) = self.reverted_sfx {
                batching_factor = batching_factor.saturating_add(sfx.len() as u16)
            }

            batching_factor
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
        // Message hash as H256 (32b)
        pub hash: H256,
        // Executor on target
        pub executor_on_target: H160,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type ActiveSetSize: Get<u32>;
        type CommitteeSize: Get<u32>;
        type BatchingWindow: Get<BlockNumberFor<Self>>;
        type RepatriationPeriod: Get<BlockNumberFor<Self>>;
        type ShufflingFrequency: Get<BlockNumberFor<Self>>;
        type MaxBatchSize: Get<u32>;
        type RewardMultiplier: Get<BalanceOf<Self>>;
        type TreasuryAccounts: TreasuryAccountProvider<Self::AccountId>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type RandomnessSource: Randomness<Self::Hash, BlockNumberFor<Self>>;
        type DefaultCommission: Get<Percent>;
        type MinNominatorBond: Get<BalanceOf<Self>>;
        type MinAttesterBond: Get<BalanceOf<Self>>;
        type Portal: Portal<Self>;
        type Rewards: RewardsWriteApi<Self::AccountId, BalanceOf<Self>, BlockNumberFor<Self>>;
        type ReadSFX: ReadSFX<Self::Hash, Self::AccountId, BalanceOf<Self>, BlockNumberFor<Self>>;
        type Xdns: Xdns<Self, BalanceOf<Self>>;
        type LightClientAsyncAPI: LightClientAsyncAPI<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn attesters)]
    pub type Attesters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, AttesterInfo>;

    #[pallet::storage]
    pub type NextCommittee<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    pub type CurrentCommittee<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    pub type PreviousCommittee<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    pub type CommitteeTransitionOn<T: Config> = StorageMap<_, Identity, TargetId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn attestations_influx)]
    pub type AttestationsInflux<T: Config> =
        StorageDoubleMap<_, Identity, TargetId, Identity, H256, InfluxMessage<BlockNumberFor<T>>>;

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
    #[pallet::getter(fn invulnerable_attester)]
    pub type InvulnerableAttester<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

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
    pub type NextBatch<T: Config> =
        StorageMap<_, Identity, TargetId, BatchMessage<BlockNumberFor<T>>>;

    #[pallet::storage]
    #[pallet::getter(fn next_committee_on_target)]
    pub type NextCommitteeOnTarget<T: Config> =
        StorageMap<_, Identity, TargetId, CommitteeTransition>;

    #[pallet::storage]
    #[pallet::getter(fn batches_to_sign)]
    pub type BatchesToSign<T: Config> =
        StorageMap<_, Identity, TargetId, Vec<BatchMessage<BlockNumberFor<T>>>>;

    #[pallet::storage]
    #[pallet::getter(fn batches)]
    pub type Batches<T: Config> =
        StorageMap<_, Identity, TargetId, Vec<BatchMessage<BlockNumberFor<T>>>>;

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
    #[pallet::getter(fn paid_finality_fees)]
    pub type PaidFinalityFees<T: Config> =
        StorageMap<_, Blake2_128Concat, TargetId, Vec<BalanceOf<T>>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AttesterRegistered(T::AccountId),
        AttesterDeregistrationScheduled(T::AccountId, BlockNumberFor<T>),
        AttesterDeregistered(T::AccountId),
        AttestationSubmitted(T::AccountId),
        // Attesters(AttestersEvent),
        BatchingFactorRead(Vec<(TargetId, Option<BatchingFactor>)>),
        BatchCommitted(
            TargetId,
            BatchMessage<BlockNumberFor<T>>,
            Vec<u8>,
            H256,
            BalanceOf<T>,
        ),
        ConfirmationRewardCalculated(
            TargetId,
            u32,          // batch index
            BalanceOf<T>, // current reward
            Percent,      // auto regression param before
            Percent,      // auto regression param after
        ),
        CollusionWithPermanentSlashDetected(TargetId, H256),
        UserFinalityFeeEstimated(TargetId, BalanceOf<T>),
        NewAttestationBatch(TargetId, BatchMessage<BlockNumberFor<T>>),
        NewAttestationMessageHash(TargetId, H256, ExecutionVendor),
        NewConfirmationBatch(TargetId, BatchMessage<BlockNumberFor<T>>, Vec<u8>, H256),
        Nominated(T::AccountId, T::AccountId, BalanceOf<T>),
        NewTargetActivated(TargetId),
        NewTargetProposed(TargetId),
        AttesterAgreedToNewTarget(T::AccountId, TargetId, Vec<u8>),
        CurrentPendingAttestationBatches(TargetId, Vec<(u32, H256)>),
        AttestationsRemovedFromLateBatches(Vec<u32>),
        AttestationTargetRemoved(TargetId, Vec<TargetId>),
        // ShufflingCompleted(current committee, previous committee, next committee)
        ShufflingCompleted(Vec<T::AccountId>, Vec<T::AccountId>, Vec<T::AccountId>),
    }

    #[pallet::error]
    pub enum Error<T> {
        AttesterNotFound,
        ArithmeticOverflow,
        InvalidSignature,
        InvalidMessage,
        InvalidTargetInclusionProof,
        UnexpectedBatchHashRecoveredFromCommitment,
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
        InfluxSignatureAlreadySubmitted,
        InfluxMessageHashIncorrect,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(60_000)]
        pub fn register_attester(
            origin: OriginFor<T>,
            self_nominate_amount: BalanceOf<T>,
            ecdsa_key: [u8; 33],
            ed25519_key: [u8; 32],
            sr25519_key: [u8; 32],
            custom_commission: Option<Percent>,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin.clone())?;

            Self::do_register_attester(
                account_id,
                self_nominate_amount,
                ecdsa_key,
                ed25519_key,
                sr25519_key,
                custom_commission,
            )
        }

        #[pallet::weight(60_000)]
        pub fn register_invulnerable_attester(
            origin: OriginFor<T>,
            self_nominate_amount: BalanceOf<T>,
            ecdsa_key: [u8; 33],
            ed25519_key: [u8; 32],
            sr25519_key: [u8; 32],
            custom_commission: Option<Percent>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let account_id = T::TreasuryAccounts::get_treasury_account(TreasuryAccount::Escrow);
            Self::do_register_attester(
                account_id.clone(),
                self_nominate_amount,
                ecdsa_key,
                ed25519_key,
                sr25519_key,
                custom_commission,
            )?;

            InvulnerableAttester::<T>::put(&account_id);

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
                        .checked_mul(&BlockNumberFor::<T>::from(2u32))
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

            AttestationTargets::<T>::mutate(|targets| {
                if let Some(index) = targets.iter().position(|x| x == &target) {
                    targets.remove(index);
                }
            });
            PendingAttestationTargets::<T>::mutate(|pending| {
                if let Some(index) = pending.iter().position(|x| x == &target) {
                    pending.remove(index);
                }
            });

            // Purge all attestations for the target
            Batches::<T>::remove(target);
            BatchesToSign::<T>::remove(target);
            NextBatch::<T>::remove(target);

            Self::deposit_event(Event::AttestationTargetRemoved(
                target,
                AttestationTargets::<T>::get(),
            ));

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
        pub fn submit_for_influx_attestation(
            // Must be signed by the attester in current Committee
            origin: OriginFor<T>,
            // Message being a hash of the message to sign
            message: H256,
            // Message hash of the message
            message_hash: H256,
            // Message hash of the message
            height_there: BlockNumberFor<T>,
            // Target of the attestation
            target: TargetId,
            // Signature of the message
            signature: Vec<u8>,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;

            // Ensure target is activated
            ensure!(
                AttestationTargets::<T>::get().contains(&target),
                Error::<T>::TargetNotActive
            );

            // Lookup the attester in the storage
            let attester = Attesters::<T>::get(&account_id).ok_or(Error::<T>::NotRegistered)?;

            // Check if Current Committee
            ensure!(
                CurrentCommittee::<T>::get().contains(&account_id),
                Error::<T>::NotInCurrentCommittee
            );

            // Check if the attester has agreed to the target
            let recoverable = AttestersAgreements::<T>::get(&account_id, &target)
                .ok_or(Error::<T>::AttesterDidNotAgreeToNewTarget)?;

            // Get the codec of target from XDNS
            let target_codec = <T as Config>::Xdns::get_target_codec(&target)?;

            let mut message_bytes = message.as_bytes().to_vec();
            // add target to the message (assumed encoded on 32 bytes)
            message_bytes.extend_from_slice(&target.encode());
            // add height_there to the message (assumed encoded on 4 bytes)
            message_bytes.extend_from_slice(&height_there.encode());

            // Check if message_hash is correct
            let recalculate_message_hash: H256 = match target_codec {
                Codec::Scale => {
                    let substrate_message_hash =
                        sp_core::hashing::blake2_256(message_bytes.as_slice());
                    H256::from(substrate_message_hash)
                },
                Codec::Rlp => {
                    let evm_message_hash = sp_core::hashing::keccak_256(message_bytes.as_slice());
                    H256::from(evm_message_hash)
                },
            };

            ensure!(
                recalculate_message_hash == message_hash,
                Error::<T>::InfluxMessageHashIncorrect
            );

            // Get XDNS targer's vendor
            let vendor = <T as Config>::Xdns::get_verification_vendor(&target)
                .map_err(|_| Error::<T>::XdnsTargetNotActive)?;

            let is_verified = attester
                .verify_attestation_signature(
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    &message_hash.encode(),
                    &signature,
                    recoverable,
                    &vendor,
                )
                .map_err(|_| Error::<T>::InvalidSignature)?;

            ensure!(is_verified, Error::<T>::InvalidSignature);

            // Recover or create attestation influx from storage
            let maybe_attestation_influx_msg = AttestationsInflux::<T>::get(&target, &message_hash);

            let mut attestation_influx = if let Some(mut attestation) = maybe_attestation_influx_msg
            {
                // Check if the attestation is already submitted into Signatures vector
                ensure!(
                    !attestation.signatures.contains(&signature),
                    Error::<T>::InfluxSignatureAlreadySubmitted
                );
                // Update Influx attestation with attestator's signature
                attestation.signatures.push(signature);
                attestation
            } else {
                InfluxMessage {
                    message_hash,
                    message,
                    height_there,
                    gateway: target,
                    signatures: vec![signature],
                    created: <frame_system::Pallet<T>>::block_number(),
                    status: BatchStatus::PendingAttestation,
                }
            };

            // Set status based on if signed by majority of the committee
            let quorum = (T::CommitteeSize::get() * 2 / 3) as usize;
            let status = if attestation_influx.signatures.len() >= quorum {
                BatchStatus::ReadyForSubmissionByMajority
            } else if (attestation_influx.signatures.len() as u32) == T::CommitteeSize::get() {
                BatchStatus::ReadyForSubmissionFullyApproved
            } else {
                BatchStatus::PendingAttestation
            };
            attestation_influx.status = status;
            // Save Influx attestation into storage
            AttestationsInflux::<T>::insert(&target, &message_hash, attestation_influx);

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

            // TODO: Generalize attesters to work with multiple ExecutionVendor architecture.
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

            #[cfg(not(feature = "test-skip-verification"))]
            let target_codec = T::Xdns::get_target_codec(&target)?;

            // ToDo: Check the source address of the batch ensuring matches Escrow contract address.
            let _target_escrow_address = T::Xdns::get_escrow_account(&target)?;

            let escrow_batch_success_descriptor = b"EscrowBatchSuccess:Event(\
                MessageHash:H256,\
                BeneficiaryOnTarget:Account20,\
            )"
            .to_vec();

            #[cfg(not(feature = "test-skip-verification"))]
            let escrow_inclusion_receipt = T::Portal::verify_event_inclusion(
                target,
                SpeedMode::Finalized,
                None,
                target_inclusion_proof_encoded,
            )?; // Todo: add escrow address
            #[cfg(feature = "test-skip-verification")]
            let escrow_inclusion_receipt = InclusionReceipt::<BlockNumberFor<T>> {
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
            .recode_as(&target_codec, &Codec::Scale, true)?;

            #[cfg(feature = "test-skip-verification")]
            let recoded_batch_event_bytes = escrow_inclusion_receipt.message;

            let on_target_batch_event =
                TargetBatchDispatchEvent::decode(&mut &recoded_batch_event_bytes[..])
                    .map_err(|_| Error::<T>::InvalidTargetInclusionProof)?;

            // Assume that the event is emitted by the escrow contract is H256
            let recovered_enacted_batch_hash: H256 =
                H256::decode(&mut &on_target_batch_event.hash[..])
                    .map_err(|_| Error::<T>::UnexpectedBatchHashRecoveredFromCommitment)?;

            let batches = Self::get_batches_to_commit(target);

            let batch = match batches
                .iter()
                .find(|batch| batch.message_hash() == recovered_enacted_batch_hash)
            {
                Some(batch) => batch,
                None => {
                    // At this point we know the valid message has been recorded on target Escrow Smart Contract
                    // If we can't find the corresponding batch by the message - we have a problem - attesters are colluding.
                    log::error!(
                        "CollusionWithPermanentSlashDetected detected on target: {target:?} for message hash {recovered_enacted_batch_hash:?}"
                    );
                    // fixme: must synchronize remote block height and local block height and fetch committee responsible for attesting false batch.
                    Self::apply_permanent_attesters_slash(CurrentCommittee::<T>::get());

                    Self::deposit_event(Event::CollusionWithPermanentSlashDetected(
                        target,
                        recovered_enacted_batch_hash,
                    ));
                    // must return success here to apply permanent slash
                    return Ok(())
                },
            };

            ensure!(
                batch.status == BatchStatus::ReadyForSubmissionFullyApproved
                    || batch.status == BatchStatus::ReadyForSubmissionByMajority,
                Error::<T>::BatchNotFound
            );

            // Update the status of the batch to Committed
            Self::find_and_set_batch_as_committed(target, &recovered_enacted_batch_hash)?;

            // Follow with infallible reward payment
            let delay = <frame_system::Pallet<T>>::block_number()
                .saturating_sub(batch.available_to_commit_at)
                .saturating_sub(BlockNumberFor::<T>::from(TWO_EPOCHS_IN_LOCAL_BLOCKS_U8));

            let to_pay = Self::estimate_finality_reward(&target, delay);

            // Emit the event
            Self::deposit_event(Event::BatchCommitted(
                target,
                batch.clone(),
                batch.message(),
                batch.message_hash(),
                to_pay,
            ));

            if to_pay > Zero::zero() {
                Self::reward_submitter(
                    &submitter,
                    &on_target_batch_event.executor_on_target,
                    &target,
                    to_pay,
                )?;
                PaidFinalityFees::<T>::append(target, to_pay);
            }

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn read_pending_batches(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            for target in AttestationTargets::<T>::get() {
                let pending_attestation_batches = Self::get_all_batches_to_sign(target);
                Self::deposit_event(Event::CurrentPendingAttestationBatches(
                    target,
                    pending_attestation_batches
                        .iter()
                        .map(|batch| (batch.index, batch.message_hash()))
                        .collect::<Vec<(u32, H256)>>(),
                ));
            }

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn read_latest_batching_factor_overview(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            let mut batching_factor_overview = Vec::new();
            for target in AttestationTargets::<T>::get() {
                let batching_factor = Self::read_latest_batching_factor(&target);
                batching_factor_overview.push((target, batching_factor));
            }

            Self::deposit_event(Event::BatchingFactorRead(batching_factor_overview));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn estimate_user_finality_fee(
            origin: OriginFor<T>,
            target: TargetId,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Ensure target is active
            ensure!(
                AttestationTargets::<T>::get().contains(&target),
                Error::<T>::TargetNotActive
            );

            let finality_fee = <Pallet<T> as AttestersReadApi<
                T::AccountId,
                BalanceOf<T>,
                BlockNumberFor<T>,
            >>::estimate_finality_fee(&target);

            Self::deposit_event(Event::UserFinalityFeeEstimated(target, finality_fee));

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
            maybe_gmp_payload: Option<H256>,
        ) -> Result<(), DispatchError> {
            // Put 2 x 32 bytes of sfx_id and possible GMP payload into 64 bytes
            let mut sfx_id_and_gmp_payload = H512::zero();
            sfx_id_and_gmp_payload[..32].copy_from_slice(&sfx_id[..]);
            if let Some(gmp_payload) = maybe_gmp_payload {
                sfx_id_and_gmp_payload[32..].copy_from_slice(&gmp_payload[..]);
            }
            NextBatch::<T>::try_mutate(target, |next_batch| {
                if let Some(ref mut next_batch) = next_batch {
                    if let Some(ref mut batch_sfx) = &mut next_batch.committed_sfx {
                        if batch_sfx.contains(&sfx_id_and_gmp_payload) {
                            return Err("SfxAlreadyRequested".into())
                        } else {
                            batch_sfx.push(sfx_id_and_gmp_payload);
                        }
                        // if batch_sfx.contains(&sfx_id) {
                        //     return Err("SfxAlreadyRequested".into())
                        // } else {
                        //     batch_sfx.push(sfx_id);
                        // }
                    } else {
                        next_batch.committed_sfx = Some(vec![sfx_id_and_gmp_payload]);
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

        fn request_next_committee_attestation() -> Vec<(TargetId, u32)> {
            let mut batch_indices_on_targets = Vec::new();
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

                        batch_indices_on_targets.push((target, next_batch.index));
                    }
                }
            }
            batch_indices_on_targets
        }
    }

    /// Finality Fee Estimation and User Base Projection
    ///
    /// This module provides functionalities for estimating future finality fees and user base in a decentralized system, drawing inspiration from the concept of a pension scheme.
    ///
    /// ## Overview
    ///
    /// The aim of this module is to estimate future fees and user base in a way that balances the interests of all participants. This is achieved by employing methods similar to those used in pension systems, where fees paid by current participants are used to support earlier generations.
    ///
    /// Two primary functions provided by this module are:
    ///
    /// 1. `estimate_user_finality_fee`
    /// 2. `estimate_future_user_base`
    ///
    /// ## Functionality
    ///
    /// ### estimate_user_finality_fee
    ///
    /// This function estimates the finality fee for an individual user.
    /// The function considers the total fee paid in the most recent epoch and divides it by the number of users in that epoch to calculate a base user fee.
    /// An overcharge factor is then added to this base fee to account for fluctuations and provide a buffer.
    /// This calculation mirrors the process in pension systems where individual contributions are calculated based on total liabilities and the number of current contributors.
    ///
    /// ### estimate_future_user_base
    ///
    /// The estimate_future_user_base function forecasts the user base size for a future epoch.
    /// The function calculates the average growth rate of the user base over the past few epochs and applies this rate iteratively to project future growth.
    /// This function is similar to population projections in pension systems, which are critical in determining future contribution rates.
    ///
    /// ## Concluding Remarks
    ///
    /// The functions in this module draw inspiration from pension systems, projecting future conditions based on past data and current trends.
    /// While the context is different – a decentralized system instead of a pension scheme – the fundamental concepts are the same.
    /// The ability to estimate future fees and user base size contributes to system sustainability and fairness, much like in a well-managed pension scheme.
    impl<T: Config> AttestersReadApi<T::AccountId, BalanceOf<T>, BlockNumberFor<T>> for Pallet<T> {
        /// Getter for the current committee. Returns a Vec of AccountIds.
        fn previous_committee() -> Vec<T::AccountId> {
            PreviousCommittee::<T>::get()
        }

        /// Getter for the current committee. Returns a Vec of AccountIds.
        fn current_committee() -> Vec<T::AccountId> {
            CurrentCommittee::<T>::get()
        }

        /// Getter for the active set. Returns a Vec of AccountIds.
        fn active_set() -> Vec<T::AccountId> {
            ActiveSet::<T>::get()
        }

        /// Getter for the active set of ONLY those who haven't been permanently slashed.
        /// Returns a Vec of AccountIds.
        fn honest_active_set() -> Vec<T::AccountId> {
            let active_set = ActiveSet::<T>::get();
            active_set
                .into_iter()
                .filter(|a| !Self::is_permanently_slashed(a))
                .collect()
        }

        /// Getter for the info of the attester provided.
        fn read_attester_info(attester: &T::AccountId) -> Option<AttesterInfo> {
            Attesters::<T>::get(attester)
        }

        /// Getter for the nominations of the given attester.
        /// Returns the nominator and the balance of the nomination as a tuple (account id, balance).
        fn read_nominations(for_attester: &T::AccountId) -> Vec<(T::AccountId, BalanceOf<T>)> {
            Nominations::<T>::iter_prefix(for_attester)
                .map(|(nominator, balance)| (nominator, balance))
                .collect()
        }

        /// Getter for the attestation targets.
        fn get_activated_targets() -> Vec<TargetId> {
            AttestationTargets::<T>::get()
        }

        /// Getter for the latency status of the given target.
        /// Selects the oldest batch with PendingAttestation and return its LatencyStatus.
        fn read_attestation_latency(target: &TargetId) -> Option<LatencyStatus> {
            let mut batches = Self::get_batches(*target, BatchStatus::PendingAttestation);
            batches.sort_by(|a, b| a.created.cmp(&b.created));
            let oldest_batch = batches.first();
            oldest_batch.map(|batch| batch.latency.clone())
        }

        /// Estimation of the finality fee for the given target.
        ///
        /// For this first version, we don't take into account the number of users, i.e., there's
        /// no batching factor.
        fn estimate_finality_fee(target: &TargetId) -> BalanceOf<T> {
            let base_user_fee_for_single_user: BalanceOf<T> =
                10_000_000_000_000u128.try_into().unwrap_or_default();

            let number_of_users = Self::estimate_batching_factor(target)
                .unwrap_or(BatchingFactor {
                    latest_confirmed: 0,
                    latest_signed: 1,
                    current_next: 0,
                    up_to_last_10_confirmed: vec![],
                })
                .latest_signed as u32;

            // Retrieve the (finality) fees that were paid to the target
            let paid_finality_fees =
                PaidFinalityFees::<T>::get(target).unwrap_or(vec![base_user_fee_for_single_user]);

            // Get the LATEST paid finality fee
            let latest_fee = *paid_finality_fees
                .last()
                .unwrap_or(&base_user_fee_for_single_user);

            // Compute the base-user fee.
            // It is the latest fee, divided by the number of users.
            let base_user_fee = latest_fee
                .checked_div(&BalanceOf::<T>::from(number_of_users))
                .unwrap_or(base_user_fee_for_single_user);

            // The user fee is the (1 + overcharge-factor) * base-user-fee
            OVERCHARGE_FACTOR
                .mul_ceil(base_user_fee)
                .saturating_add(base_user_fee)
        }

        /// Compute, for a given target, the finality reward as a Balance
        fn estimate_finality_reward(
            target: &TargetId,
            blocks_delay: BlockNumberFor<T>,
        ) -> BalanceOf<T> {
            // ToDo: Move this to storage and make available to sudo only updates
            let base_user_fee_for_single_user: BalanceOf<T> =
                10_000_000_000_000u128.try_into().unwrap_or_default();

            // Retrieve the (finality) fees that were paid to the target
            let finality_fee_opt = PaidFinalityFees::<T>::get(target);

            // Get the last finality fee
            let finality_reward = finality_fee_opt
                .as_ref()
                .and_then(|fees| fees.last())
                .unwrap_or(&base_user_fee_for_single_user);

            // Create a mutable copy of finality_reward to work with
            let mut finality_reward = *finality_reward;

            let capped_delay_in_blocks = blocks_delay.min(SIX_EPOCHS_IN_LOCAL_BLOCKS_U8.into());

            // safe division since ONE_EPOCHS_IN_LOCAL_BLOCKS_U8 is assumed to be > 0
            let epochs_delayed =
                capped_delay_in_blocks / BlockNumberFor::<T>::from(ONE_EPOCHS_IN_LOCAL_BLOCKS_U8);
            // Convert to usize for range
            let epochs_delayed = TryInto::<usize>::try_into(epochs_delayed).unwrap_or(0);

            for _ in 0..epochs_delayed {
                finality_reward = REWARD_ADJUSTMENT
                    .mul_ceil(finality_reward)
                    .saturating_add(finality_reward);
            }

            // If finality fee was received within the first epoch, decrease the reward by 25%
            if capped_delay_in_blocks < BlockNumberFor::<T>::from(ONE_EPOCHS_IN_LOCAL_BLOCKS_U8) {
                finality_reward =
                    finality_reward.saturating_sub(REWARD_ADJUSTMENT.mul_ceil(finality_reward));
            }

            finality_reward
        }

        /// Estimate the batching factor for a given target.
        fn estimate_batching_factor(target: &TargetId) -> Option<BatchingFactor> {
            Self::read_latest_batching_factor(target)
        }
    }

    impl<T: Config> Pallet<T> {
        fn find_batches(
            target: TargetId,
            message: &Vec<u8>,
        ) -> Vec<BatchMessage<BlockNumberFor<T>>> {
            Batches::<T>::get(target)
                .iter()
                .find(|batches| batches.iter().any(|batch| &batch.message() == message))
                .unwrap()
                .clone()
        }

        fn do_register_attester(
            account_id: T::AccountId,
            self_nominate_amount: BalanceOf<T>,
            ecdsa_key: [u8; 33],
            ed25519_key: [u8; 32],
            sr25519_key: [u8; 32],
            custom_commission: Option<Percent>,
        ) -> DispatchResult {
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

            Self::deposit_event(Event::AttesterRegistered(account_id.clone()));

            Ok(())
        }

        fn read_latest_batching_factor(target: &TargetId) -> Option<BatchingFactor> {
            // If target isn't active yet, return None
            if !AttestationTargets::<T>::get().contains(target) {
                return None
            }

            // Read amount of confirmed and reverted sfx out of last 10 confirmed batches, or fill with 0 if there aren't enough
            let up_to_last_10_confirmed: Vec<u16> =
                Self::get_batches(*target, BatchStatus::Committed)
                    .iter()
                    .rev()
                    .take(10)
                    .map(|batch| batch.read_batching_factor())
                    .collect::<Vec<u16>>();

            let latest_confirmed = *up_to_last_10_confirmed.first().unwrap_or(&0);

            let latest_signed = match Self::get_latest_batch_to_sign(*target) {
                Some(batch) => batch.read_batching_factor(),
                None => 0,
            };

            let current_next = match NextBatch::<T>::get(target) {
                Some(batch) => batch.read_batching_factor(),
                None => 0,
            };

            Some(BatchingFactor {
                latest_confirmed,
                latest_signed,
                current_next,
                up_to_last_10_confirmed,
            })
        }

        pub fn committee_size() -> usize {
            T::CommitteeSize::get() as usize
        }

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
        fn apply_permanent_attesters_slash(attester_local_addresses: Vec<T::AccountId>) {
            for account_id in attester_local_addresses {
                PermanentSlashes::<T>::append(account_id);
            }
        }

        pub fn find_and_set_batch_as_committed(
            target: TargetId,
            message_hash: &H256,
        ) -> DispatchResult {
            Batches::<T>::try_mutate(target, |batches_option| {
                let batches = batches_option.as_mut().ok_or(Error::<T>::BatchNotFound)?;
                let batch_by_message = batches
                    .iter_mut()
                    .find(|batch| &batch.message_hash() == message_hash)
                    .ok_or(Error::<T>::BatchNotFound)?;

                batch_by_message.status = BatchStatus::Committed;
                Ok(())
            })
        }

        pub fn reward_submitter(
            submitter: &T::AccountId,
            submitter_on_target: &H160,
            target: &TargetId,
            to_pay: BalanceOf<T>,
        ) -> DispatchResult {
            let attester_recoverable: Vec<u8> = AttestersAgreements::<T>::get(submitter, target)
                .ok_or(Error::<T>::AttesterDidNotAgreeToNewTarget)?;

            if attester_recoverable == submitter_on_target.encode() {
                T::Currency::transfer(
                    &T::TreasuryAccounts::get_treasury_account(
                        t3rn_primitives::TreasuryAccount::Fee,
                    ),
                    &submitter,
                    to_pay,
                    ExistenceRequirement::KeepAlive,
                )?;

                Ok(())
            } else {
                Err(Error::<T>::AttesterDidNotAgreeToNewTarget.into())
            }
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

        pub fn is_last_transition_attested() -> bool {
            CommitteeTransitionOn::<T>::iter().all(|(target, index)| {
                // Try to find the corresponding batch
                let batch_by_index = Batches::<T>::get(target)
                    .and_then(|batches| batches.iter().cloned().find(|b| b.index == index));

                match batch_by_index {
                    // If found, check if it's not in PendingAttestation status
                    Some(batch) => batch.status != BatchStatus::PendingAttestation,
                    // If not found, consider it as "attested"
                    None => true,
                }
            })
        }

        pub fn get_batches(
            target: TargetId,
            by_status: BatchStatus,
        ) -> Vec<BatchMessage<BlockNumberFor<T>>> {
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
        ) -> Option<BatchMessage<BlockNumberFor<T>>> {
            match Batches::<T>::get(target) {
                Some(batches) => batches.iter().find(|&b| b.message() == message).cloned(),
                None => None,
            }
        }

        pub fn get_batch_by_message_hash(
            target: TargetId,
            message_hash: H256,
        ) -> Option<BatchMessage<BlockNumberFor<T>>> {
            match Batches::<T>::get(target) {
                Some(batches) => batches
                    .iter()
                    .find(|&b| b.message_hash() == message_hash)
                    .cloned(),
                None => None,
            }
        }

        pub fn get_batches_to_commit(target: TargetId) -> Vec<BatchMessage<BlockNumberFor<T>>> {
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
        ) -> Option<BatchMessage<BlockNumberFor<T>>> {
            // Get the batches to sign
            let mut batches = Self::get_batches_to_commit(target);
            batches.sort_by(|a, b| b.index.cmp(&a.index));
            batches.first().cloned()
        }

        pub fn get_all_batches_to_sign(target: TargetId) -> Vec<BatchMessage<BlockNumberFor<T>>> {
            let mut batches = Self::get_batches(target, BatchStatus::PendingAttestation);
            batches.sort_by(|a, b| b.created.cmp(&a.created));
            batches
        }

        pub fn get_latest_batch_to_sign(
            target: TargetId,
        ) -> Option<BatchMessage<BlockNumberFor<T>>> {
            Self::get_all_batches_to_sign(target).first().cloned()
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

        pub fn get_current_committee_transition_for_target(
            target: &TargetId,
        ) -> CommitteeTransition {
            let next_committee = NextCommittee::<T>::get();
            let mut committee_transition = Vec::new();

            for attester in &next_committee {
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

        pub fn get_current_committee_indices() -> CommitteeTransitionIndices {
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
            let mut next_committee = NextCommittee::<T>::get();

            let shuffle_active_set = |shuffled_active_set: &mut Vec<T::AccountId>| {
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
            };

            let mut shuffled_active_set = active_set.clone();

            shuffle_active_set(&mut shuffled_active_set);

            // Set the invulnerable attester as the first attester in the shuffled active set
            if let Some(invulnerable_attester) = InvulnerableAttester::<T>::get() {
                if let Some(first_attester) = shuffled_active_set.get_mut(0) {
                    *first_attester = invulnerable_attester;
                }
            }
            let new_committee = shuffled_active_set
                .clone()
                .into_iter()
                .take(committee_size)
                .collect::<Vec<T::AccountId>>();

            // Bootstrap case - if there is no current committee, we need to set it
            if next_committee.is_empty() {
                shuffled_active_set = active_set;
                shuffle_active_set(&mut shuffled_active_set);
                next_committee = shuffled_active_set
                    .into_iter()
                    .take(committee_size)
                    .collect::<Vec<T::AccountId>>();
            }

            CurrentCommittee::<T>::put(next_committee);
            PreviousCommittee::<T>::put(current_committee);
            NextCommittee::<T>::put(new_committee);

            full_shuffle
        }

        pub fn process_repatriations(n: BlockNumberFor<T>, aggregated_weight: Weight) -> Weight {
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
                                    for sfx_id_with_gmp_payload in batch_sfx.iter() {
                                        // Take first 32 bytes as sfx_id
                                        let sfx_id =
                                            H256::from_slice(&sfx_id_with_gmp_payload[..32]);
                                        sfx_to_repatriate.push((CircuitStatus::Committed, sfx_id));
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

        pub fn process_next_batch_window(
            n: BlockNumberFor<T>,
            aggregated_weight: Weight,
        ) -> Weight {
            let quorum = (T::CommitteeSize::get() * 2 / 3) as usize;

            for target in AttestationTargets::<T>::get() {
                let mut new_next_batch = BatchMessage {
                    created: n,
                    ..Default::default()
                };
                // If a batch exists, update its status
                Batches::<T>::mutate(target, |batches| {
                    if let Some(batches) = batches {
                        for batch in batches.iter_mut() {
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
                                // Skip if BatchingWindow overlaps with RepatriationPeriod
                                if !(n % T::RepatriationPeriod::get()).is_zero()
                                    || batch.has_no_sfx()
                                {
                                    // Mark the batch as late if it has not been attested for.
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
            n: BlockNumberFor<T>,
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
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let mut aggregated_weight: Weight = Zero::zero();
            // Check if a shuffling round has passed
            if (n % T::ShufflingFrequency::get()).is_zero() && Self::is_last_transition_attested() {
                // Process pending unnominations
                aggregated_weight = Self::process_pending_unnominations(n, aggregated_weight);
                // Update the active set of attesters
                ActiveSet::<T>::put(
                    SortedNominatedAttesters::<T>::get()
                        .iter()
                        .filter(|(account_id, _)| !Self::is_permanently_slashed(account_id))
                        .take(32)
                        .cloned()
                        .map(|(account_id, _balance)| account_id)
                        .collect::<Vec<T::AccountId>>(),
                );
                aggregated_weight += T::DbWeight::get().reads_writes(1, 1);

                // Call shuffle_committee
                Self::shuffle_committee();
                aggregated_weight += T::DbWeight::get().reads_writes(2, 2);

                Self::deposit_event(Event::ShufflingCompleted(
                    CurrentCommittee::<T>::get(),
                    PreviousCommittee::<T>::get(),
                    NextCommittee::<T>::get(),
                ));

                for (target, batch_index) in Self::request_next_committee_attestation() {
                    CommitteeTransitionOn::<T>::insert(target, batch_index);
                }
                aggregated_weight += T::DbWeight::get().reads_writes(2, 3);

                return aggregated_weight
            }

            if (n % T::BatchingWindow::get()).is_zero() {
                // Check if there any pending attestations to submit with the current batch
                aggregated_weight = Self::process_next_batch_window(n, aggregated_weight);
                let current_block = frame_system::Pallet::<T>::block_number();
                T::LightClientAsyncAPI::on_new_epoch(
                    GatewayVendor::Attesters,
                    n % T::BatchingWindow::get(),
                    LightClientHeartbeat {
                        last_heartbeat: current_block,
                        last_finalized_height: current_block,
                        last_rational_height: current_block,
                        last_fast_height: current_block,
                        is_halted: false,
                        ever_initialized: true,
                    },
                );
            }
            if (n % T::RepatriationPeriod::get()).is_zero() {
                aggregated_weight = Self::process_repatriations(n, aggregated_weight);
            }
            aggregated_weight
        }
    }

    // The genesis config type.
    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub attestation_targets: Vec<TargetId>,
        #[serde(skip)]
        pub _marker: PhantomData<T>,
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Extend the list of attestation targets
            for target in self.attestation_targets.iter() {
                AttestationTargets::<T>::append(target);
            }

            // For each target, create new batch for next window
            for target in AttestationTargets::<T>::get() {
                let new_next_batch = BatchMessage {
                    created: frame_system::Pallet::<T>::block_number(),
                    ..Default::default()
                };
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

    use crate::{TargetBatchDispatchEvent, REWARD_ADJUSTMENT};
    use codec::Encode;
    use frame_support::{
        assert_err, assert_noop, assert_ok,
        traits::{Currency, Get, Hooks, Len},
        StorageValue,
    };
    use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, Pair, RuntimePublic};
    use sp_core::{H160, H256, H512};
    use sp_runtime::traits::Keccak256;
    use sp_std::convert::TryInto;
    use t3rn_mini_mock_runtime::{
        AccountId, ActiveSet, AttestationTargets, Attesters, AttestersAgreements, AttestersError,
        AttestersEvent, AttestersStore, Balance, Balances, BatchMessage, BatchStatus, BlockNumber,
        CommitteeTransitionOn, ConfigAttesters, ConfigRewards, CurrentCommittee,
        ExistentialDeposit, ExtBuilder, FullSideEffects, InfluxMessage, LatencyStatus, MiniRuntime,
        NextBatch, NextCommitteeOnTarget, Nominations, PaidFinalityFees, PendingUnnominations,
        PermanentSlashes, PreviousCommittee, Rewards, RuntimeEvent as Event, RuntimeOrigin,
        SFX2XTXLinksMap, SortedNominatedAttesters, System, XExecSignals, ETHEREUM_TARGET,
        POLKADOT_TARGET,
    };
    use t3rn_primitives::{
        attesters::{
            ecdsa_pubkey_to_eth_address, AttesterInfo, AttestersReadApi, AttestersWriteApi,
            BatchingFactor, CommitteeRecoverable, CommitteeTransitionIndices,
        },
        circuit::{
            AdaptiveTimeout, CircuitStatus, FullSideEffect, SecurityLvl, SideEffect, XExecSignal,
        },
        claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
        TreasuryAccount, TreasuryAccountProvider,
    };
    use tiny_keccak::{Hasher, Keccak};

    #[test]
    fn estimate_user_finality_fee_for_empty_batching_factor() {
        let mut ext = ExtBuilder::default().build();

        ext.execute_with(|| {
            let target_id = ETHEREUM_TARGET;
            AttestationTargets::<MiniRuntime>::append(&target_id);
            // test data
            let _blocks_delay: BlockNumber = 1;
            let base_user_fee_for_single_user: Balance = 10_000_000_000_000u128.try_into().unwrap();
            let overcharge_32_percent_factor: Balance = 3_200_000_000_000u128.try_into().unwrap();

            let _result = Attesters::estimate_user_finality_fee(
                RuntimeOrigin::signed(AccountId::from([1u8; 32])),
                target_id,
            );

            let (_target, estimated_fee) = expect_last_event_to_emit_finality_fee_estimation();

            // For just one, has to be the same
            assert_eq!(
                estimated_fee,
                base_user_fee_for_single_user + overcharge_32_percent_factor
            );
        });
    }

    #[test]
    fn estimate_finality_fee_1_delay() {
        let mut ext = ExtBuilder::default()
            // .with_standard_sfx_abi()
            // .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            // test data
            let target_id = TargetId::default();
            let blocks_delay: BlockNumber = 1;
            let base_user_fee_for_single_user: Balance = 10_000_000_000_000u128.try_into().unwrap();

            let result = Attesters::estimate_finality_reward(&target_id, blocks_delay);

            let expected_reward = base_user_fee_for_single_user
                .saturating_sub(REWARD_ADJUSTMENT.mul_ceil(base_user_fee_for_single_user));

            // For just one, has to be the same
            assert_eq!(result, expected_reward);
        });
    }

    #[test]
    fn estimate_finality_fee_10_delay() {
        let mut ext = ExtBuilder::default()
            // .with_standard_sfx_abi()
            // .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            let target_id = TargetId::default();
            let blocks_delay: BlockNumber = 10;
            let base_user_fee_for_single_user: Balance = 10_000_000_000_000u128.try_into().unwrap();

            let result = Attesters::estimate_finality_reward(&target_id, blocks_delay);

            // let mut expected_finality_reward = PaidFinalityFees::get(target_id)
            //     .as_ref()
            //     .and_then(|fees| fees.last())
            //     .unwrap_or(&base_user_fee_for_single_user);
            // let mut expected_reward = *expected_finality_reward;
            let mut expected_reward = base_user_fee_for_single_user;

            for _ in 0..blocks_delay {
                expected_reward = REWARD_ADJUSTMENT
                    .mul_ceil(expected_reward)
                    .saturating_add(expected_reward);
            }

            // The computed result has to be less than the just a decrease
            assert!(result < expected_reward);
        });
    }

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

        assert_ok!(Attesters::deregister_attester(RuntimeOrigin::signed(
            attester.clone()
        ),));

        // Check Pending Unnomination is created
        assert!(PendingUnnominations::<MiniRuntime>::get(&attester).is_some());

        // Check Pending Unnomination is created with entire self-nomination amount
        assert_eq!(
            PendingUnnominations::<MiniRuntime>::get(&attester).unwrap(),
            vec![(attester.clone(), self_nomination_amount, 817u32)],
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
            RuntimeOrigin::signed(attester.clone()),
            10u128,
            ecdsa_key.clone().try_into().unwrap(),
            ed25519_key.clone().try_into().unwrap(),
            sr25519_key.clone().try_into().unwrap(),
            None,
        ));

        // Run to active set selection
        Attesters::on_initialize(400u32);

        assert_eq!(Attesters::invulnerable_attester(), None);

        let attester_info: AttesterInfo = AttestersStore::<MiniRuntime>::get(&attester).unwrap();
        assert_eq!(attester_info.key_ed.encode(), ed25519_key);
        assert_eq!(attester_info.key_ec.encode(), ecdsa_key);
        assert_eq!(attester_info.key_sr.encode(), sr25519_key);
        attester_info
    }

    pub fn register_attester_from_sudo_privilige_sets_as_invulnerable(
        secret_key: [u8; 32],
    ) -> AttesterInfo {
        // Register an attester
        let attester = AccountId::from(secret_key);

        let ecdsa_key = ecdsa::Pair::from_seed(&secret_key).public().to_raw_vec();
        let ed25519_key = ed25519::Pair::from_seed(&secret_key).public().to_raw_vec();
        let sr25519_key = sr25519::Pair::from_seed(&secret_key).public().to_raw_vec();

        let _ = Balances::deposit_creating(&attester, 100u128);

        // Put Minimum Nomination Bond to Escrow's Treasury Account
        let treasury_account = MiniRuntime::get_treasury_account(TreasuryAccount::Escrow);
        let _ = Balances::deposit_creating(&treasury_account, 100u128);
        assert_ok!(Attesters::register_invulnerable_attester(
            RuntimeOrigin::root(),
            10u128,
            ecdsa_key.clone().try_into().unwrap(),
            ed25519_key.clone().try_into().unwrap(),
            sr25519_key.clone().try_into().unwrap(),
            None,
        ));

        // Run to active set selection
        Attesters::on_initialize(400u32);

        assert_eq!(
            Attesters::invulnerable_attester(),
            Some(treasury_account.clone())
        );

        let attester_info: AttesterInfo =
            AttestersStore::<MiniRuntime>::get(&treasury_account).unwrap();
        assert_eq!(attester_info.key_ed.encode(), ed25519_key);
        assert_eq!(attester_info.key_ec.encode(), ecdsa_key);
        assert_eq!(attester_info.key_sr.encode(), sr25519_key);
        attester_info
    }

    pub fn make_all_agree_to_new_target_or_force_if_no_active_set(target: &TargetId) {
        assert!(!Attesters::attestation_targets().contains(target));
        assert!(Attesters::pending_attestation_targets().contains(target));
        if AttestersStore::<MiniRuntime>::iter().count() == 0 {
            let _ = Attesters::force_activate_target(RuntimeOrigin::root(), *target);
        }
        for (attester, attester_info) in AttestersStore::<MiniRuntime>::iter() {
            // assume attester agrees to eth target: deriving eth address from ecdsa key
            let derived_eth_address = ecdsa_pubkey_to_eth_address(&attester_info.key_ec);
            assert_ok!(derived_eth_address);
            assert_ok!(Attesters::agree_to_new_attestation_target(
                RuntimeOrigin::signed(attester),
                *target,
                derived_eth_address.unwrap().encode(),
            ));
        }
        assert!(!Attesters::pending_attestation_targets().contains(target));
        assert!(Attesters::attestation_targets().contains(target));
    }

    pub fn add_target_and_transition_to_next_batch(target: TargetId, index: u32) -> BlockNumber {
        let _ = Attesters::add_attestation_target(RuntimeOrigin::root(), target);
        if !Attesters::attestation_targets().contains(&target) {
            // if active set is empty, select the next active set
            if !ActiveSet::<MiniRuntime>::get().is_empty() {
                select_new_committee();
            }
            assert_eq!(Attesters::pending_attestation_targets(), [target]);
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
    fn register_attester_with_sudo_privilige_sets_as_invulnerable() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            register_attester_from_sudo_privilige_sets_as_invulnerable([1u8; 32]);
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
        messages: Vec<[u8; 32]>,
        key_type: KeyTypeId,
        target: TargetId,
        secret_key: [u8; 32],
    ) -> (H256, Vec<u8>) {
        // Check if batch with message exists and if not create one
        if Attesters::get_latest_batch_to_sign_message(target).is_none() {
            let _current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            for message in messages {
                let sfx_id_a = H256::from(message);
                let _ = Attesters::request_sfx_attestation_commit(target, sfx_id_a, None);
            }

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);
        }
        let latest_batch_hash = Attesters::get_latest_batch_to_sign_hash(target).unwrap();

        let signature: Vec<u8> = match key_type {
            ECDSA_ATTESTER_KEY_TYPE_ID => ecdsa::Pair::from_seed(&secret_key)
                .sign_prehashed(&latest_batch_hash.0)
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
            RuntimeOrigin::signed(attester),
            latest_batch_hash,
            signature.clone(),
            target,
        ));

        (latest_batch_hash, signature)
    }

    #[test]
    fn submitting_attestation_reads_as_on_time_latency_status() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            // Register an attester
            let attester = AccountId::from([1; 32]);
            let _attester_info = register_attester_with_single_private_key([1u8; 32]);
            // Submit an attestation signed with the Ed25519 key
            let sfx_id_to_sign_on: [u8; 32] = *b"message_that_needs_attestation32";
            let (_hash, _signature) = sign_and_submit_sfx_to_latest_attestation(
                attester,
                vec![sfx_id_to_sign_on],
                ECDSA_ATTESTER_KEY_TYPE_ID,
                ETHEREUM_TARGET,
                [1u8; 32],
            );

            let batch_latency = Attesters::read_attestation_latency(&ETHEREUM_TARGET);
            assert!(batch_latency.is_some());
            assert_eq!(batch_latency, Some(LatencyStatus::OnTime));
        });
    }

    #[test]
    fn submit_first_influx_attestation_with_correct_signature_sets_status_to_pending_attestation() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            // Register an attester
            let attester = AccountId::from([1; 32]);
            let _attester_info = register_attester_with_single_private_key([1u8; 32]);

            add_target_and_transition_to_next_batch(ETHEREUM_TARGET, 0);

            // Submit an attestation signed with the Ed25519 key
            let influx_message_32b = *b"message_that_needs_attestation32";
            let mut influx_message_to_sign_on: Vec<u8> = influx_message_32b.to_vec();
            let target_there: [u8; 4] = ETHEREUM_TARGET;
            let height_there: u32 = 8;
            // Add target and height to the message
            influx_message_to_sign_on.extend_from_slice(&target_there);
            influx_message_to_sign_on.extend_from_slice(&height_there.to_le_bytes());

            // Calculate the hash of the message
            let mut hasher = Keccak::v256();
            hasher.update(&influx_message_to_sign_on);
            let mut hash = [0u8; 32];
            hasher.finalize(&mut hash);
            let sfx_id_to_sign_on = H256::from(hash);

            // Sign the message
            let signature = ecdsa::Pair::from_seed(&[1u8; 32])
                .sign_prehashed(&sfx_id_to_sign_on.into())
                .encode();

            assert_ok!(Attesters::submit_for_influx_attestation(
                RuntimeOrigin::signed(attester),
                influx_message_32b.into(),
                sfx_id_to_sign_on,
                height_there,
                ETHEREUM_TARGET,
                signature.clone(),
            ));

            // current block
            assert_eq!(
                Attesters::attestations_influx(&ETHEREUM_TARGET, sfx_id_to_sign_on),
                Some(InfluxMessage {
                    message_hash: sfx_id_to_sign_on,
                    message: influx_message_32b.into(),
                    height_there,
                    gateway: ETHEREUM_TARGET,
                    signatures: vec![signature],
                    created: System::block_number(),
                    status: BatchStatus::PendingAttestation,
                })
            );
        });
    }

    #[test]
    fn submitting_attestation_reads_updates_batching_factor_overview() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            // Register an attester
            let attester = AccountId::from([1; 32]);
            let _attester_info = register_attester_with_single_private_key([1u8; 32]);
            // Submit an attestation signed with the Ed25519 key
            let sfx_id_to_sign_on: [u8; 32] = *b"message_that_needs_attestation32";
            let (_hash, _signature) = sign_and_submit_sfx_to_latest_attestation(
                attester,
                vec![sfx_id_to_sign_on],
                ECDSA_ATTESTER_KEY_TYPE_ID,
                ETHEREUM_TARGET,
                [1u8; 32],
            );

            assert_ok!(Attesters::read_latest_batching_factor_overview(
                RuntimeOrigin::signed(AccountId::from([1u8; 32]))
            ));

            let current_batching_factor = expect_last_event_to_emit_batching_factor_overview();

            assert_eq!(
                current_batching_factor,
                vec![(
                    ETHEREUM_TARGET,
                    Some(BatchingFactor {
                        latest_confirmed: 0,
                        latest_signed: 1,
                        current_next: 0,
                        up_to_last_10_confirmed: vec![]
                    })
                )]
            );
        });
    }

    #[test]
    fn register_and_submit_attestation_in_ecdsa() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
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
                vec![sfx_id_to_sign_on],
                ECDSA_ATTESTER_KEY_TYPE_ID,
                ETHEREUM_TARGET,
                [1u8; 32],
            );

            let latest_batch = Attesters::get_latest_batch_to_sign(ETHEREUM_TARGET);
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
    fn remove_and_add_back_attestation_targets_with_sudo_access() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;
            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            assert_eq!(Attesters::attestation_targets(), vec![target]);

            Attesters::remove_attestation_target(RuntimeOrigin::root(), target);

            assert_eq!(Attesters::attestation_targets(), Vec::<TargetId>::new());

            System::set_block_number(current_block_1 + 1);

            let _current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            assert_eq!(Attesters::attestation_targets(), vec![target]);
        });
    }

    #[test]
    fn double_attestation_is_not_allowed() {
        let target = ETHEREUM_TARGET;
        let _mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
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
                vec![sfx_id_to_sign_on],
                ECDSA_ATTESTER_KEY_TYPE_ID,
                target,
                [1u8; 32],
            );

            let same_signature_again = ecdsa::Pair::from_seed(&[1u8; 32])
                .sign_prehashed(&message_hash.0)
                .encode();

            assert_err!(
                Attesters::submit_attestation(
                    RuntimeOrigin::signed(attester),
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
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;
            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let sfx_id_a = H256::repeat_byte(1);
            let mut sfx_gmp_a = H512::zero();
            sfx_gmp_a[..32].copy_from_slice(&sfx_id_a[..]);

            assert_ok!(Attesters::request_sfx_attestation_commit(
                target, sfx_id_a, None
            ));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            assert_eq!(
                Attesters::get_batches(target, BatchStatus::PendingAttestation),
                vec![BatchMessage {
                    available_to_commit_at: 0,
                    committed_sfx: Some(vec![sfx_gmp_a]),
                    reverted_sfx: None,
                    next_committee: None,
                    banned_committee: None,
                    signatures: vec![],
                    status: BatchStatus::PendingAttestation,
                    created: current_block_1,
                    latency: LatencyStatus::OnTime,
                    index: 0,
                    halt: false,
                }]
            );
        });
    }

    #[test]
    fn test_successfull_process_repatriation_for_pending_attestation_with_one_fsx_reverted() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;
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
                .generate_id::<Keccak256>(mock_xtx_id.as_bytes(), 0u32);

            assert_ok!(Attesters::request_sfx_attestation_revert(target, sfx_id));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let pending_batches = Attesters::get_batches(target, BatchStatus::PendingAttestation);
            assert_eq!(pending_batches.len(), 1);
            let pending_batch = pending_batches[0].clone();
            assert_eq!(pending_batch.reverted_sfx, Some(vec![sfx_id]));
            assert_eq!(pending_batch.committed_sfx, None);
            assert_eq!(pending_batch.created, current_block_1);

            const SLASH_TREASURY_BALANCE: Balance = 100;
            let _balance = Balances::deposit_creating(
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
                    non_native_asset_id: None,
                }])
            );
        });
    }

    #[test]
    fn test_successfull_process_repatriation_for_pending_attestation_with_one_fsx() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;
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
                timeouts_at: AdaptiveTimeout::default_401(),
                speed_mode: Default::default(),
                delay_steps_at: None,
                status: CircuitStatus::Committed,
                steps_cnt: (0, 0),
            };

            let sfx_id = mock_fsx
                .input
                .generate_id::<Keccak256>(mock_xtx_id.as_bytes(), 0u32);

            let mut sfx_gmp = H512::zero();
            sfx_gmp[..32].copy_from_slice(&sfx_id[..]);

            assert_ok!(Attesters::request_sfx_attestation_commit(
                target, sfx_id, None
            ));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let pending_batches = Attesters::get_batches(target, BatchStatus::PendingAttestation);
            assert_eq!(pending_batches.len(), 1);
            let pending_batch = pending_batches[0].clone();

            assert_eq!(pending_batch.committed_sfx, Some(vec![sfx_gmp]));
            assert_eq!(pending_batch.reverted_sfx, None);
            assert_eq!(pending_batch.created, current_block_1);

            const SLASH_TREASURY_BALANCE: Balance = 100;
            let _balance = Balances::deposit_creating(
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
                    non_native_asset_id: None,
                }])
            );
        });
    }

    #[test]
    fn test_process_repatriation_changes_status_to_expired_after_repatriation_period_when_fsx_not_found(
    ) {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;
            let current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let sfx_id = H256([3u8; 32]);

            let mut sfx_gmp = H512::zero();
            sfx_gmp[..32].copy_from_slice(&sfx_id[..]);

            assert_ok!(Attesters::request_sfx_attestation_commit(
                target, sfx_id, None
            ));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let pending_batches = Attesters::get_batches(target, BatchStatus::PendingAttestation);
            assert_eq!(pending_batches.len(), 1);
            let pending_batch = pending_batches[0].clone();
            assert_eq!(pending_batch.committed_sfx, Some(vec![sfx_gmp]));
            assert_eq!(pending_batch.created, current_block_1);

            const SLASH_TREASURY_BALANCE: Balance = 100;
            let _balance = Balances::deposit_creating(
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

    fn expect_last_event_to_emit_batching_factor_overview(
    ) -> Vec<(TargetId, Option<BatchingFactor>)> {
        // Recover system event
        let events = System::events();
        let expect_desired_event = events.last();
        assert!(expect_desired_event.clone().is_some());

        match expect_desired_event {
            Some(event) => match &event.event {
                Event::Attesters(AttestersEvent::BatchingFactorRead(batching_factor_read)) =>
                    batching_factor_read.clone(),
                _ => panic!(
                    "expect_last_event_to_emit_batching_factor_overview: unexpected event type"
                ),
            },
            None =>
                panic!("expect_last_event_to_emit_batching_factor_overview: no last event emitted"),
        }
    }

    fn expect_last_event_to_emit_finality_fee_estimation() -> (TargetId, Balance) {
        // Recover system event
        let events = System::events();
        let expect_desired_event = events.last();
        assert!(expect_desired_event.clone().is_some());

        match expect_desired_event {
            Some(event) => match &event.event {
                Event::Attesters(AttestersEvent::UserFinalityFeeEstimated(
                    target,
                    finality_fee_estimation,
                )) => (*target, finality_fee_estimation.clone()),
                _ => panic!(
                    "expect_last_event_to_emit_finality_fee_estimation: unexpected event type"
                ),
            },
            None =>
                panic!("expect_last_event_to_emit_finality_fee_estimation: no last event emitted"),
        }
    }

    fn expect_last_event_to_emit_pending_attestation_batches() -> (TargetId, Vec<(u32, H256)>) {
        // Recover system event
        let events = System::events();
        let expect_pending_attestation_batches = events.last();
        assert!(expect_pending_attestation_batches.clone().is_some());

        match expect_pending_attestation_batches {
            Some(event) => match &event.event {
                Event::Attesters(AttestersEvent::CurrentPendingAttestationBatches(
                    target,
                    pending_batches,
                )) => (*target, pending_batches.clone()),
                _ => panic!(
                    "expect_last_event_to_emit_pending_attestation_batches: unexpected event type"
                ),
            },
            None => panic!(
                "expect_last_event_to_emit_pending_attestation_batches: no last event emitted"
            ),
        }
    }

    #[test]
    fn test_late_attestations_stop_committee_shuffle() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let _target: TargetId = ETHEREUM_TARGET;

            for counter in 1..33u8 {
                // Register an attester
                let _attester = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            select_new_committee();
            assert_eq!(
                CommitteeTransitionOn::<MiniRuntime>::get(ETHEREUM_TARGET),
                None
            );

            // Submit 20 attestations - not enough to reach the threshold of Majority Approval
            for counter in 1..22u8 {
                let attester = AccountId::from([counter; 32]);
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    vec![*b"message_that_needs_attestation32"],
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    ETHEREUM_TARGET,
                    [counter; 32],
                );
            }

            Attesters::read_pending_batches(RuntimeOrigin::signed(AccountId::from([1u8; 32])));
            let (target, pending_batches) = expect_last_event_to_emit_pending_attestation_batches();
            assert_eq!(target, ETHEREUM_TARGET);
            assert_eq!(pending_batches.len(), 1);

            // Get latest batch
            let latest_batch =
                Attesters::get_batches(ETHEREUM_TARGET, BatchStatus::PendingAttestation)
                    .last()
                    .unwrap()
                    .clone();

            assert_eq!(latest_batch.signatures.len(), 21);

            select_new_committee();

            assert_eq!(
                CommitteeTransitionOn::<MiniRuntime>::get(ETHEREUM_TARGET),
                Some(1_u32)
            );

            transition_to_next_batch(ETHEREUM_TARGET, 2);
            // Get the same batch by message
            let first_pending_batch =
                Attesters::get_batch_by_message(ETHEREUM_TARGET, latest_batch.message()).unwrap();

            // Keeps the attested signatures
            assert_eq!(first_pending_batch.signatures.len(), 21);

            // Submit 20 attestations - not enough to reach the threshold of Majority Approval
            for counter in 1..22u8 {
                let attester = AccountId::from([counter; 32]);
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    vec![*b"message_that_needs_attestation32"],
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    ETHEREUM_TARGET,
                    [counter; 32],
                );
            }

            Attesters::read_pending_batches(RuntimeOrigin::signed(AccountId::from([1u8; 32])));
            let (target, pending_batches) = expect_last_event_to_emit_pending_attestation_batches();
            assert_eq!(target, ETHEREUM_TARGET);
            assert_eq!(pending_batches.len(), 1);

            // Get latest batch to sign
            let second_pending_batch =
                Attesters::get_latest_batch_to_sign(ETHEREUM_TARGET).unwrap();

            assert_eq!(second_pending_batch.signatures.len(), 21);
            assert!(second_pending_batch.index != first_pending_batch.index);

            select_new_committee();

            // The committee shuffle should be stopped
            assert_eq!(
                CommitteeTransitionOn::<MiniRuntime>::get(ETHEREUM_TARGET),
                Some(1_u32)
            );

            transition_to_next_batch(ETHEREUM_TARGET, 2);

            // Get the same batch by message
            let second_batch_again_by_message =
                Attesters::get_batch_by_message(ETHEREUM_TARGET, second_pending_batch.message())
                    .unwrap();

            assert_eq!(second_batch_again_by_message.signatures.len(), 21);

            // Sign both of the late batches now by adding 1 missing attestation to each of them
            let late_attester = AccountId::from([22u8; 32]);
            let late_first_signature = ecdsa::Pair::from_seed(&[22u8; 32])
                .sign_prehashed(&first_pending_batch.message_hash().0)
                .encode();

            assert_ok!(Attesters::submit_attestation(
                RuntimeOrigin::signed(late_attester.clone()),
                first_pending_batch.message_hash(),
                late_first_signature,
                ETHEREUM_TARGET,
            ));

            let late_second_signature = ecdsa::Pair::from_seed(&[22u8; 32])
                .sign_prehashed(&second_pending_batch.message_hash().0)
                .encode();

            assert_ok!(Attesters::submit_attestation(
                RuntimeOrigin::signed(late_attester),
                second_pending_batch.message_hash(),
                late_second_signature,
                ETHEREUM_TARGET,
            ));

            // this time should be successful - no pending batches
            select_new_committee();
            Attesters::read_pending_batches(RuntimeOrigin::signed(AccountId::from([1u8; 32])));
            let (target, pending_batches) = expect_last_event_to_emit_pending_attestation_batches();
            assert_eq!(target, ETHEREUM_TARGET);
            assert_eq!(pending_batches.len(), 0);

            assert_eq!(
                CommitteeTransitionOn::<MiniRuntime>::get(ETHEREUM_TARGET),
                Some(2_u32)
            );
        });
    }

    #[test]
    fn test_process_repatriation_changes_status_to_expired_after_repatriation_period_when_no_batch_fsx_required(
    ) {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;

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
            let _balance = Balances::deposit_creating(
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
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;
            let _current_block_1 = add_target_and_transition_to_next_batch(target, 0);

            let sfx_id_a = H256::repeat_byte(1);
            let mut sfx_gmp_a = H512::zero();
            sfx_gmp_a[..32].copy_from_slice(&sfx_id_a[..]);

            assert_ok!(Attesters::request_sfx_attestation_commit(
                target, sfx_id_a, None
            ));

            let _current_block_2 = add_target_and_transition_to_next_batch(target, 1);

            let (_message_hash, expected_message_bytes) =
                calculate_hash_for_sfx_message(sfx_gmp_a.encode(), 0);

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
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;
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
            assert_ok!(Attesters::request_sfx_attestation_commit(
                target, sfx_id_a, None
            ));

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

            let mut sfx_gmp = H512::zero();
            sfx_gmp[..32].copy_from_slice(&sfx_id_a[..]);

            assert_eq!(
                Attesters::get_latest_batch_to_sign(target),
                Some(BatchMessage {
                    available_to_commit_at: 0,
                    committed_sfx: Some(vec![sfx_gmp]),
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
                    halt: false,
                })
            );

            assert_eq!(
                Attesters::get_latest_batch_to_sign_message(target),
                Some(vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 100, 47, 14, 60, 58, 245, 69, 231, 172,
                    189, 56, 176, 114, 81, 179, 153, 9, 20, 241, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 80, 80, 164, 244, 179, 249, 51, 140, 52, 114, 220, 192, 26, 135, 199, 106,
                    20, 75, 60, 156, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 37, 167, 132, 37, 241,
                    122, 126, 72, 126, 181, 102, 107, 43, 253, 147, 171, 176, 108, 112, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 196, 139, 129, 43, 180, 52, 1, 57, 44, 3, 115, 129,
                    172, 169, 52, 244, 6, 156, 5, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 208, 154,
                    209, 64, 128, 212, 178, 87, 168, 25, 164, 245, 121, 184, 72, 91, 232, 143, 8,
                    108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 176, 48, 209, 26, 139, 228, 139,
                    96, 65, 136, 87, 135, 77, 238, 230, 29, 16, 113, 224, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 74, 98, 49, 102, 35, 173, 69, 127, 2, 205, 197, 217, 151, 222, 214,
                    122, 56, 62, 197, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 200, 81, 234,
                    163, 195, 151, 105, 20, 214, 59, 130, 44, 103, 226, 1, 236, 11, 251, 184, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 88, 218, 153, 10, 143, 74, 58, 108, 167, 203, 99,
                    21, 214, 138, 20, 1, 5, 145, 115, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 193,
                    113, 3, 61, 92, 191, 247, 23, 95, 41, 223, 211, 166, 61, 218, 61, 111, 143, 56,
                    94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 242, 136, 236, 175, 21, 121, 14, 252,
                    172, 82, 137, 70, 150, 58, 109, 184, 195, 248, 33, 29, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 99, 70, 123, 2, 167, 56, 36, 8, 168, 69, 165, 235, 133, 181, 35,
                    139, 138, 77, 208, 237, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 156, 120, 75,
                    147, 204, 180, 64, 249, 29, 197, 19, 44, 116, 169, 83, 25, 73, 125, 244, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 129, 161, 247, 202, 26, 64, 224, 4, 216, 227,
                    205, 205, 183, 38, 58, 173, 217, 206, 26, 243, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 105, 26, 141, 5, 103, 143, 201, 98, 255, 15, 33, 116, 19, 67, 121, 192, 5,
                    28, 182, 134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 239, 4, 90, 85, 76, 187, 0,
                    22, 39, 94, 144, 227, 0, 47, 77, 33, 198, 242, 99, 225, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 25, 231, 227, 118, 231, 194, 19, 183, 231, 231, 228, 108, 199, 10,
                    93, 208, 134, 218, 255, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 90, 119,
                    217, 250, 126, 244, 102, 149, 27, 47, 1, 247, 36, 188, 163, 165, 130, 11, 99,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 161, 187, 166, 11, 90, 163, 112, 148,
                    207, 22, 18, 58, 221, 103, 76, 1, 88, 148, 136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 30, 50, 171, 207, 230, 219, 21, 193, 87, 7, 9, 227, 252, 2, 114, 83, 53,
                    245, 10, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 224, 245, 57, 227, 27, 53,
                    23, 15, 170, 160, 98, 175, 112, 59, 118, 168, 40, 43, 247, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 161, 67, 201, 108, 74, 81, 135, 115, 67, 196, 221, 119, 253, 98,
                    88, 79, 215, 242, 93, 180, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 159, 180,
                    15, 102, 196, 225, 50, 250, 94, 100, 228, 159, 48, 126, 2, 183, 101, 64, 248,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 215, 231, 83, 158, 167, 75, 228, 250, 203,
                    88, 197, 12, 206, 191, 6, 96, 127, 241, 148, 205, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 4, 146, 155, 184, 96, 118, 224, 159, 36, 139, 37, 73, 49, 73, 30, 54, 29,
                    59, 78, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 239, 75, 112, 1, 58, 93, 39,
                    218, 97, 215, 26, 215, 22, 254, 18, 148, 247, 72, 209, 82, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 177, 167, 217, 66, 140, 229, 200, 14, 37, 78, 101, 251, 225,
                    188, 248, 47, 100, 123, 93, 238, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 80,
                    41, 10, 165, 39, 129, 221, 108, 66, 85, 70, 123, 187, 235, 159, 119, 60, 25,
                    54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 10, 58, 110, 64, 197, 45, 43, 126,
                    50, 236, 207, 254, 100, 60, 77, 157, 170, 187, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 235, 230, 196, 217, 170, 160, 118, 159, 4, 105, 143, 152, 109, 26,
                    198, 229, 234, 199, 108, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168, 139, 113,
                    15, 175, 255, 104, 227, 215, 187, 75, 61, 215, 44, 53, 139, 91, 219, 154, 24,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 182, 230, 16, 146, 27, 10, 15, 111, 96,
                    140, 14, 31, 41, 168, 69, 85, 43, 198, 219, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 51, 37, 167, 132, 37, 241, 122, 126, 72, 126, 181, 102, 107, 43, 253,
                    147, 171, 176, 108, 112, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ])
            );

            assert_eq!(
                Attesters::get_latest_batch_to_sign_hash(target),
                Some(
                    hex_literal::hex!(
                        "42948d1310be9fed9894dc00cd7661b05fe3e048a9095823ecbb21e1a2a95c93"
                    )
                    .into()
                )
            );
        });
    }

    #[test]
    fn test_pending_attestation_batch_with_all_attestations_ordered_yields_correct_message_hash() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;

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
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 100, 47, 14, 60, 58, 245, 69, 231, 172,
                189, 56, 176, 114, 81, 179, 153, 9, 20, 241, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                80, 80, 164, 244, 179, 249, 51, 140, 52, 114, 220, 192, 26, 135, 199, 106, 20, 75,
                60, 156, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 37, 167, 132, 37, 241, 122, 126,
                72, 126, 181, 102, 107, 43, 253, 147, 171, 176, 108, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 196, 139, 129, 43, 180, 52, 1, 57, 44, 3, 115, 129, 172, 169, 52, 244,
                6, 156, 5, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 208, 154, 209, 64, 128, 212,
                178, 87, 168, 25, 164, 245, 121, 184, 72, 91, 232, 143, 8, 108, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 12, 176, 48, 209, 26, 139, 228, 139, 96, 65, 136, 87, 135, 77,
                238, 230, 29, 16, 113, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 98, 49, 102,
                35, 173, 69, 127, 2, 205, 197, 217, 151, 222, 214, 122, 56, 62, 197, 105, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 200, 81, 234, 163, 195, 151, 105, 20, 214, 59, 130,
                44, 103, 226, 1, 236, 11, 251, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 88, 218,
                153, 10, 143, 74, 58, 108, 167, 203, 99, 21, 214, 138, 20, 1, 5, 145, 115, 82, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 193, 113, 3, 61, 92, 191, 247, 23, 95, 41, 223,
                211, 166, 61, 218, 61, 111, 143, 56, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 242,
                136, 236, 175, 21, 121, 14, 252, 172, 82, 137, 70, 150, 58, 109, 184, 195, 248, 33,
                29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 70, 123, 2, 167, 56, 36, 8, 168, 69,
                165, 235, 133, 181, 35, 139, 138, 77, 208, 237, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                34, 156, 120, 75, 147, 204, 180, 64, 249, 29, 197, 19, 44, 116, 169, 83, 25, 73,
                125, 244, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 129, 161, 247, 202, 26, 64, 224, 4,
                216, 227, 205, 205, 183, 38, 58, 173, 217, 206, 26, 243, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 105, 26, 141, 5, 103, 143, 201, 98, 255, 15, 33, 116, 19, 67, 121, 192, 5,
                28, 182, 134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 239, 4, 90, 85, 76, 187, 0, 22,
                39, 94, 144, 227, 0, 47, 77, 33, 198, 242, 99, 225, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 25, 231, 227, 118, 231, 194, 19, 183, 231, 231, 228, 108, 199, 10, 93, 208,
                134, 218, 255, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 90, 119, 217, 250, 126,
                244, 102, 149, 27, 47, 1, 247, 36, 188, 163, 165, 130, 11, 99, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 3, 161, 187, 166, 11, 90, 163, 112, 148, 207, 22, 18, 58, 221, 103,
                76, 1, 88, 148, 136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 50, 171, 207, 230,
                219, 21, 193, 87, 7, 9, 227, 252, 2, 114, 83, 53, 245, 10, 71, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 51, 224, 245, 57, 227, 27, 53, 23, 15, 170, 160, 98, 175, 112, 59,
                118, 168, 40, 43, 247, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 161, 67, 201, 108, 74,
                81, 135, 115, 67, 196, 221, 119, 253, 98, 88, 79, 215, 242, 93, 180, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 126, 159, 180, 15, 102, 196, 225, 50, 250, 94, 100, 228, 159,
                48, 126, 2, 183, 101, 64, 248, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 215, 231, 83,
                158, 167, 75, 228, 250, 203, 88, 197, 12, 206, 191, 6, 96, 127, 241, 148, 205, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 146, 155, 184, 96, 118, 224, 159, 36, 139, 37,
                73, 49, 73, 30, 54, 29, 59, 78, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 239, 75,
                112, 1, 58, 93, 39, 218, 97, 215, 26, 215, 22, 254, 18, 148, 247, 72, 209, 82, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 177, 167, 217, 66, 140, 229, 200, 14, 37, 78, 101,
                251, 225, 188, 248, 47, 100, 123, 93, 238, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
                80, 41, 10, 165, 39, 129, 221, 108, 66, 85, 70, 123, 187, 235, 159, 119, 60, 25,
                54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 10, 58, 110, 64, 197, 45, 43, 126, 50,
                236, 207, 254, 100, 60, 77, 157, 170, 187, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                235, 230, 196, 217, 170, 160, 118, 159, 4, 105, 143, 152, 109, 26, 198, 229, 234,
                199, 108, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168, 139, 113, 15, 175, 255, 104,
                227, 215, 187, 75, 61, 215, 44, 53, 139, 91, 219, 154, 24, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 182, 230, 16, 146, 27, 10, 15, 111, 96, 140, 14, 31, 41, 168, 69, 85,
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
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;
            NextBatch::<MiniRuntime>::insert(target, BatchMessage::default());

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation_commit(
                target, sfx_id_a, None
            ));

            assert_noop!(
                Attesters::request_sfx_attestation_commit(target, sfx_id_a, None),
                "SfxAlreadyRequested",
            );
        });
    }

    #[test]
    fn test_adding_2_sfx_to_next_batch_and_transition_to_pending_attestation() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();
        ext.execute_with(|| {
            let target: TargetId = ETHEREUM_TARGET;

            AttestationTargets::<MiniRuntime>::put(vec![target]);
            assert_eq!(NextBatch::<MiniRuntime>::get(target), None);
            NextBatch::<MiniRuntime>::insert(target, BatchMessage::default());

            let sfx_id_a = H256::repeat_byte(1);
            assert_ok!(Attesters::request_sfx_attestation_commit(
                target, sfx_id_a, None
            ));

            let mut sfx_gmp_a = H512::zero();
            sfx_gmp_a[..32].copy_from_slice(&sfx_id_a[..]);

            // Verify that the attestation is added to the next batch
            let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
            assert_eq!(next_batch.committed_sfx, Some(vec![sfx_gmp_a]));

            // Add another SFX to the next batch
            let sfx_id_b = H256::repeat_byte(2);
            assert_ok!(Attesters::request_sfx_attestation_commit(
                target, sfx_id_b, None
            ));
            let mut sfx_gmp_b = H512::zero();
            sfx_gmp_b[..32].copy_from_slice(&sfx_id_b[..]);
            let next_batch = NextBatch::<MiniRuntime>::get(target).unwrap();
            assert_eq!(next_batch.committed_sfx, Some(vec![sfx_gmp_a, sfx_gmp_b]));

            let mut empty_batch = BatchMessage {
                available_to_commit_at: 0,
                committed_sfx: None,
                reverted_sfx: None,
                next_committee: None,
                banned_committee: None,
                signatures: vec![],
                status: BatchStatus::PendingMessage,
                created: 0,
                latency: LatencyStatus::OnTime,
                index: 0,
                halt: false,
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
                    available_to_commit_at: 0,
                    committed_sfx: Some(vec![sfx_gmp_a, sfx_gmp_b]),
                    reverted_sfx: None,
                    next_committee: None,
                    banned_committee: None,
                    signatures: vec![],
                    status: BatchStatus::PendingAttestation,
                    created: 0,
                    latency: LatencyStatus::OnTime,
                    index: 0,
                    halt: false,
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
            .with_standard_sfx_abi()
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
            // Trigger the committee first setup
            select_new_committee();
            add_target_and_transition_to_next_batch(ETHEREUM_TARGET, 0);

            // Check if the committee is set up and has the correct size
            let committee = CurrentCommittee::<MiniRuntime>::get();
            let committee_size: u32 = <MiniRuntime as ConfigAttesters>::CommitteeSize::get();
            assert_eq!(committee.len(), committee_size as usize);

            // Check that each member of the committee is in the registered attesters
            for member in &committee {
                assert!(AttestersStore::<MiniRuntime>::contains_key(member));
            }

            // Expect NextBatch message to have committee transition awaiting attestation on registered target
            let next_batch = NextBatch::<MiniRuntime>::get(ETHEREUM_TARGET).unwrap();
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
            add_target_and_transition_to_next_batch(ETHEREUM_TARGET, 1);

            let batch_0 =
                Attesters::get_batch_by_message_hash(ETHEREUM_TARGET, batch_0_hash).unwrap();
            assert_eq!(batch_0.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_0.latency, LatencyStatus::OnTime);

            // Next batch should mark the initial batch as late, but don't modify the batch_1 since there's no new messages to attest for
            add_target_and_transition_to_next_batch(ETHEREUM_TARGET, 1);
            let batch_0 =
                Attesters::get_batch_by_message_hash(ETHEREUM_TARGET, batch_0_hash).unwrap();
            assert_eq!(batch_0.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_0.latency, LatencyStatus::Late(1, 0));

            let _committee_0_on_target =
                NextCommitteeOnTarget::<MiniRuntime>::get(ETHEREUM_TARGET).unwrap();

            add_target_and_transition_to_next_batch(ETHEREUM_TARGET, 1);
            let batch_0 =
                Attesters::get_batch_by_message_hash(ETHEREUM_TARGET, batch_0_hash).unwrap();
            assert_eq!(batch_0.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_0.latency, LatencyStatus::Late(2, 0));
            // Trigger the next committee transition
            select_new_committee();
            // Retreive next batch
            let batch_1 = NextBatch::<MiniRuntime>::get(ETHEREUM_TARGET).unwrap();
            assert!(batch_1.next_committee.is_some());
            assert_eq!(batch_1.index, 1);
            let _committee_1_on_target =
                NextCommitteeOnTarget::<MiniRuntime>::get(ETHEREUM_TARGET).unwrap();
            let batch_1_hash = batch_1.message_hash();
            // todo: fix the randomness source on mini-mock (yields 0)
            // assert!(committee_0_on_target != committee_1_on_target);
            add_target_and_transition_to_next_batch(ETHEREUM_TARGET, 2);
            let batch_0 =
                Attesters::get_batch_by_message_hash(ETHEREUM_TARGET, batch_0_hash).unwrap();
            assert_eq!(batch_0.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_0.latency, LatencyStatus::Late(3, 0));
            let batch_1 =
                Attesters::get_batch_by_message_hash(ETHEREUM_TARGET, batch_1_hash).unwrap();
            assert_eq!(batch_1.status, BatchStatus::PendingAttestation);
            assert_eq!(batch_1.latency, LatencyStatus::OnTime);
        });
    }

    #[test]
    fn committee_setup_and_transition() {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
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
            // todo: RandomnessCollectiveFlip always returns 0000...0000 as random value
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
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            let sfx_id_to_sign_on: [u8; 32] = *b"message_that_needs_attestation32";
            let (_message_hash, _expected_message_bytes) =
                calculate_hash_for_sfx_message(sfx_id_to_sign_on.encode(), 0);

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
                    vec![sfx_id_to_sign_on],
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    ETHEREUM_TARGET,
                    [counter; 32],
                );
            }
            assert_eq!(
                Attesters::get_batches(
                    ETHEREUM_TARGET,
                    BatchStatus::ReadyForSubmissionFullyApproved
                )
                .len(),
                1
            );
        });
    }

    #[test]
    fn register_and_submit_21x_attestations_in_ecdsa_changes_status_to_approved_in_next_batching_window(
    ) {
        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_eth_gateway_record()
            .build();

        ext.execute_with(|| {
            let message: [u8; 32] = *b"message_that_needs_attestation32";
            let (_message_hash, _expected_message_bytes) =
                calculate_hash_for_sfx_message(message.encode(), 0);

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
                    vec![message],
                    ECDSA_ATTESTER_KEY_TYPE_ID,
                    ETHEREUM_TARGET,
                    [counter; 32],
                );
            }

            let batch = Attesters::get_latest_batch_to_sign(ETHEREUM_TARGET)
                .expect("get_latest_batch_to_sign should return a batch");

            assert_eq!(batch.status, BatchStatus::PendingAttestation);

            // Trigger batching transition
            add_target_and_transition_to_next_batch(ETHEREUM_TARGET, 1);
            let batch = Attesters::get_batch_by_message(ETHEREUM_TARGET, batch.message())
                .expect("get_batch_by_message should return a batch");
            assert_eq!(batch.status, BatchStatus::ReadyForSubmissionByMajority);
        });
    }

    fn calculate_hash_for_sfx_message(message: Vec<u8>, index: u32) -> ([u8; 32], Vec<u8>) {
        let mut message_bytes: Vec<u8> = Vec::new();
        message_bytes.extend_from_slice(message.as_slice());
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
        let target: TargetId = POLKADOT_TARGET;
        let _mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_polkadot_gateway_record()
            .build();

        ext.execute_with(|| {
            let message: [u8; 32] = *b"message_that_needs_attestation32";
            let (_message_hash, _message_bytes) =
                calculate_hash_for_sfx_message(message.encode(), 0);

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
                let (_message_hash, _signature) = sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    vec![message],
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

            let submitter = AccountId::from([1; 32]);
            // Recover submitter's agreed target eth address
            let submitter_eth_address =
                AttestersAgreements::<MiniRuntime>::get(&submitter, &target)
                    .expect("attester eth address should exist");

            let mock_valid_batch_confirmation = TargetBatchDispatchEvent {
                hash: first_batch_hash,
                executor_on_target: H160::from_slice(&submitter_eth_address),
            };

            let delay = System::block_number() - first_batch.available_to_commit_at;
            let estimated_finality_reward_payout =
                Attesters::estimate_finality_reward(&target, delay);

            const FEE_TREASURY_BALANCE: Balance = 1_000_000_000_000_000_000_000_000_000_000;
            let _bal = Balances::deposit_creating(
                &MiniRuntime::get_treasury_account(TreasuryAccount::Fee),
                FEE_TREASURY_BALANCE,
            );

            let balance_of_submitter_prior = Balances::free_balance(AccountId::from([1; 32]));

            assert_eq!(PaidFinalityFees::<MiniRuntime>::get(&target), None);

            // Commit the batch
            assert_ok!(Attesters::commit_batch(
                RuntimeOrigin::signed(submitter),
                target,
                mock_valid_batch_confirmation.encode(),
            ));

            // Check if the batch status has been updated to Committed
            let batch = Attesters::get_batch_by_message(target, first_batch_message)
                .expect("Batch by message should exist");
            assert_eq!(batch.status, BatchStatus::Committed);

            assert_eq!(
                Balances::free_balance(AccountId::from([1; 32])),
                balance_of_submitter_prior + estimated_finality_reward_payout
            );

            assert_eq!(
                PaidFinalityFees::<MiniRuntime>::get(&target),
                Some(vec![estimated_finality_reward_payout])
            );
        });
    }

    #[test]
    fn register_and_submit_32x_attestations_and_check_collusion_permanent_slash() {
        let target: TargetId = ETHEREUM_TARGET;
        let _mock_escrow_account: AccountId = AccountId::new([2u8; 32]);

        let mut ext = ExtBuilder::default()
            .with_standard_sfx_abi()
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
                let _ = AccountId::from([counter; 32]);
                register_attester_with_single_private_key([counter; 32]);
            }

            select_new_committee();

            for counter in 1..33u8 {
                // Register an attester
                let attester = AccountId::from([counter; 32]);
                // Submit an attestation signed with the Ed25519 key
                sign_and_submit_sfx_to_latest_attestation(
                    attester,
                    vec![message],
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
                hash: colluded_message.into(),
                executor_on_target: H160::from([1; 20]),
            };

            assert_ok!(
                Attesters::commit_batch(
                    RuntimeOrigin::signed(AccountId::from([1; 32])),
                    target,
                    colluded_batch_confirmation.encode(),
                ) // errors are emitted, but the call must be successfull to enact slash. AttestersError::<MiniRuntime>::CollusionWithPermanentSlashDetected // AttestersError::<MiniRuntime>::CollusionWithPermanentSlashDetected
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
                let _ = Balances::deposit_creating(&nominator, amount + ExistentialDeposit::get());
                assert_ok!(Attesters::nominate(
                    RuntimeOrigin::signed(nominator.clone()),
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
                let _ = Balances::deposit_creating(&nominator, amount + ExistentialDeposit::get());
                assert_ok!(Attesters::nominate(
                    RuntimeOrigin::signed(nominator.clone()),
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
                let _ = Balances::deposit_creating(&nominator, amount + ExistentialDeposit::get());
                assert_ok!(Attesters::nominate(
                    RuntimeOrigin::signed(nominator.clone()),
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
                            benefit_source: BenefitSource::Inflation,
                            non_native_asset_id: None,
                        },
                        ClaimableArtifacts {
                            beneficiary: attester,
                            role: CircuitRole::Staker,
                            total_round_claim: 8, // that's reward as a self-bonded staker
                            benefit_source: BenefitSource::Inflation,
                            non_native_asset_id: None,
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
                        non_native_asset_id: None,
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
            let _ = Balances::deposit_creating(&nominator, 3000 + ExistentialDeposit::get());

            for attester in &attesters {
                assert_ok!(Attesters::nominate(
                    RuntimeOrigin::signed(nominator.clone()),
                    attester.clone(),
                    1000
                ));
            }

            // Unnominate one attester
            let attester_to_unnominate = attesters[1].clone();
            assert_ok!(Attesters::unnominate(
                RuntimeOrigin::signed(nominator.clone()),
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
                        RuntimeOrigin::signed(nominator.clone()),
                        attester.clone(),
                        1000 + i as Balance
                    ));
                }
            }

            // Unnominate one attester
            let attester_to_unnominate = attesters[1].clone();
            assert_ok!(Attesters::unnominate(
                RuntimeOrigin::signed(nominator),
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

    #[test]
    fn test_filled_message_produces_expected_hash() {
        use hex_literal::hex;
        let filled_batch = BatchMessage {
            available_to_commit_at: 0,
            committed_sfx: Some(vec![
                hex!("6e906f8388de8faea67a770476ade4b76654545002126aa3ea17890fd8acdd7e0000000000000000000000000000000000000000000000000000000000000000").into(),
                hex!("580032f247eebb5c75889ab42c43dd88a1071c3950f9bbab1f901c47d5331dfa0000000000000000000000000000000000000000000000000000000000000000").into(),
                hex!("e23ab05c5ca561870b6f55d3fcb94ead2b14d8ce49ccf159b8e3449cbd5050c60000000000000000000000000000000000000000000000000000000000000000").into(),
            ]),
            reverted_sfx: Some(vec![
                hex!("ff17743a6b48933b94f38f423b15b2fc9ebcd34aab19bd81c2a69d3d052f467f").into(),
                hex!("21e5cd2c2f3e32ac4a52543a386821b079711432c2fefd4be3836ed36d129b11").into(),
            ]),
            next_committee: Some(vec![
                hex!("2b7A372d58541c3053793f022Cf28ef971F94EFA").into(),
                hex!("60eA580734420A9C23E51C7FdF455b5e0237E07C").into(),
                hex!("98DF91EF04A5C0695f8050B7Da4facC0E7d9444e").into(),
                hex!("3Cfbc429d7435fD5707390362c210bD272baE8eA").into(),
                hex!("66ed579D14Cbad8dFC352a3cEaeeE9711ea65e41").into(),
                hex!("786402fa462909785A55Ced48aa5682D99902C57").into(),
                hex!("401b7Cb06493eFDB82818F14f9Cd345C01463a81").into(),
                hex!("A2E7607A23B5A744A10a096c936AB033866D3bEe").into(),
                hex!("ac9c643B32916EA52e0fA0c3a3bBdbE120E5CA9e").into(),
                hex!("D53d6Af58A2bD8c0f86b25B1309c91f61700144F").into(),
                hex!("2feF1f5268d9732CAc331785987d45Fad487fcd6").into(),
                hex!("debc7A55486DbaCB06985ba2415b784e05a35baE").into(),
                hex!("d7b33a07Ee05B604138f94335405b55e2b6bbFdD").into(),
                hex!("1831c8F78C8b59c1300B79E308BfBf9e4fDd13B0").into(),
                hex!("361134E27Af99A288714E428C290d48F82a4895C").into(),
                hex!("5897B47E1357eD81B2D85d8f287759502E33f588").into(),
                hex!("a880bf7e031ed87d422D31BEBcC9D0339c7b95b4").into(),
                hex!("edaB03983D839E6A3a887c3Ee711a724391F8eE1").into(),
                hex!("80D80649e13268382ceA3b0a56a57078c2076fE1").into(),
                hex!("b0DE4907432a9A4aC92F4988dAa6024CD57D1b27").into(),
                hex!("5449D051328dA4cfE8d1eFe7481Ff3B690cF8696").into(),
                hex!("4705522d19458a90F06a15d9836A64e45c182c9f").into(),
                hex!("B6dE743a22A7A43Edda8b5E21E2f0Aeb70354f5B").into(),
                hex!("970c0720316BC03Cd055C5Ec74208Fe0BA3d3c44").into(),
                hex!("7905754a5B6A28D1EDf338d9Be06a49aD60D74b6").into(),
                hex!("93054A6f5eb0E1978D1e3e27AE758F17480E5988").into(),
                hex!("a185b4f947A09286FC028B034f01bAbe53d98301").into(),
                hex!("14C74Ce14e833d76dC0190651C0EbA64f3E67c79").into(),
                hex!("861fa47e5229C9079d087D6354C1Ede95D233F43").into(),
                hex!("6f9925AceFfbe67742257abFf393B123010c4A10").into(),
                hex!("A1Ea906c54379032c9857139C6f796Acf88dDb79").into(),
                hex!("6219f12779268F8A7ddf0f1E44Fd75253219d639").into(),
            ]),
            banned_committee: Some(vec![
                hex!("2b7A372d58541c3053793f022Cf28ef971F94EFA").into(),
                hex!("60eA580734420A9C23E51C7FdF455b5e0237E07C").into(),
                hex!("98DF91EF04A5C0695f8050B7Da4facC0E7d9444e").into(),
            ]),
            index: 1,
            signatures: vec![],
            created: 0, //(),
            status: BatchStatus::PendingMessage,
            latency: Default::default(),
            halt: false,
        };

        let msg = filled_batch.message();
        let msg_as_hex = hex::encode(msg);
        assert_eq!(msg_as_hex, "0000000000000000000000002b7a372d58541c3053793f022cf28ef971f94efa00000000000000000000000060ea580734420a9c23e51c7fdf455b5e0237e07c00000000000000000000000098df91ef04a5c0695f8050b7da4facc0e7d9444e0000000000000000000000003cfbc429d7435fd5707390362c210bd272bae8ea00000000000000000000000066ed579d14cbad8dfc352a3ceaeee9711ea65e41000000000000000000000000786402fa462909785a55ced48aa5682d99902c57000000000000000000000000401b7cb06493efdb82818f14f9cd345c01463a81000000000000000000000000a2e7607a23b5a744a10a096c936ab033866d3bee000000000000000000000000ac9c643b32916ea52e0fa0c3a3bbdbe120e5ca9e000000000000000000000000d53d6af58a2bd8c0f86b25b1309c91f61700144f0000000000000000000000002fef1f5268d9732cac331785987d45fad487fcd6000000000000000000000000debc7a55486dbacb06985ba2415b784e05a35bae000000000000000000000000d7b33a07ee05b604138f94335405b55e2b6bbfdd0000000000000000000000001831c8f78c8b59c1300b79e308bfbf9e4fdd13b0000000000000000000000000361134e27af99a288714e428c290d48f82a4895c0000000000000000000000005897b47e1357ed81b2d85d8f287759502e33f588000000000000000000000000a880bf7e031ed87d422d31bebcc9d0339c7b95b4000000000000000000000000edab03983d839e6a3a887c3ee711a724391f8ee100000000000000000000000080d80649e13268382cea3b0a56a57078c2076fe1000000000000000000000000b0de4907432a9a4ac92f4988daa6024cd57d1b270000000000000000000000005449d051328da4cfe8d1efe7481ff3b690cf86960000000000000000000000004705522d19458a90f06a15d9836a64e45c182c9f000000000000000000000000b6de743a22a7a43edda8b5e21e2f0aeb70354f5b000000000000000000000000970c0720316bc03cd055c5ec74208fe0ba3d3c440000000000000000000000007905754a5b6a28d1edf338d9be06a49ad60d74b600000000000000000000000093054a6f5eb0e1978d1e3e27ae758f17480e5988000000000000000000000000a185b4f947a09286fc028b034f01babe53d9830100000000000000000000000014c74ce14e833d76dc0190651c0eba64f3e67c79000000000000000000000000861fa47e5229c9079d087d6354c1ede95d233f430000000000000000000000006f9925aceffbe67742257abff393b123010c4a10000000000000000000000000a1ea906c54379032c9857139c6f796acf88ddb790000000000000000000000006219f12779268f8a7ddf0f1e44fd75253219d6390000000000000000000000002b7a372d58541c3053793f022cf28ef971f94efa00000000000000000000000060ea580734420a9c23e51c7fdf455b5e0237e07c00000000000000000000000098df91ef04a5c0695f8050b7da4facc0e7d9444e6e906f8388de8faea67a770476ade4b76654545002126aa3ea17890fd8acdd7e0000000000000000000000000000000000000000000000000000000000000000580032f247eebb5c75889ab42c43dd88a1071c3950f9bbab1f901c47d5331dfa0000000000000000000000000000000000000000000000000000000000000000e23ab05c5ca561870b6f55d3fcb94ead2b14d8ce49ccf159b8e3449cbd5050c60000000000000000000000000000000000000000000000000000000000000000ff17743a6b48933b94f38f423b15b2fc9ebcd34aab19bd81c2a69d3d052f467f21e5cd2c2f3e32ac4a52543a386821b079711432c2fefd4be3836ed36d129b1100000001");

        assert_eq!(
            filled_batch.message_hash(),
            hex!("f59f343a5c07f43e9a654e43ed9ce710005802f9e63a87111ed95072617008d0").into() // hex!("0e5ff1395ff4b94e02bad28b793efe3e27a32b3170191aae7a0a7c3c46a4a718").into()
        );
    }

    #[test]
    fn test_index_only_message_produces_expected_hash() {
        use hex_literal::hex;
        let filled_batch = BatchMessage {
            available_to_commit_at: 0,
            committed_sfx: Some(vec![]),
            reverted_sfx: Some(vec![]),
            next_committee: Some(vec![]),
            banned_committee: Some(vec![]),
            index: 1,
            signatures: vec![],
            created: 0, //(),
            status: BatchStatus::PendingMessage,
            latency: Default::default(),
            halt: false,
        };

        let msg = filled_batch.message();
        let msg_as_hex = hex::encode(msg);
        assert_eq!(msg_as_hex, "00000001");

        assert_eq!(
            filled_batch.message_hash(),
            hex!("51f81bcdfc324a0dff2b5bec9d92e21cbebc4d5e29d3a3d30de3e03fbeab8d7f").into() // hex!("0e5ff1395ff4b94e02bad28b793efe3e27a32b3170191aae7a0a7c3c46a4a718").into()
        );
    }
}
