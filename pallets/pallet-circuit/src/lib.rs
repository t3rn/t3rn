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

use frame_system::ensure_signed;
use frame_system::offchain::{SignedPayload, SigningTypes};
use frame_system::pallet_prelude::OriginFor;

use sp_runtime::RuntimeDebug;

pub use t3rn_primitives::{
    abi::{GatewayABIConfig, HasherAlgo as HA},
    side_effect::{ConfirmedSideEffect, FullSideEffect, SideEffect, SideEffectId},
    transfers::BalanceOf,
    volatile::LocalState,
    xtx::{Xtx, XtxId},
    GatewayType, *,
};
use t3rn_protocol::side_effects::loader::{SideEffectsLazyLoader, UniversalSideEffectsProtocol};
pub use t3rn_protocol::{circuit_inbound::StepConfirmation, merklize::*};

use sp_runtime::traits::Zero;
use sp_std::fmt::Debug;

use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath};
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

pub mod state;

pub use t3rn_protocol::side_effects::protocol::SideEffectConfirmationProtocol;

/// Defines application identifier for crypto keys of this module.
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

pub type SystemHashing<T> = <T as frame_system::Config>::Hashing;
use crate::state::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Get;
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;

    pub use crate::weights::WeightInfo;

    /// Current Circuit's context of active transactions
    ///
    #[pallet::storage]
    pub type InsuranceDeposits<T> = StorageDoubleMap<
        _,
        Identity,
        XExecSignalId<T>,
        Identity,
        SideEffectId<T>,
        InsuranceDeposit<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            BalanceOf<T>,
        >,
        ValueQuery,
    >;
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
            // ToDo: pallet-circuit x-t3rn# : Authorize : Check TriggerAuthRights for local triggers

            // ToDo: pallet-circuit x-t3rn# : Validate : insurance for reversible side effects if necessary

            // ToDo: pallet-circuit x-t3rn# : Charge : fees

            // ToDo: pallet-circuit x-t3rn# : Design Storage - Propose and organise the state of Circuit. Specifically inspect the state updates in between ExecDelivery + Circuit

            // ToDo: pallet-circuit x-t3rn# : Setup : Create new Xtx and modify state - get LocalState (for Xtx) + GlobalState (for Circuit) for exec

            // ToDo: pallet-circuit x-t3rn# : Emit : Connect to ExecDelivery::submit_side_effect_temp( )

            // ToDo: pallet-circuit x-t3rn# : Cancel : Execution on timeout
            // ToDo: pallet-circuit x-t3rn# : Apply - Submission : Apply changes to storage after Submit has passed
            // ToDo: pallet-circuit x-t3rn# : Apply - Confirmation : Apply changes to storage after Confirmation has passed
            // ToDo: pallet-circuit x-t3rn# : Apply - Revert : Apply changes to storage after Revert has been proven
            // ToDo: pallet-circuit x-t3rn# : Apply - Commit : Apply changes to storage after Successfully Commit has been requested
            // ToDo: pallet-circuit x-t3rn# : Apply - Cancel : Apply changes to storage after the timeout has passed

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
            origin: OriginFor<T>,
            side_effects: Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
            _input: Vec<u8>,
            _value: BalanceOf<T>,
            fee: BalanceOf<T>,
            sequential: bool,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let requester = Self::authorize(origin)?;
            // Charge: Ensure can afford
            let _available_trn_balance = Self::charge(&requester)?;
            // Setup: new xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> =
                Self::setup(CircuitExecStatus::Requested, &requester, fee);
            // Validate: Side Effects
            let full_side_effects = Self::validate(
                &side_effects,
                &mut local_xtx_ctx.use_protocol,
                &mut local_xtx_ctx.local_state,
                &requester,
                local_xtx_ctx.xtx_id,
                sequential,
            )?;
            // Apply: all necessary changes to state in 1 go
            Self::apply(
                CircuitExecStatus::Requested,
                CircuitExecStatus::Validated,
                &local_xtx_ctx,
                &side_effects,
                full_side_effects,
            )?;

            // Emit: From Circuit events
            Self::emit(local_xtx_ctx.xtx_id, &requester, &side_effects, sequential);

            Ok(().into())
        }
    }

    /// Events for the pallet.
    #[pallet::event]
    //     #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {
        RequesterNotEnoughBalance,
    }
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

pub struct LocalXtxCtx<T: Config> {
    local_state: LocalState,
    use_protocol: UniversalSideEffectsProtocol,
    xtx_id: XExecSignalId<T>,
    xtx: XExecSignal<T::AccountId, T::BlockNumber, BalanceOf<T>>,
}

impl<T: Config> Pallet<T> {
    fn setup(
        current_status: CircuitExecStatus,
        requester: &T::AccountId,
        reward: BalanceOf<T>,
    ) -> LocalXtxCtx<T> {
        match current_status {
            CircuitExecStatus::Requested => {
                // ToDo: Introduce default timeout + delay
                let (timeouts_at, delay_steps_at): (
                    Option<T::BlockNumber>,
                    Option<Vec<T::BlockNumber>>,
                ) = (None, None);

                let (x_exec_signal_id, x_exec_signal) =
                    XExecSignal::<T::AccountId, T::BlockNumber, BalanceOf<T>>::setup_fresh::<T>(
                        requester,
                        timeouts_at,
                        delay_steps_at,
                        Some(reward),
                    );

                LocalXtxCtx {
                    local_state: LocalState::new(),
                    use_protocol: UniversalSideEffectsProtocol::new(),
                    xtx_id: x_exec_signal_id,
                    xtx: x_exec_signal,
                }
            }
            _ => unimplemented!(),
        }
    }

    fn apply(
        current_status: CircuitExecStatus,
        new_status: CircuitExecStatus,
        _local_xtx_ctx: &LocalXtxCtx<T>,
        _side_effects: &Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        _full_ordered_side_effects: Vec<
            Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        >,
    ) -> Result<(), Error<T>> {
        match (current_status, new_status) {
            (CircuitExecStatus::Requested, CircuitExecStatus::Validated) => {

                // <InsuranceDeposits<T>>::insert::<
                //     XExecSignalId<T>,
                //     SideEffectId<T>,
                //     InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
                // >(
                //     xtx_id,
                //     side_effect.generate_id::<SystemHashing<T>>(),
                //     InsuranceDeposit::new(
                //         insurance,
                //         promised_reward,
                //         requester.clone(),
                //         <frame_system::Pallet<T>>::block_number(),
                //     ),
                // );
            }
            (_, _) => unimplemented!(),
        }
        Ok(())
    }

    fn emit(
        _xtx_id: XExecSignalId<T>,
        _requester: &T::AccountId,
        _side_effects: &Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        _sequential: bool,
    ) {
        // <T as pallet_exec_delivery::Config>::submit_side_effect(
        //     xtx_id,
        //     requester: requester.clone(),
        //     side_effects,
        //     sequential,
        // );
    }

    fn charge(requester: &T::AccountId) -> Result<BalanceOf<T>, Error<T>> {
        let available_trn_balance = <T as EscrowTrait>::Currency::free_balance(requester);
        Ok(available_trn_balance)
    }

    fn authorize(origin: OriginFor<T>) -> Result<T::AccountId, sp_runtime::traits::BadOrigin> {
        ensure_signed(origin)
    }

    fn validate(
        side_effects: &Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        use_protocol: &mut UniversalSideEffectsProtocol,
        local_state: &mut LocalState,
        requester: &T::AccountId,
        _xtx_id: XExecSignalId<T>,
        sequential: bool,
    ) -> Result<Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>>, &'static str>
    {
        let mut full_side_effects: Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>> =
            vec![];

        for side_effect in side_effects.iter() {
            // ToDo: Generate Circuit's params as default ABI from let abi = pallet_xdns::get_abi(target_id)
            let gateway_abi = Default::default();
            use_protocol.notice_gateway(side_effect.target);
            use_protocol
                .validate_args::<T::AccountId, T::BlockNumber, BalanceOf<T>, SystemHashing<T>>(
                    side_effect.clone(),
                    gateway_abi,
                    local_state,
                )?;

            if let Some(insurance_and_reward) =
                UniversalSideEffectsProtocol::check_if_insurance_required::<
                    T::AccountId,
                    T::BlockNumber,
                    BalanceOf<T>,
                    SystemHashing<T>,
                >(side_effect.clone(), local_state)?
            {
                let (insurance, reward) = (insurance_and_reward[0], insurance_and_reward[1]);
                Self::request_side_effect_insurance(
                    Default::default(), // ToDo: Obtain XtxId before let x_tx_id: XtxId<T> = new_xtx.generate_xtx_id::<T>();
                    side_effect.clone(),
                    insurance,
                    reward,
                    requester,
                    local_state,
                )?;
            }
            full_side_effects.push(FullSideEffect {
                input: side_effect.clone(),
                confirmed: None,
            })
        }

        let full_side_effects_steps: Vec<
            Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        > = match sequential {
            false => vec![full_side_effects],
            true => {
                let mut sequential_order: Vec<
                    Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
                > = vec![];
                for fse in full_side_effects.iter() {
                    sequential_order.push(vec![fse.clone()]);
                }
                sequential_order
            }
        };

        Ok(full_side_effects_steps)
    }

    /// On-submit
    fn request_side_effect_insurance(
        xtx_id: XExecSignalId<T>,
        side_effect: SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        insurance: BalanceOf<T>,
        promised_reward: BalanceOf<T>,
        requester: &T::AccountId,
        _local_state: &mut LocalState,
    ) -> Result<(), Error<T>> {
        // ToDo: Prepare Treasury submodule with Vault Constant
        let VAULT: T::AccountId = Default::default();
        let res =
            <T as EscrowTrait>::Currency::transfer(requester, &VAULT, promised_reward, AllowDeath); // should not fail
        debug_assert!(res.is_ok());

        <InsuranceDeposits<T>>::insert::<
            XExecSignalId<T>,
            SideEffectId<T>,
            InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        >(
            xtx_id,
            side_effect.generate_id::<SystemHashing<T>>(),
            InsuranceDeposit::new(
                insurance,
                promised_reward,
                requester.clone(),
                <frame_system::Pallet<T>>::block_number(),
            ),
        );
        Ok(())
    }

    fn deposit_side_effect_insurance_lock() -> Result<(), &'static str> {
        Ok(())
    }
}
