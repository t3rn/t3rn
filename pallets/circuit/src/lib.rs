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
    dispatch::{DispatchResultWithPostInfo, Dispatchable, GetDispatchInfo},
    ensure,
    traits::{Currency, ExistenceRequirement::AllowDeath, Get},
    weights::Weight,
    RuntimeDebug,
};
use frame_system::{
    ensure_signed,
    offchain::{SignedPayload, SigningTypes},
    pallet_prelude::OriginFor,
};
use sp_core::H256;
use sp_runtime::{
    traits::{CheckedAdd, Zero},
    DispatchError, KeyTypeId,
};
use sp_std::{convert::TryInto, vec, vec::Vec};

pub use t3rn_types::{
    bid::SFXBid,
    fsx::FullSideEffect,
    sfx::{ConfirmedSideEffect, HardenedSideEffect, SecurityLvl, SideEffect, SideEffectId},
};

pub use t3rn_primitives::{
    account_manager::{AccountManager, Outcome},
    circuit::{XExecSignalId, XExecStepSideEffectId},
    claimable::{BenefitSource, CircuitRole},
    executors::Executors,
    gateway::{GatewayABIConfig, HasherAlgo as HA},
    portal::{HeightResult, Portal},
    volatile::LocalState,
    xdns::Xdns,
    xtx::{Xtx, XtxId},
    GatewayType, *,
};

use crate::machine::{Machine, *};
pub use state::XExecSignal;

use t3rn_abi::{recode::Codec, sfx_abi::SFXAbi};

pub use t3rn_primitives::light_client::InclusionReceipt;
use t3rn_primitives::{attesters::AttestersWriteApi, circuit::ReadSFX};
pub use t3rn_sdk_primitives::signal::{ExecutionSignal, SignalKind};
use t3rn_types::fsx::TargetId;

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

//

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
        xp_format::XbiResult,
    };
    use sp_std::borrow::ToOwned;
    use t3rn_primitives::{
        attesters::AttestersWriteApi,
        circuit::{
            CircuitDLQ, CircuitSubmitAPI, LocalStateExecutionView, LocalTrigger, OnLocalTrigger,
            ReadSFX,
        },
        portal::Portal,
        xdns::Xdns,
        SpeedMode,
    };
    use t3rn_types::migrations::v13::FullSideEffectV13;

    pub use crate::weights::WeightInfo;

    pub type EscrowBalance<T> = BalanceOf<T>;

    #[pallet::storage]
    #[pallet::getter(fn storage_migrations_done)]
    pub type StorageMigrations<T: Config> = StorageValue<_, u32, ValueQuery>;

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
        // Collection of timeout blocks (on the slowest remote target (there) and here (on t3rn/t0rn)), where the emergency height is a set constant via config (400blocks),
        //  but the primary timeout to look at is AdaptiveTimeout here and there calculated based on advancing epochs of each target.
        AdaptiveTimeout<<T as frame_system::Config>::BlockNumber, [u8; 4]>,
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

    #[pallet::storage]
    #[pallet::getter(fn get_dlq)]
    pub type DLQ<T> = StorageMap<
        _,
        Identity,
        XExecSignalId<T>,
        (
            <T as frame_system::Config>::BlockNumber,
            Vec<TargetId>,
            SpeedMode,
        ),
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
        type Xdns: Xdns<Self, BalanceOf<Self>>;

        type Attesters: AttestersWriteApi<Self::AccountId, DispatchError>;

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
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            0
        }

        fn on_finalize(_n: T::BlockNumber) {
            // x-t3rn#4: Go over open Xtx and cancel if necessary
        }

        fn offchain_worker(_n: T::BlockNumber) {}

        fn on_runtime_upgrade() -> Weight {
            // Define the maximum weight of this migration.
            let max_weight = T::DbWeight::get().reads_writes(10, 10);
            // Define the current storage migration version.
            const CURRENT_STORAGE_VERSION: u32 = 1;
            // Migrate the storage entries.
            StorageMigrations::<T>::try_mutate(|current_version| {
                match *current_version {
                    0 => {
                        // Storage Migration: FSX::SFX updates field "encoded_action: Vec<u8>" to "action: Action: [u8; 4]"
                        // Storage Migration Details: 16-03-2023; v1.3.0-rc -> v1.4.0-rc
                        // Iterate through the old storage entries and migrate them.
                        FullSideEffects::<T>::translate(
                            |_,
                             value: Vec<
                                Vec<FullSideEffectV13<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
                            >| {
                                Some(
                                    value
                                        .into_iter()
                                        .map(|v| v.into_iter().map(FullSideEffect::from).collect())
                                        .collect(),
                                )
                            },
                        );

                        // Set migrations_done to true
                        *current_version = CURRENT_STORAGE_VERSION;

                        // Return the weight consumed by the migration.
                        Ok::<Weight, DispatchError>(max_weight)
                    },
                    // Add more migration cases here, if needed in the future
                    _ => {
                        // No migration needed.
                        Ok::<Weight, DispatchError>(0 as Weight)
                    },
                }
            })
            .unwrap_or(0)
        }
    }

    impl<T: Config> CircuitDLQ<T> for Pallet<T> {
        fn process_dlq(n: T::BlockNumber) -> Weight {
            Self::process_dlq(n)
        }

        fn process_adaptive_xtx_timeout_queue(
            n: T::BlockNumber,
            verifier: &GatewayVendor,
        ) -> Weight {
            Self::process_adaptive_xtx_timeout_queue(n, verifier)
        }
    }

    impl<T: Config> ReadSFX<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber> for Pallet<T> {
        fn get_fsx_of_xtx(xtx_id: T::Hash) -> Result<Vec<T::Hash>, DispatchError> {
            let full_side_effects = FullSideEffects::<T>::get(xtx_id)
                .ok_or::<DispatchError>(Error::<T>::XtxNotFound.into())?;

            let fsx_ids: Vec<T::Hash> = full_side_effects
                .iter()
                .flat_map(|fsx_vec| {
                    fsx_vec.iter().enumerate().map(|(index, fsx)| {
                        fsx.input
                            .generate_id::<SystemHashing<T>>(xtx_id.as_ref(), index as u32)
                    })
                })
                .collect::<Vec<T::Hash>>();

            Ok(fsx_ids)
        }

        fn get_fsx_status(fsx_id: T::Hash) -> Result<CircuitStatus, DispatchError> {
            let xtx_id = SFX2XTXLinksMap::<T>::get(fsx_id)
                .ok_or::<DispatchError>(Error::<T>::XtxNotFound.into())?;

            Ok(Self::get_xtx_status(xtx_id)?.0)
        }

        // Look up the FSX by its ID and return the FSX if it exists
        fn get_fsx(
            fsx_id: T::Hash,
        ) -> Result<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>, DispatchError>
        {
            let xtx_id = SFX2XTXLinksMap::<T>::get(fsx_id)
                .ok_or::<DispatchError>(Error::<T>::XtxNotFound.into())?;

            let full_side_effects = FullSideEffects::<T>::get(xtx_id)
                .ok_or::<DispatchError>(Error::<T>::XtxNotFound.into())?;

            // Early return on empty vector
            if full_side_effects.is_empty() {
                return Err(Error::<T>::FSXNotFoundById.into())
            }

            for fsx_step in &full_side_effects {
                for (index, fsx) in fsx_step.iter().enumerate() {
                    if fsx
                        .input
                        .generate_id::<SystemHashing<T>>(xtx_id.as_ref(), index as u32)
                        == fsx_id
                    {
                        // Return a reference instead of a clone
                        return Ok(fsx.clone())
                    }
                }
            }

            Err(Error::<T>::FSXNotFoundById.into())
        }

        fn get_fsx_requester(fsx_id: T::Hash) -> Result<T::AccountId, DispatchError> {
            let xtx_id = SFX2XTXLinksMap::<T>::get(fsx_id)
                .ok_or::<DispatchError>(Error::<T>::XtxNotFound.into())?;

            let xtx = XExecSignals::<T>::get(xtx_id)
                .ok_or::<DispatchError>(Error::<T>::XtxNotFound.into())?;

            Ok(xtx.requester)
        }

        fn get_xtx_status(
            xtx_id: T::Hash,
        ) -> Result<(CircuitStatus, AdaptiveTimeout<T::BlockNumber, TargetId>), DispatchError>
        {
            XExecSignals::<T>::get(xtx_id)
                .map(|xtx| (xtx.status, xtx.timeouts_at))
                .ok_or(Error::<T>::XtxNotFound.into())
        }
    }

    impl<T: Config> CircuitSubmitAPI<T, BalanceOf<T>> for Pallet<T> {
        fn on_extrinsic_trigger(
            origin: OriginFor<T>,
            side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>>,
            speed_mode: SpeedMode,
        ) -> DispatchResultWithPostInfo {
            Self::on_extrinsic_trigger(origin, side_effects, speed_mode)
        }

        fn on_remote_origin_trigger(
            origin: OriginFor<T>,
            order_origin: T::AccountId,
            side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>>,
            speed_mode: SpeedMode,
        ) -> DispatchResultWithPostInfo {
            Self::on_remote_origin_trigger(origin, order_origin, side_effects, speed_mode)
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
                    let mut local_ctx = Machine::<T>::setup(&[], &requester, None)?;
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

        fn on_local_trigger(
            origin: &OriginFor<T>,
            trigger: LocalTrigger<T>,
        ) -> Result<LocalStateExecutionView<T, BalanceOf<T>>, DispatchError> {
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
                    let mut local_ctx = Machine::<T>::setup(&[], &requester, None)?;
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

            Self::load_local_state(origin, Some(xtx_id))
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
            let _execution_state_view =
                <Self as OnLocalTrigger<T, BalanceOf<T>>>::on_local_trigger(
                    &origin,
                    LocalTrigger::<T>::decode(&mut &trigger[..])
                        .map_err(|_| Error::<T>::InsuranceBondNotRequired)?,
                )?;
            Ok(())
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

        #[pallet::weight(<T as pallet::Config>::WeightInfo::cancel_xtx())]
        pub fn trigger_dlq(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            Self::process_dlq(<frame_system::Pallet<T>>::block_number());
            Ok(().into())
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_extrinsic_trigger())]
        pub fn on_remote_origin_trigger(
            origin: OriginFor<T>,
            order_origin: T::AccountId,
            side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>>,
            speed_mode: SpeedMode,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let _ = Self::authorize(origin, CircuitRole::Executor)?;

            // Skip remote origin withdrawals - they are already handled by the remote origin
            let requester = match OrderOrigin::<T::AccountId>::new(&order_origin) {
                OrderOrigin::Local(_) => return Err(Error::<T>::InvalidOrderOrigin.into()),
                OrderOrigin::Remote(_) => order_origin.clone(),
            };

            Self::do_on_extrinsic_trigger(requester, side_effects, speed_mode)
        }

        #[pallet::weight(<T as pallet::Config>::WeightInfo::on_extrinsic_trigger())]
        pub fn on_extrinsic_trigger(
            origin: OriginFor<T>,
            side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>>,
            speed_mode: SpeedMode,
        ) -> DispatchResultWithPostInfo {
            // Authorize: Retrieve sender of the transaction.
            let requester = Self::authorize(origin, CircuitRole::Requester)?;

            Self::do_on_extrinsic_trigger(requester, side_effects, speed_mode)
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
                |current_fsx, _local_state, _steps_cnt, __status, _requester| {
                    Self::confirm(xtx_id, current_fsx, &sfx_id, &confirmation).map_err(|e| {
                        log::error!("Self::confirm hit an error -- {:?}", e);
                        Error::<T>::ConfirmationFailed
                    })?;
                    Ok(PrecompileResult::TryConfirm(sfx_id, confirmation))
                },
                |status_change, local_ctx| {
                    Self::deposit_event(Event::SideEffectConfirmed(sfx_id));
                    if status_change.1 == CircuitStatus::FinishedAllSteps
                        || status_change.1 == CircuitStatus::Committed
                    {
                        Self::request_sfx_attestation(local_ctx);
                        // ToDo: uncomment when price + costs estimates are implemented
                        // T::Xdns::estimate_costs(Machine::read_current_step_fsx(local_ctx));
                    }
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
        Result(T::AccountId, AccountId32, XbiResult, Data, Data),
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
        // Listeners - users + SDK + +executors + attesters to know FSX is resolved
        XTransactionFSXCommitted(XExecSignalId<T>),
        // Listeners - users + SDK + +executors + attesters to know Xtx is fully resolved
        XTransactionXtxCommitted(XExecSignalId<T>),
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
        SuccessfulFSXCommitAttestationRequest(H256),
        UnsuccessfulFSXCommitAttestationRequest(H256),
        SuccessfulFSXRevertAttestationRequest(H256),
        UnsuccessfulFSXRevertAttestationRequest(H256),
    }

    #[pallet::error]
    pub enum Error<T> {
        UpdateAttemptDoubleRevert,
        UpdateAttemptDoubleKill,
        UpdateStateTransitionDisallowed,
        UpdateForcedStateTransitionDisallowed,
        UpdateXtxTriggeredWithUnexpectedStatus,
        ConfirmationFailed,
        InvalidOrderOrigin,
        ApplyTriggeredWithUnexpectedStatus,
        BidderNotEnoughBalance,
        RequesterNotEnoughBalance,
        AssetsFailedToWithdraw,
        SanityAfterCreatingSFXDepositsFailed,
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
        GatewayNotActive,
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
        XtxNotFound,
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
        FailedToCommitFSX,
        XBIExitFailedOnSFXConfirmation,
        UnsupportedRole,
        InvalidLocalTrigger,
        SignalQueueFull,
        ArithmeticErrorOverflow,
        ArithmeticErrorUnderflow,
        ArithmeticErrorDivisionByZero,
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
                CircuitStatus::Committed =>
                    Self::deposit_event(Event::XTransactionXtxCommitted(xtx_id)),
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

    fn do_on_extrinsic_trigger(
        requester: T::AccountId,
        side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>>,
        speed_mode: SpeedMode,
    ) -> DispatchResultWithPostInfo {
        // Setup: new xtx context with SFX validation
        let mut fresh_xtx = Machine::<T>::setup(
            &side_effects,
            &requester,
            Some(T::Xdns::estimate_adaptive_timeout_on_slowest_target(
                side_effects
                    .iter()
                    .map(|sfx| sfx.target)
                    .collect::<Vec<TargetId>>(),
                &speed_mode,
                T::XtxTimeoutDefault::get(),
            )),
        )?;

        fresh_xtx.xtx.set_speed_mode(speed_mode);
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
        local_ctx: &mut LocalXtxCtx<T, BalanceOf<T>>,
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

        // Verify each requested asset is supported by the gateway
        let all_targets = side_effects
            .iter()
            .map(|sfx| sfx.target)
            .collect::<Vec<TargetId>>();

        ensure!(
            Self::ensure_all_gateways_are_active(all_targets),
            Error::<T>::GatewayNotActive
        );

        for (index, sfx) in side_effects.iter().enumerate() {
            let gateway_type = <T as Config>::Xdns::get_gateway_type_unsafe(&sfx.target);
            let security_lvl = determine_security_lvl(gateway_type);

            let sfx_abi: SFXAbi = match <T as Config>::Xdns::get_sfx_abi(&sfx.target, sfx.action) {
                Some(sfx_abi) => sfx_abi,
                None => return Err("SFX not allowed/registered on requested target gateway"),
            };
            // todo: store the codec info in gateway's records and use it here
            sfx.validate(sfx_abi, &Codec::Scale)?;

            if let Some(next) = side_effects.get(index + 1) {
                if sfx.reward_asset_id != next.reward_asset_id {
                    return Err(
                        "SFX validate failed - enforce all SFX to have the same reward asset field",
                    )
                }
            }

            let submission_target_height = match T::Portal::get_fast_height(sfx.target)? {
                HeightResult::Height(block_numer) => block_numer,
                HeightResult::NotActive =>
                    return Err("SFX validate failed - get_latest_finalized_height returned None"),
            };

            full_side_effects.push(FullSideEffect {
                input: sfx.clone(),
                confirmed: None,
                security_lvl,
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
        step_side_effects: &mut Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        sfx_id: &SideEffectId<T>,
        confirmation: &ConfirmedSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>,
    ) -> Result<(), DispatchError> {
        // Double check there are some side effects for that Xtx - should have been checked at API level tho already
        if step_side_effects.is_empty() {
            return Err(DispatchError::Other("Xtx has an empty single step."))
        }

        // Ensure all gateways are active
        // Verify each requested asset is supported by the gateway
        let all_targets = step_side_effects
            .iter()
            .map(|sfx| sfx.input.target)
            .collect::<Vec<TargetId>>();

        ensure!(
            Self::ensure_all_gateways_are_active(all_targets),
            Error::<T>::GatewayNotActive
        );

        let fsx_opt = step_side_effects
            .iter_mut()
            .find(|fsx| fsx.calc_sfx_id::<SystemHashing<T>, T>(xtx_id) == *sfx_id);

        let fsx = match fsx_opt {
            Some(fsx) if fsx.confirmed.is_none() => {
                fsx.confirmed = Some(confirmation.clone());
                fsx.clone()
            },
            Some(_) => return Err(DispatchError::Other("Side Effect already confirmed")),
            None =>
                return Err(DispatchError::Other(
                    "Unable to find matching Side Effect in given Xtx to confirm",
                )),
        };
        log::debug!("Order confirmed!");

        #[cfg(not(feature = "test-skip-verification"))]
        let xtx = Machine::<T>::load_xtx(xtx_id)?.xtx;

        // confirm the payload is included in the specified block, and return the SideEffect params as defined in XDNS.
        // this could be multiple events!
        #[cfg(not(feature = "test-skip-verification"))]
        let inclusion_receipt = <T as Config>::Portal::verify_event_inclusion(
            fsx.input.target,
            xtx.speed_mode,
            None, //ToDo - load pallet index or contract address here
            confirmation.inclusion_data.clone(),
        )
        .map_err(|_| DispatchError::Other("SideEffect confirmation of inclusion failed"))?;

        log::debug!("Inclusion confirmed!");

        // ToDo: handle misbehavior
        #[cfg(not(feature = "test-skip-verification"))]
        log::debug!(
            "SFX confirmation inclusion receipt: {:?}",
            inclusion_receipt
        );

        let sfx_abi =
            <T as Config>::Xdns::get_sfx_abi(&fsx.input.target, fsx.input.action).ok_or({
                DispatchError::Other("Unable to find matching Side Effect descriptor in XDNS")
            })?;

        #[cfg(feature = "test-skip-verification")]
        let inclusion_receipt = InclusionReceipt::<T::BlockNumber> {
            message: confirmation.inclusion_data.clone(),
            including_header: [0u8; 32].encode(),
            height: T::BlockNumber::zero(),
        }; // Empty encoded_event_params for testing purposes

        #[cfg(not(feature = "test-skip-verification"))]
        if inclusion_receipt.height > fsx.submission_target_height {
            return Err(DispatchError::Other(
                "SideEffect confirmation of inclusion failed - inclusion height is higher than target",
            ))
        }

        fsx.input.confirm(
            sfx_abi,
            inclusion_receipt.message,
            // todo: store the codec info in gateway's records and use it here
            &Codec::Scale,
            &Codec::Scale,
        )?;

        log::debug!("Confirmation success");

        Ok(())
    }

    pub fn get_all_xtx_targets(xtx_id: XExecSignalId<T>) -> Vec<TargetId> {
        // Get FSX of XTX
        let fsx_of_xtx = match <Pallet<T>>::get_fsx_of_xtx(xtx_id) {
            Ok(fsx) => fsx,
            Err(_) => return vec![],
        };

        // Map FSX to targets
        fsx_of_xtx
            .into_iter()
            .filter_map(|fsx_id| {
                <Pallet<T>>::get_fsx(fsx_id)
                    .ok()
                    .map(|fsx| fsx.input.target)
            })
            .collect()
    }

    /// At the XTX submission fn verify ensures that all of the gateways are active.
    /// At either confirmation or revert attempt, ensure that all of the gateways are active, so that Executor won't be slashed.
    pub fn ensure_all_gateways_are_active(targets: Vec<TargetId>) -> bool {
        for target in targets.into_iter() {
            let gateway_activity_overview = match <T as Config>::Xdns::read_last_activity(target) {
                Some(gateway_activity_overview) => gateway_activity_overview,
                None => {
                    log::warn!("Failing to ensure_all_gateways_are_active. Target gateway is not registered in XDNS. Observe XDNS::gateway_activity_overview for more updates");
                    return false
                },
            };

            if !gateway_activity_overview.is_active {
                log::warn!(
                    "Failing to ensure_all_gateways_are_active. Target gateway is currently not active. Observe XDNS::gateway_activity_overview for more updates"
                );
                return false
            }
        }
        true
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
        xtx_id: T::Hash,
        sfx_id: T::Hash,
    ) -> Result<Option<SFXBid<T::AccountId, BalanceOf<T>, u32>>, Error<T>> {
        let local_ctx = Machine::<T>::load_xtx(xtx_id)?;
        let current_step_fsx = Machine::<T>::read_current_step_fsx(&local_ctx);
        let fsx = current_step_fsx
            .iter()
            .find(|fsx| fsx.calc_sfx_id::<SystemHashing<T>, T>(xtx_id) == sfx_id)
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

    pub fn process_xtx_tick_queue(
        n: T::BlockNumber,
        kill_interval: T::BlockNumber,
        max_allowed_weight: Weight,
    ) -> Weight {
        let mut current_weight: Weight = 0;
        if kill_interval == T::BlockNumber::zero() {
            return current_weight
        } else if n % kill_interval == T::BlockNumber::zero() {
            // Go over all unfinished Xtx to find those that should be killed
            let _processed_xtx_revert_count = <PendingXtxBidsTimeoutsMap<T>>::iter()
                .filter(|(_xtx_id, timeout_at)| timeout_at <= &n)
                .map(|(xtx_id, _timeout_at)| {
                    if current_weight <= max_allowed_weight {
                        current_weight =
                            current_weight.saturating_add(Self::process_tick_one(xtx_id));
                    }
                })
                .count();
        }
        current_weight
    }

    pub fn process_adaptive_xtx_timeout_queue(
        n: T::BlockNumber,
        _verifier: &GatewayVendor,
    ) -> Weight {
        let mut current_weight: Weight = 0;

        // Go over all unfinished Xtx to find those that timed out
        let _processed_xtx_revert_count = <PendingXtxTimeoutsMap<T>>::iter()
            .filter(|(_xtx_id, adaptive_timeout)| {
                // ToDo: consider filtering out by adaptive_timeout.verifier == verifier
                adaptive_timeout.estimated_height_here < n
            })
            .map(|(xtx_id, _timeout_at)| {
                // if current_weight <= max_allowed_weight {
                current_weight = current_weight.saturating_add(Self::process_revert_one(xtx_id).0);
                // }
            })
            .count();
        current_weight
    }

    pub fn process_emergency_revert_xtx_queue(
        n: T::BlockNumber,
        revert_interval: T::BlockNumber,
        max_allowed_weight: Weight,
    ) -> Weight {
        let mut current_weight: Weight = 0;
        // Scenario 1: all the timeout s can be handled in the block space
        // Scenario 2: all but 5 timeouts can be handled
        //     - add the 5 timeouts to an immediate queue for the next block
        if revert_interval == T::BlockNumber::zero() {
            return current_weight
        } else if n % revert_interval == T::BlockNumber::zero() {
            // Go over all unfinished Xtx to find those that timed out
            let _processed_xtx_revert_count = <PendingXtxTimeoutsMap<T>>::iter()
                .filter(|(_xtx_id, adaptive_timeout)| adaptive_timeout.emergency_timeout_here <= n)
                .map(|(xtx_id, _timeout_at)| {
                    if current_weight <= max_allowed_weight {
                        current_weight =
                            current_weight.saturating_add(Self::process_revert_one(xtx_id).0);
                    }
                })
                .count();
        }
        current_weight
    }

    pub fn get_adaptive_timeout(
        xtx_id: T::Hash,
        maybe_speed_mode: Option<SpeedMode>,
    ) -> AdaptiveTimeout<T::BlockNumber, TargetId> {
        let all_targets = Self::get_all_xtx_targets(xtx_id);
        T::Xdns::estimate_adaptive_timeout_on_slowest_target(
            all_targets,
            &maybe_speed_mode.unwrap_or(SpeedMode::Finalized),
            T::XtxTimeoutDefault::get(),
        )
    }

    /// Adds a cross-chain transaction (Xtx) to the Dead Letter Queue (DLQ).
    ///
    /// # Arguments
    ///
    /// * `xtx_id` - The ID of the Xtx to be added to the DLQ.
    /// * `targets` - The targets of the Xtx.
    /// * `speed_mode` - The speed mode of the Xtx.
    ///
    /// # Returns
    ///
    /// A tuple containing the weight of the operation and a boolean indicating whether the operation was successful.
    pub fn add_xtx_to_dlq(
        xtx_id: T::Hash,
        targets: Vec<TargetId>,
        speed_mode: SpeedMode,
    ) -> (Weight, bool) {
        if <DLQ<T>>::contains_key(xtx_id) {
            return (T::DbWeight::get().reads(1), false)
        }

        <DLQ<T>>::insert(
            xtx_id,
            (
                <frame_system::Module<T>>::block_number(),
                targets,
                speed_mode,
            ),
        );
        <XExecSignals<T>>::mutate(xtx_id, |xtx| {
            if let Some(xtx) = xtx {
                xtx.timeouts_at.dlq = Some(<frame_system::Module<T>>::block_number());
            } else {
                log::error!(
                    "Xtx not found in XExecSignals for xtx_id when add_xtx_to_dlq: {:?}",
                    xtx_id
                )
            }
        });

        // Remove the Xtx from the PendingXtxTimeoutsMap if exists
        if <PendingXtxTimeoutsMap<T>>::contains_key(xtx_id) {
            <PendingXtxTimeoutsMap<T>>::remove(xtx_id);
        }

        (
            T::DbWeight::get().reads_writes(2, 3), // 2 reads (DLQ, XExecSignals), 3 writes (DLQ, XExecSignals, PendingXtxTimeoutsMap)
            true,
        )
    }

    /// Removes a cross-chain transaction (Xtx) from the Dead Letter Queue (DLQ).
    ///
    /// # Arguments
    ///
    /// * `xtx_id` - The ID of the Xtx to be removed from the DLQ.
    ///
    /// # Returns
    ///
    /// A tuple containing the weight of the operation and a boolean indicating whether the operation was successful.
    pub fn remove_xtx_from_dlq(xtx_id: T::Hash) -> (Weight, bool) {
        let dlq_entry = match <DLQ<T>>::take(xtx_id) {
            Some(dlq_entry) => dlq_entry,
            None => return (T::DbWeight::get().reads(1), false),
        };

        let adaptive_timeout = Self::get_adaptive_timeout(xtx_id, Some(dlq_entry.2));
        <PendingXtxTimeoutsMap<T>>::insert(xtx_id, &adaptive_timeout);

        <XExecSignals<T>>::mutate(xtx_id, |xtx| {
            if let Some(xtx) = xtx {
                xtx.timeouts_at = adaptive_timeout;
            } else {
                log::error!(
                    "Xtx not found in XExecSignals for xtx_id when remove_xtx_from_dlq: {:?}",
                    xtx_id
                )
            }
        });

        (
            T::DbWeight::get().reads_writes(2, 3), // 2 reads (DLQ, XExecSignals), 3 writes (DLQ, XExecSignals, PendingXtxTimeoutsMap)
            true,
        )
    }

    /// Processes the Dead Letter Queue (DLQ).
    ///
    /// # Arguments
    ///
    /// * `_n` - The current block number.
    ///
    /// # Returns
    ///
    /// The total weight of the operation.
    pub fn process_dlq(_n: T::BlockNumber) -> Weight {
        <DLQ<T>>::iter()
            .map(|(xtx_id, (_block_number, targets, _speed_mode))| {
                if Self::ensure_all_gateways_are_active(targets) {
                    Self::remove_xtx_from_dlq(xtx_id).0
                } else {
                    T::DbWeight::get().reads(1)
                }
            })
            .sum()
    }

    /// Processes a single cross-chain transaction (Xtx) revert operation.
    ///
    /// # Arguments
    ///
    /// * `xtx_id` - The ID of the Xtx to be processed.
    ///
    /// # Returns
    ///
    /// A tuple containing the weight of the operation and a boolean indicating whether the operation was successful.
    pub fn process_revert_one(xtx_id: XExecSignalId<T>) -> (Weight, bool) {
        const REVERT_WRITES: Weight = 2;
        const REVERT_READS: Weight = 1;

        let all_targets = Self::get_all_xtx_targets(xtx_id);
        if !Self::ensure_all_gateways_are_active(all_targets.clone()) {
            return Self::add_xtx_to_dlq(xtx_id, all_targets, SpeedMode::Finalized)
        }

        let success: bool =
            Machine::<T>::revert(xtx_id, Cause::Timeout, |_status_change, local_ctx| {
                Self::request_sfx_attestation(local_ctx);
                Self::deposit_event(Event::XTransactionXtxRevertedAfterTimeOut(xtx_id));
            });

        (
            T::DbWeight::get().reads_writes(REVERT_READS, REVERT_WRITES),
            success,
        )
    }

    pub fn request_sfx_attestation(local_ctx: &LocalXtxCtx<T, BalanceOf<T>>) {
        Machine::<T>::read_current_step_fsx(local_ctx)
            .iter()
            .for_each(|fsx| {
                if fsx.security_lvl == SecurityLvl::Escrow {
                    let sfx_id: H256 = H256::from_slice(
                        fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id)
                            .as_ref(),
                    );
                    match local_ctx.xtx.status {
                        CircuitStatus::Reverted(_) =>
                            match T::Attesters::request_sfx_attestation_revert(
                                fsx.input.target,
                                sfx_id,
                            ) {
                                Ok(_) => {
                                    Self::deposit_event(
                                        Event::SuccessfulFSXRevertAttestationRequest(sfx_id),
                                    );
                                },
                                Err(_) => {
                                    Self::deposit_event(
                                        Event::UnsuccessfulFSXRevertAttestationRequest(sfx_id),
                                    );
                                },
                            },
                        CircuitStatus::FinishedAllSteps | CircuitStatus::Committed =>
                            match T::Attesters::request_sfx_attestation_commit(
                                fsx.input.target,
                                sfx_id,
                            ) {
                                Ok(_) => {
                                    Self::deposit_event(
                                        Event::SuccessfulFSXCommitAttestationRequest(sfx_id),
                                    );
                                },
                                Err(_) => {
                                    Self::deposit_event(
                                        Event::UnsuccessfulFSXCommitAttestationRequest(sfx_id),
                                    );
                                },
                            },
                        _ => {},
                    }
                }
            });
    }

    pub fn process_tick_one(xtx_id: XExecSignalId<T>) -> Weight {
        const KILL_WRITES: Weight = 4;
        const KILL_READS: Weight = 1;

        Machine::<T>::compile_infallible(
            &mut Machine::<T>::load_xtx(xtx_id).expect("xtx_id corresponds to a valid Xtx when reading from PendingXtxBidsTimeoutsMap storage"),
            |current_fsx, _local_state, _steps_cnt, status, _requester| {
                match status {
                    CircuitStatus::InBidding => match current_fsx.iter().all(|fsx| fsx.best_bid.is_some()) {
                        true => PrecompileResult::ForceUpdateStatus(CircuitStatus::Ready),
                        false => PrecompileResult::TryKill(Cause::Timeout)
                    },
                    _ => PrecompileResult::TryKill(Cause::Timeout)
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

        T::DbWeight::get().reads_writes(KILL_READS, KILL_WRITES)
    }

    // TODO: we also want to save some space for timeouts, split the weight distribution 50-50
    pub fn process_signal_queue(
        _n: T::BlockNumber,
        _interval: T::BlockNumber,
        max_allowed_weight: Weight,
    ) -> Weight {
        let queue_len = <SignalQueue<T>>::decode_len().unwrap_or(0);
        if queue_len == 0 {
            return 0
        }
        let db_weight = T::DbWeight::get();
        let mut queue = <SignalQueue<T>>::get();
        let mut processed_weight = 0 as Weight;

        while !queue.is_empty() && processed_weight < max_allowed_weight {
            // Cannot panic due to loop condition
            let (_requester, signal) = &mut queue.swap_remove(0);

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
                            processed_weight += db_weight.reads_writes(2 as Weight, 1 as Weight);
                        },
                    );
                },
                Err(err) => match err {
                    Error::XtxDoesNotExist => {
                        log::error!(
                                "Failed to process signal is for non-existent Xtx: {:?}. Removing from queue.",
                                signal.execution_id
                            );
                    },
                    _ => {
                        log::error!("Failed to process signal ID {:?} with Err: {:?}. Sliding back to queue.", signal.execution_id, err);
                        queue.slide(0, queue.len());
                    },
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
    ) -> Result<LocalXtxCtx<T, BalanceOf<T>>, Error<T>> {
        let xtx_id = <Self as Store>::SFX2XTXLinksMap::get(sfx_id)
            .ok_or(Error::<T>::LocalSideEffectExecutionNotApplicable)?;
        Machine::<T>::load_xtx(xtx_id)
    }
}
