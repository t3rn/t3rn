// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License")
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
//!
//! ## Overview
//!
//! Circuit MVP
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use frame_system::offchain::{SignedPayload, SigningTypes};

use sp_runtime::RuntimeDebug;

pub use t3rn_primitives::{
    abi::{GatewayABIConfig, HasherAlgo as HA},
    side_effect::{ConfirmedSideEffect, FullSideEffect, SideEffect},
    transfers::BalanceOf,
    volatile::LocalState,
    xtx::{Xtx, XtxId},
    GatewayType, *,
};
pub use t3rn_protocol::{circuit_inbound::StepConfirmation, merklize::*};

use sp_runtime::KeyTypeId;
pub type Bytes = sp_core::Bytes;

pub use pallet::*;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;

pub mod weights;

pub use t3rn_protocol::side_effects::protocol::SideEffectConfirmationProtocol;

/// Defines application identifier for crypto keys of this module.
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

pub type SystemHashing<T> = <T as frame_system::Config>::Hashing;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;

    use super::*;
    pub use crate::weights::WeightInfo;

    /// Current Circuit's context of active transactions
    ///
    //     /// The currently active composable transactions, indexed according to the order of creation.
    //     #[pallet::storage]
    //     pub type ActiveXtxMap<T> = StorageMap<
    //         _,
    //         Blake2_128Concat,
    //         XtxId<T>,
    //         Xtx<
    //             <T as frame_system::Config>::AccountId,
    //             <T as frame_system::Config>::BlockNumber,
    //             BalanceOf<T>,
    //         >,
    //         OptionQuery,
    //     >;

    /// This pallet's configuration trait
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_balances::Config
        // + pallet_contracts_registry::Config
        // + pallet_exec_delivery::Config
        + pallet_xdns::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The overarching dispatch call type.
        type Call: From<Call<Self>>;

        type WeightInfo: weights::WeightInfo;

        type PalletId: Get<PalletId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
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

            // if module block number
            // x-t3rn#4: Go over open Xtx and cancel if necessary
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

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Used by other pallets that want to create the exec order
        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_local_trigger())]
        pub fn on_local_trigger(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // ToDo: pallet-circuit x-t3rn# : Check TriggerAuthRights for local triggers

            // ToDo: pallet-circuit x-t3rn# : Insurance for reversible side effects if necessary

            // ToDo: pallet-circuit x-t3rn# : Charge fees

            // ToDo: pallet-circuit x-t3rn# : Design Storage - Propose and organise the state of Circuit. Specifically inspect the state updates in between ExecDelivery + Circuit

            // ToDo: pallet-circuit x-t3rn# : Create new Xtx and modify state - get LocalState (for Xtx) + GlobalState (for Circuit) for exec

            // ToDo: pallet-circuit x-t3rn# : Connect to ExecDelivery::submit_side_effect_temp( )

            // ToDo: pallet-circuit x-t3rn# : Cancel Execution on timeout

            unimplemented!();
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_local_trigger())]
        pub fn on_xcm_trigger(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // ToDo: Check TriggerAuthRights for local triggers
            unimplemented!();
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_local_trigger())]
        pub fn on_remote_gateway_trigger(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // ToDo: Check TriggerAuthRights for remote gateway triggers
            unimplemented!();
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_local_trigger())]
        pub fn on_extrinsics_trigger(
            _origin: OriginFor<T>,
            _side_effects: Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
            _input: Vec<u8>,
            _value: BalanceOf<T>,
            _reward: BalanceOf<T>,
            _sequential: bool,
        ) -> DispatchResultWithPostInfo {
            // ToDo: Check TriggerAuthRights for remote gateway triggers
            unimplemented!();
        }
    }

    /// Events for the pallet.
    #[pallet::event]
    //     #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {}
}

pub fn get_xtx_status() {}
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

impl<T: Config> Pallet<T> {}
