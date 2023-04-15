#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use codec::{Decode, Encode, MaxEncodedLen};
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::{ecdsa, ed25519, sr25519, Pair};
    use sp_std::convert::TryInto;
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn attesters)]
    pub type Attesters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, AttesterInfo>;

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
            ecdsa_key: ecdsa::Public,
            ed25519_key: ed25519::Public,
            sr25519_key: sr25519::Public,
        ) -> DispatchResult {
            // Registration logic
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn submit_attestation(
            origin: OriginFor<T>,
            message: Vec<u8>,
            signature: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // Ensure the sender is a registered attester
            let attester = Attesters::<T>::get(&sender).ok_or(Error::<T>::NotRegistered)?;

            // Verify the attestation signature
            ensure!(
                Self::verify_attestation_signature(&attester, &message, &signature),
                Error::<T>::AttestationSignatureInvalid
            );

            // Emit AttestationSubmitted event
            Self::deposit_event(Event::AttestationSubmitted(sender));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn nominate(origin: OriginFor<T>, attester: T::AccountId) -> DispatchResult {
            // Nomination logic
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn verify_attestation_signature(
            attester: &AttesterInfo,
            message: &[u8],
            signature: &[u8],
        ) -> bool {
            let ecdsa_sig_valid = ecdsa::Pair::verify_weak(signature, message, &attester.ecdsa_key);
            let ed25519_sig_valid =
                ed25519::Pair::verify_weak(signature, message, &attester.ed25519_key);
            let sr25519_sig_valid =
                sr25519::Pair::verify_weak(signature, message, &attester.sr25519_key);

            ecdsa_sig_valid || ed25519_sig_valid || sr25519_sig_valid
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

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    pub struct AttesterInfo {
        pub ecdsa_key: ecdsa::Public,
        pub ed25519_key: ed25519::Public,
        pub sr25519_key: sr25519::Public,
    }
}
