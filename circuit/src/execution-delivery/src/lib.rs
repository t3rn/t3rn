// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! <!-- markdown-link-check-disable -->
//! # Offchain Worker Example Pallet
//!
//! The Offchain Worker Example: A simple pallet demonstrating
//! concepts, APIs and structures common to most offchain workers.
//!
//! Run `cargo doc --package pallet-example-offchain-worker --open` to view this module's
//! documentation.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//!
//! ## Overview
//!
//! In this example we are going to build a very simplistic, naive and definitely NOT
//! production-ready oracle for BTC/USD price.
//! Offchain Worker (OCW) will be triggered after every block, fetch the current price
//! and prepare either signed or unsigned transaction to feed the result back on chain.
//! The on-chain logic will simply aggregate the results and store last `64` values to compute
//! the average price.
//! Additional logic in OCW is put in place to prevent spamming the network with both signed
//! and unsigned transactions, and custom `UnsignedValidator` makes sure that there is only
//! one unsigned transaction floating in the network.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_system::offchain::{AppCrypto, CreateSignedTransaction, SignedPayload, SigningTypes};

use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
    RuntimeDebug,
};
use sp_std::vec::Vec;
use t3rn_primitives::*;

#[cfg(test)]
mod tests;

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
pub mod crypto {
    use super::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
    };
    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for TestAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// This pallet's configuration trait
    #[pallet::config]
    pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
        /// The identifier type for an offchain worker.
        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The overarching dispatch call type.
        type Call: From<Call<Self>>;

        // Configuration parameters

        /// A grace period after we send transaction.
        ///
        /// To avoid sending too many transactions, we only attempt to send one
        /// every `GRACE_PERIOD` blocks. We use Local Storage to coordinate
        /// sending between distinct runs of this offchain worker.
        #[pallet::constant]
        type GracePeriod: Get<Self::BlockNumber>;

        /// Number of blocks of cooldown after unsigned transaction is included.
        ///
        /// This ensures that we only accept unsigned transactions once, every `UnsignedInterval` blocks.
        #[pallet::constant]
        type UnsignedInterval: Get<Self::BlockNumber>;

        /// A configuration for base priority of unsigned transactions.
        ///
        /// This is exposed so that it can be tuned for particular runtime, when
        /// multiple pallets send unsigned transactions.
        #[pallet::constant]
        type UnsignedPriority: Get<TransactionPriority>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            // Anything that needs to be done at the start of the block.
            // We don't do anything here.
            0
        }

        fn on_finalize(_n: T::BlockNumber) {
            // We don't do anything here.
        }

        // A runtime code run after every block and have access to extended set of APIs.
        //
        // For instance you can generate extrinsics for the upcoming produced block.
        fn offchain_worker(_n: T::BlockNumber) {
            // We don't do anything here.
            // but we could dispatch extrinsic (transaction/unsigned/inherent) using
            // sp_io::submit_extrinsic
        }
    }

    /// A public part of the pallet.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn submit_composable_exec_order(
            origin: OriginFor<T>,
            io_schedule: Vec<u8>,
            components: Vec<Compose<T::AccountId, u64>>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let who = ensure_signed(origin)?;

            let inter_schedule: InterExecSchedule<T::AccountId, u64> =
                Self::decompose_io_schedule(components, io_schedule).expect("Wrong io schedule");

            for phase in inter_schedule.phases.clone() {
                for step in phase.steps {
                    // extract GATEWAY_ID (InstanceId) and send_message to correct
                    // message lane (probably must be multiple as each lane is assigned to clients address)
                    // ideally use collection of different clients and dispatch to different clients but with std gateway
                    // receiver on the other end of message lane.
                    // message lane : verify finality : gateway id : finality_verifier :: verify single message lane by gateway id
                    Self::deposit_event(Event::NewPhase(who.clone(), 0, step.compose.name.clone()));
                }
            }

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn dummy_check_payload_origin(
            origin: OriginFor<T>,
            _price_payload: Payload<T::Public, T::BlockNumber>,
            _signature: T::Signature,
        ) -> DispatchResultWithPostInfo {
            // This ensures that the function can only be called via unsigned transaction.
            ensure_none(origin)?;
            Ok(().into())
        }
    }

    /// Events for the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event generated when new price is accepted to contribute to the average.
        /// \[who, phase, name\]
        NewPhase(T::AccountId, u8, Vec<u8>),
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        /// Validate unsigned call to this module.
        ///
        /// By default unsigned transactions are disallowed, but implementing the validator
        /// here we make sure that some particular calls (the ones produced by offchain worker)
        /// are being whitelisted and marked as valid.
        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            // Firstly let's check that we call the right function.
            if let Call::dummy_check_payload_origin(ref payload, ref signature) = call {
                let signature_valid =
                    SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
                if !signature_valid {
                    return InvalidTransaction::BadProof.into();
                }
                // Set a blank task initially
                ValidTransaction::with_tag_prefix("BlankTaskOffchainWorker")
                    // The transaction is only valid for next 5 blocks. After that it's
                    // going to be revalidated by the pool.
                    .longevity(5)
                    // It's fine to propagate that transaction to other peers, which means it can be
                    // created even by nodes that don't produce blocks.
                    // Note that sometimes it's better to keep it for yourself (if you are the block
                    // producer), since for instance in some schemes others may copy your solution and
                    // claim a reward.
                    .propagate(true)
                    .build()
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }
}

/// Payload used by this example crate to hold price
/// data required to submit a transaction.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Payload<Public, BlockNumber> {
    block_number: BlockNumber,
    public: Public,
}

impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public, T::BlockNumber> {
    fn public(&self) -> T::Public {
        self.public.clone()
    }
}

impl<T: Config> Pallet<T> {
    #[allow(dead_code)]
    pub fn say_hello() -> &'static str {
        "hello"
    }

    pub fn decompose_io_schedule(
        _components: Vec<Compose<T::AccountId, u64>>,
        _io_schedule: Vec<u8>,
    ) -> Result<InterExecSchedule<T::AccountId, u64>, &'static str> {
        let inter_schedule = InterExecSchedule::default();
        Ok(inter_schedule)
    }
}
