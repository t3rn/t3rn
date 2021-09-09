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
use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_support::ensure;
use frame_system::offchain::{SignedPayload, SigningTypes};
use hex_literal::hex;
use sp_application_crypto::Public;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    traits::{Convert, Hash, Zero},
    RuntimeAppPublic, RuntimeDebug,
};

pub use crate::exec_composer::ExecComposer;
pub use crate::message_assembly::circuit_inbound::StepConfirmation;
use crate::message_assembly::merklize::*;
use pallet_contracts_registry::{RegistryContract, RegistryContractId};

use bp_runtime::ChainId;
pub use pallet::*;
use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::abi::{ContractActionDesc, GatewayABIConfig, HasherAlgo as HA};
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::*;
use volatile_vm::VolatileVM;

#[cfg(test)]
pub mod tests;

#[cfg(test)]
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;

pub mod exec_composer;
pub mod message_assembly;

pub use crate::message_assembly::test_utils as message_test_utils;

pub type CurrentHash<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Hash;
pub type CurrentHasher<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Hasher;
pub type CurrentHeader<T, I> =
    <<T as pallet_multi_finality_verifier::Config<I>>::BridgedChain as bp_runtime::Chain>::Header;

type DefaultPolkadotLikeGateway = ();
type PolkadotLikeValU64Gateway = pallet_multi_finality_verifier::Instance1;
type EthLikeKeccak256ValU64Gateway = pallet_multi_finality_verifier::Instance2;
type EthLikeKeccak256ValU32Gateway = pallet_multi_finality_verifier::Instance3;

pub fn init_bridge_instance<T: pallet_multi_finality_verifier::Config<I>, I: 'static>(
    origin: T::Origin,
    first_header: Vec<u8>,
    authorities: Option<Vec<T::AccountId>>,
    gateway_id: bp_runtime::ChainId,
) -> DispatchResultWithPostInfo {
    let header: CurrentHeader<T, I> = Decode::decode(&mut &first_header[..])
        .map_err(|_| "Decoding error: received GenericPrimitivesHeader -> CurrentHeader<T>")?;

    let init_data = bp_header_chain::InitializationData {
        header,
        authority_list: authorities
            .unwrap_or(vec![])
            .iter()
            .map(|id| {
                (
                    sp_finality_grandpa::AuthorityId::from_slice(&id.encode()),
                    1,
                )
            })
            .collect::<Vec<_>>(),
        set_id: 1,
        is_halted: false,
    };

    pallet_multi_finality_verifier::Pallet::<T, I>::initialize_single(origin, init_data, gateway_id)
}

pub fn get_roots_from_bridge<T: pallet_multi_finality_verifier::Config<I>, I: 'static>(
    block_hash: Bytes,
    gateway_id: bp_runtime::ChainId,
) -> Result<(sp_core::H256, sp_core::H256), Error<T>> {
    let gateway_block_hash: CurrentHash<T, I> = Decode::decode(&mut &block_hash[..])
        .map_err(|_| Error::<T>::StepConfirmationDecodingError)?;

    let (extrinsics_root, storage_root): (CurrentHash<T, I>, CurrentHash<T, I>) =
        pallet_multi_finality_verifier::Pallet::<T, I>::get_imported_roots(
            gateway_id,
            gateway_block_hash,
        )
        .ok_or(Error::<T>::StepConfirmationBlockUnrecognised)?;

    let extrinsics_root_h256: sp_core::H256 = Decode::decode(&mut &extrinsics_root.encode()[..])
        .map_err(|_| Error::<T>::StepConfirmationDecodingError)?;

    let storage_root_h256: sp_core::H256 = Decode::decode(&mut &storage_root.encode()[..])
        .map_err(|_| Error::<T>::StepConfirmationDecodingError)?;

    Ok((extrinsics_root_h256, storage_root_h256))
}

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

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

pub type AuthorityId = crate::message_assembly::signer::app::Public;

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct Xtx<AccountId, BlockNumber, Hash, BalanceOf> {
    /// The total estimated worth of tx (accumulated value being transferred and estimated fees)
    pub estimated_worth: BalanceOf,

    /// The total worth so far of tx (accumulated value being transferred and estimated fees)
    pub current_worth: BalanceOf,

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

    pub schedule: XtxSchedule<AccountId, BlockNumber, Hash, BalanceOf>,
    // /// Current phase
    // pub phase_compilation_context: PhaseCompilationContext<BlockNumber>,
    /// Result
    pub result_status: Vec<u8>,

    /// Block numbers when each phase phase has started
    pub phases_blockstamps: (BlockNumber, BlockNumber),
}

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct StepEntry<AccountId, BlockNumber, Hash, BalanceOf> {
    contract_id: Hash,
    cost: BalanceOf,
    result: Option<Vec<u8>>,
    input: Vec<u8>,
    dest: Option<AccountId>,
    value: BalanceOf,
    proof: Option<Hash>,
    relayer: Option<AccountId>,
    gateway_id: Option<bp_runtime::ChainId>,
    updated_at: BlockNumber,
}

/// Schedule consist of phases
/// The first phase, execution / computation phase may consist out of many rounds
/// Each round can consist out of many parallel steps
/// schedule:
///     vector of phases, where
///         phase: vector of rounds, where
///             round: vector of steps
pub type RoundEntry<AccountId, BlockNumber, Hash, BalanceOf> =
    Vec<StepEntry<AccountId, BlockNumber, Hash, BalanceOf>>;

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct XtxSchedule<AccountId, BlockNumber, Hash, BalanceOf> {
    phases: Vec<RoundEntry<AccountId, BlockNumber, Hash, BalanceOf>>,
}

impl<
        AccountId: Encode + Clone,
        BlockNumber: Ord + Copy + Zero + Encode,
        Hash: Ord + Copy + Encode,
        BalanceOf: Copy + Zero + Encode + Decode + Default,
    > XtxSchedule<AccountId, BlockNumber, Hash, BalanceOf>
{
    /// Simplify scheduled exec to a sequence of rounds with a single steps
    pub fn new_sequential_from_contracts(
        contracts: Vec<RegistryContract<Hash, AccountId, BalanceOf, BlockNumber>>,
        contract_ids: Vec<Hash>,
        compose_steps: Vec<Compose<AccountId, BalanceOf>>,
        _contract_action_descriptions: Vec<ContractActionDesc<Hash, ChainId, AccountId>>,
        gateway_id: Option<ChainId>,
        updated_at: BlockNumber,
    ) -> Result<XtxSchedule<AccountId, BlockNumber, Hash, BalanceOf>, &'static str> {
        ensure!(
            contracts.len() == contract_ids.len() && contract_ids.len() == compose_steps.len(),
            "XtxSchedule::new_sequential_from_contracts - contracts length must be equal to contracts ids and number of steps",
        );

        let mut phases: Vec<RoundEntry<AccountId, BlockNumber, Hash, BalanceOf>> = vec![];

        for (i, _contract) in contracts.iter().enumerate() {
            let current_step = compose_steps
                .get(i as usize)
                .expect("steps and contracts length ensured to be equal above");
            let curr_step_entry = StepEntry::<AccountId, BlockNumber, Hash, BalanceOf> {
                contract_id: *contract_ids
                    .get(i as usize)
                    .expect("contract_ids and contracts length ensured to be equal above"),
                cost: Default::default(), // ToDo: Generating cost estimate is part of Milestone 2.
                result: None, // ToDo: Read expected output from contract_action_descriptions
                input: current_step.input_data.clone(), // ToDo: Prefer to read input from contract_action_descriptions
                dest: Some(current_step.dest.clone()),  // Some if a smart contract
                value: Decode::decode(&mut &current_step.value.encode()[..]).map_err(|_| {
                    "Can't decode value from step in XtxSchedule::new_sequential_from_contracts"
                })?,
                proof: None,
                updated_at: updated_at.clone(),
                relayer: None,
                gateway_id,
            };
            phases.push(vec![curr_step_entry]);
        }

        let xtx_schedule = XtxSchedule::<AccountId, BlockNumber, Hash, BalanceOf> { phases };

        Ok(xtx_schedule)
    }

    pub fn new_from_inter_exec_schedule(
        _inter_schedule: InterExecSchedule<AccountId, BalanceOf>,
        _rounds: Vec<RoundEntry<AccountId, BlockNumber, Hash, BalanceOf>>,
    ) {
        // ToDo: Create new one based on inter_schedule provided by requester and discovered rounds order.
        unimplemented!();
    }
}

// check frame/democracy/src/vote.rs
impl<
        AccountId: Encode,
        BlockNumber: Ord + Copy + Zero + Encode,
        Hash: Ord + Copy + Encode,
        BalanceOf: Copy + Zero + Encode + Decode,
    > Xtx<AccountId, BlockNumber, Hash, BalanceOf>
{
    pub fn new(
        // Estimated worth (values transferred + aggregated fees)
        estimated_worth: BalanceOf,
        // Current, actual aggregated worth
        current_worth: BalanceOf,
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
        schedule: XtxSchedule<AccountId, BlockNumber, Hash, BalanceOf>,
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
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

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
            BalanceOf<T>,
        >,
        OptionQuery,
    >;

    /// This pallet's configuration trait
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_bridge_messages::Config
        + pallet_balances::Config
        + VolatileVM
        + pallet_contracts_registry::Config
        + pallet_xdns::Config
        + pallet_contracts::Config
        + pallet_evm::Config
        + pallet_multi_finality_verifier::Config<DefaultPolkadotLikeGateway>
        + pallet_multi_finality_verifier::Config<PolkadotLikeValU64Gateway>
        + pallet_multi_finality_verifier::Config<EthLikeKeccak256ValU64Gateway>
        + pallet_multi_finality_verifier::Config<EthLikeKeccak256ValU32Gateway>
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
            components: Vec<Compose<T::AccountId, BalanceOf<T>>>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let requester = ensure_signed(origin)?;
            ensure!(
                !(components.len() == 0 || io_schedule.len() == 0),
                "empty parameters submitted for execution order",
            );

            let inter_schedule: InterExecSchedule<T::AccountId, BalanceOf<T>> =
                Self::decompose_io_schedule(components.clone(), io_schedule.clone())
                    .expect("Wrong io schedule");

            let escrow_account = select_validator_for_x_tx_dummy::<T>(io_schedule.clone())?;

            // In dry run we would like to:
            // 1. Parse and validate the syntax of unseen in the on-chain registry contracts
            //     1.2. Add them to the on-chain registry
            // 2. Fetch all of the contracts from on-chain registry involved in that execution and dry run as one xtx.
            let (new_xtx, contracts, _contract_ids, _contract_descriptions) =
                Self::dry_run_whole_xtx(
                    inter_schedule.clone(),
                    requester.clone(),
                    escrow_account.clone(),
                )?;

            let x_tx_id: XtxId<T> = new_xtx.generate_xtx_id::<T>();

            ActiveXtxMap::<T>::insert(x_tx_id, &new_xtx);

            // Every time before the execution - preload the all of the involved contracts to the VM
            ExecComposer::preload_bunch_of_contracts::<T>(contracts.clone(), requester.clone())?;

            let submitter = Self::select_authority(escrow_account.clone())?;

            let first_step = Self::first_unprocessed_step(new_xtx.clone())?;

            let (value, input_data, gateway_id) = (
                first_step.value,
                first_step.input,
                None, // Assign None for on-chain targets
            );

            // ToDo: Work out max gas limit acceptable by each escrow
            let gas_limit = u64::max_value();

            // ToDo: Pick up execution for the last unconfirmed step
            let (circuit_outbound_messages, _last_executed_contract_no) =
                ExecComposer::pre_run_bunch_until_break::<T>(
                    contracts,
                    escrow_account.clone(),
                    submitter,
                    requester.clone(),
                    value,
                    input_data,
                    gas_limit,
                    gateway_id,
                    // ToDo: Generate Circuit's params as default ABI
                    Default::default(),
                )?;

            // ToDo: Enact on the info about round finished.

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
            first_header: Vec<u8>,
            authorities: Option<Vec<T::AccountId>>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            pallet_xdns::Pallet::<T>::add_new_xdns_record(
                origin.clone(),
                url,
                gateway_id,
                gateway_abi.clone(),
                gateway_vendor,
                gateway_type,
                gateway_genesis,
            )?;

            let res = match (gateway_abi.hasher, gateway_abi.block_number_type_size) {
                (HA::Blake2, 32) => init_bridge_instance::<T, DefaultPolkadotLikeGateway>(
                    origin,
                    first_header,
                    authorities,
                    gateway_id,
                )?,
                (HA::Blake2, 64) => init_bridge_instance::<T, PolkadotLikeValU64Gateway>(
                    origin,
                    first_header,
                    authorities,
                    gateway_id,
                )?,
                (HA::Keccak256, 32) => init_bridge_instance::<T, EthLikeKeccak256ValU32Gateway>(
                    origin,
                    first_header,
                    authorities,
                    gateway_id,
                )?,
                (HA::Keccak256, 64) => init_bridge_instance::<T, EthLikeKeccak256ValU64Gateway>(
                    origin,
                    first_header,
                    authorities,
                    gateway_id,
                )?,
                (_, _) => init_bridge_instance::<T, DefaultPolkadotLikeGateway>(
                    origin,
                    first_header,
                    authorities,
                    gateway_id,
                )?,
            };

            Ok(res.into())
        }

        #[pallet::weight(0)]
        pub fn submit_step_confirmation(
            origin: OriginFor<T>,
            step_confirmation: StepConfirmation,
            xtx_id: XtxId<T>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let _relayer_id = ensure_signed(origin)?;

            let xtx: Xtx<T::AccountId, T::BlockNumber, T::Hash, BalanceOf<T>> =
                ActiveXtxMap::<T>::get(xtx_id.clone())
                    .expect("submitted to confirm step id does not match with any Xtx");

            let current_step = xtx.schedule.phases[xtx.current_round as usize].clone()
                [step_confirmation.clone().step_index as usize]
                .clone();

            // ToDo: parse events to discover their content and verify execution

            // Check inclusion relying on data in palet-multi-verifier
            let gateway_id = current_step
                .gateway_id
                .expect("Confirmation step for remote (Some) gateways only");

            let gateway_xdns_record = pallet_xdns::Pallet::<T>::best_available(
                current_step
                    .gateway_id
                    .expect("Foreign gateway id expected at receiving steps confirmation"),
            )?;

            let declared_block_hash = step_confirmation.proof.block_hash;

            let (extrinsics_root_h256, storage_root_h256) = match (
                gateway_xdns_record.gateway_abi.hasher.clone(),
                gateway_xdns_record.gateway_abi.block_number_type_size,
            ) {
                (HA::Blake2, 32) => get_roots_from_bridge::<T, DefaultPolkadotLikeGateway>(
                    declared_block_hash,
                    gateway_id,
                )?,
                (HA::Blake2, 64) => get_roots_from_bridge::<T, PolkadotLikeValU64Gateway>(
                    declared_block_hash,
                    gateway_id,
                )?,
                (HA::Keccak256, 32) => get_roots_from_bridge::<T, EthLikeKeccak256ValU32Gateway>(
                    declared_block_hash,
                    gateway_id,
                )?,
                (HA::Keccak256, 64) => get_roots_from_bridge::<T, EthLikeKeccak256ValU64Gateway>(
                    declared_block_hash,
                    gateway_id,
                )?,
                (_, _) => get_roots_from_bridge::<T, DefaultPolkadotLikeGateway>(
                    declared_block_hash,
                    gateway_id,
                )?,
            };

            let expected_root = match step_confirmation.proof.proof_trie_pointer {
                ProofTriePointer::State => storage_root_h256,
                ProofTriePointer::Transaction => extrinsics_root_h256,
                ProofTriePointer::Receipts => storage_root_h256,
            };

            if let Err(computed_root) = check_merkle_proof(
                expected_root,
                step_confirmation.proof.proof_data.into_iter(),
                gateway_xdns_record.gateway_abi.hasher,
            ) {
                log::trace!(
                    target: "circuit-runtime",
                    "Step confirmation check failed: inclusion root mismatch. Expected: {}, computed: {}",
                    expected_root,
                    computed_root,
                );

                Err(Error::<T>::StepConfirmationInvalidInclusionProof.into())
            } else {
                // ToDo: Enact on the confirmation step and save the update
                // Self::update_xtx(&xtx, xtx_id, step_confirmation);
                // Self::maybe_resume_xtx(&xtx);
                Ok(().into())
            }
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
        IOScheduleNoEndingSemicolon,
        IOScheduleEmpty,
        IOScheduleUnknownCompose,
        ProcessStepGatewayNotRecognised,
        StepConfirmationBlockUnrecognised,
        StepConfirmationGatewayNotRecognised,
        StepConfirmationInvalidInclusionProof,
        StepConfirmationDecodingError,
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
    /// Receives a list of available components and an io schedule in text format
    /// and parses it to create an execution schedule
    pub fn decompose_io_schedule(
        _components: Vec<Compose<T::AccountId, BalanceOf<T>>>,
        _io_schedule: Vec<u8>,
    ) -> Result<InterExecSchedule<T::AccountId, BalanceOf<T>>, &'static str> {
        // set constants
        const WHITESPACE_MATRIX: [u8; 4] = [b' ', b'\t', b'\r', b'\n'];
        const PHASE_SEPARATOR: u8 = b'|';
        const STEP_SEPARATOR: u8 = b',';
        const SCHEDULE_END: u8 = b';';

        // trims all whitespace chars from io_schedule vector
        fn trim_whitespace(input_string: Vec<u8>) -> Vec<u8> {
            let mut result = input_string.clone();

            // checks if character is whitespace
            let is_whitespace = |x: &u8| WHITESPACE_MATRIX.contains(x);

            let mut i = 0;
            while i < result.len() {
                if is_whitespace(&result[i]) {
                    result.remove(i);
                } else {
                    i += 1;
                }
            }
            result
        }

        // converts an exec_step vector string to an ExecStep
        // throws error if a component is not found
        let to_exec_step = |name: Vec<u8>| {
            let compose = _components
                .clone()
                .into_iter()
                .find(|comp| comp.name.encode() == name.encode());
            match compose {
                Some(value) => Ok(ExecStep { compose: value }),
                None => Err(Error::<T>::IOScheduleUnknownCompose),
            }
        };

        // splits a phase vector into ExecSteps
        let split_into_steps = |phase: Vec<u8>| {
            phase
                .split(|char| char.eq(&STEP_SEPARATOR))
                .filter(|step| !step.is_empty())
                .map(|step| to_exec_step(step.to_vec()))
                .collect()
        };

        // splits an io_schedule into phases and then into steps
        let split_into_phases = |io_schedule: Vec<u8>| {
            io_schedule
                .split(|character| character.eq(&PHASE_SEPARATOR))
                .filter(|phase| !phase.is_empty())
                .map(|phase| {
                    let steps: Result<Vec<ExecStep<T::AccountId, BalanceOf<T>>>, crate::Error<T>> =
                        split_into_steps(phase.to_vec());
                    ensure!(steps.is_ok(), Error::<T>::IOScheduleUnknownCompose);
                    Ok(ExecPhase {
                        steps: steps.unwrap(),
                    })
                })
                .collect()
        };

        let mut cloned = trim_whitespace(_io_schedule);

        // make sure schedule is not empty
        // probably irrelevant since there is already a check for that
        let last_char = cloned.last();
        ensure!(last_char.is_some(), Error::<T>::IOScheduleEmpty);
        // make sure the schedule ends correctly and remove ending character or panic
        let ends_correctly = last_char.eq(&Some(&SCHEDULE_END));
        ensure!(ends_correctly, Error::<T>::IOScheduleNoEndingSemicolon);
        cloned.remove(cloned.len() - 1);

        // make sure schedule can be split into phases
        let phases: Result<Vec<ExecPhase<T::AccountId, BalanceOf<T>>>, crate::Error<T>> =
            split_into_phases(cloned);
        ensure!(phases.is_ok(), Error::<T>::IOScheduleUnknownCompose);

        Ok(InterExecSchedule {
            phases: phases.unwrap(),
        })
    }

    /// Dry run submitted cross-chain transaction
    /// User can additionally submit the IO schedule which comes on top as an additional order maker.
    /// inter_schedule was analysed already and we at this point we can be sure within
    ///    the inter_schedule components are in the correct order. At least an order that requester expects.
    /// Task of the dry_run here is the decompose the phases into additional rounds that can be submitted in parallel.
    /// The output is cross-chain transaction with a fixed schedule that covers all future steps of the incoming rounds and phases.
    pub fn dry_run_whole_xtx(
        inter_schedule: InterExecSchedule<T::AccountId, BalanceOf<T>>,
        escrow_account: T::AccountId,
        requester: T::AccountId,
    ) -> Result<
        (
            Xtx<T::AccountId, T::BlockNumber, T::Hash, BalanceOf<T>>,
            Vec<RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber>>,
            Vec<RegistryContractId<T>>,
            Vec<ContractActionDesc<T::Hash, ChainId, T::AccountId>>,
        ),
        &'static str,
    > {
        let mut contracts = vec![];
        let mut unseen_contracts = vec![];
        let mut seen_contracts = vec![];
        let mut contract_ids = vec![];
        let mut action_descriptions = vec![];
        let mut composes = vec![];

        // ToDo: Better phases getter
        let first_phase = inter_schedule
            .phases
            .get(0)
            .expect("At least one phase should always be there in inter_schedule");

        // Check if there are some unseen contracts - if yes dry_run them in a single context. If fine - add to the contracts-repo.
        for step in &first_phase.steps {
            let mut protocol_part_of_contract = step.compose.code_txt.clone();
            protocol_part_of_contract.extend(step.compose.bytes.clone());
            let key = T::Hashing::hash(Encode::encode(&mut protocol_part_of_contract).as_ref());

            // If invalid new contract was submitted for execution - break. Otherwise, add the new contract to on-chain registry.
            if !pallet_contracts_registry::ContractsRegistry::<T>::contains_key(key) {
                let unseen_contract =
                    ExecComposer::dry_run_single_contract::<T>(step.compose.clone())?;
                // Assuming dry run step went well, add the contract now
                pallet_contracts_registry::ContractsRegistry::<T>::insert(key, &unseen_contract);
                unseen_contracts.push(unseen_contract.clone());
                action_descriptions.extend(unseen_contract.action_descriptions);
            } else {
                // Query for the existent contract and push to queue.
                let seen_contract = pallet_contracts_registry::ContractsRegistry::<T>::get(key)
                    .expect("contains_key called above before accessing the contract");
                action_descriptions.extend(seen_contract.action_descriptions.clone());
                seen_contracts.push(seen_contract);
            }
            contract_ids.push(key);
            composes.push(step.compose.clone());
        }

        contracts.extend(seen_contracts);
        contracts.extend(unseen_contracts);

        let (current_block_no, block_zero) = (
            <frame_system::Pallet<T>>::block_number(),
            T::BlockNumber::zero(),
        );

        let xtx_schedule = XtxSchedule::new_sequential_from_contracts(
            contracts.clone(),
            contract_ids.clone(),
            composes.clone(),
            action_descriptions.clone(),
            None,
            current_block_no.clone(),
        )?;

        let max_steps = contracts.len() as u32;

        let new_xtx = Xtx::<
            T::AccountId,
            T::BlockNumber,
            <T as frame_system::Config>::Hash,
            BalanceOf<T>,
        >::new(
            Default::default(),
            Default::default(),
            requester.clone(),
            escrow_account.clone(),
            vec![],
            0,
            max_steps,
            0,
            0,
            vec![],
            (current_block_no, block_zero),
            xtx_schedule,
        );

        Ok((new_xtx, contracts, contract_ids, action_descriptions))
    }

    pub fn process_phase(
        x_tx_id: XtxId<T>,
        _components: Vec<Compose<T::AccountId, BalanceOf<T>>>,
        escrow_account: T::AccountId,
        _schedule: InterExecSchedule<T::AccountId, BalanceOf<T>>,
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
        _round_steps: RoundEntry<T::AccountId, T::BlockNumber, T::Hash, BalanceOf<T>>,
        _escrow_account: T::AccountId,
        _requester: T::AccountId,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        let current_round_messages: Vec<CircuitOutboundMessage> = vec![];

        let _constructed_outbound_messages = &mut Vec::<CircuitOutboundMessage>::new();

        Ok(current_round_messages)
    }

    pub fn first_unprocessed_step(
        xtx: Xtx<T::AccountId, T::BlockNumber, <T as frame_system::Config>::Hash, BalanceOf<T>>,
    ) -> Result<StepEntry<T::AccountId, T::BlockNumber, T::Hash, BalanceOf<T>>, &'static str> {
        let current_step = xtx.schedule.phases[xtx.current_round as usize].clone()
            [xtx.current_step as usize]
            .clone();

        Ok(current_step)
    }

    pub fn select_authority(escrow_account: T::AccountId) -> Result<AuthorityId, &'static str> {
        let mut local_keys = AuthorityId::all();

        local_keys.sort();

        let auth = AuthorityId::from_slice(escrow_account.encode().as_slice());

        let submitter = local_keys
            .binary_search(&auth)
            .ok()
            .map(|location| local_keys[location].clone())
            .ok_or("Can't match authority for given account")?;

        Ok(submitter)
    }

    // ToDo: complete_xtx
    fn complete_xtx(
        _xtx: Xtx<T::AccountId, T::BlockNumber, <T as frame_system::Config>::Hash, BalanceOf<T>>,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        // Decide on the next execution phase and enact on it
        Ok(vec![])
    }
}
