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
use crate::{bids::Bids, state::*};
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
// use pallet_xbi_portal_enter::t3rn_sfx::xbi_result_2_sfx_confirmation;
use sp_runtime::{
    traits::{CheckedAdd, Zero},
    KeyTypeId,
};
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
    volatile::LocalState,
    xdns::Xdns,
    xtx::{Xtx, XtxId},
    GatewayType, *,
};

use t3rn_protocol::side_effects::{
    confirm::protocol::*,
    loader::{UniversalSideEffectsProtocol},
};

use crate::machine::{Machine, *};
pub use state::XExecSignal;

pub use t3rn_protocol::{circuit_inbound::StepConfirmation, merklize::*};
pub use t3rn_sdk_primitives::signal::{ExecutionSignal, SignalKind};

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod bids;
pub mod escrow;
pub mod machine;
pub mod square_up;
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

reexport_currency_types!();

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
    use pallet_xbi_portal::{
        substrate_abi::{AccountId20, AccountId32, AssetId, Data, Gas, Value, ValueEvm},
        xbi_format::XbiCheckOutStatus,
    };
    use sp_std::borrow::ToOwned;
    use t3rn_primitives::{
        circuit::{LocalStateExecutionView, LocalTrigger, OnLocalTrigger},
        portal::Portal,
        xdns::Xdns,
    };

    pub use crate::weights::WeightInfo;

    pub type EscrowBalance<T> = BalanceOf<T>;

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

    /// Temporary bidding timeouts map for SFX executions. Cleaned out each Config::BidsInterval,
    ///     where for each FSX::best_bid bidders are assigned for SFX::enforce_executor or Xtx is dropped.
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
                    BalanceOf<T>,
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

        type Currency: Currency<Self::AccountId>;

        /// A type that provides access to Xdns
        type Xdns: Xdns<Self>;

        type Executors: Executors<Self, BalanceOf<Self>>;

        /// A type that provides access to AccountManager
        type AccountManager: AccountManager<
            Self::AccountId,
            BalanceOf<Self>,
            Self::Hash,
            Self::BlockNumber,
            u32,
        >;

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
            let deletion_counter: u32 = 0;

            // Check for expiring bids each block
            <PendingXtxBidsTimeoutsMap<T>>::iter().filter(|(_xtx_id, bidding_timeouts_at)| {
                // ToDo consider moving xtx_bids to xtx_ctx in order to self update to always determine status
                bidding_timeouts_at <= &frame_system::Pallet::<T>::block_number()
            }).map(|(xtx_id, _bidding_timeouts_at)| {
                if deletion_counter <= T::DeletionQueueLimit::get() {
                    Machine::<T>::compile_infallible(
                        &mut Machine::<T>::load_xtx(xtx_id).expect("xtx_id corresponds to a valid Xtx when reading from PendingXtxBidsTimeoutsMap storage"),
                        |current_fsx, _local_state, _steps_cnt, status, _requester| {
                            match status {
                                CircuitStatus::PendingBidding | CircuitStatus::InBidding => {},
                                _ => return PrecompileResult::TryKill(Cause::Timeout)
                            }
                            match current_fsx.iter().all(|fsx| fsx.best_bid.is_some()) {
                                true => PrecompileResult::ForceUpdateStatus(CircuitStatus::Ready),
                                false => PrecompileResult::TryKill(Cause::Timeout)
                            }
                        },
                        |_status_change, local_ctx| {
                            // Account fees and charges happens internally in Machine::apply
                            Self::emit_status_update(
                                local_ctx.xtx_id,
                                Some(local_ctx.xtx.clone()),
                                None,
                            );
                        },
                    );
                    deletion_counter.checked_add(1).unwrap_or_else(|| {
                        log::error!("XtxBiddingInterval::DeletionQueueLimit is too low, causing overflow");
                        u32::MAX
                    });
                }
            }).count();

            // Scenario 1: all the timeout s can be handled in the block space
            // Scenario 2: all but 5 timeouts can be handled
            //     - add the 5 timeouts to an immediate queue for the next block
            if n % T::XtxTimeoutCheckInterval::get() == T::BlockNumber::from(0u8) {
                // Go over all unfinished Xtx to find those that timed out
                <PendingXtxTimeoutsMap<T>>::iter().filter(|(_xtx_id, timeout_at)| {
                    timeout_at <= &frame_system::Pallet::<T>::block_number()
                }).map(|(xtx_id, _timeout_at)| {
                    if deletion_counter <= T::DeletionQueueLimit::get() {
                        let _success: bool = Machine::<T>::revert(
                            xtx_id,
                            Cause::Timeout,
                            |_status_change, _local_ctx| {
                                Self::deposit_event(
                                    Event::XTransactionXtxRevertedAfterTimeOut(xtx_id),
                                );
                            },
                        );
                        deletion_counter.checked_add(1).unwrap_or_else(|| {
                            log::error!("XtxTimeoutCheckInterval::DeletionQueueLimit is too low, causing overflow");
                            u32::MAX
                        });
                    }
                }).count();
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

            // We must apply the state only if its generated and fresh
            let local_ctx = match maybe_xtx_id {
                Some(xtx_id) => Machine::<T>::load_xtx(xtx_id)?,
                None => {
                    let mut local_ctx = Machine::<T>::setup(&[], &requester)?;
                    Machine::<T>::compile(&mut local_ctx, no_mangle, no_post_updates)?;
                    local_ctx
                },
            };

            let hardened_side_effects = local_ctx
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

            let local_state_execution_view = LocalStateExecutionView::<T, BalanceOf<T>>::new(
                local_ctx.xtx_id,
                local_ctx.local_state.clone(),
                hardened_side_effects,
                local_ctx.xtx.steps_cnt,
            );

            log::debug!(
                target: "runtime::circuit",
                "load_local_state with status: {:?}",
                local_ctx.xtx.status
            );

            Ok(local_state_execution_view)
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

            let mut local_ctx = match trigger.maybe_xtx_id {
                Some(xtx_id) => Machine::<T>::load_xtx(xtx_id)?,
                None => {
                    let mut local_ctx = Machine::<T>::setup(&[], &requester)?;
                    Machine::<T>::compile(&mut local_ctx, no_mangle, no_post_updates)?;
                    local_ctx
                },
            };

            let xtx_id = local_ctx.xtx_id;
            log::debug!(
                target: "runtime::circuit",
                "submit_side_effects xtx state with status: {:?}",
                local_ctx.xtx.status.clone()
            );

            Machine::<T>::compile(
                &mut local_ctx,
                |current_fsx, local_state, steps_cnt, status, _requester| {
                    match Self::exec_in_xtx_ctx(xtx_id, local_state, current_fsx, steps_cnt) {
                        Err(err) => {
                            if status == CircuitStatus::Ready {
                                return Ok(PrecompileResult::TryKill(Cause::IntentionalKill))
                            }
                            Err(err)
                        },
                        Ok(new_fsx) => Ok(PrecompileResult::TryUpdateFSX(new_fsx)),
                    }
                },
                |_status_change, _local_ctx| {
                    // Emit: From Circuit events
                    // ToDo: impl FSX convert to SFX
                    // Self::emit_sfx(local_ctx.xtx_id, &requester, &local_ctx.full_side_effects.into());
                    Ok(())
                },
            )?;

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
            let attempting_requester = Self::authorize(origin, CircuitRole::Requester)?;

            Machine::<T>::compile(
                &mut Machine::<T>::load_xtx(xtx_id)?,
                |current_fsx, _local_state, _steps_cnt, status, requester| {
                    if attempting_requester != requester || status > CircuitStatus::PendingBidding {
                        return Err(Error::<T>::UnauthorizedCancellation)
                    }
                    // Drop cancellation in case some bids have already been posted
                    if current_fsx.iter().any(|fsx| fsx.best_bid.is_some()) {
                        return Err(Error::<T>::UnauthorizedCancellation)
                    }
                    Ok(PrecompileResult::TryKill(Cause::IntentionalKill))
                },
                no_post_updates,
            )?;

            Ok(().into())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::cancel_xtx())]
        pub fn revert(origin: OriginFor<T>, xtx_id: T::Hash) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            let _success =
                Machine::<T>::revert(xtx_id, Cause::IntentionalKill, infallible_no_post_updates);
            Ok(().into())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_extrinsic_trigger())]
        pub fn on_extrinsic_trigger(
            origin: OriginFor<T>,
            side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>>,
            _sequential: bool,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let requester = Self::authorize(origin, CircuitRole::Requester)?;
            // Setup: new xtx context with SFX validation
            let mut fresh_xtx = Machine::<T>::setup(&side_effects, &requester)?;
            // Compile: apply the new state post squaring up and emit
            Machine::<T>::compile(
                &mut fresh_xtx,
                |_, _, _, _, _| Ok(PrecompileResult::TryRequest),
                |_status_change, local_ctx| {
                    // Emit: circuit events
                    Self::emit_sfx(local_ctx.xtx_id, &requester, &side_effects);
                    Ok(())
                },
            )?;

            Ok(().into())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::bid_sfx())]
        pub fn bid_sfx(
            origin: OriginFor<T>, // Active relayer
            sfx_id: SideEffectId<T>,
            bid_amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let bidder = Self::authorize(origin, CircuitRole::Executor)?;
            // retrieve xtx_id
            let xtx_id = <Self as Store>::SFX2XTXLinksMap::get(sfx_id)
                .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;

            Machine::<T>::compile(
                &mut Machine::<T>::load_xtx(xtx_id)?,
                |_current_fsx, _local_state, _steps_cnt, _status, _requester| {
                    // Check if Xtx is in the bidding state
                    Ok(PrecompileResult::TryBid((
                        sfx_id,
                        bid_amount,
                        bidder.clone(),
                    )))
                },
                |_status_change, _local_ctx| {
                    Self::deposit_event(Event::SFXNewBidReceived(
                        sfx_id,
                        bidder.clone(),
                        bid_amount,
                    ));
                    Ok(())
                },
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
                BalanceOf<T>,
            >,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let _executor = Self::authorize(origin, CircuitRole::Executor)?;
            let xtx_id = <Self as Store>::SFX2XTXLinksMap::get(sfx_id)
                .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;

            Machine::<T>::compile(
                &mut Machine::<T>::load_xtx(xtx_id)?,
                |current_fsx, local_state, _steps_cnt, __status, _requester| {
                    Self::confirm(xtx_id, current_fsx, &local_state, &sfx_id, &confirmation)
                        .map_err(|e| {
                            log::error!("Self::confirm hit an error -- {:?}", e);
                            Error::<T>::ConfirmationFailed
                        })?;
                    Ok(PrecompileResult::TryUpdateFSX(current_fsx.clone()))
                },
                |_status_change, local_ctx| {
                    Self::deposit_event(Event::SideEffectConfirmed(sfx_id));
                    // Emit: From Circuit events
                    Self::emit_status_update(
                        local_ctx.xtx_id,
                        Some(local_ctx.xtx.clone()),
                        Some(local_ctx.full_side_effects.clone()),
                    );
                    Ok(())
                },
            )?;

            Ok(().into())
        }
    }

    use crate::machine::{no_mangle, Machine};

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
        // Notification(T::AccountId, AccountId32, XBINotificationKind, Data, Data),
        Result(T::AccountId, AccountId32, XbiCheckOutStatus, Data, Data),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and pending
        XTransactionReceivedForExec(XExecSignalId<T>),
        // New best bid for SFX has been accepted. Account here is an executor.
        SFXNewBidReceived(
            SideEffectId<T>,
            <T as frame_system::Config>::AccountId,
            BalanceOf<T>,
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
            Vec<SideEffect<<T as frame_system::Config>::AccountId, BalanceOf<T>>>,
            Vec<SideEffectId<T>>,
        ),
        // Listeners - executioners/relayers to know that certain SideEffects are no longer valid
        // ToDo: Implement Xtx timeout!
        CancelledSideEffects(
            <T as frame_system::Config>::AccountId,
            XtxId<T>,
            Vec<SideEffect<<T as frame_system::Config>::AccountId, BalanceOf<T>>>,
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
        EscrowTransfer(
            // ToDo: Inspect if Xtx needs to be here and how to process from protocol
            T::AccountId, // from
            T::AccountId, // to
            BalanceOf<T>, // value
        ),
    }

    #[pallet::error]
    pub enum Error<T> {
        UpdateAttemptDoubleRevert,
        UpdateAttemptDoubleKill,
        UpdateStateTransitionDisallowed,
        UpdateForcedStateTransitionDisallowed,
        UpdateXtxTriggeredWithUnexpectedStatus,
        ConfirmationFailed,
        ApplyTriggeredWithUnexpectedStatus,
        BidderNotEnoughBalance,
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
        BiddingRejectedBidTooHigh,
        BiddingRejectedInsuranceTooLow,
        BiddingRejectedBetterBidFound,
        BiddingRejectedFailedToDepositBidderBond,
        BiddingFailedExecutorsBalanceTooLowToReserve,
        InsuranceBondAlreadyDeposited,
        InvalidFTXStateEmptyBidForReadyXtx,
        InvalidFTXStateEmptyConfirmationForFinishedXtx,
        InvalidFTXStateUnassignedExecutorForReadySFX,
        InvalidFTXStateIncorrectExecutorForReadySFX,
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
        XtxDoesNotExist,
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
        FSXNotFoundById,
        LocalSideEffectExecutionNotApplicable,
        LocalExecutionUnauthorized,
        OnLocalTriggerFailedToSetupXtx,
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
        ArithmeticErrorOverflow,
        ArithmeticErrorUnderflow,
        ArithmeticErrorDivisionByZero,
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
    fn emit_sfx(
        xtx_id: XExecSignalId<T>,
        subjected_account: &T::AccountId,
        side_effects: &Vec<SideEffect<T::AccountId, BalanceOf<T>>>,
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
        maybe_xtx: Option<XExecSignal<T::AccountId, T::BlockNumber>>,
        maybe_full_side_effects: Option<
            Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>>,
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
                CircuitStatus::Reverted(ref _cause) =>
                    Self::deposit_event(Event::XTransactionXtxRevertedAfterTimeOut(xtx_id)),
                CircuitStatus::Killed(ref _cause) =>
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
        side_effects: &[SideEffect<T::AccountId, BalanceOf<T>>],
        local_ctx: &mut LocalXtxCtx<T>,
    ) -> Result<(), &'static str> {
        let mut full_side_effects: Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>> =
            vec![];

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

            let decoded_sfx_type = sfx.validate(gateway_abi)?;
            allowed_side_effects
                .iter()
                .find(|(&x, _)| x == decoded_sfx_type)
                .ok_or("SFX not allowed on that target")?;

            if let Some(next) = side_effects.get(index + 1) {
                if sfx.reward_asset_id != next.reward_asset_id {
                    return Err(
                        "SFX validate failed - enforce all SFX to have the same reward asset field",
                    )
                }
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

        let mut escrow_sfx_step: Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>> =
            vec![];
        let mut optimistic_sfx_step: Vec<
            FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>,
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
        xtx_id: XExecSignalId<T>,
        step_side_effects: &mut Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
        >,
        local_state: &LocalState,
        sfx_id: &SideEffectId<T>,
        confirmation: &ConfirmedSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            BalanceOf<T>,
        >,
    ) -> Result<(), &'static str> {
        fn confirm_order<T: Config>(
            xtx_id: XExecSignalId<T>,
            sfx_id: SideEffectId<T>,
            confirmation: &ConfirmedSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
            step_side_effects: &mut Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    BalanceOf<T>,
                >,
            >,
        ) -> Result<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
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
        let fsx = confirm_order::<T>(xtx_id, *sfx_id, confirmation, step_side_effects)?;

        log::debug!("Order confirmed!");

        let mut side_effect_id: [u8; 4] = [0, 0, 0, 0];
        side_effect_id.copy_from_slice(&fsx.input.encoded_action[0..4]);

        // confirm the payload is included in the specified block, and return the SideEffect params as defined in XDNS.
        // this could be multiple events!
        let (params, source) = <T as Config>::Portal::confirm_and_decode_payload_params(
            fsx.input.target,
            fsx.submission_target_height,
            confirmation.inclusion_data.clone(),
            side_effect_id,
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
            local_state,
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
        _full_side_effects: &mut Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        _steps_cnt: (u32, u32),
    ) -> Result<Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>, Error<T>> {
        Ok(vec![])
    }

    /// The account ID of the Circuit Vault.
    pub fn account_id() -> T::AccountId {
        <T as Config>::SelfAccountId::get()
    }

    /// Get pending Bids for SFX - Pending meaning that the SFX is still In Bidding
    pub fn get_pending_sfx_bids(
        xtx_id: XExecSignalId<T>,
        sfx_id: SideEffectId<T>,
    ) -> Result<Option<SFXBid<T::AccountId, BalanceOf<T>, u32>>, Error<T>> {
        let local_ctx = Machine::<T>::load_xtx(xtx_id)?;
        let current_step_fsx = Machine::<T>::read_current_step_fsx(&local_ctx);
        let fsx = current_step_fsx
            .iter()
            .find(|fsx| fsx.generate_id::<SystemHashing<T>, T>(xtx_id) == sfx_id)
            .ok_or(Error::<T>::FSXNotFoundById)?;

        match &fsx.best_bid {
            Some(bid) => match &fsx.input.enforce_executor {
                // Bid posted for this SFX has already been accepted, therefore Bid isn't pending.
                Some(_executor) => Ok(None),
                // Bid has been posted for this SFX but not yet accepted, therefore pending.
                None => Ok(Some(bid.clone())),
            },
            // No bids posted for this SFX
            None => Ok(None),
        }
    }

    pub fn convert_side_effects(
        side_effects: Vec<Vec<u8>>,
    ) -> Result<Vec<SideEffect<T::AccountId, BalanceOf<T>>>, &'static str> {
        let side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>> = side_effects
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
        let mut remaining_key_budget = if let Some(v) = T::SignalQueueDepth::get().checked_div(4) {
            v
        } else {
            log::error!("Division error on signal queue depth (`SignalQueueDepth::get()`).");
            T::SignalQueueDepth::get()
        };
        let mut processed_weight = 0 as Weight;

        while !queue.is_empty() && remaining_key_budget > 0 {
            // Cannot panic due to loop condition
            let (_requester, signal) = &mut queue[0];

            // worst case 4 from setup
            if let Some(v) = processed_weight.checked_add(db_weight.reads(4 as Weight) as Weight) {
                processed_weight = v
            }
            match Machine::<T>::load_xtx(signal.execution_id) {
                Ok(local_ctx) => {
                    let _success: bool = Machine::<T>::kill(
                        local_ctx.xtx_id,
                        Cause::IntentionalKill,
                        |_status_change, _local_ctx| {
                            queue.swap_remove(0);
                            remaining_key_budget -= 1;
                            // apply has 2
                            processed_weight += db_weight.reads_writes(2 as Weight, 1 as Weight);
                        },
                    );
                },
                Err(_err) => {
                    log::error!("Could not handle signal");
                    // Slide the erroneous signal to the back
                    queue.slide(0, queue.len());
                },
            }
        }
        // Initial read of queue and update
        if let Some(v) =
            processed_weight.checked_add(db_weight.reads_writes(1 as Weight, 1 as Weight))
        {
            processed_weight = v
        } else {
            log::error!("Could not initial read of queue and update")
        }

        <SignalQueue<T>>::put(queue);

        processed_weight
    }

    pub fn recover_local_ctx_by_sfx_id(
        sfx_id: SideEffectId<T>,
    ) -> Result<LocalXtxCtx<T>, Error<T>> {
        let xtx_id = <Self as Store>::SFX2XTXLinksMap::get(sfx_id)
            .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;
        Machine::<T>::load_xtx(xtx_id)
    }
}
