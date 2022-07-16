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
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::escrow::Escrow;
pub use crate::pallet::*;
use codec::{Decode, Encode};
use frame_support::{
    dispatch::{Dispatchable, GetDispatchInfo},
    traits::{Currency, ExistenceRequirement::AllowDeath, Get},
    weights::Weight,
    RuntimeDebug,
};
use frame_system::{
    ensure_signed,
    offchain::{SignedPayload, SigningTypes},
    pallet_prelude::OriginFor,
};
use sp_runtime::{
    traits::{AccountIdConversion, Saturating, Zero},
    KeyTypeId,
};
use sp_std::{boxed::Box, convert::TryInto, vec, vec::Vec};
pub use t3rn_primitives::{
    abi::{GatewayABIConfig, HasherAlgo as HA, Type},
    side_effect::{ConfirmedSideEffect, FullSideEffect, SideEffect, SideEffectId},
    volatile::LocalState,
    xtx::{Xtx, XtxId},
    GatewayType, *,
};
pub use t3rn_sdk_primitives::signal::{ExecutionSignal, SignalKind};

use t3rn_primitives::{
    circuit_portal::CircuitPortal,
    side_effect::{ConfirmationOutcome, HardenedSideEffect, SecurityLvl},
    transfers::EscrowedBalanceOf,
    xdns::Xdns,
};
use t3rn_protocol::side_effects::{
    confirm::{
        ethereum::EthereumSideEffectsParser, protocol::*, substrate::SubstrateSideEffectsParser,
    },
    loader::{SideEffectsLazyLoader, UniversalSideEffectsProtocol},
};
pub use t3rn_protocol::{circuit_inbound::StepConfirmation, merklize::*};

use pallet_xbi_portal::{primitives::xbi::XBIPortal, xbi_format::XBIInstr};

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;

pub mod weights;

pub mod state;

pub mod escrow;

/// Defines application identifier for crypto keys of this module.
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

pub type SystemHashing<T> = <T as frame_system::Config>::Hashing;
pub type EscrowCurrencyOf<T> = <<T as pallet::Config>::Escrowed as EscrowTrait<T>>::Currency;
use crate::state::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Inspect, Mutate},
            Get,
        },
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use orml_traits::MultiCurrency;
    use pallet_xbi_portal::xbi_codec::{XBICheckOutStatus, XBIMetadata, XBINotificationKind};
    use pallet_xbi_portal_enter::t3rn_sfx::sfx_2_xbi;
    use sp_std::borrow::ToOwned;

    use t3rn_primitives::{
        circuit::{LocalStateExecutionView, LocalTrigger, OnLocalTrigger},
        circuit_portal::CircuitPortal,
        xdns::Xdns,
    };

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
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
        OptionQuery,
    >;

    /// Current Circuit's context of active insurance deposits
    ///
    #[pallet::storage]
    #[pallet::getter(fn get_active_timing_links)]
    pub type ActiveXExecSignalsTimingLinks<T> = StorageMap<
        _,
        Identity,
        XExecSignalId<T>,
        <T as frame_system::Config>::BlockNumber,
        OptionQuery,
    >;

    /// Current Circuit's context of active insurance deposits
    ///
    #[pallet::storage]
    #[pallet::getter(fn local_side_effects)]
    pub type LocalSideEffects<T> = StorageDoubleMap<
        _,
        Identity,
        XExecSignalId<T>,
        Identity,
        XExecStepSideEffectId<T>,
        (u32, Option<<T as frame_system::Config>::AccountId>),
        OptionQuery, // Vec<(usize, Vec<SideEffectId<T>>)>
    >;
    /// Current Circuit's context of active insurance deposits
    ///
    #[pallet::storage]
    #[pallet::getter(fn local_side_effects_links)]
    pub type LocalSideEffectsLinks<T> = StorageDoubleMap<
        _,
        Identity,
        XExecSignalId<T>,
        Identity,
        SideEffectId<T>,
        XExecStepSideEffectId<T>,
        OptionQuery,
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
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
        OptionQuery,
    >;

    /// Current Circuit's context of active full side effects (requested + confirmation proofs)
    #[pallet::storage]
    #[pallet::getter(fn get_xtx_insurance_links)]
    pub type XtxInsuranceLinks<T> =
        StorageMap<_, Identity, XExecSignalId<T>, Vec<SideEffectId<T>>, ValueQuery>;

    /// Current Circuit's context of active full side effects (requested + confirmation proofs)
    #[pallet::storage]
    #[pallet::getter(fn get_local_xtx_state)]
    pub type LocalXtxStates<T> = StorageMap<_, Identity, XExecSignalId<T>, LocalState, OptionQuery>;

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
                    EscrowedBalanceOf<T, <T as Config>::Escrowed>,
                >,
            >,
        >,
        OptionQuery,
    >;

    /// Current Circuit's context of active full side effects (requested + confirmation proofs)
    #[pallet::storage]
    #[pallet::getter(fn get_escrow_side_effects_pending_relay)]
    pub type EscrowedSideEffectsPendingRelay<T> = StorageMap<
        _,
        Identity,
        XExecSignalId<T>,
        Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, <T as Config>::Escrowed>,
            >,
        >,
        OptionQuery,
    >;

    /// Handles queued signals
    ///
    /// This operation is performed lazily in `on_initialize`.
    #[pallet::storage]
    #[pallet::getter(fn get_signal_queue)]
    pub(crate) type SignalQueue<T: Config> = StorageValue<
        _,
        BoundedVec<(T::AccountId, ExecutionSignal<T::Hash>), T::SignalQueueDepth>,
        ValueQuery,
    >;

    /// This pallet's configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The Circuit's pallet id
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// The Circuit's self gateway id
        #[pallet::constant]
        type SelfGatewayId: Get<[u8; 4]>;

        /// The Circuit's self parachain id
        #[pallet::constant]
        type SelfParaId: Get<u32>;

        /// The Circuit's Default Xtx timeout
        #[pallet::constant]
        type XtxTimeoutDefault: Get<Self::BlockNumber>;

        /// The Circuit's Xtx timeout check interval
        #[pallet::constant]
        type XtxTimeoutCheckInterval: Get<Self::BlockNumber>;

        /// The Circuit's deletion queue limit - preventing potential
        ///     delay when queue is too long in on_initialize
        #[pallet::constant]
        type DeletionQueueLimit: Get<u32>;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// A dispatchable call.
        type Call: Parameter
            + Dispatchable<Origin = Self::Origin>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: weights::WeightInfo;

        /// A type that provides MultiCurrency support
        type MultiCurrency: MultiCurrency<Self::AccountId>;

        /// A type that provides inspection and mutation to some fungible assets
        type Balances: Inspect<Self::AccountId> + Mutate<Self::AccountId>;

        /// A type that provides access to Xdns
        type Xdns: Xdns<Self>;

        type XBIPortal: XBIPortal<Self>;

        // type FreeVM: FreeVM<Self>;

        /// A type that manages escrow, and therefore balances
        type Escrowed: EscrowTrait<Self>;

        /// A type that provides portal functionality
        type CircuitPortal: CircuitPortal<Self>;

        /// The maximum number of signals that can be queued for handling.
        ///
        /// When a signal from 3vm is requested, we add it to the queue to be handled by on_initialize
        ///
        /// This allows us to process the highest priority and mitigate any race conditions from additional steps.
        ///
        /// The reasons for limiting the queue depth are:
        ///
        /// 1. The queue is in storage in order to be persistent between blocks. We want to limit
        /// 	the amount of storage that can be consumed.
        /// 2. The queue is stored in a vector and needs to be decoded as a whole when reading
        ///		it at the end of each block. Longer queues take more weight to decode and hence
        ///		limit the amount of items that can be deleted per block.
        #[pallet::constant]
        type SignalQueueDepth: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(n: T::BlockNumber) -> Weight {
            let weight = Self::process_signal_queue();

            // Check every XtxTimeoutCheckInterval blocks

            // what happens if the weight for the block is consumed, do these timeouts need to wait
            // for the next check interval to handle them? maybe we need an immediate queue
            //
            // Scenario 1: all the timeouts can be handled in the block space
            // Scenario 2: all but 5 timeouts can be handled
            //     - add the 5 timeouts to an immediate queue for the next block
            if n % T::XtxTimeoutCheckInterval::get() == T::BlockNumber::from(0u8) {
                let mut deletion_counter: u32 = 0;
                // Go over all unfinished Xtx to find those that timed out
                <ActiveXExecSignalsTimingLinks<T>>::iter()
                    .find(|(_xtx_id, timeout_at)| {
                        timeout_at <= &frame_system::Pallet::<T>::block_number()
                    })
                    .map(|(xtx_id, _timeout_at)| {
                        if deletion_counter > T::DeletionQueueLimit::get() {
                            return
                        }
                        let mut local_xtx_ctx = Self::setup(
                            CircuitStatus::RevertTimedOut,
                            &Self::account_id(),
                            Zero::zero(),
                            Some(xtx_id),
                        )
                        .unwrap();

                        Self::kill(&mut local_xtx_ctx, CircuitStatus::RevertTimedOut);

                        Self::emit(
                            local_xtx_ctx.xtx_id,
                            Some(local_xtx_ctx.xtx),
                            &Self::account_id(),
                            &vec![],
                            None,
                        );
                        deletion_counter += 1;
                    });
            }

            // Anything that needs to be done at the start of the block.
            // We don't do anything here.
            // ToDo: Do active xtx signals overview and Cancel if time elapsed
            weight
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

    impl<T: Config> OnLocalTrigger<T> for Pallet<T> {
        fn load_local_state(
            origin: &OriginFor<T>,
            maybe_xtx_id: Option<T::Hash>,
        ) -> Result<LocalStateExecutionView<T>, DispatchError> {
            let requester = Self::authorize(origin.to_owned(), CircuitRole::ContractAuthor)?;

            let fresh_or_revoked_exec = match maybe_xtx_id {
                Some(_xtx_id) => CircuitStatus::Ready,
                None => CircuitStatus::Requested,
            };

            let mut local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                fresh_or_revoked_exec,
                &requester,
                Zero::zero(),
                maybe_xtx_id,
            )?;
            log::debug!(
                target: "runtime::circuit",
                "load_local_state with status: {:?}",
                local_xtx_ctx.xtx.status
            );

            if maybe_xtx_id.is_none() {
                Self::apply(&mut local_xtx_ctx, None, None)?;
            }

            // There should be no apply step since no change could have happen during the state access
            let hardened_side_effects = local_xtx_ctx
                .full_side_effects
                .iter()
                .map(|step| {
                    step.iter()
                        .map(|fsx| {
                            Ok(fsx
                                .clone()
                                .harden()
                                .map_err(|_e| Error::<T>::FailedToHardenFullSideEffect)?
                                .into())
                        })
                        .collect::<Result<Vec<HardenedSideEffect>, Error<T>>>()
                })
                .collect::<Result<Vec<Vec<HardenedSideEffect>>, Error<T>>>()?;

            // There should be no apply step since no change could have happen during the state access
            Ok(LocalStateExecutionView::<T>::new(
                local_xtx_ctx.xtx_id,
                local_xtx_ctx.local_state.clone(),
                hardened_side_effects,
                local_xtx_ctx.xtx.steps_cnt,
            ))
        }

        fn on_local_trigger(origin: &OriginFor<T>, trigger: LocalTrigger<T>) -> DispatchResult {
            log::debug!(
                target: "runtime::circuit",
                "Handling on_local_trigger xtx: {:?}, contract: {:?}, side_effects: {:?}",
                trigger.maybe_xtx_id,
                trigger.contract,
                trigger.submitted_side_effects
            );
            // Authorize: Retrieve sender of the transaction.
            let requester = Self::authorize(origin.to_owned(), CircuitRole::ContractAuthor)?;

            let fresh_or_revoked_exec = match trigger.maybe_xtx_id {
                Some(_xtx_id) => CircuitStatus::Ready,
                None => CircuitStatus::Requested,
            };
            // Setup: new xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                fresh_or_revoked_exec.clone(),
                &requester,
                Zero::zero(),
                trigger.maybe_xtx_id,
            )?;

            log::debug!(
                target: "runtime::circuit",
                "submit_side_effects xtx state with status: {:?}",
                local_xtx_ctx.xtx.status
            );

            // Charge: Ensure can afford
            // ToDo: Charge requester for contract with gas_estimation
            Self::charge(&requester, Zero::zero()).map_err(|_e| {
                if fresh_or_revoked_exec == CircuitStatus::Ready {
                    Self::kill(&mut local_xtx_ctx, CircuitStatus::RevertKill)
                }
                Error::<T>::ContractXtxKilledRunOutOfFunds
            })?;

            // ToDo: This should be converting the side effect from local trigger to FSE
            let side_effects = Self::exec_in_xtx_ctx(
                local_xtx_ctx.xtx_id,
                local_xtx_ctx.local_state.clone(),
                local_xtx_ctx.full_side_effects.clone(),
                local_xtx_ctx.xtx.steps_cnt,
            )
            .map_err(|_e| {
                if fresh_or_revoked_exec == CircuitStatus::Ready {
                    Self::kill(&mut local_xtx_ctx, CircuitStatus::RevertKill)
                }
                Error::<T>::ContractXtxKilledRunOutOfFunds
            })?;

            // ToDo: Align whether 3vm wants enfore side effects sequence into steps
            let sequential = false;
            // Validate: Side Effects
            Self::validate(&side_effects, &mut local_xtx_ctx, &requester, sequential)?;

            // Apply: all necessary changes to state in 1 go
            let (_, added_full_side_effects) = Self::apply(&mut local_xtx_ctx, None, None)?;

            // Emit: From Circuit events
            Self::emit(
                local_xtx_ctx.xtx_id,
                Some(local_xtx_ctx.xtx),
                &requester,
                &side_effects,
                added_full_side_effects,
            );

            Ok(())
        }

        fn on_signal(origin: &OriginFor<T>, signal: ExecutionSignal<T::Hash>) -> DispatchResult {
            log::debug!(target: "runtime::circuit", "Handling on_signal {:?}", signal);
            let requester = Self::authorize(origin.to_owned(), CircuitRole::ContractAuthor)?;

            <SignalQueue<T>>::mutate(|q| {
                q.try_push((requester, signal))
                    .map_err(|_| Error::<T>::SignalQueueFull)
            })?;
            Ok(())
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Used by other pallets that want to create the exec order
        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_local_trigger())]
        pub fn on_local_trigger(origin: OriginFor<T>, trigger: Vec<u8>) -> DispatchResult {
            <Self as OnLocalTrigger<T>>::on_local_trigger(
                &origin,
                LocalTrigger::<T>::decode(&mut &trigger[..])
                    .map_err(|_| Error::<T>::InsuranceBondNotRequired)?,
            )
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_local_trigger())]
        pub fn on_xcm_trigger(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // ToDo: Check TriggerAuthRights for local triggers
            unimplemented!();
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_local_trigger())]
        pub fn on_remote_gateway_trigger(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            unimplemented!();
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_extrinsic_trigger())]
        pub fn on_extrinsic_trigger(
            origin: OriginFor<T>,
            side_effects: Vec<
                SideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
            >,
            fee: EscrowedBalanceOf<T, T::Escrowed>,
            sequential: bool,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let requester = Self::authorize(origin, CircuitRole::Requester)?;
            // Charge: Ensure can afford
            Self::charge(&requester, fee)?;
            log::info!("on_extrinsic_trigger -- finished charged");

            // Setup: new xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> =
                Self::setup(CircuitStatus::Requested, &requester, fee, None)?;
            log::info!(
                "on_extrinsic_trigger -- finished setup -- xtx id {:?}",
                local_xtx_ctx.xtx_id
            );
            // Validate: Side Effects
            Self::validate(&side_effects, &mut local_xtx_ctx, &requester, sequential).map_err(
                |e| {
                    log::info!("Self::validate hit an error -- {:?}", e);
                    Error::<T>::SideEffectsValidationFailed
                },
            )?;
            log::info!("on_extrinsic_trigger -- finished validate");

            // Apply: all necessary changes to state in 1 go
            let (_, added_full_side_effects) = Self::apply(&mut local_xtx_ctx, None, None)?;
            log::info!("on_extrinsic_trigger -- finished apply");

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
            log::info!(
                "bond insurance deposit -- start -- xtx id {:?} + se id {:?}",
                xtx_id,
                side_effect_id
            );

            // Authorize: Retrieve sender of the transaction.
            let relayer = Self::authorize(origin, CircuitRole::Relayer)?;

            log::info!(
                "bond insurance deposit -- authorized -- xtx id {:?} + se id {:?}",
                xtx_id,
                side_effect_id
            );
            // Setup: retrieve local xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingInsurance,
                &relayer,
                Zero::zero(),
                Some(xtx_id),
            )?;

            log::info!("bond insurance deposit -- setup finished");
            let (maybe_xtx_changed, _) = if let Some((_id, insurance_deposit)) = local_xtx_ctx
                .insurance_deposits
                .iter_mut()
                .find(|(id, _)| *id == side_effect_id)
            {
                Self::charge(&relayer, insurance_deposit.insurance)?;

                log::info!("bond insurance deposit -- charged");
                insurance_deposit.bonded_relayer = Some(relayer.clone());
                // ToDo: Consider removing status from insurance_deposit since redundant with relayer: Option<Relayer>
                insurance_deposit.status = CircuitStatus::Bonded;

                let insurance_deposit_copy = insurance_deposit.clone();
                // Apply: all necessary changes to state in 1 go
                Self::apply(
                    &mut local_xtx_ctx,
                    Some((side_effect_id, insurance_deposit_copy)),
                    None,
                )
            } else {
                Err(Error::<T>::InsuranceBondNotRequired)
            }?;

            log::info!("bond insurance deposit -- applied");
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

        #[pallet::weight(<T as pallet::Config>::WeightInfo::execute_side_effects_with_xbi())]
        pub fn execute_side_effects_with_xbi(
            origin: OriginFor<T>, // Active relayer
            xtx_id: XExecSignalId<T>,
            side_effect: SideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, <T as Config>::Escrowed>,
            >,
            max_exec_cost: u128,
            max_notifications_cost: u128,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let executor = Self::authorize(origin, CircuitRole::Relayer)?;

            // Setup: retrieve local xtx context
            let local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingExecution,
                &executor,
                Zero::zero(),
                Some(xtx_id),
            )?;

            let side_effect_id = side_effect.generate_id::<SystemHashing<T>>();
            // Verify allowance for local execution
            let side_effect_link =
                <Self as Store>::LocalSideEffectsLinks::get(local_xtx_ctx.xtx_id, side_effect_id)
                    .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;
            let (step_no, maybe_assignee) =
                <Self as Store>::LocalSideEffects::get(local_xtx_ctx.xtx_id, side_effect_link)
                    .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;

            if local_xtx_ctx.xtx.steps_cnt.0 != step_no || maybe_assignee.is_some() {
                return Err(Error::<T>::LocalSideEffectExecutionNotApplicable.into())
            }

            let _encoded_4b_action: [u8; 4] =
                Decode::decode(&mut side_effect.encoded_action.encode().as_ref())
                    .expect("Encoded Type was already validated before saving");

            let xbi =
                sfx_2_xbi::<T, T::Escrowed>(
                    &side_effect,
                    XBIMetadata::new_with_default_timeouts(
                        Decode::decode(&mut &side_effect_id.encode()[..])
                            .expect("SFX ID at XBI conversion should always decode to H256"),
                        <T as Config>::Xdns::get_gateway_para_id(&side_effect.target)?,
                        T::SelfParaId::get(),
                        max_exec_cost,
                        max_notifications_cost,
                        Some(Decode::decode(&mut &executor.encode()[..]).expect(
                            "Executor at XBI conversion should always decode to AccountId32",
                        )),
                    ),
                )
                .map_err(|_| Error::<T>::FailedToConvertSFX2XBI)?;

            T::XBIPortal::do_check_in_xbi(xbi).map_err(|_| Error::<T>::FailedToConvertSFX2XBI)?;

            Ok(().into())
        }

        /// Blind version should only be used for testing - unsafe since skips inclusion proof check.
        #[pallet::weight(< T as Config >::WeightInfo::confirm_side_effect())]
        pub fn confirm_commit_revert_relay(
            origin: OriginFor<T>,
            xtx_id: XtxId<T>,
            side_effect: SideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            confirmation: ConfirmedSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            inclusion_proof: Option<Vec<Vec<u8>>>,
            block_hash: Option<Vec<u8>>,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let relayer = Self::authorize(origin, CircuitRole::Relayer)?;

            // Setup: retrieve local xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::Finished,
                &relayer,
                Zero::zero(),
                Some(xtx_id),
            )?;

            let mut updated_list: Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                >,
            > = vec![];

            if let Some(to_confirm_list) =
                <Self as Store>::EscrowedSideEffectsPendingRelay::get(local_xtx_ctx.xtx_id)
            {
                let side_effect_id = side_effect.generate_id::<SystemHashing<T>>();
                let mut found: Option<
                    FullSideEffect<
                        <T as frame_system::Config>::AccountId,
                        <T as frame_system::Config>::BlockNumber,
                        EscrowedBalanceOf<T, T::Escrowed>,
                    >,
                > = None;

                for to_confirm in to_confirm_list {
                    if to_confirm.input.generate_id::<SystemHashing<T>>() == side_effect_id {
                        found = Some(to_confirm);
                    } else {
                        updated_list.push(to_confirm);
                    }
                }
                if found.is_none() {
                    return Err(Error::<T>::RelayEscrowedFailedNothingToConfirm.into())
                }
            } else {
                return Err(Error::<T>::RelayEscrowedFailedNothingToConfirm.into())
            };

            let _status = Self::confirm(
                &mut local_xtx_ctx,
                &relayer,
                &side_effect,
                &confirmation,
                inclusion_proof,
                block_hash,
            )?;

            // Apply: all necessary changes to state in 1 go
            let (maybe_xtx_changed, assert_full_side_effects_changed) = Self::apply(
                &mut local_xtx_ctx,
                None,
                Some((
                    updated_list,
                    &side_effect,
                    &relayer,
                    CircuitStatus::Committed,
                )),
            )?;

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

        /// Blind version should only be used for testing - unsafe since skips inclusion proof check.
        #[pallet::weight(< T as Config >::WeightInfo::confirm_side_effect())]
        pub fn confirm_side_effect(
            origin: OriginFor<T>,
            xtx_id: XtxId<T>,
            side_effect: SideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            confirmation: ConfirmedSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            inclusion_proof: Option<Vec<Vec<u8>>>,
            block_hash: Option<Vec<u8>>,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let relayer = Self::authorize(origin, CircuitRole::Relayer)?;

            log::info!(
                "confirm side effect -- start -- xtx id {:?} + se id {:?}",
                xtx_id,
                side_effect.generate_id::<SystemHashing<T>>()
            );

            // Setup: retrieve local xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingExecution,
                &relayer,
                Zero::zero(),
                Some(xtx_id),
            )?;

            log::info!(
                "confirm side effect -- start -- xtx id {:?} + se id {:?}",
                xtx_id,
                side_effect.generate_id::<SystemHashing<T>>()
            );

            Self::confirm(
                &mut local_xtx_ctx,
                &relayer,
                &side_effect,
                &confirmation,
                inclusion_proof,
                block_hash,
            )?;
            log::info!(
                "confirm side effect -- confirmed -- xtx id {:?} + se id {:?}",
                xtx_id,
                side_effect.generate_id::<SystemHashing<T>>()
            );

            // Apply: all necessary changes to state in 1 go
            let (maybe_xtx_changed, assert_full_side_effects_changed) =
                Self::apply(&mut local_xtx_ctx, None, None)?;

            log::info!(
                "confirm side effect -- applied -- xtx id {:?} + se id {:?}",
                xtx_id,
                side_effect.generate_id::<SystemHashing<T>>()
            );

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

    use pallet_xbi_portal::xbi_abi::{
        AccountId20, AccountId32, AssetId, Data, Gas, Value, ValueEvm,
    };

    /// Events for the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        // XBI Exit events - consider moving to separate XBIPortalExit pallet.
        Transfer(T::AccountId, AccountId32, AccountId32, Value),
        TransferAssets(T::AccountId, AssetId, AccountId32, AccountId32, Value),
        TransferORML(T::AccountId, AssetId, AccountId32, AccountId32, Value),
        AddLiquidity(T::AccountId, AssetId, AssetId, Value, Value, Value),
        Swap(T::AccountId, AssetId, AssetId, Value, Value, Value),
        CallNative(T::AccountId, Data),
        CallEvm(
            T::AccountId,
            AccountId20,
            AccountId20,
            Value,
            Data,
            Gas,
            ValueEvm,
            Option<ValueEvm>,
            Option<ValueEvm>,
            Vec<(AccountId20, Vec<sp_core::H256>)>,
        ),
        CallWasm(T::AccountId, AccountId32, Value, Gas, Option<Value>, Data),
        CallCustom(
            T::AccountId,
            AccountId32,
            AccountId32,
            Value,
            Data,
            Gas,
            Data,
        ),
        Notification(T::AccountId, AccountId32, XBINotificationKind, Data, Data),
        Result(T::AccountId, AccountId32, XBICheckOutStatus, Data, Data),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and pending
        XTransactionReceivedForExec(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and ready
        XTransactionReadyForExec(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and finished
        XTransactionStepFinishedExec(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and finished
        XTransactionXtxFinishedExecAllSteps(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and finished
        XTransactionXtxRevertedAfterTimeOut(XExecSignalId<T>),
        // Listeners - executioners/relayers to know new challenges and perform offline risk/reward calc
        //  of whether side effect is worth picking up
        NewSideEffectsAvailable(
            <T as frame_system::Config>::AccountId,
            XExecSignalId<T>,
            Vec<
                SideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                >,
            >,
            Vec<SideEffectId<T>>,
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
                    EscrowedBalanceOf<T, T::Escrowed>,
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
                        EscrowedBalanceOf<T, T::Escrowed>,
                    >,
                >,
            >,
        ),
        EscrowTransfer(
            // ToDo: Inspect if Xtx needs to be here and how to process from protocol
            T::AccountId,                                  // from
            T::AccountId,                                  // to
            EscrowedBalanceOf<T, <T as Config>::Escrowed>, // value
        ),
    }

    #[pallet::error]
    pub enum Error<T> {
        ApplyTriggeredWithUnexpectedStatus,
        RequesterNotEnoughBalance,
        ContractXtxKilledRunOutOfFunds,
        ChargingTransferFailed,
        RewardTransferFailed,
        RefundTransferFailed,
        SideEffectsValidationFailed,
        InsuranceBondNotRequired,
        InsuranceBondAlreadyDeposited,
        SetupFailed,
        SetupFailedXtxNotFound,
        SetupFailedXtxStorageArtifactsNotFound,
        SetupFailedIncorrectXtxStatus,
        EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished,
        FatalXtxTimeoutXtxIdNotMatched,
        RelayEscrowedFailedNothingToConfirm,
        FatalCommitSideEffectWithoutConfirmationAttempt,
        FatalErroredCommitSideEffectConfirmationAttempt,
        FatalErroredRevertSideEffectConfirmationAttempt,
        SetupFailedUnknownXtx,
        FailedToHardenFullSideEffect,
        SetupFailedDuplicatedXtx,
        SetupFailedEmptyXtx,
        ApplyFailed,
        DeterminedForbiddenXtxStatus,
        LocalSideEffectExecutionNotApplicable,
        FailedToConvertSFX2XBI,
        FailedToConvertXBI2SFX,
        FailedToEnterXBIPortal,
        FailedToExitXBIPortal,
        UnsupportedRole,
        InvalidLocalTrigger,
        SignalQueueFull,
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
        reward: EscrowedBalanceOf<T, T::Escrowed>,
        xtx_id: Option<XExecSignalId<T>>,
    ) -> Result<LocalXtxCtx<T>, Error<T>> {
        match current_status {
            CircuitStatus::Requested => {
                if let Some(id) = xtx_id {
                    if <Self as Store>::XExecSignals::contains_key(id) {
                        return Err(Error::<T>::SetupFailedDuplicatedXtx)
                    }
                }
                // ToDo: Introduce default delay
                let (timeouts_at, delay_steps_at): (T::BlockNumber, Option<Vec<T::BlockNumber>>) = (
                    T::XtxTimeoutDefault::get() + frame_system::Pallet::<T>::block_number(),
                    None,
                );

                log::info!(
                    "New Xtx will timeout at: {:?} vs current block = {:?}",
                    timeouts_at,
                    frame_system::Pallet::<T>::block_number()
                );

                let (x_exec_signal_id, x_exec_signal) = XExecSignal::<
                    T::AccountId,
                    T::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                >::setup_fresh::<T>(
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
            },
            CircuitStatus::PendingInsurance => {
                if let Some(id) = xtx_id {
                    if !<Self as Store>::XExecSignals::contains_key(id) {
                        return Err(Error::<T>::SetupFailedUnknownXtx)
                    }
                    let xtx = <Self as Store>::XExecSignals::get(id)
                        .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;
                    // if xtx.status != CircuitStatus::PendingInsurance {
                    //     return Err(Error::<T>::SetupFailedIncorrectXtxStatus)
                    // }
                    let insurance_deposits = <Self as Store>::XtxInsuranceLinks::get(id)
                        .iter()
                        .map(|&se_id| {
                            (
                                se_id,
                                <Self as Store>::InsuranceDeposits::get(id, se_id)
                                    .expect("Should not be state inconsistency"),
                            )
                        })
                        .collect::<Vec<(
                            SideEffectId<T>,
                            InsuranceDeposit<
                                T::AccountId,
                                T::BlockNumber,
                                EscrowedBalanceOf<T, T::Escrowed>,
                            >,
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
            },
            CircuitStatus::Ready
            | CircuitStatus::PendingExecution
            | CircuitStatus::Finished
            | CircuitStatus::RevertTimedOut => {
                if let Some(id) = xtx_id {
                    let xtx = <Self as Store>::XExecSignals::get(id)
                        .ok_or(Error::<T>::SetupFailedUnknownXtx)?;
                    // Make sure in case of commit_relay to only check finished Xtx
                    if current_status == CircuitStatus::Finished
                        && xtx.status < CircuitStatus::Finished
                    {
                        log::debug!(
                            "Incorrect status current_status: {:?} xtx_status {:?}",
                            current_status,
                            xtx.status
                        );
                        return Err(Error::<T>::SetupFailedIncorrectXtxStatus)
                    }
                    let insurance_deposits = <Self as Store>::XtxInsuranceLinks::get(id)
                        .iter()
                        .map(|&se_id| {
                            (
                                se_id,
                                <Self as Store>::InsuranceDeposits::get(id, se_id)
                                    .expect("Should not be state inconsistency"),
                            )
                        })
                        .collect::<Vec<(
                            SideEffectId<T>,
                            InsuranceDeposit<
                                T::AccountId,
                                T::BlockNumber,
                                EscrowedBalanceOf<T, T::Escrowed>,
                            >,
                        )>>();

                    let full_side_effects = <Self as Store>::FullSideEffects::get(id)
                        .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;
                    let local_state = <Self as Store>::LocalXtxStates::get(id)
                        .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;

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
            },
            _ => unimplemented!(),
        }
    }

    /// Returns: Returns changes written to the state if there are any.
    ///     For now only returns Xtx and FullSideEffects that changed.
    fn apply(
        local_ctx: &mut LocalXtxCtx<T>,
        maybe_insurance_tuple: Option<(
            SideEffectId<T>,
            InsuranceDeposit<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        )>,
        maybe_escrowed_confirmation: Option<(
            Vec<FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>>,
            &SideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
            &T::AccountId,
            CircuitStatus,
        )>,
    ) -> Result<
        (
            Option<XExecSignal<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>>,
            Option<
                Vec<
                    Vec<
                        FullSideEffect<
                            T::AccountId,
                            T::BlockNumber,
                            EscrowedBalanceOf<T, T::Escrowed>,
                        >,
                    >,
                >,
            >,
        ),
        Error<T>,
    > {
        // Apply will try to move the status of Xtx from the current to the closest valid one.
        let current_status = local_ctx.xtx.status.clone();

        match current_status {
            CircuitStatus::Requested => {
                <FullSideEffects<T>>::insert::<
                    XExecSignalId<T>,
                    Vec<
                        Vec<
                            FullSideEffect<
                                T::AccountId,
                                T::BlockNumber,
                                EscrowedBalanceOf<T, T::Escrowed>,
                            >,
                        >,
                    >,
                >(local_ctx.xtx_id, local_ctx.full_side_effects.clone());

                // Iterate over full side effects to detect ones to execute locally.
                fn is_local<T: Config>(gateway_id: &[u8; 4]) -> bool {
                    if *gateway_id == T::SelfGatewayId::get() {
                        return true
                    }
                    let gateway_type = <T as Config>::Xdns::get_gateway_type_unsafe(gateway_id);
                    gateway_type == GatewayType::ProgrammableInternal(0)
                }

                let steps_side_effects_ids: Vec<(
                    usize,
                    SideEffectId<T>,
                    XExecStepSideEffectId<T>,
                )> = local_ctx
                    .full_side_effects
                    .clone()
                    .iter()
                    .enumerate()
                    .flat_map(|(cnt, fse)| {
                        fse.iter()
                            .map(|full_side_effect| full_side_effect.input.clone())
                            .filter(|side_effect| is_local::<T>(&side_effect.target))
                            .map(|side_effect| side_effect.generate_id::<SystemHashing<T>>())
                            .map(|side_effect_hash| {
                                (
                                    cnt,
                                    side_effect_hash,
                                    XExecSignal::<
                                        T::AccountId,
                                        T::BlockNumber,
                                        EscrowedBalanceOf<T, <T as Config>::Escrowed>,
                                    >::generate_step_id::<T>(
                                        side_effect_hash, cnt
                                    ),
                                )
                            })
                            .collect::<Vec<(usize, SideEffectId<T>, XExecStepSideEffectId<T>)>>()
                    })
                    .collect();

                for (step_cnt, side_effect_id, step_side_effect_id) in steps_side_effects_ids {
                    <LocalSideEffects<T>>::insert::<
                        XExecSignalId<T>,
                        XExecStepSideEffectId<T>,
                        (u32, Option<T::AccountId>),
                    >(
                        local_ctx.xtx_id,
                        step_side_effect_id,
                        (step_cnt as u32, None),
                    );
                    <LocalSideEffectsLinks<T>>::insert::<
                        XExecSignalId<T>,
                        SideEffectId<T>,
                        XExecStepSideEffectId<T>,
                    >(local_ctx.xtx_id, side_effect_id, step_side_effect_id);
                }

                let mut ids_with_insurance: Vec<SideEffectId<T>> = vec![];
                for (side_effect_id, insurance_deposit) in &local_ctx.insurance_deposits {
                    <InsuranceDeposits<T>>::insert::<
                        XExecSignalId<T>,
                        SideEffectId<T>,
                        InsuranceDeposit<
                            T::AccountId,
                            T::BlockNumber,
                            EscrowedBalanceOf<T, T::Escrowed>,
                        >,
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
                local_ctx.xtx.steps_cnt = (0, local_ctx.full_side_effects.len() as u32);

                <ActiveXExecSignalsTimingLinks<T>>::insert::<XExecSignalId<T>, T::BlockNumber>(
                    local_ctx.xtx_id,
                    local_ctx.xtx.timeouts_at,
                );

                <XExecSignals<T>>::insert::<
                    XExecSignalId<T>,
                    XExecSignal<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
                >(local_ctx.xtx_id, local_ctx.xtx.clone());

                Ok((
                    Some(local_ctx.xtx.clone()),
                    Some(local_ctx.full_side_effects.to_vec()),
                ))
            },
            CircuitStatus::PendingInsurance => {
                if let Some((side_effect_id, insurance_deposit)) = maybe_insurance_tuple {
                    <Self as Store>::InsuranceDeposits::mutate(
                        local_ctx.xtx_id,
                        side_effect_id,
                        |x| *x = Some(insurance_deposit),
                    );
                    let new_status = CircuitStatus::determine_effects_insurance_status::<T>(
                        &local_ctx.insurance_deposits,
                    );

                    if new_status != local_ctx.xtx.status {
                        local_ctx.xtx.status = new_status;

                        <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                            *x = Some(local_ctx.xtx.clone())
                        });
                        Ok((Some(local_ctx.xtx.clone()), None))
                    } else {
                        Ok((None, None))
                    }
                } else {
                    Err(Error::<T>::ApplyFailed)
                }
            },
            CircuitStatus::RevertTimedOut => {
                <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                <Self as Store>::ActiveXExecSignalsTimingLinks::remove(local_ctx.xtx_id);

                Self::enact_step_side_effects(local_ctx)?;

                Ok((
                    Some(local_ctx.xtx.clone()),
                    Some(local_ctx.full_side_effects.clone()),
                ))
            },
            CircuitStatus::Ready
            | CircuitStatus::Bonded
            | CircuitStatus::PendingExecution
            | CircuitStatus::Finished => {
                // Update set of full side effects assuming the new confirmed has appeared
                <Self as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.full_side_effects.clone())
                });

                let new_status = CircuitStatus::determine_xtx_status::<T>(
                    &local_ctx.full_side_effects,
                    &local_ctx.insurance_deposits,
                )?;

                local_ctx.xtx.status = new_status;
                // Check whether all of the side effects in this steps are confirmed - the status now changes to CircuitStatus::Finished
                if local_ctx.full_side_effects[local_ctx.xtx.steps_cnt.0 as usize]
                    .clone()
                    .iter()
                    .filter(|&fse| fse.confirmed.is_none())
                    .next()
                    .is_none()
                {
                    local_ctx.xtx.steps_cnt =
                        (local_ctx.xtx.steps_cnt.0 + 1, local_ctx.xtx.steps_cnt.1);

                    local_ctx.xtx.status = CircuitStatus::Finished;

                    // All of the steps are completed - the xtx has been finalized
                    if local_ctx.xtx.steps_cnt.0 == local_ctx.xtx.steps_cnt.1 {
                        local_ctx.xtx.status = CircuitStatus::FinishedAllSteps;
                        <Self as Store>::ActiveXExecSignalsTimingLinks::remove(local_ctx.xtx_id);
                        Self::enact_step_side_effects(local_ctx)?
                    }
                }
                <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                if local_ctx.xtx.status.clone() > CircuitStatus::Ready {
                    Ok((
                        Some(local_ctx.xtx.clone()),
                        Some(local_ctx.full_side_effects.clone()),
                    ))
                } else {
                    Ok((None, Some(local_ctx.full_side_effects.to_vec())))
                }
            },
            // Fires only for confirmation of escrowed execution on remote targets
            CircuitStatus::FinishedAllSteps => {
                if let Some((side_effects, side_effect, relayer, completion_status)) =
                    maybe_escrowed_confirmation
                {
                    <Self as Store>::EscrowedSideEffectsPendingRelay::mutate(
                        local_ctx.xtx_id,
                        |s| *s = Some(side_effects.clone()),
                    );

                    Self::reward_escrow_relayer(relayer, side_effect)?;

                    if side_effects.is_empty() {
                        local_ctx.xtx.status = completion_status;
                        <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                            *x = Some(local_ctx.xtx.clone())
                        });
                        Ok((
                            Some(local_ctx.xtx.clone()),
                            Some(local_ctx.full_side_effects.clone()),
                        ))
                    } else {
                        Ok((None, Some(local_ctx.full_side_effects.to_vec())))
                    }
                } else {
                    Err(Error::<T>::ApplyTriggeredWithUnexpectedStatus)
                }
            },
            _ => Err(Error::<T>::ApplyTriggeredWithUnexpectedStatus),
        }
    }

    fn emit(
        xtx_id: XExecSignalId<T>,
        maybe_xtx: Option<
            XExecSignal<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        >,
        subjected_account: &T::AccountId,
        side_effects: &Vec<
            SideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        >,
        maybe_full_side_effects: Option<
            Vec<
                Vec<
                    FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
                >,
            >,
        >,
    ) {
        if !side_effects.is_empty() {
            Self::deposit_event(Event::NewSideEffectsAvailable(
                subjected_account.clone(),
                xtx_id,
                // ToDo: Emit circuit outbound messages -> side effects
                side_effects.to_vec(),
                side_effects
                    .iter()
                    .map(|se| se.generate_id::<SystemHashing<T>>())
                    .collect::<Vec<SideEffectId<T>>>(),
            ));
        }
        if let Some(xtx) = maybe_xtx {
            match xtx.status {
                CircuitStatus::PendingInsurance =>
                    Self::deposit_event(Event::XTransactionReceivedForExec(xtx_id)),
                CircuitStatus::Ready =>
                    Self::deposit_event(Event::XTransactionReadyForExec(xtx_id)),
                CircuitStatus::Finished =>
                    Self::deposit_event(Event::XTransactionStepFinishedExec(xtx_id)),
                CircuitStatus::FinishedAllSteps =>
                    Self::deposit_event(Event::XTransactionXtxFinishedExecAllSteps(xtx_id)),
                CircuitStatus::RevertTimedOut =>
                    Self::deposit_event(Event::XTransactionXtxRevertedAfterTimeOut(xtx_id)),
                _ => {},
            }
            if xtx.status >= CircuitStatus::PendingExecution {
                if let Some(full_side_effects) = maybe_full_side_effects {
                    Self::deposit_event(Event::SideEffectsConfirmed(xtx_id, full_side_effects));
                }
            }
        }
    }

    fn kill(local_ctx: &mut LocalXtxCtx<T>, cause: CircuitStatus) {
        local_ctx.xtx.status = cause;
        Self::apply(local_ctx, None, None)
            .expect("Panic: apply triggered by panic should never fail");
    }

    fn charge(
        requester: &T::AccountId,
        fee: EscrowedBalanceOf<T, T::Escrowed>,
    ) -> Result<EscrowedBalanceOf<T, T::Escrowed>, Error<T>> {
        let available_trn_balance = EscrowCurrencyOf::<T>::free_balance(requester);
        let new_balance = available_trn_balance.saturating_sub(fee);
        let vault: T::AccountId = Self::account_id();
        EscrowCurrencyOf::<T>::transfer(requester, &vault, fee, AllowDeath)
            .map_err(|_| Error::<T>::ChargingTransferFailed)?; // should not fail
        Ok(new_balance)
    }

    fn enact_step_side_effects(local_ctx: &mut LocalXtxCtx<T>) -> Result<(), Error<T>> {
        let current_step = local_ctx.xtx.steps_cnt.0;
        let mut escrowed_to_confirm: Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
        > = vec![];

        match local_ctx.xtx.status {
            CircuitStatus::RevertTimedOut | CircuitStatus::Reverted => {
                if let Some(outer) = local_ctx.full_side_effects.get(current_step as usize) {
                    for fse in outer {
                        let encoded_4b_action: [u8; 4] =
                            Decode::decode(&mut fse.input.encoded_action.encode().as_ref())
                                .expect("Encoded Type was already validated before saving");

                        let confirmed = fse.confirmed.clone().unwrap_or(ConfirmedSideEffect {
                            err: Some(ConfirmationOutcome::TimedOut),
                            output: None,
                            encoded_effect: vec![0],
                            inclusion_proof: None,
                            executioner: Self::account_id(),
                            received_at: <frame_system::Pallet<T>>::block_number(),
                            cost: None,
                        });
                        match fse.security_lvl {
                            SecurityLvl::Optimistic => {
                                let _ = Self::enact_insurance(
                                    local_ctx,
                                    &fse.input,
                                    InsuranceEnact::RefundBoth,
                                )?;
                            },
                            SecurityLvl::Escrowed => {
                                if fse.input.target == T::SelfGatewayId::get() {
                                    Escrow::<T>::revert(
                                        &encoded_4b_action,
                                        fse.input.encoded_args.clone(),
                                        Self::account_id(),
                                        confirmed.executioner.clone(),
                                    )
                                    .map_err(|_| {
                                        Error::<T>::FatalErroredRevertSideEffectConfirmationAttempt
                                    })?
                                }
                                escrowed_to_confirm.push(fse.clone());
                            },
                            SecurityLvl::Dirty => {},
                        }
                    }
                }
            },
            CircuitStatus::Finished | CircuitStatus::FinishedAllSteps => {
                if current_step == 0 {
                    return Err(Error::<T>::EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished)
                }
                for fse in &local_ctx.full_side_effects[(current_step - 1) as usize] {
                    let encoded_4b_action: [u8; 4] =
                        Decode::decode(&mut fse.input.encoded_action.encode().as_ref())
                            .expect("Encoded Type was already validated before saving");
                    // Perhaps redundant check for data integrity - make sure the confirmation is there & not an error
                    let confirmation = if let Some(ref confirmed) = fse.confirmed {
                        if confirmed.err.is_some() {
                            return Err(Error::<T>::FatalErroredCommitSideEffectConfirmationAttempt)
                        }
                        confirmed.clone()
                    } else {
                        return Err(Error::<T>::FatalCommitSideEffectWithoutConfirmationAttempt)
                    };
                    match fse.security_lvl {
                        SecurityLvl::Optimistic => {
                            let _ = Self::enact_insurance(
                                local_ctx,
                                &fse.input,
                                InsuranceEnact::Reward,
                            )?;
                        },
                        SecurityLvl::Escrowed => {
                            if fse.input.target == T::SelfGatewayId::get() {
                                Escrow::<T>::commit(
                                    &encoded_4b_action,
                                    fse.input.encoded_args.clone(),
                                    Self::account_id(),
                                    confirmation.executioner.clone(),
                                )
                                .map_err(|_| {
                                    Error::<T>::FatalErroredCommitSideEffectConfirmationAttempt
                                })?
                            }
                            escrowed_to_confirm.push(fse.clone());
                        },
                        SecurityLvl::Dirty => {},
                    }
                }
            },
            _ => {},
        }

        if !escrowed_to_confirm.is_empty() {
            <Self as Store>::EscrowedSideEffectsPendingRelay::insert(
                local_ctx.xtx_id,
                escrowed_to_confirm,
            );
        }

        Ok(())
    }

    fn reward_escrow_relayer(
        relayer: &T::AccountId,
        side_effect: &SideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
    ) -> Result<(), Error<T>> {
        // Reward insurance
        EscrowCurrencyOf::<T>::transfer(&Self::account_id(), relayer, side_effect.prize, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed) // should not fail
    }

    fn enact_insurance(
        local_ctx: &LocalXtxCtx<T>,
        side_effect: &SideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        enact_status: InsuranceEnact,
    ) -> Result<bool, Error<T>> {
        let side_effect_id = side_effect.generate_id::<SystemHashing<T>>();
        // Reward insurance
        // Check if the side effect was insured and if the relayer matches the bonded one
        return if let Some((_id, insurance_request)) = local_ctx
            .insurance_deposits
            .iter()
            .find(|(id, _)| *id == side_effect_id)
        {
            if let Some(bonded_relayer) = &insurance_request.bonded_relayer {
                match enact_status {
                    InsuranceEnact::Reward => {
                        // Reward relayer with and give back his insurance from Vault
                        EscrowCurrencyOf::<T>::transfer(
                            &Self::account_id(),
                            bonded_relayer,
                            insurance_request.insurance + insurance_request.reward,
                            AllowDeath,
                        )
                        .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail
                    },
                    InsuranceEnact::RefundBoth => {
                        EscrowCurrencyOf::<T>::transfer(
                            &Self::account_id(),
                            &insurance_request.requester,
                            insurance_request.reward,
                            AllowDeath,
                        )
                        .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail

                        EscrowCurrencyOf::<T>::transfer(
                            &Self::account_id(),
                            bonded_relayer,
                            insurance_request.insurance,
                            AllowDeath,
                        )
                        .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail
                    },
                    InsuranceEnact::RefundAndPunish => {
                        EscrowCurrencyOf::<T>::transfer(
                            &Self::account_id(),
                            &insurance_request.requester,
                            insurance_request.reward,
                            AllowDeath,
                        )
                        .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail
                    },
                }
            } else {
                // This is a forbidden state which should have not happened -
                //  at this point all of the insurances should have a bonded relayer assigned
                return Err(Error::<T>::RefundTransferFailed)
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn authorize(
        origin: OriginFor<T>,
        role: CircuitRole,
    ) -> Result<T::AccountId, sp_runtime::traits::BadOrigin> {
        match role {
            CircuitRole::Requester | CircuitRole::ContractAuthor => ensure_signed(origin),
            // ToDo: Handle active Relayer authorisation
            CircuitRole::Relayer => ensure_signed(origin),
            // ToDo: Handle other CircuitRoles
            _ => unimplemented!(),
        }
    }

    fn validate(
        side_effects: &[SideEffect<
            T::AccountId,
            T::BlockNumber,
            EscrowedBalanceOf<T, T::Escrowed>,
        >],
        local_ctx: &mut LocalXtxCtx<T>,
        requester: &T::AccountId,
        _sequential: bool,
    ) -> Result<(), &'static str> {
        let mut full_side_effects: Vec<
            FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        > = vec![];

        for side_effect in side_effects.iter() {
            let gateway_abi = <T as Config>::Xdns::get_abi(side_effect.target)?;
            let allowed_side_effects =
                <T as Config>::Xdns::allowed_side_effects(&side_effect.target);

            log::info!("validate -- prize decoded {:?}", side_effect.prize.clone());
            log::info!("validate -- prize encode {:?}", side_effect.prize.encode());

            local_ctx
                .use_protocol
                .notice_gateway(side_effect.target, allowed_side_effects);
            local_ctx
                .use_protocol
                .validate_args::<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>, SystemHashing<T>>(
                    side_effect.clone(),
                    gateway_abi,
                    &mut local_ctx.local_state,
                )?;

            if let Some(insurance_and_reward) =
                UniversalSideEffectsProtocol::check_if_insurance_required::<
                    T::AccountId,
                    T::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                    SystemHashing<T>,
                >(side_effect.clone(), &mut local_ctx.local_state)?
            {
                let (insurance, reward) = (insurance_and_reward[0], insurance_and_reward[1]);
                log::info!(
                    "circuit -- validation passed and discovered opt insurance {:?} reward {:?}",
                    insurance,
                    reward
                );

                log::info!(
                    "circuit -- for side effect id {:?}",
                    side_effect.generate_id::<SystemHashing<T>>()
                );
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
                let submission_target_height = T::CircuitPortal::read_cmp_latest_target_height(
                    side_effect.target,
                    None,
                    None,
                )?;

                full_side_effects.push(FullSideEffect {
                    input: side_effect.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height,
                })
            } else {
                fn determine_dirty_vs_escrowed_lvl<T: Config>(
                    side_effect: &SideEffect<
                        <T as frame_system::Config>::AccountId,
                        <T as frame_system::Config>::BlockNumber,
                        EscrowedBalanceOf<T, T::Escrowed>,
                    >,
                ) -> SecurityLvl {
                    fn is_escrowed<T: Config>(chain_id: &ChainId) -> bool {
                        let gateway_type = <T as Config>::Xdns::get_gateway_type_unsafe(chain_id);
                        gateway_type == GatewayType::ProgrammableInternal(0)
                            || gateway_type == GatewayType::OnCircuit(0)
                    }
                    if is_escrowed::<T>(&side_effect.target) {
                        return SecurityLvl::Escrowed
                    }
                    SecurityLvl::Dirty
                }
                let submission_target_height = T::CircuitPortal::read_cmp_latest_target_height(
                    side_effect.target,
                    None,
                    None,
                )?;

                full_side_effects.push(FullSideEffect {
                    input: side_effect.clone(),
                    confirmed: None,
                    security_lvl: determine_dirty_vs_escrowed_lvl::<T>(side_effect),
                    submission_target_height,
                });
            }
        }

        // Circuit's automatic side effect ordering: execute escrowed asap, then line up optimistic ones
        full_side_effects.sort_by(|a, b| b.security_lvl.partial_cmp(&a.security_lvl).unwrap());

        let mut full_side_effects_steps: Vec<
            Vec<FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>>,
        > = vec![vec![]];

        for sorted_fse in full_side_effects {
            let current_step = full_side_effects_steps
                .last_mut()
                .expect("Vector initialized at declaration");

            // Push to the single step as long as there's no Dirty side effect
            if sorted_fse.security_lvl != SecurityLvl::Dirty
                // Or if there was no Optimistic/Escrow side effects before
                || sorted_fse.security_lvl == SecurityLvl::Dirty && current_step.is_empty()
            {
                current_step.push(sorted_fse);
            } else if sorted_fse.security_lvl == SecurityLvl::Dirty {
                // R#1: there only can be max 1 dirty side effect at each step.
                full_side_effects_steps.push(vec![sorted_fse])
            }
        }

        local_ctx.full_side_effects = full_side_effects_steps;

        Ok(())
    }

    fn confirm(
        local_ctx: &mut LocalXtxCtx<T>,
        _relayer: &T::AccountId,
        side_effect: &SideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, T::Escrowed>,
        >,
        confirmation: &ConfirmedSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
        inclusion_proof: Option<Vec<Vec<u8>>>,
        block_hash: Option<Vec<u8>>,
    ) -> Result<(), &'static str> {
        let confirm_inclusion = |fsx: &FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, T::Escrowed>,
        >| {
            // ToDo: Remove below after testing inclusion
            // Temporarily allow skip inclusion if proofs aren't provided
            if !(block_hash.is_none() && inclusion_proof.is_none()) {
                <T as Config>::CircuitPortal::confirm_event_inclusion(
                    side_effect.target,
                    confirmation.encoded_effect.clone(),
                    fsx.submission_target_height.clone(),
                    inclusion_proof,
                    block_hash,
                )
            } else {
                Ok(())
            }
        };

        let confirm_execution = |gateway_vendor,
                                 value_abi_unsigned_type,
                                 state_copy,
                                 security_lvl,
                                 security_coordinates| {
            let mut side_effect_id: [u8; 4] = [0, 0, 0, 0];
            side_effect_id.copy_from_slice(&side_effect.encoded_action[0..4]);
            let side_effect_interface =
                <T as Config>::Xdns::fetch_side_effect_interface(side_effect_id);

            // I guess this could be omitted, as SE submission would prevent this?
            if let Err(msg) = side_effect_interface {
                return Err(msg)
            }

            confirm_with_vendor::<
                T,
                SubstrateSideEffectsParser,
                EthereumSideEffectsParser<
                    <<T as Config>::CircuitPortal as CircuitPortal<T>>::EthVerifier,
                >,
                SubstrateSideEffectsParser,
            >(
                gateway_vendor,
                value_abi_unsigned_type,
                &Box::new(side_effect_interface.unwrap()),
                confirmation.encoded_effect.clone().into(),
                state_copy,
                Some(
                    side_effect
                        .generate_id::<SystemHashing<T>>()
                        .as_ref()
                        .to_vec()
                        .into(),
                ),
                security_lvl,
                security_coordinates,
            )
        };

        fn confirm_order<T: Config>(
            side_effect: &SideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            confirmation: &ConfirmedSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            full_side_effects: &mut [Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                >,
            >],
        ) -> Result<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            &'static str,
        > {
            // ToDo: Extract as a separate function and migrate tests from Xtx
            let input_side_effect_id = side_effect.generate_id::<SystemHashing<T>>();
            let mut unconfirmed_step_no: Option<usize> = None;

            for (i, step) in full_side_effects.iter_mut().enumerate() {
                // Double check there are some side effects for that Xtx - should have been checked at API level tho already
                if step.is_empty() {
                    return Err("Xtx has an empty single step.")
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
                            Ok(full_side_effect.clone())
                        } else {
                            Err("Attempt to confirm side effect from the next step, \
                                    but there still is at least one unfinished step")
                        }
                    }
                }
            }
            Err("Unable to find matching Side Effect in given Xtx to confirm")
        }

        let fsx = confirm_order::<T>(side_effect, confirmation, &mut local_ctx.full_side_effects)?;
        confirm_inclusion(&fsx)?;
        confirm_execution(
            <T as Config>::Xdns::best_available(side_effect.target)?.gateway_vendor,
            <T as Config>::Xdns::get_gateway_value_unsigned_type_unsafe(&side_effect.target),
            &local_ctx.local_state,
            fsx.security_lvl,
            <T as Config>::Xdns::get_gateway_security_coordinates(&side_effect.target)?,
        )?;
        Ok(())
    }

    // ToDo: This should be called as a 3vm trait injection @Don
    pub fn exec_in_xtx_ctx(
        _xtx_id: T::Hash,
        _local_state: LocalState,
        _full_side_effects: Vec<
            Vec<FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>>,
        >,
        _steps_cnt: (u32, u32),
    ) -> Result<
        Vec<SideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>>,
        &'static str,
    > {
        Ok(vec![])
    }

    /// The account ID of the Circuit Vault.
    pub fn account_id() -> T::AccountId {
        <T as Config>::PalletId::get().into_account()
    }

    // TODO: we also want to save some space for timeouts, split the weight distribution 50-50
    pub(crate) fn process_signal_queue() -> Weight {
        let queue_len = <SignalQueue<T>>::decode_len().unwrap_or(0);
        if queue_len == 0 {
            return 0
        }
        let db_weight = T::DbWeight::get();

        let mut queue = <SignalQueue<T>>::get();

        // We can do an easy process and only process CONSTANT / something signals for now
        let mut remaining_key_budget = T::SignalQueueDepth::get() / 4;
        let mut processed_weight = 0;

        while !queue.is_empty() && remaining_key_budget > 0 {
            // Cannot panic due to loop condition
            let (requester, signal) = &mut queue[0];

            let intended_status = match signal.kind {
                SignalKind::Complete => CircuitStatus::Finished, // Fails bc no executor tried to execute, maybe a new enum?
                SignalKind::Kill(_) => CircuitStatus::RevertKill,
            };

            // worst case 4 from setup
            processed_weight += db_weight.reads(4 as Weight);
            match Self::setup(
                CircuitStatus::Ready,
                &requester,
                Zero::zero(),
                Some(signal.execution_id),
            ) {
                Ok(mut local_xtx_ctx) => {
                    Self::kill(&mut local_xtx_ctx, intended_status);

                    queue.swap_remove(0);

                    remaining_key_budget -= 1;
                    // apply has 2
                    processed_weight += db_weight.reads_writes(2 as Weight, 1 as Weight);
                },
                Err(_err) => {
                    log::error!("Could not handle signal");
                    // Slide the erroneous signal to the back
                    queue.slide(0, queue.len());
                },
            }
        }
        // Initial read of queue and update
        processed_weight += db_weight.reads_writes(1 as Weight, 1 as Weight);

        <SignalQueue<T>>::put(queue);

        processed_weight
    }

    pub fn do_xbi_exit(
        xbi_checkin: pallet_xbi_portal::xbi_format::XBICheckIn<T::BlockNumber>,
        _xbi_checkout: pallet_xbi_portal::xbi_format::XBICheckOut,
    ) -> Result<(), Error<T>> {
        // todo#1: recover xtx_id by sfx_id

        // todo#2: local fail Xtx if xbi_checkout::result errored

        // todo#3: load local_ctx with self::setup if xtx available
        let escrow_source = Self::account_id();
        let executor = if let Some(known_origin) = xbi_checkin.xbi.metadata.maybe_known_origin {
            known_origin
        } else {
            return Err(Error::<T>::FailedToExitXBIPortal)
        };
        let xbi_exit_event = match xbi_checkin.xbi.instr {
            XBIInstr::CallNative { payload } => Ok(Event::<T>::CallNative(escrow_source, payload)),
            XBIInstr::CallEvm {
                source,
                dest,
                value,
                input,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                nonce,
                access_list,
            } => Ok(Event::<T>::CallEvm(
                escrow_source,
                source,
                dest,
                value,
                input,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                nonce,
                access_list,
            )),
            XBIInstr::CallWasm {
                dest,
                value,
                gas_limit,
                storage_deposit_limit,
                data,
            } => Ok(Event::<T>::CallWasm(
                escrow_source,
                dest,
                value,
                gas_limit,
                storage_deposit_limit,
                data,
            )),
            XBIInstr::CallCustom {
                caller,
                dest,
                value,
                input,
                limit,
                additional_params,
            } => Ok(Event::<T>::CallCustom(
                escrow_source,
                caller,
                dest,
                value,
                input,
                limit,
                additional_params,
            )),
            XBIInstr::Transfer { dest, value } =>
                Ok(Event::<T>::Transfer(escrow_source, executor, dest, value)),
            XBIInstr::TransferORML {
                currency_id,
                dest,
                value,
            } => Ok(Event::<T>::TransferORML(
                escrow_source,
                currency_id,
                executor,
                dest,
                value,
            )),
            XBIInstr::TransferAssets {
                currency_id,
                dest,
                value,
            } => Ok(Event::<T>::TransferAssets(
                escrow_source,
                currency_id,
                executor,
                dest,
                value,
            )),
            XBIInstr::Result {
                outcome,
                output,
                witness,
            } => Ok(Event::<T>::Result(
                escrow_source,
                executor,
                outcome,
                output,
                witness,
            )),
            XBIInstr::Notification {
                kind,
                instruction_id,
                extra,
            } => Ok(Event::<T>::Notification(
                escrow_source,
                executor,
                kind,
                instruction_id,
                extra,
            )),
            _ => Err(Error::<T>::FailedToExitXBIPortal),
        }?;

        Self::deposit_event(xbi_exit_event);

        Ok(())
    }
}
