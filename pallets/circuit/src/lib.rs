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

use t3rn_protocol::side_effects::confirm::ethereum::EthereumSideEffectsParser;
use t3rn_protocol::side_effects::confirm::protocol::*;
use t3rn_protocol::side_effects::confirm::substrate::SubstrateSideEffectsParser;

use t3rn_protocol::side_effects::loader::{SideEffectsLazyLoader, UniversalSideEffectsProtocol};
pub use t3rn_protocol::{circuit_inbound::StepConfirmation, merklize::*};

use sp_runtime::traits::{AccountIdConversion, Saturating, Zero};

use sp_std::fmt::Debug;

use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath, Get};

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

    /// Current Circuit's context of active insurance deposits
    ///
    #[pallet::storage]
    #[pallet::getter(fn get_insurance_deposits)]
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

    /// Current Circuit's context of active transactions
    ///
    #[pallet::storage]
    #[pallet::getter(fn get_x_exec_signals)]
    pub type XExecSignals<T> = StorageMap<
        _,
        Identity,
        XExecSignalId<T>,
        XExecSignal<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            BalanceOf<T>,
        >,
        ValueQuery,
    >;

    /// Current Circuit's context of active full side effects (requested + confirmation proofs)
    #[pallet::storage]
    #[pallet::getter(fn get_xtx_insurance_links)]
    pub type XtxInsuranceLinks<T> =
        StorageMap<_, Identity, XExecSignalId<T>, Vec<SideEffectId<T>>, ValueQuery>;

    /// Current Circuit's context of active full side effects (requested + confirmation proofs)
    #[pallet::storage]
    #[pallet::getter(fn get_local_xtx_state)]
    pub type LocalXtxStates<T> = StorageMap<_, Identity, XExecSignalId<T>, LocalState, ValueQuery>;

    /// Current Circuit's context of active full side effects (requested + confirmation proofs)
    #[pallet::storage]
    #[pallet::getter(fn get_full_side_effects)]
    pub type FullSideEffects<T> = StorageMap<
        _,
        Identity,
        XExecSignalId<T>,
        Vec<
            Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    BalanceOf<T>,
                >,
            >,
        >,
        ValueQuery,
    >;

    /// This pallet's configuration trait
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_balances::Config
        + pallet_circuit_portal::Config
        + pallet_xdns::Config
    {
        /// The Circuit's pallet id
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The overarching dispatch call type.
        type Call: From<Call<Self>>;

        /// Weight infos
        type WeightInfo: weights::WeightInfo;
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
            // ToDo: Do active xtx signals overview and Cancel if time elapsed
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

            // ToDo: pallet-circuit x-t3rn# : Design Storage - Propose and organise the state of Circuit. Specifically inspect the state updates in between CircuitPortal + Circuit

            // ToDo: pallet-circuit x-t3rn# : Setup : Create new Xtx and modify state - get LocalState (for Xtx) + GlobalState (for Circuit) for exec

            // ToDo: pallet-circuit x-t3rn# : Emit : Connect to CircuitPortal::submit_side_effect_temp( )

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

            // Because no incentive for external relayers
            // - Composable can only call the CALL::3VM Contract
            // -

            // Writing an app on t3rn - i can create 2 smart contracts - one one composable and the other on Circuit
            //      writing additional smart contracts guarantes steps atomicity
            unimplemented!();
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_extrinsic_trigger())]
        pub fn on_extrinsic_trigger(
            origin: OriginFor<T>,
            side_effects: Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
            fee: BalanceOf<T>,
            sequential: bool,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let requester = Self::authorize(origin, CircuitRole::Requester)?;

            // Charge: Ensure can afford
            Self::charge(&requester, fee)?;
            // Setup: new xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> =
                Self::setup(CircuitStatus::Requested, &requester, fee, None)?;
            // Validate: Side Effects
            Self::validate(&side_effects, &mut local_xtx_ctx, &requester, sequential)?;
            // Apply: all necessary changes to state in 1 go
            let (_, added_full_side_effects) = Self::apply(&mut local_xtx_ctx, None)?;
            // Emit: From Circuit events
            Self::emit(
                local_xtx_ctx.xtx_id,
                Some(local_xtx_ctx.xtx),
                &requester,
                &side_effects,
                added_full_side_effects,
            );

            Ok(().into())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_insurance_deposit())]
        pub fn bond_insurance_deposit(
            origin: OriginFor<T>, // Active relayer
            xtx_id: XExecSignalId<T>,
            side_effect_id: SideEffectId<T>,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let relayer = Self::authorize(origin, CircuitRole::Relayer)?;

            // Setup: retrieve local xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingInsurance,
                &relayer,
                Zero::zero(),
                Some(xtx_id),
            )?;

            let (maybe_xtx_changed, _) = if let Some((_id, insurance_deposit)) = local_xtx_ctx
                .insurance_deposits
                .iter_mut()
                .find(|(id, _)| *id == side_effect_id)
            {
                Self::charge(&relayer, insurance_deposit.insurance)?;

                insurance_deposit.bonded_relayer = Some(relayer.clone());
                // ToDo: Consider removing status from insurance_deposit since redundant with relayer: Option<Relayer>
                insurance_deposit.status = CircuitStatus::Bonded;

                let insurance_deposit_copy = insurance_deposit.clone();
                // Apply: all necessary changes to state in 1 go
                Self::apply(
                    &mut local_xtx_ctx,
                    Some((side_effect_id, insurance_deposit_copy)),
                )
            } else {
                Err(Error::<T>::InsuranceBondNotRequired)
            }?;

            // Emit: From Circuit events
            Self::emit(
                local_xtx_ctx.xtx_id,
                maybe_xtx_changed,
                &relayer,
                &vec![],
                None,
            );

            Ok(().into())
        }

        /// Blind version should only be used for testing - unsafe since skips inclusion proof check.
        #[pallet::weight(< T as Config >::WeightInfo::confirm_side_effect())]
        pub fn confirm_side_effect(
            origin: OriginFor<T>,
            xtx_id: XtxId<T>,
            side_effect: SideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
            confirmation: ConfirmedSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
            inclusion_proof: Option<Vec<Vec<u8>>>,
            block_hash: Option<Vec<u8>>,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let relayer = Self::authorize(origin, CircuitRole::Relayer)?;

            // Setup: retrieve local xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingExecution,
                &relayer,
                Zero::zero(),
                Some(xtx_id),
            )?;

            let _full_side_effect = Self::confirm(
                &mut local_xtx_ctx,
                &relayer,
                &side_effect,
                &confirmation,
                inclusion_proof,
                block_hash,
            )?;

            // FixMe: Reward should be triggered by apply after the whole Xtx finishes
            Self::enact_insurance(&local_xtx_ctx, &side_effect, InsuranceEnact::Reward)?;

            // Apply: all necessary changes to state in 1 go
            let (maybe_xtx_changed, assert_full_side_effects_changed) =
                Self::apply(&mut local_xtx_ctx, None)?;

            // Emit: From Circuit events
            Self::emit(
                local_xtx_ctx.xtx_id,
                maybe_xtx_changed,
                &relayer,
                &vec![],
                assert_full_side_effects_changed,
            );

            Ok(().into())
        }
    }

    /// Events for the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and pending
        XTransactionReceivedForExec(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and ready
        XTransactionReadyForExec(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and finished
        XTransactionFinishedExec(XExecSignalId<T>),
        // Listeners - executioners/relayers to know new challenges and perform offline risk/reward calc
        //  of whether side effect is worth picking up
        NewSideEffectsAvailable(
            <T as frame_system::Config>::AccountId,
            XExecSignalId<T>,
            Vec<
                SideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    BalanceOf<T>,
                >,
            >,
        ),
        // Listeners - executioners/relayers to know that certain SideEffects are no longer valid
        // ToDo: Implement Xtx timeout!
        CancelledSideEffects(
            <T as frame_system::Config>::AccountId,
            XtxId<T>,
            Vec<
                SideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    BalanceOf<T>,
                >,
            >,
        ),
        // Listeners - executioners/relayers to know whether they won the confirmation challenge
        SideEffectsConfirmed(
            XtxId<T>,
            Vec<
                Vec<
                    FullSideEffect<
                        <T as frame_system::Config>::AccountId,
                        <T as frame_system::Config>::BlockNumber,
                        BalanceOf<T>,
                    >,
                >,
            >,
        ),
    }

    #[pallet::error]
    pub enum Error<T> {
        RequesterNotEnoughBalance,
        ChargingTransferFailed,
        RewardTransferFailed,
        RefundTransferFailed,
        InsuranceBondNotRequired,
        InsuranceBondAlreadyDeposited,
        SetupFailed,
        SetupFailedIncorrectXtxStatus,
        SetupFailedUnknownXtx,
        SetupFailedDuplicatedXtx,
        SetupFailedEmptyXtx,
        ApplyFailed,
        DeterminedForbiddenXtxStatus,
        UnsupportedRole,
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

impl<T: Config> Pallet<T> {
    fn setup(
        current_status: CircuitStatus,
        requester: &T::AccountId,
        reward: BalanceOf<T>,
        xtx_id: Option<XExecSignalId<T>>,
    ) -> Result<LocalXtxCtx<T>, Error<T>> {
        match current_status {
            CircuitStatus::Requested => {
                if let Some(id) = xtx_id {
                    if <Self as Store>::XExecSignals::contains_key(id) {
                        return Err(Error::<T>::SetupFailedDuplicatedXtx);
                    }
                }
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

                Ok(LocalXtxCtx {
                    local_state: LocalState::new(),
                    use_protocol: UniversalSideEffectsProtocol::new(),
                    xtx_id: x_exec_signal_id,
                    xtx: x_exec_signal,
                    insurance_deposits: vec![],
                    full_side_effects: vec![],
                })
            }
            CircuitStatus::PendingInsurance => {
                if let Some(id) = xtx_id {
                    if !<Self as Store>::XExecSignals::contains_key(id) {
                        return Err(Error::<T>::SetupFailedUnknownXtx);
                    }
                    let xtx = <Self as Store>::XExecSignals::get(id);
                    if xtx.status != CircuitStatus::PendingInsurance {
                        return Err(Error::<T>::SetupFailedIncorrectXtxStatus);
                    }
                    let insurance_deposits = <Self as Store>::XtxInsuranceLinks::get(id)
                        .iter()
                        .map(|&se_id| (se_id, <Self as Store>::InsuranceDeposits::get(id, se_id)))
                        .collect::<Vec<(
                            SideEffectId<T>,
                            InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
                        )>>();

                    Ok(LocalXtxCtx {
                        local_state: LocalState::new(),
                        use_protocol: UniversalSideEffectsProtocol::new(),
                        xtx_id: id,
                        xtx,
                        insurance_deposits,
                        full_side_effects: vec![], // Update of full side effects won't be needed to update the insurance info
                    })
                } else {
                    Err(Error::<T>::SetupFailedEmptyXtx)
                }
            }
            CircuitStatus::PendingExecution => {
                if let Some(id) = xtx_id {
                    if !<Self as Store>::XExecSignals::contains_key(id) {
                        return Err(Error::<T>::SetupFailedUnknownXtx);
                    }
                    let xtx = <Self as Store>::XExecSignals::get(id);
                    if xtx.status < CircuitStatus::Ready {
                        return Err(Error::<T>::SetupFailedIncorrectXtxStatus);
                    }
                    let insurance_deposits = <Self as Store>::XtxInsuranceLinks::get(id)
                        .iter()
                        .map(|&se_id| (se_id, <Self as Store>::InsuranceDeposits::get(id, se_id)))
                        .collect::<Vec<(
                            SideEffectId<T>,
                            InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
                        )>>();

                    let full_side_effects = <Self as Store>::FullSideEffects::get(id);
                    let local_state = <Self as Store>::LocalXtxStates::get(id);

                    Ok(LocalXtxCtx {
                        local_state,
                        use_protocol: UniversalSideEffectsProtocol::new(),
                        xtx_id: id,
                        xtx,
                        insurance_deposits,
                        // We need to retrieve full side effects to validate the confirmation order
                        full_side_effects,
                    })
                } else {
                    Err(Error::<T>::SetupFailedEmptyXtx)
                }
            }
            _ => unimplemented!(),
        }
    }

    /// Returns: Returns changes written to the state if there are any.
    ///     For now only returns Xtx and FullSideEffects that changed.
    fn apply(
        local_ctx: &mut LocalXtxCtx<T>,
        maybe_insurance_tuple: Option<(
            SideEffectId<T>,
            InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        )>,
    ) -> Result<
        (
            Option<XExecSignal<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
            Option<Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>>>,
        ),
        Error<T>,
    > {
        // Apply will try to move the status of Xtx from the current to the closest valid one.
        let current_status = local_ctx.xtx.status.clone();

        match current_status {
            CircuitStatus::Requested => {
                <FullSideEffects<T>>::insert::<
                    XExecSignalId<T>,
                    Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>>,
                >(local_ctx.xtx_id, local_ctx.full_side_effects.clone());

                let mut ids_with_insurance: Vec<SideEffectId<T>> = vec![];
                for (side_effect_id, insurance_deposit) in &local_ctx.insurance_deposits {
                    <InsuranceDeposits<T>>::insert::<
                        XExecSignalId<T>,
                        SideEffectId<T>,
                        InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
                    >(
                        local_ctx.xtx_id, *side_effect_id, insurance_deposit.clone()
                    );
                    ids_with_insurance.push(*side_effect_id);
                }
                <XtxInsuranceLinks<T>>::insert::<XExecSignalId<T>, Vec<SideEffectId<T>>>(
                    local_ctx.xtx_id,
                    ids_with_insurance,
                );
                <LocalXtxStates<T>>::insert::<XExecSignalId<T>, LocalState>(
                    local_ctx.xtx_id,
                    local_ctx.local_state.clone(),
                );
                local_ctx.xtx.status = CircuitStatus::determine_xtx_status(
                    &local_ctx.full_side_effects,
                    &local_ctx.insurance_deposits,
                )?;

                <XExecSignals<T>>::insert::<
                    XExecSignalId<T>,
                    XExecSignal<T::AccountId, T::BlockNumber, BalanceOf<T>>,
                >(local_ctx.xtx_id, local_ctx.xtx.clone());

                Ok((
                    Some(local_ctx.xtx.clone()),
                    Some(local_ctx.full_side_effects.to_vec()),
                ))
            }
            CircuitStatus::PendingInsurance => {
                if let Some((side_effect_id, insurance_deposit)) = maybe_insurance_tuple {
                    <Self as Store>::InsuranceDeposits::mutate(
                        local_ctx.xtx_id,
                        side_effect_id,
                        |x| *x = insurance_deposit,
                    );
                    let new_status = CircuitStatus::determine_effects_insurance_status::<T>(
                        &local_ctx.insurance_deposits,
                    );

                    if new_status != local_ctx.xtx.status {
                        local_ctx.xtx.status = new_status;

                        <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                            *x = local_ctx.xtx.clone()
                        });
                        Ok((Some(local_ctx.xtx.clone()), None))
                    } else {
                        Ok((None, None))
                    }
                } else {
                    Err(Error::<T>::ApplyFailed)
                }
            }
            CircuitStatus::Ready | CircuitStatus::PendingExecution => {
                // Update set of full side effects assuming the new confirmed has appeared
                <Self as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                    *x = local_ctx.full_side_effects.clone()
                });

                let new_status = CircuitStatus::determine_xtx_status::<T>(
                    &local_ctx.full_side_effects,
                    &local_ctx.insurance_deposits,
                )?;

                if new_status != local_ctx.xtx.status {
                    local_ctx.xtx.status = new_status;
                    <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                        *x = local_ctx.xtx.clone()
                    });
                    Ok((
                        Some(local_ctx.xtx.clone()),
                        Some(local_ctx.full_side_effects.clone()),
                    ))
                } else {
                    Ok((None, Some(local_ctx.full_side_effects.to_vec())))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn emit(
        xtx_id: XExecSignalId<T>,
        maybe_xtx: Option<XExecSignal<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        subjected_account: &T::AccountId,
        side_effects: &Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        maybe_full_side_effects: Option<
            Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>>,
        >,
    ) {
        if let Some(xtx) = maybe_xtx {
            match xtx.status {
                CircuitStatus::PendingInsurance => {
                    Self::deposit_event(Event::XTransactionReceivedForExec(xtx_id))
                }
                CircuitStatus::Ready => {
                    Self::deposit_event(Event::XTransactionReadyForExec(xtx_id))
                }
                CircuitStatus::Finished => {
                    Self::deposit_event(Event::XTransactionFinishedExec(xtx_id))
                }
                _ => {}
            }
            if xtx.status >= CircuitStatus::PendingExecution {
                if let Some(full_side_effects) = maybe_full_side_effects {
                    Self::deposit_event(Event::SideEffectsConfirmed(xtx_id, full_side_effects));
                }
            }
        }
        if !side_effects.is_empty() {
            Self::deposit_event(Event::NewSideEffectsAvailable(
                subjected_account.clone(),
                xtx_id,
                // ToDo: Emit circuit outbound messages -> side effects
                side_effects.to_vec(),
            ));
        }
    }

    fn charge(requester: &T::AccountId, fee: BalanceOf<T>) -> Result<BalanceOf<T>, Error<T>> {
        let available_trn_balance = <T as EscrowTrait>::Currency::free_balance(requester);
        let new_balance = available_trn_balance.saturating_sub(fee);
        let vault: T::AccountId = Self::account_id();
        <T as EscrowTrait>::Currency::transfer(requester, &vault, fee, AllowDeath)
            .map_err(|_| Error::<T>::ChargingTransferFailed)?; // should not fail
        Ok(new_balance)
    }

    fn enact_insurance(
        local_ctx: &LocalXtxCtx<T>,
        side_effect: &SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        enact_status: InsuranceEnact,
    ) -> Result<bool, Error<T>> {
        let side_effect_id = side_effect.generate_id::<SystemHashing<T>>();
        // Reward insurance
        // Check if the side effect was insured and if the relayer matches the bonded one
        // ToDo: FIX_ME: WRONG! ONLY REWARD RELAYERS AFTER THE WHOLE STEP IS COMPLETED, NOT INDIVIDUALLY PER CONFIRMATION
        return if let Some((_id, insurance_request)) = local_ctx
            .insurance_deposits
            .iter()
            .find(|(id, _)| *id == side_effect_id)
        {
            if let Some(bonded_relayer) = &insurance_request.bonded_relayer {
                match enact_status {
                    InsuranceEnact::Reward => {
                        // Reward relayer with and give back his insurance from Vault
                        <T as EscrowTrait>::Currency::transfer(
                            &Self::account_id(),
                            bonded_relayer,
                            insurance_request.insurance + insurance_request.reward,
                            AllowDeath,
                        )
                        .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail
                    }
                    InsuranceEnact::RefundBoth => {
                        <T as EscrowTrait>::Currency::transfer(
                            &Self::account_id(),
                            &insurance_request.requester,
                            insurance_request.reward,
                            AllowDeath,
                        )
                        .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail

                        <T as EscrowTrait>::Currency::transfer(
                            &Self::account_id(),
                            bonded_relayer,
                            insurance_request.insurance,
                            AllowDeath,
                        )
                        .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail
                    }
                    InsuranceEnact::RefundAndPunish => {
                        <T as EscrowTrait>::Currency::transfer(
                            &Self::account_id(),
                            &insurance_request.requester,
                            insurance_request.reward,
                            AllowDeath,
                        )
                        .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail
                    }
                }
            } else {
                // This is a forbidden state which should have not happened -
                //  at this point all of the insurances should have a bonded relayer assigned
                return Err(Error::<T>::RefundTransferFailed);
            }
            Ok(true)
        } else {
            Ok(false)
        };
    }

    fn authorize(
        origin: OriginFor<T>,
        role: CircuitRole,
    ) -> Result<T::AccountId, sp_runtime::traits::BadOrigin> {
        match role {
            CircuitRole::Requester => ensure_signed(origin),
            // ToDo: Handle active Relayer authorisation
            CircuitRole::Relayer => ensure_signed(origin),
            // ToDo: Handle other CircuitRoles
            _ => unimplemented!(),
        }
    }

    fn validate(
        side_effects: &Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        local_ctx: &mut LocalXtxCtx<T>,
        requester: &T::AccountId,
        sequential: bool,
    ) -> Result<(), &'static str> {
        let mut full_side_effects: Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>> =
            vec![];

        for side_effect in side_effects.iter() {
            // ToDo: Generate Circuit's params as default ABI from let abi = pallet_xdns::get_abi(target_id)
            let gateway_abi = Default::default();
            local_ctx.use_protocol.notice_gateway(side_effect.target);
            local_ctx
                .use_protocol
                .validate_args::<T::AccountId, T::BlockNumber, BalanceOf<T>, SystemHashing<T>>(
                    side_effect.clone(),
                    gateway_abi,
                    &mut local_ctx.local_state,
                )?;

            if let Some(insurance_and_reward) =
                UniversalSideEffectsProtocol::check_if_insurance_required::<
                    T::AccountId,
                    T::BlockNumber,
                    BalanceOf<T>,
                    SystemHashing<T>,
                >(side_effect.clone(), &mut local_ctx.local_state)?
            {
                let (insurance, reward) = (insurance_and_reward[0], insurance_and_reward[1]);

                Self::charge(requester, reward)?;

                local_ctx.insurance_deposits.push((
                    side_effect.generate_id::<SystemHashing<T>>(),
                    InsuranceDeposit::new(
                        insurance,
                        reward,
                        requester.clone(),
                        <frame_system::Pallet<T>>::block_number(),
                    ),
                ));
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

        local_ctx.full_side_effects = full_side_effects_steps;

        Ok(())
    }

    fn confirm(
        local_ctx: &mut LocalXtxCtx<T>,
        _relayer: &T::AccountId,
        side_effect: &SideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            BalanceOf<T>,
        >,
        confirmation: &ConfirmedSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            BalanceOf<T>,
        >,
        inclusion_proof: Option<Vec<Vec<u8>>>,
        block_hash: Option<Vec<u8>>,
    ) -> Result<
        FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            BalanceOf<T>,
        >,
        &'static str,
    > {
        let confirm_inclusion = || {
            // ToDo: Remove below after testing inclusion
            // Temporarily allow skip inclusion if proofs aren't provided
            if !(block_hash.is_none() && inclusion_proof.is_none()) {
                pallet_circuit_portal::Pallet::<T>::confirm_inclusion(
                    side_effect.target,
                    confirmation.encoded_effect.clone(),
                    ProofTriePointer::State,
                    block_hash,
                    inclusion_proof,
                )
            } else {
                Ok(())
            }
        };

        let confirm_execution = |gateway_vendor, state_copy| {
            confirm_with_vendor_by_action_id::<
                T,
                SubstrateSideEffectsParser,
                EthereumSideEffectsParser<<T as pallet_circuit_portal::Config>::EthVerifier>,
            >(
                gateway_vendor,
                side_effect.encoded_action.clone(),
                confirmation.encoded_effect.clone(),
                state_copy,
                Some(
                    side_effect
                        .generate_id::<SystemHashing<T>>()
                        .as_ref()
                        .to_vec(),
                ),
            )
        };

        fn confirm_order<T: Config>(
            side_effect: &SideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
            confirmation: &ConfirmedSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
            full_side_effects: &mut Vec<
                Vec<
                    FullSideEffect<
                        <T as frame_system::Config>::AccountId,
                        <T as frame_system::Config>::BlockNumber,
                        BalanceOf<T>,
                    >,
                >,
            >,
        ) -> Result<bool, &'static str> {
            // ToDo: Extract as a separate function and migrate tests from Xtx
            let input_side_effect_id = side_effect.generate_id::<SystemHashing<T>>();
            let mut unconfirmed_step_no: Option<usize> = None;

            for (i, step) in full_side_effects.iter_mut().enumerate() {
                // Double check there are some side effects for that Xtx - should have been checked at API level tho already
                if step.is_empty() {
                    return Err("Xtx has an empty single step.");
                }
                for mut full_side_effect in step.iter_mut() {
                    if full_side_effect.confirmed.is_none() {
                        // Mark the first step no with encountered unconfirmed side effect
                        if unconfirmed_step_no.is_none() {
                            unconfirmed_step_no = Some(i);
                        }
                        // Recalculate the ID for each input side effect and compare with the input one.
                        // Check the current unconfirmed step before attempt to confirm the full side effect.
                        return if full_side_effect.input.generate_id::<SystemHashing<T>>()
                            == input_side_effect_id
                            && unconfirmed_step_no == Some(i)
                        {
                            // We found the side effect to confirm from inside the unconfirmed step.
                            full_side_effect.confirmed = Some(confirmation.clone());
                            Ok(true)
                        } else {
                            Err("Attempt to confirm side effect from the next step, \
                                    but there still is at least one unfinished step")
                        };
                    }
                }
            }

            Ok(false)
        }

        if !confirm_order::<T>(side_effect, confirmation, &mut local_ctx.full_side_effects)? {
            return Err(
                "Side effect confirmation wasn't matched with full side effects order from state",
            );
        }
        confirm_inclusion()?;
        confirm_execution(
            pallet_xdns::Pallet::<T>::best_available(side_effect.target)?.gateway_vendor,
            &local_ctx.local_state,
        )?;

        Ok(FullSideEffect {
            input: side_effect.clone(),
            confirmed: Some(confirmation.clone()),
        })
    }

    /// The account ID of the Circuit Vault.
    pub fn account_id() -> T::AccountId {
        <T as pallet::Config>::PalletId::get().into_account()
    }
}
