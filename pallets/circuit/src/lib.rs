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
#![feature(box_syntax)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub use crate::pallet::*;
use crate::{optimistic::Optimistic, state::*};
use codec::{Decode, Encode};
use frame_support::{
    dispatch::{Dispatchable, GetDispatchInfo},
    traits::{Currency, ExistenceRequirement::AllowDeath, Get, ReservableCurrency},
    weights::Weight,
    RuntimeDebug,
};
use frame_system::{
    ensure_signed,
    offchain::{SignedPayload, SigningTypes},
    pallet_prelude::OriginFor,
};
use pallet_xbi_portal::{
    primitives::xbi::XBIPortal,
    xbi_format::{XBICheckIn, XBICheckOut, XBIInstr},
};
use pallet_xbi_portal_enter::t3rn_sfx::xbi_result_2_sfx_confirmation;
use sp_runtime::{traits::Zero, KeyTypeId};
use sp_std::{boxed::Box, convert::TryInto, vec, vec::Vec};

pub use t3rn_primitives::{
    abi::{GatewayABIConfig, HasherAlgo as HA, Type},
    account_manager::{AccountManager, Outcome},
    circuit::{XExecSignalId, XExecStepSideEffectId},
    circuit_portal::CircuitPortal,
    claimable::{BenefitSource, CircuitRole},
    executors::Executors,
    portal::Portal,
    side_effect::{
        ConfirmedSideEffect, FullSideEffect, HardenedSideEffect, SFXBid, SecurityLvl, SideEffect,
        SideEffectId,
    },
    transfers::EscrowedBalanceOf,
    volatile::LocalState,
    xdns::Xdns,
    xtx::{Xtx, XtxId},
    GatewayType, *,
};

use t3rn_protocol::side_effects::{
    confirm::protocol::*,
    loader::{SideEffectsLazyLoader, UniversalSideEffectsProtocol},
};

pub use t3rn_protocol::{circuit_inbound::StepConfirmation, merklize::*};
pub use t3rn_sdk_primitives::signal::{ExecutionSignal, SignalKind};

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod escrow;
pub mod optimistic;
pub mod state;
pub mod weights;

/// Defines application identifier for crypto keys of this module.
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

pub type SystemHashing<T> = <T as frame_system::Config>::Hashing;
pub type EscrowCurrencyOf<T> = <<T as pallet::Config>::Escrowed as EscrowTrait<T>>::Currency;

type BalanceOf<T> = EscrowBalance<T>;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Inspect, Mutate},
            Get,
        },
    };
    use frame_system::pallet_prelude::*;
    use pallet_xbi_portal::xbi_codec::{XBICheckOutStatus, XBIMetadata, XBINotificationKind};
    use pallet_xbi_portal_enter::t3rn_sfx::sfx_2_xbi;
    use sp_runtime::traits::Hash;

    use pallet_xbi_portal::{
        primitives::xbi::{XBIPromise, XBIStatus},
        sabi::Sabi,
    };
    use sp_std::borrow::ToOwned;

    use t3rn_primitives::{
        circuit::{LocalStateExecutionView, LocalTrigger, OnLocalTrigger},
        portal::Portal,
        xdns::Xdns,
    };

    pub use crate::weights::WeightInfo;

    pub type EscrowBalance<T> = EscrowedBalanceOf<T, <T as Config>::Escrowed>;

    /// Temporary bids for SFX executions. Cleaned out each Config::BidsInterval, where are moved from
    ///     PendingSFXBids to FSX::accepted_bids
    ///
    #[pallet::storage]
    #[pallet::getter(fn get_pending_sfx_bids)]
    pub type PendingSFXBids<T> = StorageDoubleMap<
        _,
        Identity,
        XExecSignalId<T>,
        Identity,
        SideEffectId<T>,
        SFXBid<
            <T as frame_system::Config>::AccountId,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
        OptionQuery,
    >;

    /// Links mapping SFX 2 XTX
    ///
    #[pallet::storage]
    #[pallet::getter(fn get_sfx_2_xtx_links)]
    pub type SFX2XTXLinksMap<T> =
        StorageMap<_, Identity, SideEffectId<T>, XExecSignalId<T>, OptionQuery>;

    /// Current Circuit's context of active Xtx used for the on_initialize clock to discover
    ///     the ones pending for execution too long, that eventually need to be killed
    ///
    #[pallet::storage]
    #[pallet::getter(fn get_active_timing_links)]
    pub type PendingXtxTimeoutsMap<T> = StorageMap<
        _,
        Identity,
        XExecSignalId<T>,
        <T as frame_system::Config>::BlockNumber,
        OptionQuery,
    >;

    /// Temporary bids for SFX executions. Cleaned out each Config::BidsInterval, where are moved from
    ///     PendingSFXBids to FSX::accepted_bids
    ///
    #[pallet::storage]
    #[pallet::getter(fn get_pending_xtx_bids_timeouts)]
    pub type PendingXtxBidsTimeoutsMap<T> = StorageMap<
        _,
        Identity,
        XExecSignalId<T>,
        <T as frame_system::Config>::BlockNumber,
        OptionQuery,
    >;

    /// Current Circuit's context of all accepted for execution cross-chain transactions.
    ///
    /// All Xtx that has been initially paid out by users will be left here.
    ///     Even if the timeout has been exceeded, they will eventually end with the Circuit::RevertedTimeout
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

    /// LocalXtxStates stores the map of LocalState - additional state to be used to communicate between SFX that belong to the same Xtx
    ///
    /// - @Circuit::Requested: create LocalXtxStates array without confirmations or bids
    /// - @Circuit::PendingExecution: entries to LocalState can be updated.
    /// If no bids have been received @Circuit::PendingBidding, LocalXtxStates entries are removed since Xtx won't be executed
    #[pallet::storage]
    #[pallet::getter(fn get_local_xtx_state)]
    pub type LocalXtxStates<T> = StorageMap<_, Identity, XExecSignalId<T>, LocalState, OptionQuery>;

    /// Current Circuit's context of active full side effects (requested + confirmation proofs)
    /// Lifecycle tips:
    /// FSX entries are created at the time of Xtx submission, where still uncertain whether Xtx will be accepted
    ///     for execution (picked up in the bidding process).
    /// - @Circuit::Requested: create FSX array without confirmations or bids
    /// - @Circuit::Bonded -> Ready: add bids to FSX
    /// - @Circuit::PendingExecution -> add more confirmations at receipt
    ///
    /// If no bids have been received @Circuit::PendingBidding, FSX entries will stay - just without the Bid.
    ///     The details on Xtx status might be played back by looking up with the SFX2XTXLinksMap
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
        /// The Circuit's account id
        #[pallet::constant]
        type SelfAccountId: Get<Self::AccountId>;

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

        /// The Circuit's SFX Bidding Period
        #[pallet::constant]
        type SFXBiddingPeriod: Get<Self::BlockNumber>;

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
            + From<Call<Self>>
            + From<frame_system::Call<Self>>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: weights::WeightInfo;

        /// A type that provides inspection and mutation to some fungible assets
        type Balances: Inspect<Self::AccountId> + Mutate<Self::AccountId>;

        /// A type that provides access to Xdns
        type Xdns: Xdns<Self>;

        type XBIPortal: XBIPortal<Self>;

        type XBIPromise: XBIPromise<Self, <Self as Config>::Call>;

        type Executors: Executors<
            Self,
            <<Self::Escrowed as EscrowTrait<Self>>::Currency as frame_support::traits::Currency<
                Self::AccountId,
            >>::Balance,
        >;

        /// A type that provides access to AccountManager
        type AccountManager: AccountManager<
            Self::AccountId,
            <<Self::Escrowed as EscrowTrait<Self>>::Currency as frame_support::traits::Currency<
                Self::AccountId,
            >>::Balance,
            Self::Hash,
            Self::BlockNumber,
        >;

        // type FreeVM: FreeVM<Self>;

        /// A type that manages escrow, and therefore balances
        type Escrowed: EscrowTrait<Self>;

        /// A type that gives access to the new portal functionality
        type Portal: Portal<Self>;

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

            // Check for expiring bids each block
            <PendingXtxBidsTimeoutsMap<T>>::iter()
                .find(|(_xtx_id, bidding_timeouts_at)| {
                    // ToDo consider moving xtx_bids to xtx_ctx in order to self update to always determine status
                    bidding_timeouts_at <= &frame_system::Pallet::<T>::block_number()
                })
                .map(|(xtx_id, _bidding_timeouts_at)| {
                    let mut local_ctx = Self::setup(
                        CircuitStatus::PendingBidding,
                        &Self::account_id(),
                        Zero::zero(),
                        Some(xtx_id),
                    )
                    .unwrap();

                    // Ensure Circuit::PendingBidding status
                    if local_ctx.xtx.status != CircuitStatus::PendingBidding {
                        Self::kill(&mut local_ctx, CircuitStatus::DroppedAtBidding);
                        return
                    }

                    let current_step = local_ctx.xtx.steps_cnt.0;
                    for mut fsx in local_ctx.full_side_effects[current_step as usize].iter_mut() {
                        let sfx_id = fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
                        if let Some(best_sfx_bid) = <PendingSFXBids<T>>::get(xtx_id, sfx_id) {
                            fsx.best_bid = Some(best_sfx_bid);
                        } else {
                            // error - some FSX don't have bids
                            Self::kill(&mut local_ctx, CircuitStatus::DroppedAtBidding);
                            Self::emit_status_update(
                                local_ctx.xtx_id,
                                Some(local_ctx.xtx),
                                None,
                            );
                            return
                        }
                    }

                    let status_change = Self::update(&mut local_ctx).unwrap();

                    Self::square_up(&mut local_ctx, None)
                        .expect("Expect Bonding Bids at square up to be infallible since funds of requester have been reserved at the SFX submission");

                    Self::apply(
                        &mut local_ctx,
                        status_change,
                    );

                    Self::emit_status_update(
                        local_ctx.xtx_id,
                        Some(local_ctx.xtx),
                        None,
                    );
                });
            // Go over pending Bids to discover whether

            // Scenario 1: all the timeout s can be handled in the block space
            // Scenario 2: all but 5 timeouts can be handled
            //     - add the 5 timeouts to an immediate queue for the next block
            if n % T::XtxTimeoutCheckInterval::get() == T::BlockNumber::from(0u8) {
                let mut deletion_counter: u32 = 0;
                // Go over all unfinished Xtx to find those that timed out
                <PendingXtxTimeoutsMap<T>>::iter()
                    .find(|(_xtx_id, timeout_at)| {
                        timeout_at <= &frame_system::Pallet::<T>::block_number()
                    })
                    .map(|(xtx_id, _timeout_at)| {
                        if deletion_counter > T::DeletionQueueLimit::get() {
                            return
                        }
                        let mut local_xtx_ctx = Self::setup(
                            CircuitStatus::Ready,
                            &Self::account_id(),
                            Zero::zero(),
                            Some(xtx_id),
                        )
                        .unwrap();
                        log::info!("Timeout Xtx: {:?}", xtx_id);

                        Self::kill(&mut local_xtx_ctx, CircuitStatus::RevertTimedOut);

                        Self::emit_status_update(
                            local_xtx_ctx.xtx_id,
                            Some(local_xtx_ctx.xtx),
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

    impl<T: Config> OnLocalTrigger<T, BalanceOf<T>> for Pallet<T> {
        fn load_local_state(
            origin: &OriginFor<T>,
            maybe_xtx_id: Option<T::Hash>,
        ) -> Result<LocalStateExecutionView<T, BalanceOf<T>>, DispatchError> {
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

            // There should be no apply step since no change could have happen during the state access
            let hardened_side_effects = local_xtx_ctx
                .full_side_effects
                .iter()
                .map(|step| {
                    step.iter()
                        .map(|fsx| {
                            let effect: HardenedSideEffect<
                                T::AccountId,
                                T::BlockNumber,
                                BalanceOf<T>,
                            > = fsx.clone().try_into().map_err(|e| {
                                log::debug!(
                                    target: "runtime::circuit",
                                    "Error converting side effect to runtime: {:?}",
                                    e
                                );
                                Error::<T>::FailedToHardenFullSideEffect
                            })?;
                            Ok(effect)
                        })
                        .collect::<Result<
                            Vec<HardenedSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
                            Error<T>,
                        >>()
                })
                .collect::<Result<
                    Vec<Vec<HardenedSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>>,
                    Error<T>,
                >>()?;

            // We must apply the state only if its generated and fresh
            if maybe_xtx_id.is_none() {
                // Update local context
                let status_change = Self::update(&mut local_xtx_ctx)?;

                let _ = Self::apply(&mut local_xtx_ctx, status_change);
            }

            // There should be no apply step since no change could have happen during the state access
            Ok(LocalStateExecutionView::<T, BalanceOf<T>>::new(
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

            // Account fees and charges
            Self::square_up(&mut local_xtx_ctx, None)?;

            // Update local context
            let status_change = Self::update(&mut local_xtx_ctx)?;

            // Apply: all necessary changes to state in 1 go
            let (_, added_full_side_effects) = Self::apply(&mut local_xtx_ctx, status_change);

            // Emit: From Circuit events
            Self::emit_sfx(local_xtx_ctx.xtx_id, &requester, &side_effects);

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
            <Self as OnLocalTrigger<T, BalanceOf<T>>>::on_local_trigger(
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

        #[pallet::weight(<T as pallet::Config>::WeightInfo::cancel_xtx())]
        pub fn cancel_xtx(origin: OriginFor<T>, xtx_id: T::Hash) -> DispatchResultWithPostInfo {
            let requester = Self::authorize(origin, CircuitRole::Requester)?;
            // Setup: new xtx context
            let mut local_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingBidding,
                &requester,
                Zero::zero(),
                Some(xtx_id),
            )?;

            if requester != local_ctx.xtx.requester
                || local_ctx.xtx.status > CircuitStatus::PendingBidding
            {
                return Err(Error::<T>::UnauthorizedCancellation.into())
            }

            // Drop cancellation in case some bids have already been posted
            if Self::get_current_step_fsx(&local_ctx)
                .iter()
                .any(|fsx| fsx.best_bid.is_some())
            {
                return Err(Error::<T>::UnauthorizedCancellation.into())
            }

            Self::kill(&mut local_ctx, CircuitStatus::DroppedAtBidding);

            Ok(().into())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_extrinsic_trigger())]
        pub fn on_extrinsic_trigger(
            origin: OriginFor<T>,
            side_effects: Vec<SideEffect<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>>,
            fee: EscrowedBalanceOf<T, T::Escrowed>,
            sequential: bool,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let requester = Self::authorize(origin, CircuitRole::Requester)?;
            // Setup: new xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> =
                Self::setup(CircuitStatus::Requested, &requester, fee, None)?;

            // Validate: Side Effects
            Self::validate(&side_effects, &mut local_xtx_ctx, &requester, sequential).map_err(
                |e| {
                    log::error!("Self::validate hit an error -- {:?}", e);
                    Error::<T>::SideEffectsValidationFailed
                },
            )?;

            // Account fees and charges
            Self::square_up(&mut local_xtx_ctx, None)?;

            // Update local context
            let status_change = Self::update(&mut local_xtx_ctx)?;

            // Apply: all necessary changes to state in 1 go
            let (_, added_full_side_effects) = Self::apply(&mut local_xtx_ctx, status_change);

            // Emit: From Circuit events
            Self::emit_sfx(local_xtx_ctx.xtx_id, &requester, &side_effects);

            Ok(().into())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::bid_sfx())]
        pub fn bid_sfx(
            origin: OriginFor<T>, // Active relayer
            sfx_id: SideEffectId<T>,
            bid_amount: EscrowedBalanceOf<T, T::Escrowed>,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let executor = Self::authorize(origin, CircuitRole::Executor)?;

            // retrieve xtx_id
            let xtx_id = <Self as Store>::SFX2XTXLinksMap::get(sfx_id)
                .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;

            // Setup: retrieve local xtx context
            let mut local_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingBidding,
                &executor,
                Zero::zero(),
                Some(xtx_id),
            )?;

            // Check for the previous bids for SFX.
            // ToDo: Consider moving to setup to keep the rule of single storage access at setup.
            let current_accepted_bid =
                crate::Pallet::<T>::storage_read_sfx_accepted_bid(&mut local_ctx, sfx_id);

            let accepted_as_best_bid = Optimistic::<T>::try_bid_4_sfx(
                &mut local_ctx,
                &executor,
                bid_amount,
                sfx_id,
                current_accepted_bid,
            )?;

            crate::Pallet::<T>::storage_write_new_sfx_accepted_bid(
                &mut local_ctx,
                sfx_id,
                accepted_as_best_bid,
            );

            Self::deposit_event(Event::SFXNewBidReceived(
                sfx_id,
                executor.clone(),
                bid_amount,
            ));

            Ok(().into())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::execute_side_effects_with_xbi())]
        pub fn execute_side_effects_with_xbi(
            origin: OriginFor<T>, // Active relayer
            xtx_id: XExecSignalId<T>,
            side_effect: SideEffect<
                <T as frame_system::Config>::AccountId,
                EscrowedBalanceOf<T, <T as Config>::Escrowed>,
            >,
            max_exec_cost: u128,
            max_notifications_cost: u128,
        ) -> DispatchResultWithPostInfo {
            let sfx_id = side_effect.generate_id::<SystemHashing<T>>(xtx_id.as_ref(), 0u32);

            if T::XBIPortal::get_status(sfx_id) != XBIStatus::UnknownId {
                return Err(Error::<T>::SideEffectIsAlreadyScheduledToExecuteOverXBI.into())
            }
            // Authorize: Retrieve sender of the transaction.
            let executor = Self::authorize(origin, CircuitRole::Executor)?;

            // Setup: retrieve local xtx context
            let mut local_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingExecution,
                &executor,
                Zero::zero(),
                Some(xtx_id),
            )?;

            let xbi =
                sfx_2_xbi::<T, T::Escrowed>(
                    &side_effect,
                    XBIMetadata::new_with_default_timeouts(
                        XbiId::<T>::local_hash_2_xbi_id(sfx_id)?,
                        T::Xdns::get_gateway_para_id(&side_effect.target)?,
                        T::SelfParaId::get(),
                        max_exec_cost,
                        max_notifications_cost,
                        Some(Sabi::account_bytes_2_account_32(executor.encode()).map_err(
                            |_| Error::<T>::FailedToCreateXBIMetadataDueToWrongAccountConversion,
                        )?),
                    ),
                )
                .map_err(|_e| Error::<T>::FailedToConvertSFX2XBI)?;

            // Use encoded XBI hash as ID for the executor's charge
            let charge_id = T::Hashing::hash(&xbi.encode()[..]);
            let total_max_rewards = xbi.metadata.total_max_costs_in_local_currency()?;

            // fixme: must be solved with charging and update status order if XBI is the first SFX
            if local_ctx.xtx.status == CircuitStatus::Ready {
                local_ctx.xtx.status = CircuitStatus::PendingExecution;
            }

            Self::square_up(
                &mut local_ctx,
                Some((charge_id, executor, total_max_rewards)),
            )?;

            T::XBIPromise::then(
                xbi,
                pallet::Call::<T>::on_xbi_sfx_resolved { sfx_id }.into(),
            )?;

            let status_change = Self::update(&mut local_ctx)?;

            Self::apply(&mut local_ctx, status_change);

            Ok(().into())
        }

        #[pallet::weight(< T as Config >::WeightInfo::confirm_side_effect())]
        pub fn on_xbi_sfx_resolved(
            _origin: OriginFor<T>,
            sfx_id: T::Hash,
        ) -> DispatchResultWithPostInfo {
            Self::do_xbi_exit(
                T::XBIPortal::get_check_in(sfx_id)?,
                T::XBIPortal::get_check_out(sfx_id)?,
            )?;
            Ok(().into())
        }

        /// Blind version should only be used for testing - unsafe since skips inclusion proof check.
        #[pallet::weight(< T as Config >::WeightInfo::confirm_side_effect())]
        pub fn confirm_side_effect(
            origin: OriginFor<T>,
            sfx_id: SideEffectId<T>,
            confirmation: ConfirmedSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let executor = Self::authorize(origin, CircuitRole::Executor)?;
            let xtx_id = <Self as Store>::SFX2XTXLinksMap::get(sfx_id)
                .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;

            // Setup: retrieve local xtx context
            let mut local_xtx_ctx: LocalXtxCtx<T> = Self::setup(
                CircuitStatus::PendingExecution,
                &executor,
                Zero::zero(),
                Some(xtx_id),
            )?;

            Self::confirm(&mut local_xtx_ctx, &executor, &sfx_id, &confirmation)?;

            let status_change = Self::update(&mut local_xtx_ctx)?;

            // Apply: all necessary changes to state in 1 go
            let (maybe_xtx_changed, assert_full_side_effects_changed) =
                Self::apply(&mut local_xtx_ctx, status_change);

            Self::deposit_event(Event::SideEffectConfirmed(sfx_id));

            // Emit: From Circuit events
            Self::emit_status_update(
                local_xtx_ctx.xtx_id,
                maybe_xtx_changed,
                assert_full_side_effects_changed,
            );

            Ok(().into())
        }
    }

    use crate::CircuitStatus::RevertKill;
    use pallet_xbi_portal::xbi_abi::{
        AccountId20, AccountId32, AssetId, Data, Gas, Value, ValueEvm, XbiId,
    };
    use t3rn_primitives::side_effect::SFXBid;

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
            ValueEvm,
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
        // New best bid for SFX has been accepted. Account here is an executor.
        SFXNewBidReceived(
            SideEffectId<T>,
            <T as frame_system::Config>::AccountId,
            EscrowedBalanceOf<T, T::Escrowed>,
        ),
        // An executions SideEffect was confirmed.
        SideEffectConfirmed(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and ready
        XTransactionReadyForExec(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and finished
        XTransactionStepFinishedExec(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and finished
        XTransactionXtxFinishedExecAllSteps(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and finished
        XTransactionXtxRevertedAfterTimeOut(XExecSignalId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and finished
        XTransactionXtxDroppedAtBidding(XExecSignalId<T>),
        // Listeners - executioners/relayers to know new challenges and perform offline risk/reward calc
        //  of whether side effect is worth picking up
        NewSideEffectsAvailable(
            <T as frame_system::Config>::AccountId,
            XExecSignalId<T>,
            Vec<
                SideEffect<
                    <T as frame_system::Config>::AccountId,
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
        UpdateXtxTriggeredWithUnexpectedStatus,
        ApplyTriggeredWithUnexpectedStatus,
        RequesterNotEnoughBalance,
        ContractXtxKilledRunOutOfFunds,
        ChargingTransferFailed,
        ChargingTransferFailedAtPendingExecution,
        XtxChargeFailedRequesterBalanceTooLow,
        XtxChargeBondDepositFailedCantAccessBid,
        FinalizeSquareUpFailed,
        CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed,
        RewardTransferFailed,
        RefundTransferFailed,
        SideEffectsValidationFailed,
        InsuranceBondNotRequired,
        BiddingInactive,
        BiddingRejectedBidBelowDust,
        BiddingRejectedExecutorNotEnoughBalance,
        BiddingRejectedBidTooHigh,
        BiddingRejectedBetterBidFound,
        BiddingFailedExecutorsBalanceTooLowToReserve,
        InsuranceBondAlreadyDeposited,
        SetupFailed,
        SetupFailedXtxNotFound,
        SetupFailedXtxStorageArtifactsNotFound,
        SetupFailedIncorrectXtxStatus,
        SetupFailedDuplicatedXtx,
        SetupFailedEmptyXtx,
        SetupFailedXtxAlreadyFinished,
        SetupFailedXtxWasDroppedAtBidding,
        SetupFailedXtxReverted,
        SetupFailedXtxRevertedTimeout,
        SetupFailedUnknownXtx,
        InvalidFSXBidStateLocated,
        EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished,
        FatalXtxTimeoutXtxIdNotMatched,
        RelayEscrowedFailedNothingToConfirm,
        FatalCommitSideEffectWithoutConfirmationAttempt,
        FatalErroredCommitSideEffectConfirmationAttempt,
        FatalErroredRevertSideEffectConfirmationAttempt,
        FailedToHardenFullSideEffect,
        ApplyFailed,
        DeterminedForbiddenXtxStatus,
        SideEffectIsAlreadyScheduledToExecuteOverXBI,
        LocalSideEffectExecutionNotApplicable,
        LocalExecutionUnauthorized,
        UnauthorizedCancellation,
        FailedToConvertSFX2XBI,
        FailedToCheckInOverXBI,
        FailedToCreateXBIMetadataDueToWrongAccountConversion,
        FailedToConvertXBIResult2SFXConfirmation,
        FailedToEnterXBIPortal,
        FailedToExitXBIPortal,
        XBIExitFailedOnSFXConfirmation,
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
                    full_side_effects: vec![],
                })
            },
            CircuitStatus::Ready
            | CircuitStatus::PendingExecution
            | CircuitStatus::PendingBidding
            | CircuitStatus::Finished => {
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

                    let full_side_effects = <Self as Store>::FullSideEffects::get(id)
                        .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;
                    let local_state = <Self as Store>::LocalXtxStates::get(id)
                        .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;

                    Ok(LocalXtxCtx {
                        local_state,
                        use_protocol: UniversalSideEffectsProtocol::new(),
                        xtx_id: id,
                        xtx,
                        full_side_effects,
                    })
                } else {
                    Err(Error::<T>::SetupFailedEmptyXtx)
                }
            },
            CircuitStatus::FinishedAllSteps => Err(Error::<T>::SetupFailedXtxAlreadyFinished),
            CircuitStatus::RevertKill => Err(Error::<T>::SetupFailedXtxRevertedTimeout),
            CircuitStatus::RevertMisbehaviour => Err(Error::<T>::SetupFailedXtxReverted),
            CircuitStatus::Committed => Err(Error::<T>::SetupFailedXtxAlreadyFinished),
            CircuitStatus::Reverted => Err(Error::<T>::SetupFailedXtxReverted),
            CircuitStatus::RevertTimedOut => Err(Error::<T>::SetupFailedXtxRevertedTimeout),
            CircuitStatus::DroppedAtBidding => Err(Error::<T>::SetupFailedXtxWasDroppedAtBidding),
        }
    }

    // Updates local xtx context without touching the storage.
    fn update(
        mut local_ctx: &mut LocalXtxCtx<T>,
    ) -> Result<(CircuitStatus, CircuitStatus), Error<T>> {
        let current_status = local_ctx.xtx.status.clone();

        // Apply will try to move the status of Xtx from the current to the closest valid one.
        match current_status {
            CircuitStatus::Requested => {
                local_ctx.xtx.steps_cnt = (0, local_ctx.full_side_effects.len() as u32);
            },
            CircuitStatus::RevertTimedOut => {},
            CircuitStatus::Ready | CircuitStatus::PendingExecution | CircuitStatus::Finished => {
                // Check whether all of the side effects in this steps are confirmed - the status now changes to CircuitStatus::Finished
                if !Self::get_current_step_fsx(local_ctx)
                    .iter()
                    .any(|fsx| fsx.confirmed.is_none())
                {
                    local_ctx.xtx.steps_cnt =
                        (local_ctx.xtx.steps_cnt.0 + 1, local_ctx.xtx.steps_cnt.1);

                    local_ctx.xtx.status = CircuitStatus::Finished;

                    // All of the steps are completed - the xtx has been finalized
                    if local_ctx.xtx.steps_cnt.0 == local_ctx.xtx.steps_cnt.1 {
                        local_ctx.xtx.status = CircuitStatus::FinishedAllSteps;
                        return Ok((current_status, CircuitStatus::FinishedAllSteps))
                    }
                }
            },
            _ => {},
        }

        let new_status = CircuitStatus::determine_xtx_status(&local_ctx.full_side_effects)?;
        local_ctx.xtx.status = new_status.clone();

        Ok((current_status, new_status))
    }

    /// Returns: Returns changes written to the state if there are any.
    ///     For now only returns Xtx and FullSideEffects that changed.
    fn apply(
        local_ctx: &mut LocalXtxCtx<T>,
        status_change: (CircuitStatus, CircuitStatus),
    ) -> (
        Option<XExecSignal<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>>,
        Option<
            Vec<
                Vec<
                    FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
                >,
            >,
        >,
    ) {
        // let current_status = local_ctx.xtx.status.clone();
        let (old_status, new_status) = (status_change.0, status_change.1);

        match old_status {
            CircuitStatus::Requested => {
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
                    .flat_map(|(cnt, fsx_step)| {
                        fsx_step
                            .iter()
                            .map(|full_side_effect| {
                                full_side_effect
                                    .generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id)
                            })
                            .map(|side_effect_hash| {
                                (
                                    cnt,
                                    side_effect_hash,
                                    XExecSignal::<
                                        T::AccountId,
                                        T::BlockNumber,
                                        EscrowedBalanceOf<T, <T as Config>::Escrowed>,
                                    >::generate_step_id::<T>(
                                        local_ctx.xtx_id, cnt
                                    ),
                                )
                            })
                            .collect::<Vec<(usize, SideEffectId<T>, XExecStepSideEffectId<T>)>>()
                    })
                    .collect();

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

                for (_step_cnt, side_effect_id, _step_side_effect_id) in steps_side_effects_ids {
                    <SFX2XTXLinksMap<T>>::insert::<SideEffectId<T>, XExecSignalId<T>>(
                        side_effect_id,
                        local_ctx.xtx_id,
                    );
                }

                <LocalXtxStates<T>>::insert::<XExecSignalId<T>, LocalState>(
                    local_ctx.xtx_id,
                    local_ctx.local_state.clone(),
                );
                <PendingXtxTimeoutsMap<T>>::insert::<XExecSignalId<T>, T::BlockNumber>(
                    local_ctx.xtx_id,
                    local_ctx.xtx.timeouts_at,
                );
                <PendingXtxBidsTimeoutsMap<T>>::insert::<XExecSignalId<T>, T::BlockNumber>(
                    local_ctx.xtx_id,
                    T::SFXBiddingPeriod::get() + frame_system::Pallet::<T>::block_number(),
                );
                <XExecSignals<T>>::insert::<
                    XExecSignalId<T>,
                    XExecSignal<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
                >(local_ctx.xtx_id, local_ctx.xtx.clone());

                (
                    Some(local_ctx.xtx.clone()),
                    Some(local_ctx.full_side_effects.to_vec()),
                )
            },
            CircuitStatus::PendingBidding => {
                match new_status {
                    CircuitStatus::Ready => {
                        <Self as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                            *x = Some(local_ctx.full_side_effects.clone())
                        });
                        <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                            *x = Some(local_ctx.xtx.clone())
                        });
                    },
                    CircuitStatus::DroppedAtBidding => {
                        // Clean all associated Xtx entries
                        <Self as Store>::XExecSignals::remove(local_ctx.xtx_id);
                        <Self as Store>::PendingXtxTimeoutsMap::remove(local_ctx.xtx_id);
                        <Self as Store>::LocalXtxStates::remove(local_ctx.xtx_id);
                        <Self as Store>::FullSideEffects::remove(local_ctx.xtx_id);
                        for fsx_step in &local_ctx.full_side_effects {
                            for fsx in fsx_step {
                                <Self as Store>::SFX2XTXLinksMap::remove(
                                    fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id),
                                );
                            }
                        }
                    },
                    _ => {},
                }
                // Always clean temporary PendingSFXBids and TimeoutsMap after bidding
                <Self as Store>::PendingSFXBids::remove_prefix(local_ctx.xtx_id, None);
                <Self as Store>::PendingXtxBidsTimeoutsMap::remove(local_ctx.xtx_id);
                (
                    Some(local_ctx.xtx.clone()),
                    Some(local_ctx.full_side_effects.to_vec()),
                )
            },
            CircuitStatus::RevertTimedOut => {
                <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                <Self as Store>::PendingXtxTimeoutsMap::remove(local_ctx.xtx_id);
                (
                    Some(local_ctx.xtx.clone()),
                    Some(local_ctx.full_side_effects.clone()),
                )
            },
            // fixme: Separate for Bonded
            CircuitStatus::Ready | CircuitStatus::PendingExecution | CircuitStatus::Finished => {
                // Update set of full side effects assuming the new confirmed has appeared
                <Self as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.full_side_effects.clone())
                });

                <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });
                if local_ctx.xtx.status.clone() > CircuitStatus::Ready {
                    (
                        Some(local_ctx.xtx.clone()),
                        Some(local_ctx.full_side_effects.clone()),
                    )
                } else {
                    (None, Some(local_ctx.full_side_effects.to_vec()))
                }
            },
            CircuitStatus::FinishedAllSteps => {
                // todo: cleanup all of the local storage
                // TODO cleanup sfx2xtx map
                <Self as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                <Self as Store>::PendingXtxTimeoutsMap::remove(local_ctx.xtx_id);
                (
                    Some(local_ctx.xtx.clone()),
                    Some(local_ctx.full_side_effects.clone()),
                )
            },
            _ => (None, None),
        }
    }

    fn emit_sfx(
        xtx_id: XExecSignalId<T>,
        subjected_account: &T::AccountId,
        side_effects: &Vec<SideEffect<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>>,
    ) {
        if !side_effects.is_empty() {
            Self::deposit_event(Event::NewSideEffectsAvailable(
                subjected_account.clone(),
                xtx_id,
                // ToDo: Emit circuit outbound messages -> side effects
                side_effects.to_vec(),
                side_effects
                    .iter()
                    .enumerate()
                    .map(|(index, se)| {
                        se.generate_id::<SystemHashing<T>>(xtx_id.as_ref(), index as u32)
                    })
                    .collect::<Vec<SideEffectId<T>>>(),
            ));
            // ToDo remove this
            Self::deposit_event(Event::XTransactionReceivedForExec(xtx_id));
        }
    }

    fn emit_status_update(
        xtx_id: XExecSignalId<T>,
        maybe_xtx: Option<
            XExecSignal<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        >,
        maybe_full_side_effects: Option<
            Vec<
                Vec<
                    FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
                >,
            >,
        >,
    ) {
        if let Some(xtx) = maybe_xtx {
            match xtx.status {
                CircuitStatus::PendingBidding =>
                    Self::deposit_event(Event::XTransactionReceivedForExec(xtx_id)),
                CircuitStatus::Ready =>
                    Self::deposit_event(Event::XTransactionReadyForExec(xtx_id)),
                CircuitStatus::Finished =>
                    Self::deposit_event(Event::XTransactionStepFinishedExec(xtx_id)),
                CircuitStatus::FinishedAllSteps =>
                    Self::deposit_event(Event::XTransactionXtxFinishedExecAllSteps(xtx_id)),
                CircuitStatus::RevertTimedOut =>
                    Self::deposit_event(Event::XTransactionXtxRevertedAfterTimeOut(xtx_id)),
                CircuitStatus::DroppedAtBidding =>
                    Self::deposit_event(Event::XTransactionXtxDroppedAtBidding(xtx_id)),
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
        local_ctx.xtx.status = cause.clone();
        if let CircuitStatus::RevertTimedOut = cause {
            Optimistic::<T>::try_slash(local_ctx);
        } else {
            Optimistic::<T>::try_dropped_at_bidding_refund(local_ctx);
        }

        Self::square_up(local_ctx, None)
            .expect("Expect Revert and RevertKill options to square up to be infallible");

        Self::apply(local_ctx, (cause.clone(), cause));
    }

    fn square_up(
        local_ctx: &mut LocalXtxCtx<T>,
        maybe_xbi_execution_charge: Option<(
            T::Hash,
            <T as frame_system::Config>::AccountId,
            EscrowedBalanceOf<T, T::Escrowed>,
        )>,
    ) -> Result<(), Error<T>> {
        let requester = local_ctx.xtx.requester.clone();

        let unreserve_requester_xtx_max_rewards = |current_step_fsx: &Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, <T as Config>::Escrowed>,
            >,
        >| {
            let mut total_max_rewards = Zero::zero();
            for fsx in current_step_fsx.iter() {
                total_max_rewards += fsx.input.max_reward;
            }

            <T::Escrowed as EscrowTrait<T>>::Currency::unreserve(&requester, total_max_rewards);
        };
        match local_ctx.xtx.status {
            CircuitStatus::Requested => {
                let mut total_max_rewards = Zero::zero();
                for fsx in Self::get_current_step_fsx(local_ctx).iter() {
                    total_max_rewards += fsx.input.max_reward;
                }
                <T::Escrowed as EscrowTrait<T>>::Currency::reserve(&requester, total_max_rewards)
                    .map_err(|_e| Error::<T>::XtxChargeFailedRequesterBalanceTooLow)?;
            },
            CircuitStatus::DroppedAtBidding => {
                unreserve_requester_xtx_max_rewards(Self::get_current_step_fsx(local_ctx));
            },
            CircuitStatus::Ready => {
                let current_step_sfx = Self::get_current_step_fsx(local_ctx);
                // Unreserve the max_rewards and replace with possibly lower bids of executor in following loop
                unreserve_requester_xtx_max_rewards(current_step_sfx);
                for fsx in current_step_sfx.iter() {
                    let charge_id = fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
                    let bid_4_fsx: &SFXBid<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>> =
                        if let Some(bid) = &fsx.best_bid {
                            bid
                        } else {
                            return Err(Error::<T>::XtxChargeBondDepositFailedCantAccessBid)
                        };

                    if bid_4_fsx.bid > Zero::zero() {
                        <T as Config>::AccountManager::deposit(
                            charge_id,
                            &requester,
                            Zero::zero(),
                            bid_4_fsx.bid,
                            BenefitSource::TrafficRewards,
                            CircuitRole::Requester,
                            Some(bid_4_fsx.executor.clone()),
                        )
                        .map_err(|_e| Error::<T>::ChargingTransferFailed)?;
                    }
                }
            },
            CircuitStatus::PendingExecution => {
                let (charge_id, executor_payee, charge_fee) =
                    maybe_xbi_execution_charge.ok_or(Error::<T>::ChargingTransferFailed)?;

                <T as Config>::AccountManager::deposit(
                    charge_id,
                    &executor_payee,
                    charge_fee,
                    Zero::zero(),
                    BenefitSource::TrafficFees,
                    CircuitRole::Executor,
                    None,
                )
                .map_err(|_e| Error::<T>::ChargingTransferFailedAtPendingExecution)?;
            },
            // todo: make sure callable once
            // todo: distinct between RevertTimedOut to iterate over all steps vs single step for Revert
            CircuitStatus::RevertTimedOut
            | CircuitStatus::Reverted
            | CircuitStatus::RevertMisbehaviour => {
                log::info!("RevertTimedOut or Reverted or RevertMisbehaviour");
                log::info!("xtx_id: {:?}", local_ctx.xtx_id);
                Optimistic::<T>::try_slash(local_ctx);
                log::info!("slashing worked");
                for fsx in Self::get_current_step_fsx(local_ctx).iter() {
                    let charge_id = fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
                    <T as Config>::AccountManager::try_finalize(
                        charge_id,
                        Outcome::Revert,
                        None,
                        None,
                    );
                }
            },
            CircuitStatus::Finished | CircuitStatus::FinishedAllSteps => {
                Optimistic::<T>::try_unbond(local_ctx)?;
                for fsx in Self::get_current_step_fsx(local_ctx).iter() {
                    let charge_id = fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
                    let confirmed = if let Some(confirmed) = &fsx.confirmed {
                        Ok(confirmed)
                    } else {
                        Err(Error::<T>::CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed)
                    }?;
                    // todo: Verify that cost can be repatriated on this occation and whether XBI Exec resoliution is expected for particular FSX
                    <T as Config>::AccountManager::finalize(
                        charge_id,
                        Outcome::Commit,
                        Some(confirmed.executioner.clone()),
                        confirmed.cost,
                    )
                    .map_err(|_e| Error::<T>::FinalizeSquareUpFailed)?;
                }
            },
            _ => {},
        }

        Ok(())
    }

    fn authorize(
        origin: OriginFor<T>,
        role: CircuitRole,
    ) -> Result<T::AccountId, sp_runtime::traits::BadOrigin> {
        match role {
            CircuitRole::Requester | CircuitRole::ContractAuthor => ensure_signed(origin),
            // ToDo: Handle active Relayer authorisation
            CircuitRole::Relayer => ensure_signed(origin),
            // ToDo: Handle bonded Executor authorisation
            CircuitRole::Executor => ensure_signed(origin),
            // ToDo: Handle other CircuitRoles
            _ => unimplemented!(),
        }
    }

    fn validate(
        side_effects: &[SideEffect<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>],
        local_ctx: &mut LocalXtxCtx<T>,
        _requester: &T::AccountId,
        _sequential: bool,
    ) -> Result<(), &'static str> {
        let mut full_side_effects: Vec<
            FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        > = vec![];

        pub fn determine_security_lvl(gateway_type: GatewayType) -> SecurityLvl {
            if gateway_type == GatewayType::ProgrammableInternal(0)
                || gateway_type == GatewayType::OnCircuit(0)
            {
                SecurityLvl::Escrow
            } else {
                SecurityLvl::Optimistic
            }
        }

        // ToDo: Handle empty SFX case as error instead - must satisfy requirements of LocalTrigger
        if side_effects.is_empty() {
            local_ctx.full_side_effects = vec![vec![]];
            return Ok(())
        }

        for (index, sfx) in side_effects.iter().enumerate() {
            let gateway_abi = <T as Config>::Xdns::get_abi(sfx.target)?;
            let gateway_type = <T as Config>::Xdns::get_gateway_type_unsafe(&sfx.target);

            let allowed_side_effects = <T as Config>::Xdns::allowed_side_effects(&sfx.target);

            local_ctx
                .use_protocol
                .notice_gateway(sfx.target, allowed_side_effects);

            local_ctx
                .use_protocol
                .validate_args::<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>, SystemHashing<T>>(
                    sfx.clone(),
                    gateway_abi,
                    &mut local_ctx.local_state,
                    &local_ctx.xtx_id.as_ref(),
                    index as u32
                ).map_err(|e| {
                log::debug!(target: "runtime::circuit", "validate -- error validating side effects {:?}", e);
                e
            })?;

            let (insurance, reward) = if let Some(insurance_and_reward) =
                UniversalSideEffectsProtocol::ensure_required_insurance::<
                    T::AccountId,
                    T::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                    SystemHashing<T>,
                >(
                    sfx.clone(),
                    &mut local_ctx.local_state,
                    &local_ctx.xtx_id.as_ref(),
                    index as u32,
                )? {
                (insurance_and_reward[0], insurance_and_reward[1])
            } else {
                return Err(
                    "SFX must have its insurance and reward linked into the last arguments list",
                )
            };
            if sfx.max_reward != reward {
                return Err("Side_effect max_reward must be equal to reward of Optional Insurance")
            }
            if sfx.insurance != insurance {
                return Err("Side_effect insurance must be equal to reward of Optional Insurance")
            }
            let submission_target_height = T::Portal::get_latest_finalized_height(sfx.target)?
                .ok_or("target height not found")?;

            full_side_effects.push(FullSideEffect {
                input: sfx.clone(),
                confirmed: None,
                security_lvl: determine_security_lvl(gateway_type),
                submission_target_height,
                best_bid: None,
                index: index as u32,
            });
        }
        // Circuit's automatic side effect ordering: execute escrowed asap, then line up optimistic ones
        full_side_effects.sort_by(|a, b| b.security_lvl.partial_cmp(&a.security_lvl).unwrap());

        let mut escrow_sfx_step: Vec<
            FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        > = vec![];
        let mut optimistic_sfx_step: Vec<
            FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        > = vec![];

        // Split for 2 following steps of Escrow and Optimistic and
        for sorted_fsx in full_side_effects.iter() {
            if sorted_fsx.security_lvl == SecurityLvl::Escrow {
                escrow_sfx_step.push(sorted_fsx.clone());
            } else if sorted_fsx.security_lvl == SecurityLvl::Optimistic {
                optimistic_sfx_step.push(sorted_fsx.clone());
            }
        }

        // full_side_effects_steps should be non-empty at this point
        if escrow_sfx_step.is_empty() {
            local_ctx.full_side_effects = vec![optimistic_sfx_step.clone()];
        } else if optimistic_sfx_step.is_empty() {
            local_ctx.full_side_effects = vec![escrow_sfx_step.clone()];
        } else {
            local_ctx.full_side_effects =
                vec![escrow_sfx_step.clone(), optimistic_sfx_step.clone()];
        }

        Ok(())
    }

    fn confirm(
        local_ctx: &mut LocalXtxCtx<T>,
        _relayer: &T::AccountId,
        sfx_id: &SideEffectId<T>,
        confirmation: &ConfirmedSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
    ) -> Result<(), &'static str> {
        fn confirm_order<T: Config>(
            xtx_id: XExecSignalId<T>,
            sfx_id: SideEffectId<T>,
            confirmation: &ConfirmedSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            step_side_effects: &mut Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                >,
            >,
        ) -> Result<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
            &'static str,
        > {
            // Double check there are some side effects for that Xtx - should have been checked at API level tho already
            if step_side_effects.is_empty() {
                return Err("Xtx has an empty single step.")
            }

            // Find sfx object index in the current step
            match step_side_effects
                .iter()
                .position(|fsx| fsx.generate_id::<SystemHashing<T>, T>(xtx_id) == sfx_id)
            {
                Some(index) => {
                    // side effect found in current step
                    if step_side_effects[index].confirmed.is_none() {
                        // side effect unconfirmed currently
                        step_side_effects[index].confirmed = Some(confirmation.clone());
                        Ok(step_side_effects[index].clone())
                    } else {
                        Err("Side Effect already confirmed")
                    }
                },
                None => Err("Unable to find matching Side Effect in given Xtx to confirm"),
            }
        }

        // confirm order of current season, by passing the side_effects of it to confirm order.
        let fsx = confirm_order::<T>(
            local_ctx.xtx_id,
            *sfx_id,
            confirmation,
            &mut local_ctx.full_side_effects[local_ctx.xtx.steps_cnt.0 as usize],
        )?;

        log::debug!("Order confirmed!");

        let mut side_effect_id: [u8; 4] = [0, 0, 0, 0];
        side_effect_id.copy_from_slice(&fsx.input.encoded_action[0..4]);

        // confirm the payload is included in the specified block, and return the SideEffect params as defined in XDNS.
        // this could be multiple events!
        let (params, source) = <T as Config>::Portal::confirm_and_decode_payload_params(
            fsx.input.target,
            fsx.submission_target_height,
            confirmation.inclusion_data.clone(),
            side_effect_id.clone(),
        )
        .map_err(|_| "SideEffect confirmation failed!")?;
        // ToDo: handle misbehaviour
        log::debug!("SFX confirmation params: {:?}", params);

        let side_effect_interface =
            <T as Config>::Xdns::fetch_side_effect_interface(side_effect_id);

        log::debug!("Found SFX interface!");

        confirmation_plug::<T>(
            &Box::new(side_effect_interface.unwrap()),
            params,
            source,
            &local_ctx.local_state,
            Some(sfx_id.as_ref().to_vec()),
            fsx.security_lvl,
            <T as Config>::Xdns::get_gateway_security_coordinates(&fsx.input.target)?,
        )
        .map_err(|_| "Execution can't be confirmed.")?;
        log::debug!("confirmation plug ok");

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
    ) -> Result<Vec<SideEffect<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>>, &'static str>
    {
        Ok(vec![])
    }

    /// The account ID of the Circuit Vault.
    pub fn account_id() -> T::AccountId {
        <T as Config>::SelfAccountId::get()
    }

    pub fn convert_side_effects(
        side_effects: Vec<Vec<u8>>,
    ) -> Result<Vec<SideEffect<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>>, &'static str>
    {
        let side_effects: Vec<SideEffect<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>> =
            side_effects
                .into_iter()
                .filter_map(|se| se.try_into().ok()) // TODO: maybe not
                .collect();
        if side_effects.is_empty() {
            Err("No side effects provided")
        } else {
            Ok(side_effects)
        }
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
                requester,
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

    pub(self) fn get_current_step_fsx(
        local_ctx: &LocalXtxCtx<T>,
    ) -> &Vec<
        FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
    > {
        let current_step = local_ctx.xtx.steps_cnt.0;
        &local_ctx.full_side_effects[current_step as usize]
    }

    pub(self) fn get_current_step_fsx_by_security_lvl(
        local_ctx: &mut LocalXtxCtx<T>,
        security_lvl: SecurityLvl,
    ) -> Vec<
        FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
    > {
        log::info!("get_current_step_fsx");
        log::info!("xtx_id: {:?}", local_ctx.xtx_id);
        let current_step = local_ctx.xtx.steps_cnt.0;
        local_ctx.full_side_effects[current_step as usize]
            .iter()
            .filter(|&fsx| fsx.security_lvl == security_lvl)
            .cloned()
            .collect()
    }

    pub(self) fn storage_write_new_sfx_accepted_bid(
        local_ctx: &mut LocalXtxCtx<T>,
        sfx_id: SideEffectId<T>,
        sfx_bid: SFXBid<
            <T as frame_system::Config>::AccountId,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
    ) {
        <PendingSFXBids<T>>::insert(local_ctx.xtx_id, sfx_id, sfx_bid)
    }

    pub(self) fn storage_read_sfx_accepted_bid(
        local_ctx: &mut LocalXtxCtx<T>,
        sfx_id: SideEffectId<T>,
    ) -> Option<
        SFXBid<
            <T as frame_system::Config>::AccountId,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
    > {
        // fixme: This accesses storage and therefor breaks the rule of a single-storage access at setup.
        <PendingSFXBids<T>>::get(local_ctx.xtx_id, sfx_id)
    }

    pub(self) fn get_fsx_total_rewards(
        fsxs: &[FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >],
    ) -> EscrowedBalanceOf<T, <T as Config>::Escrowed> {
        let mut acc_rewards: EscrowedBalanceOf<T, <T as Config>::Escrowed> = Zero::zero();

        for fsx in fsxs {
            acc_rewards += fsx.get_bond_value(fsx.input.max_reward);
        }

        acc_rewards
    }

    pub(self) fn recover_fsx_by_id(
        sfx_id: SideEffectId<T>,
        local_ctx: &LocalXtxCtx<T>,
    ) -> Result<
        FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >,
        Error<T>,
    > {
        let current_step = local_ctx.xtx.steps_cnt.0;
        let maybe_fsx = local_ctx.full_side_effects[current_step as usize]
            .iter()
            .filter(|&fsx| fsx.confirmed.is_none())
            .find(|&fsx| fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id) == sfx_id);

        if let Some(fsx) = maybe_fsx {
            Ok(fsx.clone())
        } else {
            Err(Error::<T>::LocalSideEffectExecutionNotApplicable)
        }
    }

    pub(self) fn recover_local_ctx_by_sfx_id(
        sfx_id: SideEffectId<T>,
    ) -> Result<LocalXtxCtx<T>, Error<T>> {
        let xtx_id = <Self as Store>::SFX2XTXLinksMap::get(sfx_id)
            .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;
        Self::setup(
            CircuitStatus::PendingExecution,
            &Self::account_id(),
            Zero::zero(),
            Some(xtx_id),
        )
        // todo: ensure recovered via XBIPromise are Local (Type::Internal)
    }

    pub fn do_xbi_exit(
        xbi_checkin: XBICheckIn<T::BlockNumber>,
        _xbi_checkout: XBICheckOut,
    ) -> Result<(), Error<T>> {
        // Recover SFX ID from XBI Metadata
        let sfx_id: SideEffectId<T> =
            Decode::decode(&mut &xbi_checkin.xbi.metadata.id.encode()[..])
                .expect("XBI metadata id conversion should always decode to Sfx ID");

        let mut local_xtx_ctx: LocalXtxCtx<T> = Self::recover_local_ctx_by_sfx_id(sfx_id)?;

        let fsx = Self::recover_fsx_by_id(sfx_id, &local_xtx_ctx)?;

        // todo#2: local fail Xtx if xbi_checkout::result errored

        let escrow_source = Self::account_id();
        let executor = if let Some(ref known_origin) = xbi_checkin.xbi.metadata.maybe_known_origin {
            known_origin.clone()
        } else {
            return Err(Error::<T>::FailedToExitXBIPortal)
        };
        let executor_decoded = Decode::decode(&mut &executor.encode()[..])
            .expect("XBI metadata executor conversion should always decode to local Account ID");

        let xbi_exit_event = match xbi_checkin.clone().xbi.instr {
            XBIInstr::CallNative { payload } => Ok(Event::<T>::CallNative(escrow_source, payload)),
            XBIInstr::CallEvm {
                source,
                target,
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
                target,
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

        Self::deposit_event(xbi_exit_event.clone());

        let confirmation = xbi_result_2_sfx_confirmation::<T, T::Escrowed>(
            xbi_checkin.xbi,
            xbi_exit_event.encode(),
            executor_decoded,
        )
        .map_err(|_| Error::<T>::FailedToConvertXBIResult2SFXConfirmation)?;

        let sfx_id = &fsx.generate_id::<SystemHashing<T>, T>(local_xtx_ctx.xtx_id);
        Self::confirm(
            &mut local_xtx_ctx,
            &Self::account_id(),
            sfx_id,
            &confirmation,
        )
        .map_err(|_e| Error::<T>::XBIExitFailedOnSFXConfirmation)?;
        Ok(())
    }
}
