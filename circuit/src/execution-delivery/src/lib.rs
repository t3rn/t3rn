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
use frame_support::{ensure, Blake2_128Concat};
use frame_system::offchain::{SignedPayload, SigningTypes};

use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    traits::{Convert, Hash, Zero},
    RuntimeDebug,
};

pub use crate::message_assembly::circuit_inbound::StepConfirmation;
pub use crate::message_assembly::circuit_outbound::CircuitOutboundMessage;
use hex_literal::hex;
use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::abi::{Type, GatewayABIConfig};
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::*;

use versatile_wasm::VersatileWasm;

#[cfg(test)]
mod tests;

pub mod exec_composer;
pub mod message_assembly;

use crate::exec_composer::ExecComposer;

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

pub fn select_validator_for_x_tx_dummy<T: Config>(
    _io_schedule: Vec<u8>,
) -> Result<T::AccountId, &'static str> {
    // This is the well-known Substrate account of Alice (5GrwvaEF...)
    let default_recepient =
        hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");

    let dummy_escrow_alice =
        T::AccountId::decode(&mut &default_recepient[..]).expect("should not fail for dummy data");

    Ok(dummy_escrow_alice)
}

pub type XtxId<T> = <T as frame_system::Config>::Hash;

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct Xtx<AccountId, BlockNumber, Hash> {
    /// The total estimated worth of tx (accumulated value being transferred and estimated fees)
    pub estimated_worth: u128,

    /// The total worth so far of tx (accumulated value being transferred and estimated fees)
    pub current_worth: u128,

    /// The owner of the bid
    pub requester: AccountId,

    /// Validator acting as an escrow
    pub escrow_account: AccountId,

    /// Encoded content of composable tx
    pub payload: Vec<u8>,

    /// Current step
    pub current_step: u32,

    /// Current step
    pub steps_no: u32,

    /// Current phase
    pub current_phase: u32,

    /// Current round
    pub current_round: u32,

    pub schedule: XtxSchedule<AccountId, BlockNumber, Hash>,
    // /// Current phase
    // pub phase_compilation_context: PhaseCompilationContext<BlockNumber>,
    /// Result
    pub result_status: Vec<u8>,

    /// Block numbers when each phase phase has started
    pub phases_blockstamps: (BlockNumber, BlockNumber),
}

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct StepEntry<AccountId, BlockNumber, Hash> {
    compose_id: Hash,
    cost: u128,
    result: Option<Vec<u8>>,
    input: Vec<u8>,
    dest: AccountId,
    value: u128,
    proof: Option<Hash>,
    updated_at: BlockNumber,
    relayer: Option<AccountId>,
    gateway_id: bp_runtime::ChainId,
}

/// Schedule consist of phases
/// The first phase, execution / computation phase may consist out of many rounds
/// Each round can consist out of many parallel steps
/// schedule:
///     vector of phases, where
///         phase: vector of rounds, where
///             round: vector of steps

pub type RoundEntry<AccountId, BlockNumber, Hash> = Vec<StepEntry<AccountId, BlockNumber, Hash>>;

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct XtxSchedule<AccountId, BlockNumber, Hash> {
    phases: Vec<RoundEntry<AccountId, BlockNumber, Hash>>,
}

// check frame/democracy/src/vote.rs
impl<AccountId: Encode, BlockNumber: Ord + Copy + Zero + Encode, Hash: Ord + Copy + Encode>
    Xtx<AccountId, BlockNumber, Hash>
{
    pub fn new(
        // Estimated worth (values transferred + aggregated fees)
        estimated_worth: u128,
        // Current, actual aggregated worth
        current_worth: u128,
        // Requester of xtx
        requester: AccountId,
        // Validator's account acting as an escrow for this xtx
        escrow_account: AccountId,
        // Encoded data
        payload: Vec<u8>,
        // Current step no
        current_step: u32,
        // Max no of steps
        steps_no: u32,
        // Current phase (exec, revert, commit)
        current_phase: u32,
        // Current round (consists of parallel steps)
        current_round: u32,
        // Results
        result_status: Vec<u8>,
        // Block numbers of two phases
        phases_blockstamps: (BlockNumber, BlockNumber),
        // Block numbers of two phases
        schedule: XtxSchedule<AccountId, BlockNumber, Hash>,
    ) -> Self {
        Xtx {
            estimated_worth,
            current_worth,
            requester,
            escrow_account,
            payload,
            steps_no,
            current_phase,
            current_round,
            current_step,
            result_status,
            phases_blockstamps,
            schedule,
        }
    }

    pub fn update_payload(&mut self, new_payload: Vec<u8>) {
        self.payload = new_payload;
    }

    pub fn generate_xtx_id<T: Config>(&self) -> XtxId<T> {
        T::Hashing::hash(Encode::encode(self).as_ref())
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Current Circuit's context of active transactions
    ///
    /// The currently active composable transactions, indexed according to the order of creation.
    #[pallet::storage]
    pub type ActiveXtxMap<T> = StorageMap<
        _,
        Blake2_128Concat,
        XtxId<T>,
        Xtx<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            <T as frame_system::Config>::Hash,
        >,
        OptionQuery,
    >;

    /// This pallet's configuration trait
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_bridge_messages::Config
        + pallet_balances::Config
        + VersatileWasm
        + pallet_contracts_registry::Config
        + pallet_im_online::Config
        + pallet_xdns::Config
        + pallet_contracts::Config
        + pallet_evm::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The overarching dispatch call type.
        type Call: From<Call<Self>>;

        type AccountId32Converter: Convert<Self::AccountId, [u8; 32]>;

        type ToStandardizedGatewayBalance: Convert<BalanceOf<Self>, u128>;
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
            let requester = ensure_signed(origin)?;
            ensure!(
                !(components.len() == 0 || io_schedule.len() == 0),
                "empty parameters submitted for execution order",
            );

            let inter_schedule: InterExecSchedule<T::AccountId, u64> =
                Self::decompose_io_schedule(components.clone(), io_schedule.clone())
                    .expect("Wrong io schedule");

            let escrow_account = select_validator_for_x_tx_dummy::<T>(io_schedule.clone())?;

            let new_xtx = Self::dry_run_whole_xtx(
                inter_schedule.clone(),
                requester.clone(),
                escrow_account.clone(),
            )?;
            let x_tx_id: XtxId<T> = new_xtx.generate_xtx_id::<T>();

            ActiveXtxMap::<T>::insert(x_tx_id.clone(), new_xtx);

            let circuit_outbound_messages = Self::process_phase(
                x_tx_id.clone(),
                components,
                escrow_account.clone(),
                inter_schedule.clone(),
            )?;

            Self::deposit_event(Event::StoredNewStep(
                requester.clone(),
                x_tx_id,
                circuit_outbound_messages,
            ));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn register_gateway(
            origin: OriginFor<T>,
            url: Vec<u8>,
            gateway_id: bp_runtime::ChainId,
            gateway_abi: GatewayABIConfig,
            gateway_vendor: t3rn_primitives::GatewayVendor,
            gateway_type: t3rn_primitives::GatewayType,
            gateway_genesis: GatewayGenesisConfig,
            _first_header: GenericFirstHeader,
            _authorities: Option<Vec<T::AccountId>>,
        ) -> DispatchResultWithPostInfo {

            // Retrieve sender of the transaction.
            pallet_xdns::Pallet::<T>::add_new_xdns_record(
                origin,
                url,
                gateway_id,
                gateway_abi,
                gateway_vendor,
                gateway_type,
                gateway_genesis,
            )?;

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn submit_step_confirmation(
            origin: OriginFor<T>,
            step_confirmation: StepConfirmation,
            xtx_id: XtxId<T>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let _relayer_id = ensure_signed(origin)?;

            let xtx: Xtx<T::AccountId, T::BlockNumber, T::Hash> =
                ActiveXtxMap::<T>::get(xtx_id.clone())
                    .expect("submitted to confirm step id does not match with any Xtx");

            let _current_step = xtx.schedule.phases[xtx.current_round as usize].clone()
                [step_confirmation.step_index as usize]
                .clone();

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
        /// News steps that were just added for relayers to deliver.
        /// \[who, id, steps\]
        StoredNewStep(T::AccountId, XtxId<T>, Vec<CircuitOutboundMessage>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Non existent public key.
        InvalidKey,
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
        // ToDo: Rewrite in no-std compatible way without external Regex lib
        // use regex::bytes::Regex;
        // for caps in Regex::new(
        //     r"(?P<compose_name>[\w]+)|(?P<next_phase>[>]+)|(?P<parallel_step>[|]+)|(?P<end>[;]+)",
        // )
        //     .unwrap()
        //     .captures_iter(&io_schedule[..])
        // {
        //     if let Some(name) = caps.name("compose_name") {
        //         if let Some(selected_compose) = components.clone().into_iter().find(|comp| {
        //             comp.name.encode() == name.as_bytes().encode()
        //         }) {
        //             let new_step = ExecStep {
        //                 compose: selected_compose.clone(),
        //             };
        //             if let Some(last_phase) = inter_schedule.phases.last_mut() {
        //                 last_phase.steps.push(new_step);
        //             } else {
        //                 inter_schedule.phases.push(ExecPhase {
        //                     steps: vec![new_step]
        //                 });
        //             }
        //         } else {
        //             return Err("Error::<T>::UnknownIOScheduleCompose");
        //         }
        //     }
        //     if let Some(name) = caps.name("next_phase") {
        //         inter_schedule.phases.push(ExecPhase::default());
        //     }
        //     if let Some(name) = caps.name("parallel_step") {
        //     }
        //     if let Some(name) = caps.name("end") {
        //         return Ok(inter_schedule)
        //     }
        // };
        // Err("Error::<T>::IOScheduleNoEndingSemicolon")
    }


    /// Dry run submitted cross-chain transaction
    /// User can additionally submit the IO schedule which comes on top as an additional order maker.
    /// inter_schedule was analysed already and we at this point we can be sure within
    ///    the inter_schedule components are in the correct order. At least an order that requester expects.
    /// Task of the dry_run here is the decompose the phases into additional rounds that can be submitted in parallel.
    /// The output is cross-chain transaction with a fixed schedule that covers all future steps of the incoming rounds and phases.
    pub fn dry_run_whole_xtx(
        _inter_schedule: InterExecSchedule<T::AccountId, u64>,
        escrow_account: T::AccountId,
        requester: T::AccountId,
    ) -> Result<Xtx<T::AccountId, T::BlockNumber, <T as frame_system::Config>::Hash>, &'static str>
    {
        let _current_round: RoundEntry<T::AccountId, T::BlockNumber, T::Hash> = vec![];

        let (current_block_no, block_zero) = (
            <frame_system::Pallet<T>>::block_number(),
            T::BlockNumber::zero(),
        );
        let max_steps = 3;

        let new_xtx = Xtx::<T::AccountId, T::BlockNumber, <T as frame_system::Config>::Hash>::new(
            0,
            0,
            requester.clone(),
            escrow_account.clone(),
            vec![],
            0,
            max_steps,
            0,
            0,
            vec![],
            (current_block_no, block_zero),
            Default::default(),
        );

        // ExecComposer::dry_run_single_contract::<T>(contract, escrow_account, requester, step.dest, value, step.input, step.gateway_id);

        Ok(new_xtx)
    }

    pub fn process_phase(
        x_tx_id: XtxId<T>,
        _components: Vec<Compose<T::AccountId, u64>>,
        escrow_account: T::AccountId,
        _schedule: InterExecSchedule<T::AccountId, u64>,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        let current_xtx =
            ActiveXtxMap::<T>::get(x_tx_id).ok_or("Cross-chain tx not found while process_step")?;

        if current_xtx.current_step > current_xtx.steps_no {
            Self::complete_xtx(current_xtx.clone())
        } else {
            let steps_in_current_round = current_xtx
                .schedule
                .phases
                .get(current_xtx.current_round as usize)
                .expect("Each round in schedule should be aligned with current_round in storage");

            Self::process_round(
                steps_in_current_round.to_vec(),
                escrow_account,
                current_xtx.requester,
            )
        }
    }

    pub fn process_round(
        round_steps: RoundEntry<T::AccountId, T::BlockNumber, T::Hash>,
        escrow_account: T::AccountId,
        requester: T::AccountId,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        let mut current_round_messages: Vec<CircuitOutboundMessage> = vec![];

        for step in round_steps {
            let single_step_outbound_messages =
                Self::process_step(step, escrow_account.clone(), requester.clone())?;
            current_round_messages.extend(single_step_outbound_messages);
        }

        Ok(current_round_messages)
    }

    pub fn process_step(
        step: StepEntry<T::AccountId, T::BlockNumber, T::Hash>,
        escrow_account: T::AccountId,
        requester: T::AccountId,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        use sp_runtime::RuntimeAppPublic;
        let contract =
            pallet_contracts_registry::Pallet::<T>::contracts_registry(step.compose_id.clone())
                .expect(
                    // let contract = ContractsRegistry::<T>::get(step.compose_id.clone()).expect(
                    "contract id in steps should be matching contracts registry",
                );

        let value = BalanceOf::<T>::from(
            sp_std::convert::TryInto::<u32>::try_into(step.value)
                .map_err(|_e| "Can't cast value in dry_run_single_contract")?,
        );

        let local_keys = <T as pallet_im_online::Config>::AuthorityId::all();

        // ToDo: Select validators to submit by his public key, like:
        // let submitter = local_keys.binary_search(&escrow_account.into()).ok().map(|location| local_keys[location].clone()).ok_or("Can't match")?;
        let submitter = local_keys[0].clone();

        ExecComposer::pre_run_single_contract::<T>(
            contract,
            escrow_account,
            submitter,
            requester,
            step.dest,
            value,
            step.input,
            step.gateway_id,
        )
        .into()
    }

    /// Submit round (parallel steps) for execution.
    /// Boils down to emitting steps entries as an event watched by relayers
    pub fn submit_round(_round_messages: Vec<CircuitOutboundMessage>) -> Result<(), &'static str> {
        // Decide on the next execution phase and enact on it
        Ok(())
    }

    fn complete_xtx(
        _xtx: Xtx<T::AccountId, T::BlockNumber, <T as frame_system::Config>::Hash>,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        // Decide on the next execution phase and enact on it
        Ok(vec![])
    }

    /// Proof each single step of execution.
    /// After all steps in the round has been confirmed, this would also call for process_phase.
    pub fn confirm_step(
        _x_tx_id: [u8; 32],
        _step_no: u32,
        _io_schedule: Vec<u8>,
        _state_proof: Vec<u8>,
        _extrinsics_proof: Vec<u8>,
        _block_hash: Vec<u8>,
        _finality_proof: Vec<u8>,
    ) -> Result<(), &'static str> {
        // Validate step

        // Process next step for execution

        Ok(())
    }
}
