#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub use crate::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    t3rn_primitives::reexport_currency_types!();

    use codec::{Decode, Encode};
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_application_crypto::RuntimePublic;
    use sp_core::{crypto::KeyTypeId, ecdsa, ed25519, sr25519};
    use sp_runtime::{traits::Zero, RuntimeAppPublic};
    use sp_std::{convert::TryInto, prelude::*};

    use sp_runtime::traits::Verify;

    // Key types for attester crypto
    pub const ECDSA_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"ecat");
    pub const ED25519_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"edat");
    pub const SR25519_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"srat");

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub enum AttestationFor {
        Xtx,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
    pub enum AttestationStatus {
        Pending,
        Timeout,
        Approved,
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
                for_: AttestationFor::Xtx,
                status: AttestationStatus::Pending,
                signature: Vec::new(),
            }
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type ActiveSetSize: Get<u32>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn attesters)]
    pub type Attesters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, AttesterInfo>;

    #[pallet::storage]
    #[pallet::getter(fn active_set)]
    pub type ActiveSet<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn sorted_nominated_attesters)]
    pub type SortedNominatedAttesters<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn attestations)]
    pub type Attestations<T: Config> =
        StorageMap<_, Identity, T::Hash, Attestation<T::AccountId, Vec<u8>>>;

    #[pallet::storage]
    #[pallet::getter(fn nominations)]
    pub type Nominations<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, (T::AccountId, BalanceOf<T>)>;

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
        NotRegistered,
        AlreadyNominated,
        MissingNominations,
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

            let is_verified = attester
                .verify_attestation_signature(ECDSA_ATTESTER_KEY_TYPE_ID, &message, &signature)
                .map_err(|_| Error::<T>::InvalidSignature)?;

            ensure!(is_verified, Error::<T>::AttestationSignatureInvalid);

            match attestation_for {
                AttestationFor::Xtx => {
                    let xtx_hash: T::Hash = Decode::decode(&mut &message[..])
                        .map_err(|_| Error::<T>::InvalidMessage)?;
                    let xtx_attestation = Attestations::<T>::get(xtx_hash).unwrap_or_default();

                    // Check if the attester has already signed the attestation
                    ensure!(
                        !xtx_attestation
                            .signature
                            .iter()
                            .any(|(attester, _)| attester == &account_id),
                        Error::<T>::AttestationDoubleSignAttempt
                    );

                    let signature_chain: Vec<(T::AccountId, Vec<u8>)> = xtx_attestation
                        .signature
                        .into_iter()
                        .chain(vec![(account_id.clone(), signature)])
                        .collect();

                    let status = if signature_chain.len() < T::ActiveSetSize::get() as usize {
                        AttestationStatus::Pending
                    } else {
                        AttestationStatus::Approved
                    };

                    Attestations::<T>::insert(
                        xtx_hash,
                        Attestation {
                            for_: attestation_for,
                            status,
                            signature: signature_chain,
                        },
                    );
                },
            }

            Self::deposit_event(Event::AttestationSubmitted(account_id));

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
        pub fn do_nominate(
            _nominator: T::AccountId,
            attester: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let mut nomination = match Nominations::<T>::get(&attester) {
                Some(nomination) => nomination,
                None => (attester.clone(), Zero::zero()),
            };

            nomination.1 += amount;

            // Update the nomination storage item
            Nominations::<T>::insert(&attester, nomination);

            // Update the sorted list of nominated attesters
            SortedNominatedAttesters::<T>::try_mutate(|attesters| {
                if let Some(index) = attesters.iter().position(|a| a == &attester) {
                    // Update the existing nomination amount
                    attesters[index] = attester.clone();
                } else {
                    // Add the new attester to the list
                    attesters.push(attester.clone());
                }

                // Sort the attesters by their nomination amount
                attesters.sort_unstable_by_key(|a| -> BalanceOf<T> {
                    Nominations::<T>::get(a)
                        .map(|n| n.1)
                        .and_then(|bal| BalanceOf::<T>::try_into(bal).ok())
                        .unwrap_or_else(Zero::zero)
                });

                // Keep only the top 32 attesters in the list
                attesters.truncate(32);

                Ok::<(), Error<T>>(())
            })?;

            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: T::BlockNumber) -> Weight {
            if (n % 400u32.into()).is_zero() {
                // Update the active set of attesters
                ActiveSet::<T>::put(
                    SortedNominatedAttesters::<T>::get()
                        .iter()
                        .take(32)
                        .cloned()
                        .collect::<Vec<T::AccountId>>(),
                );
                T::DbWeight::get().reads_writes(1, 1)
            } else {
                0
            }
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
    use frame_support::{assert_ok, traits::Hooks};
    use sp_application_crypto::{ecdsa, ed25519, sr25519, KeyTypeId, Pair, RuntimePublic};
    use sp_core::H256;
    use std::convert::TryInto;
    use t3rn_mini_mock_runtime::{
        AccountId, ActiveSet, AttestationFor, AttestationStatus, Attesters, AttestersError,
        Balance, ExtBuilder, MiniRuntime, Nominations, Origin, SortedNominatedAttesters,
    };

    fn register_attester_with_single_private_key(private_key: [u8; 32]) {
        // Register an attester
        let attester = AccountId::from(private_key);

        let secret_key = [1u8; 32];
        let ecdsa_key = ecdsa::Pair::from_seed(&secret_key).public().to_raw_vec();
        let ed25519_key = ed25519::Pair::from_seed(&secret_key).public().to_raw_vec();
        let sr25519_key = sr25519::Pair::from_seed(&secret_key).public().to_raw_vec();

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
            AttestationFor::Xtx,
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
            sign_and_submit_attestation(attester, message, ECDSA_ATTESTER_KEY_TYPE_ID, [1u8; 32]);

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
                [1u8; 32],
            );

            let same_signature_again = ecdsa::Pair::from_seed(&[1u8; 32]).sign(&message).encode();

            frame_support::assert_err!(
                Attesters::submit_attestation(
                    Origin::signed(attester),
                    message.to_vec(),
                    same_signature_again,
                    AttestationFor::Xtx,
                ),
                AttestersError::<MiniRuntime>::AttestationDoubleSignAttempt
            );
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
                    [1u8; 32],
                );
            }

            let attestation =
                Attesters::attestations(H256::from(*b"message_that_needs_attestation32"))
                    .expect("Attestation should exist");
            assert_eq!(attestation.status, AttestationStatus::Approved);
        });
    }

    #[test]
    fn attester_nomination() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register 64 attesters
            let mut attesters = Vec::new();
            for i in 0..64 {
                let attester = AccountId::from([i as u8; 32]);
                register_attester_with_single_private_key([i as u8; 32]);
                attesters.push(attester);
            }

            // Nominate the attesters
            let mut nominations = Vec::new();
            for i in 0..64 {
                let nominator = AccountId::from([(i + 1) as u8; 32]);
                let attester = attesters[i].clone();
                let amount = 1000;
                let nomination = (attester.clone(), amount);
                Nominations::<MiniRuntime>::insert(&attester, nomination);
                SortedNominatedAttesters::<MiniRuntime>::try_mutate(|attesters| {
                    if let Some(index) = attesters.iter().position(|a| a == &attester) {
                        attesters[index] = attester.clone();
                    } else {
                        attesters.push(attester.clone());
                    }
                    attesters.sort_unstable_by_key(|a| {
                        Nominations::<MiniRuntime>::get(a)
                            .map(|bal| bal.1 as Balance)
                            .unwrap_or(0)
                    });
                    attesters.truncate(32);
                    Ok::<(), AttestersError<MiniRuntime>>(())
                })
                .unwrap();
                nominations.push((nominator, attester, amount));
            }

            Attesters::on_initialize(400);

            // Check that the top 32 attesters are the ones with the most nominations
            let active_set = ActiveSet::<MiniRuntime>::get();
            assert_eq!(active_set.len(), 32);
            let top_nominated_attesters = SortedNominatedAttesters::<MiniRuntime>::get();
            for i in 0..32 {
                assert_eq!(active_set[i], top_nominated_attesters[i]);
            }
        });
    }
}
