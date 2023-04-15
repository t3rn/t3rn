#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub use crate::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use codec::{Decode, Encode};
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_application_crypto::RuntimePublic;
    use sp_runtime::RuntimeAppPublic;

    use sp_core::{
        crypto::{KeyTypeId},
        ecdsa, ed25519, sr25519,
    };
    

    use sp_runtime::traits::Verify;
    use sp_std::{convert::TryInto, prelude::*};

    // Key types for attester crypto
    const ECDSA_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"ecat");
    const ED25519_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"edat");
    const SR25519_ATTESTER_KEY_TYPE_ID: KeyTypeId = KeyTypeId(*b"srat");

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn attesters)]
    pub type Attesters<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AttesterInfo<T::AccountId>>;

    #[pallet::storage]
    #[pallet::getter(fn nominations)]
    pub type Nominations<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AttesterRegistered(T::AccountId),
        AttestationSubmitted(T::AccountId),
        Nominated(T::AccountId, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidSignature,
        AlreadyRegistered,
        PublicKeyMissing,
        AttestationSignatureInvalid,
        NotRegistered,
        AlreadyNominated,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_attester(
            origin: OriginFor<T>,
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

            // Registration logic
            Attesters::<T>::insert(
                &account_id,
                AttesterInfo {
                    account_id: account_id.clone(),
                    key_ec: ecdsa_key,
                    key_ed: ed25519_key,
                    key_sr: sr25519_key,
                },
            );

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn submit_attestation(
            origin: OriginFor<T>,
            message: Vec<u8>,
            signature: Vec<u8>,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;

            // Lookup the attester in the storage
            let attester = Attesters::<T>::get(&account_id).ok_or(Error::<T>::NotRegistered)?;

            let is_verified = attester
                .verify_attestation_signature(ECDSA_ATTESTER_KEY_TYPE_ID, &message, &signature)
                .map_err(|_| Error::<T>::InvalidSignature)?;

            ensure!(is_verified, Error::<T>::AttestationSignatureInvalid);

            Self::deposit_event(Event::AttestationSubmitted(attester.account_id));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn nominate(_origin: OriginFor<T>, _attester: T::AccountId) -> DispatchResult {
            // Nomination logic
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

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
    pub struct AttesterInfo<AccountId> {
        pub account_id: AccountId,
        pub key_ed: [u8; 32],
        pub key_ec: [u8; 33],
        pub key_sr: [u8; 32],
    }

    impl<AccountId> AttesterInfo<AccountId> {
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
    // use crate::{AttesterInfo, Config, Error, Event, Pallet};
    use codec::Encode;
    use frame_support::{
        assert_ok,
    };
    use sp_application_crypto::{ecdsa, ed25519, sr25519, Pair, RuntimePublic};
    use std::convert::TryInto;
    use t3rn_mini_mock_runtime::{
        AccountId, Attesters, ExtBuilder, Origin,
    };

    #[test]
    fn register_attester_from_single_private_key() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register an attester
            let attester = AccountId::from([1; 32]);

            let secret_key = [1u8; 32];
            let ecdsa_key = ecdsa::Pair::from_seed(&secret_key).public().to_raw_vec();
            let ed25519_key = ed25519::Pair::from_seed(&secret_key).public().to_raw_vec();
            let sr25519_key = sr25519::Pair::from_seed(&secret_key).public().to_raw_vec();

            assert_ok!(Attesters::register_attester(
                Origin::signed(attester),
                ecdsa_key.try_into().unwrap(),
                ed25519_key.try_into().unwrap(),
                sr25519_key.try_into().unwrap(),
            ));
        });
    }

    #[test]
    fn register_and_submit_attestation_in_ecdsa() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Register an attester
            let attester = AccountId::from([1; 32]);

            let secret_key = [1u8; 32];
            let ecdsa_key = ecdsa::Pair::from_seed(&secret_key).public().to_raw_vec();
            let ed25519_key = ed25519::Pair::from_seed(&secret_key).public().to_raw_vec();
            let sr25519_key = sr25519::Pair::from_seed(&secret_key).public().to_raw_vec();

            assert_ok!(Attesters::register_attester(
                Origin::signed(attester.clone()),
                ecdsa_key.try_into().unwrap(),
                ed25519_key.try_into().unwrap(),
                sr25519_key.try_into().unwrap(),
            ));

            // Submit an attestation signed with the Ed25519 key
            let message = b"message".to_vec();
            let key_pair = ecdsa::Pair::from_seed(&secret_key);
            let signature = key_pair.sign(&message);

            assert_ok!(Attesters::submit_attestation(
                Origin::signed(attester),
                message,
                signature.encode(),
            ));
        });
    }
}
