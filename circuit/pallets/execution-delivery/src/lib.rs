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

pub use crate::exec_composer::ExecComposer;
pub use crate::message_assembly::circuit_inbound::StepConfirmation;
use crate::message_assembly::merklize::*;
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_support::ensure;
use frame_support::traits::{Currency, EnsureOrigin, Get};
use frame_system::offchain::{SignedPayload, SigningTypes};
use frame_system::RawOrigin;
use hex_literal::hex;
use pallet_contracts_registry::{RegistryContract, RegistryContractId};
use sp_application_crypto::Public;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    traits::{Convert, Hash, Saturating, Zero},
    RuntimeAppPublic, RuntimeDebug,
};

use bp_runtime::ChainId;
pub use pallet::*;
use sp_runtime::traits::AccountIdConversion;
use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::abi::{ContractActionDesc, GatewayABIConfig, HasherAlgo as HA};
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::*;
use volatile_vm::VolatileVM;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;

pub mod weights;
use weights::WeightInfo;

pub mod exec_composer;
pub mod message_assembly;

pub use crate::message_assembly::test_utils as message_test_utils;
pub mod xbridges;
pub use xbridges::{
    get_roots_from_bridge, init_bridge_instance, CurrentHash, CurrentHasher, CurrentHeader,
    DefaultPolkadotLikeGateway, EthLikeKeccak256ValU32Gateway, EthLikeKeccak256ValU64Gateway,
    PolkadotLikeValU64Gateway,
};

pub mod xtx;
pub use xtx::{Xtx, XtxId};

pub mod side_effect;
pub use side_effect::{InboundSideEffect, OutboundSideEffect, SideEffect};
pub type AllowedSideEffect = Vec<u8>;

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

pub fn select_validator_for_x_tx_dummy<T: Config>() -> Result<T::AccountId, &'static str> {
    // This is the well-known Substrate account of Alice (5GrwvaEF...)
    let default_recepient =
        hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");

    let dummy_escrow_alice =
        T::AccountId::decode(&mut &default_recepient[..]).expect("should not fail for dummy data");

    Ok(dummy_escrow_alice)
}

// todo: Implement and move as independent submodule
pub type SideEffectsDFD = Vec<u8>;
pub type GenericDFD = Vec<u8>;
pub type SideEffectId = Bytes;

pub type AuthorityId = crate::message_assembly::signer::app::Public;
pub(crate) type SystemHashing<T> = <T as frame_system::Config>::Hashing;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;

    use super::*;
    use crate::WeightInfo;
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
        + snowbridge_basic_channel::outbound::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The overarching dispatch call type.
        type Call: From<Call<Self>>;

        type AccountId32Converter: Convert<Self::AccountId, [u8; 32]>;

        type ToStandardizedGatewayBalance: Convert<BalanceOf<Self>, u128>;

        type WeightInfo: weights::WeightInfo;

        type PalletId: Get<PalletId>;
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

    #[pallet::call]
    impl<T: Config> Pallet<T> {


        /// Temporary entry for submitting a side effect directly for validation and event emittance
        /// It's temporary, since will be replaced with a DFD, which allows to specify exactly the nature of argument
        /// (SideEffect vs ComposableContract vs LocalContract or Mix)
        #[pallet::weight(<T as Config>::WeightInfo::submit_exec())]
        pub fn submit_side_effect_temp(
            origin: OriginFor<T>,
            inbound_side_effect: InboundSideEffect<T>,
            input: Vec<u8>,
            value: BalanceOf<T>,
            reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let requester = ensure_signed(origin)?;
            // Ensure can afford
            ensure!(
                <T as EscrowTrait>::Currency::free_balance(&requester).saturating_sub(reward)
                    >= BalanceOf::<T>::from(0 as u32),
                Error::<T>::RequesterNotEnoughBalance,
            );

            let side_effect = ExecDelivery::execute_side_effect(inbound_side_effect)?;

            let x_tx_id: XtxId<T> = new_xtx.generate_xtx_id::<T>();
            ActiveXtxMap::<T>::insert(x_tx_id, &new_xtx);

            Self::deposit_event(Event::XTransactionReceivedForExec(
                x_tx_id.clone(),
                // ToDo: Emit side effects DFD
                Default::default(),
            ));

            Self::deposit_event(Event::NewSideEffectsAvailable(
                requester.clone(),
                x_tx_id.clone(),
                // ToDo: Emit circuit outbound messages -> side effects
                vec![side_effect],
            ));


            Ok(().into())
        }

        #[pallet::weight(<T as Config>::WeightInfo::submit_exec())]
        pub fn submit_exec_dfd(
            origin: OriginFor<T>,
            generic_dfd: GenericDFD<T>,
            input: Vec<u8>,
            value: BalanceOf<T>,
            reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            // ToDo: Parse DFD to discover the flow of
            // enum SubmittedInputArtifact  {
            //      SideEffect,
            //      ComposableContract,
            //      LocalContract (?maybe?),
            // let submitted_artifacts: Vec<SubmittedInputArtifact> = read_generic_dfd(generic_dfd);
            // submitted_artifacts.iter().map(|submitted_artifact| {
            //  match submitted_artifact {
            //      case SubmittedInputArtifact::SideEffect {
            //          ExecDelivery::execute_side_effect(submitted_artifact)
            //      },
            //      case SubmittedInputArtifact::ComposableContract {
            //          VVM::execute_single_contract(submitted_artifact)
            //      },
            //      case SubmittedInputArtifact::LocalContract {
            //          PalletContracts::call(submitted_artifact.address, input)
            //      },
            // })
            unimplemented!();
        }


        #[pallet::weight(<T as Config>::WeightInfo::submit_exec())]
        pub fn submit_exec(
            origin: OriginFor<T>,
            contract_id: RegistryContractId<T>,
            input: Vec<u8>,
            value: BalanceOf<T>,
            reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let requester = ensure_signed(origin)?;

            // Ensure can afford
            ensure!(
                <T as EscrowTrait>::Currency::free_balance(&requester).saturating_sub(reward)
                    >= BalanceOf::<T>::from(0 as u32),
                Error::<T>::RequesterNotEnoughBalance,
            );

            let contract =
                if !<pallet_contracts_registry::ContractsRegistry<T>>::contains_key(&contract_id) {
                    Err(Error::<T>::ContractDoesNotExists)?
                } else {
                    pallet_contracts_registry::ContractsRegistry::<T>::get(&contract_id)
                        .expect("contains_key called above before accessing the contract")
                };

            Self::submit_xtx_execution(vec![contract], requester, input, value, reward)?;

            Ok(().into())
        }

        /// Will be deprecated in v1.0.0-RC
        #[pallet::weight(<T as Config>::WeightInfo::submit_composable_exec_order() + <T as Config>::WeightInfo::decompose_io_schedule())]
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

            let _escrow_account = select_validator_for_x_tx_dummy::<T>()?;

            // In dry run we would like to:
            // 1. Parse and validate the syntax of unseen in the on-chain registry contracts
            //     1.2. Add them to the on-chain registry
            // 2. Fetch all of the contracts from on-chain registry involved in that execution and dry run as one xtx.
            let (contracts, _contract_ids, _contract_descriptions) =
                Self::dry_run_whole_xtx(inter_schedule.clone(), requester.clone())?;

            let initial_input = vec![];
            let initial_value = Default::default();
            let initial_reward = Default::default();

            Self::submit_xtx_execution(
                contracts,
                requester.clone(),
                initial_input,
                initial_value,
                initial_reward,
            )?;

            Ok(().into())
        }

        /// Blind version should only be used for testing - unsafe since skips inclusion proof check.
        #[pallet::weight(<T as Config>::WeightInfo::confirm_side_effect_blind())]
        pub fn confirm_side_effect_blind(
            origin: OriginFor<T>,
            xtx_id: XtxId<T>,
            side_effect: SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>,
            _inclusion_proof: Option<Bytes>,
        ) -> DispatchResultWithPostInfo {
            // ToDo #CNF-1: Reward releyers for inbound message dispatch.
            let relayer_id = ensure_signed(origin)?;

            // ToDo #CNF-2: Check validity of execution by parsing
            //  the side effect against incoming target's format and checking its validity

            // ToDo #CNF-3: Check validity of inclusion - skip in _blind version for testing
            // Verify whether the side effect completes the Xtx
            let _xtx: Xtx<T::AccountId, T::BlockNumber, BalanceOf<T>> =
                ActiveXtxMap::<T>::get(xtx_id.clone())
                    .expect("submitted to confirm step id does not match with any Xtx");

            Self::deposit_event(Event::SideEffectConfirmed(
                relayer_id.clone(),
                xtx_id,
                side_effect,
                0,
            ));

            // ToDo: Check whether xtx.side_effects_dfd is now completed before completing xtx
            Self::deposit_event(Event::XTransactionSuccessfullyCompleted(xtx_id.clone()));

            Ok(().into())
        }

        // ToDo: Create and move higher to main Circuit pallet
        #[pallet::weight(<T as Config>::WeightInfo::register_gateway_default_polka())]
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
            allowed_side_effects: Vec<AllowedSideEffect>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            pallet_xdns::Pallet::<T>::add_new_xdns_record(
                origin.clone(),
                url,
                gateway_id,
                gateway_abi.clone(),
                gateway_vendor.clone(),
                gateway_type.clone(),
                gateway_genesis,
                allowed_side_effects.clone(),
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

            Self::deposit_event(Event::NewGatewayRegistered(
                gateway_id,           // gateway id
                gateway_type,         // type - external, programmable, tx-only
                gateway_vendor,       // vendor - substrate, eth etc.
                allowed_side_effects, // allowed side effects / enabled methods
            ));

            Ok(res.into())
        }

        // ToDo: Create and move higher to main Circuit pallet
        #[pallet::weight(<T as Config>::WeightInfo::update_gateway())]
        pub fn update_gateway(
            _origin: OriginFor<T>,
            gateway_id: bp_runtime::ChainId,
            _url: Option<Vec<u8>>,
            _gateway_abi: Option<GatewayABIConfig>,
            _authorities: Option<Vec<T::AccountId>>,
            allowed_side_effects: Option<Vec<AllowedSideEffect>>,
        ) -> DispatchResultWithPostInfo {
            // ToDo: Implement!
            Self::deposit_event(Event::GatewayUpdated(
                gateway_id,           // gateway id
                allowed_side_effects, // allowed side effects / enabled methods
            ));
            Ok(().into())
        }

        #[pallet::weight(<T as Config>::WeightInfo::confirm_side_effect())]
        pub fn confirm_side_effect(
            origin: OriginFor<T>,
            xtx_id: XtxId<T>,
            side_effect: SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>,
            _inclusion_proof: Option<Bytes>,
            // ToDo: Replace step_confirmation with inclusion_proof
            step_confirmation: StepConfirmation,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let relayer_id = ensure_signed(origin)?;
            // ToDo: parse events to discover their content and verify execution

            let _xtx: Xtx<T::AccountId, T::BlockNumber, BalanceOf<T>> =
                ActiveXtxMap::<T>::get(xtx_id.clone())
                    .expect("submitted to confirm step id does not match with any Xtx");

            // ToDo: Read gateway_id from xtx GatewaysDFD
            let gateway_id = Default::default();

            let gateway_xdns_record = pallet_xdns::Pallet::<T>::best_available(gateway_id)?;

            let declared_block_hash = step_confirmation.proof.block_hash;

            // Check inclusion relying on data in palet-multi-verifier
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

                Err(Error::<T>::SideEffectConfirmationInvalidInclusionProof.into())
            } else {
                // ToDo: Enact on the confirmation step and save the update
                // Self::update_xtx(&xtx, xtx_id, step_confirmation);
                Self::deposit_event(Event::SideEffectConfirmed(
                    relayer_id.clone(),
                    xtx_id.clone(),
                    side_effect,
                    0,
                ));

                // ToDo: Check whether xtx.side_effects_dfd is now completed before completing xtx
                Self::deposit_event(Event::XTransactionSuccessfullyCompleted(xtx_id.clone()));
                Ok(().into())
            }
        }
    }

    /// Events for the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Listeners - users + SDK + UI to know whether their request has ended
        XTransactionSuccessfullyCompleted(XtxId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and pending
        XTransactionReceivedForExec(XtxId<T>, SideEffectsDFD),
        // Listeners - executioners/relayers to know new challenges and perform offline risk/reward calc
        //  of whether side effect is worth picking up
        NewSideEffectsAvailable(
            T::AccountId,
            XtxId<T>,
            Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        ),
        // Listeners - executioners/relayers to know that certain SideEffects are no longer valid
        // ToDo: Implement Xtx timeout!
        CancelledSideEffects(
            T::AccountId,
            XtxId<T>,
            Vec<SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        ),
        // Listeners - executioners/relayers to know whether they won the confirmation challenge
        SideEffectConfirmed(
            T::AccountId, // winner
            XtxId<T>,
            SideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>,
            u64, // reward?
        ),
        // Listeners - remote targets integrators/registrants
        NewGatewayRegistered(
            bp_runtime::ChainId,    // gateway id
            GatewayType,            // type - external, programmable, tx-only
            GatewayVendor,          // vendor - substrate, eth etc.
            Vec<AllowedSideEffect>, // allowed side effects / enabled methods
        ),
        GatewayUpdated(
            bp_runtime::ChainId,  // gateway id
            Option<Vec<Vec<u8>>>, // allowed side effects / enabled methods
        ),
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
        SideEffectConfirmationInvalidInclusionProof,
        StepConfirmationDecodingError,
        ContractDoesNotExists,
        RequesterNotEnoughBalance,
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
    fn account_id() -> T::AccountId {
        T::PalletId::get().into_account()
    }
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
        _requester: T::AccountId,
    ) -> Result<
        (
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
            let key =
                SystemHashing::<T>::hash(Encode::encode(&mut protocol_part_of_contract).as_ref());

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

        let (_current_block_no, _block_zero) = (
            <frame_system::Pallet<T>>::block_number(),
            T::BlockNumber::zero(),
        );

        Ok((contracts, contract_ids, action_descriptions))
    }

    pub fn submit_xtx_execution(
        contracts: Vec<RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber>>,
        requester: T::AccountId,
        input: Vec<u8>,
        value: BalanceOf<T>,
        reward: BalanceOf<T>,
    ) -> Result<(), &'static str> {
        // ToDo: Refactor loading
        let _preload_response =
            ExecComposer::preload_bunch_of_contracts::<T>(contracts.clone(), Default::default());

        // ToDo: Work out max gas limit acceptable by each escrow
        let gas_limit = u64::max_value();
        let escrow_account = select_validator_for_x_tx_dummy::<T>()?;
        let submitter = Self::select_authority(escrow_account.clone())?;

        let (_circuit_outbound_messages, _last_executed_contract_no) =
            ExecComposer::pre_run_bunch_until_break::<T>(
                contracts,
                // ToDo: Remove escrow account from the execution pre-requisites. Leave Side Effects unassigned
                escrow_account.clone(),
                submitter.clone(),
                requester.clone(),
                value,
                input.clone(),
                gas_limit,
                None, // Circuit as a local Gateway ID = None
                // ToDo: Generate Circuit's params as default ABI
                Default::default(),
            )?;

        // ToDo: Introduce default timeout + delay
        let (timeouts_at, delay_steps_at) = (None, None);
        let new_xtx = Xtx::<T::AccountId, T::BlockNumber, BalanceOf<T>>::new(
            requester.clone(),
            input,
            timeouts_at,
            delay_steps_at,
            Some(reward),
        );

        let x_tx_id: XtxId<T> = new_xtx.generate_xtx_id::<T>();
        ActiveXtxMap::<T>::insert(x_tx_id, &new_xtx);

        Self::deposit_event(Event::XTransactionReceivedForExec(
            x_tx_id.clone(),
            // ToDo: Emit side effects DFD
            Default::default(),
        ));

        Self::deposit_event(Event::NewSideEffectsAvailable(
            requester.clone(),
            x_tx_id.clone(),
            // ToDo: Emit circuit outbound messages -> side effects
            vec![],
        ));

        Ok(())
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
}

/// Simple ensure origin from the exec delivery
pub struct EnsureExecDelivery<T>(sp_std::marker::PhantomData<T>);

impl<
        T: pallet::Config,
        O: Into<Result<RawOrigin<T::AccountId>, O>> + From<RawOrigin<T::AccountId>>,
    > EnsureOrigin<O> for EnsureExecDelivery<T>
{
    type Success = T::AccountId;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        let loan_id = T::PalletId::get().into_account();
        o.into().and_then(|o| match o {
            RawOrigin::Signed(who) if who == loan_id => Ok(loan_id),
            r => Err(O::from(r)),
        })
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin() -> O {
        let loan_id = T::PalletId::get().into_account();
        O::from(RawOrigin::Signed(loan_id))
    }
}
