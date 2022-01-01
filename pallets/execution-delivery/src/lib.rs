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
use frame_support::dispatch::DispatchResultWithPostInfo;

use frame_support::traits::{Currency, EnsureOrigin, Get};
use frame_system::offchain::{SignedPayload, SigningTypes};
use frame_system::RawOrigin;

use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    traits::{AccountIdConversion, Convert, Saturating},
    RuntimeDebug,
};
use sp_std::vec;
use sp_std::vec::*;

pub use t3rn_primitives::{
    abi::{GatewayABIConfig, HasherAlgo as HA},
    side_effect::{ConfirmedSideEffect, FullSideEffect, SideEffect},
    transfers::BalanceOf,
    volatile::LocalState,
    xtx::{Xtx, XtxId},
    bridges::runtime as bp_runtime,
    bridges::chain_circuit as bp_circuit,
    GatewayType, *,
};
use t3rn_protocol::side_effects::confirm::protocol::confirm_with_vendor_by_action_id;
pub use t3rn_protocol::{circuit_inbound::StepConfirmation, merklize::*};

pub type Bytes = Vec<u8>;

pub use pallet::*;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;

pub mod weights;

use weights::WeightInfo;

pub mod xbridges;

pub use xbridges::{
    get_roots_from_bridge, init_bridge_instance, CurrentHash, CurrentHasher, CurrentHeader,
    DefaultPolkadotLikeGateway, EthLikeKeccak256ValU32Gateway, EthLikeKeccak256ValU64Gateway,
    PolkadotLikeValU64Gateway,
};

use t3rn_protocol::side_effects::confirm::substrate::SubstrateSideEffectsParser;
use t3rn_protocol::side_effects::loader::{SideEffectsLazyLoader, UniversalSideEffectsProtocol};
pub use t3rn_protocol::side_effects::protocol::SideEffectConfirmationProtocol;
use t3rn_protocol::side_effects::protocol::TransferSideEffectProtocol;

pub type AllowedSideEffect = Vec<u8>;

/// Defines application identifier for crypto keys of this module.
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

// todo: Implement and move as independent submodule
pub type SideEffectsDFD = Vec<u8>;
pub type GenericDFD = Vec<u8>;
pub type SideEffectId = Bytes;

pub type SystemHashing<T> = <T as frame_system::Config>::Hashing;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use snowbridge_core::Verifier;
    use t3rn_protocol::side_effects::confirm::ethereum::EthereumSideEffectsParser;

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
        + pallet_balances::Config
        + pallet_contracts_registry::Config
        + pallet_xdns::Config
        + pallet_contracts::Config
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

        type AccountId32Converter: Convert<<Self as frame_system::Config>::AccountId, [u8; 32]>;

        type ToStandardizedGatewayBalance: Convert<BalanceOf<Self>, u128>;

        type WeightInfo: weights::WeightInfo;

        type PalletId: Get<PalletId>;

        type EthVerifier: Verifier;
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
        fn on_initialize(_n: <T as frame_system::Config>::BlockNumber) -> Weight {
            // Anything that needs to be done at the start of the block.
            // We don't do anything here.
            0
        }

        fn on_finalize(_n: <T as frame_system::Config>::BlockNumber) {
            // We don't do anything here.

            // if module block number
            // x-t3rn#4: Go over open Xtx and cancel if necessary
        }

        // A runtime code run after every block and have access to extended set of APIs.
        //
        // For instance you can generate extrinsics for the upcoming produced block.
        fn offchain_worker(_n: <T as frame_system::Config>::BlockNumber) {
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
        #[pallet::weight(< T as Config >::WeightInfo::submit_exec())]
        pub fn submit_side_effects_temp(
            origin: OriginFor<T>,
            side_effects: Vec<SideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>>,
            input: Vec<u8>,
            _value: BalanceOf<T>,
            reward: BalanceOf<T>,
            sequential: bool,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let requester = ensure_signed(origin)?;
            // Ensure can afford
            ensure!(
                <T as EscrowTrait>::Currency::free_balance(&requester).saturating_sub(reward)
                    >= BalanceOf::<T>::from(0 as u32),
                Error::<T>::RequesterNotEnoughBalance,
            );

            let mut full_side_effects: Vec<
                FullSideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>,
            > = vec![];

            let mut use_protocol = UniversalSideEffectsProtocol::new();
            let mut local_state = LocalState::new();

            for side_effect in side_effects.iter() {
                // ToDo: Generate Circuit's params as default ABI from let abi = pallet_xdns::get_abi(target_id)
                let gateway_abi = Default::default();

                use_protocol.notice_gateway(side_effect.target);
                use_protocol
                    .validate_args::<<T as frame_system::Config>::AccountId, T::BlockNumber, BalanceOf<T>, SystemHashing<T>>(
                        side_effect.clone(),
                        gateway_abi,
                        &mut local_state,
                    )?;

                full_side_effects.push(FullSideEffect {
                    input: side_effect.clone(),
                    confirmed: None,
                })
            }

            let full_side_effects_steps = match sequential {
                false => vec![full_side_effects],
                true => {
                    let mut sequential_order: Vec<
                        Vec<FullSideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>>,
                    > = vec![];
                    for fse in full_side_effects.iter() {
                        sequential_order.push(vec![fse.clone()]);
                    }
                    sequential_order
                }
            };

            // ToDo: Introduce default timeout + delay
            let (timeouts_at, delay_steps_at) = (None, None);
            let new_xtx = Xtx::<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>::new(
                requester.clone(),
                input,
                timeouts_at,
                delay_steps_at,
                Some(reward),
                local_state,
                // ToDo: Missing GenericDFD to link side effects / composable contracts with the Xtx
                full_side_effects_steps,
            );
            let x_tx_id: XtxId<T> = new_xtx.generate_xtx_id::<T>();

            ActiveXtxMap::<T>::insert(x_tx_id, &new_xtx);

            Self::submit_side_effects(x_tx_id, requester, side_effects, sequential);

            Ok(().into())
        }

        #[pallet::weight(< T as Config >::WeightInfo::submit_exec())]
        pub fn submit_exec_dfd(
            _origin: OriginFor<T>,
            _generic_dfd: GenericDFD,
            _input: Vec<u8>,
            _value: BalanceOf<T>,
            _reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            // ToDo: Parse DFD to discover the flow of Xtx:
            // E.g.
            // $ComposableContract#0x232233223($TransferSideEffect(
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

        /// Blind version should only be used for testing - unsafe since skips inclusion proof check.
        #[pallet::weight(< T as Config >::WeightInfo::confirm_side_effect_blind())]
        pub fn confirm_side_effect_blind(
            origin: OriginFor<T>,
            xtx_id: XtxId<T>,
            side_effect: SideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>,
            confirmed_side_effect: ConfirmedSideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>,
            _inclusion_proof: Option<Bytes>,
        ) -> DispatchResultWithPostInfo {
            // ToDo #CNF-1: Reward releyers for inbound message dispatch.
            let relayer_id = ensure_signed(origin)?;

            // ToDo #CNF-1: Check validity of inclusion - skip in _blind version for testing
            // Verify whether the side effect completes the Xtx
            let mut xtx: Xtx<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>> =
                ActiveXtxMap::<T>::get(xtx_id.clone())
                    .expect("submitted to confirm step id does not match with any Xtx");

            let mut state_copy = xtx.local_state.clone();
            let gateway_vendor = pallet_xdns::Pallet::<T>::best_available(side_effect.target)?;
            let side_effect_id = side_effect.generate_id::<SystemHashing<T>>();

            confirm_with_vendor_by_action_id::<
                T,
                SubstrateSideEffectsParser,
                EthereumSideEffectsParser<T::EthVerifier>,
            >(
                gateway_vendor.gateway_vendor,
                side_effect.encoded_action.clone(),
                confirmed_side_effect.encoded_effect.clone(),
                &mut state_copy,
                Some(side_effect_id.as_ref().to_vec()),
            )?;

            // Check if the side effect has been deposited with respect to the execution order
            if xtx.complete_side_effect::<bp_circuit::Hasher>(
                confirmed_side_effect.clone(),
                side_effect.clone(),
            )? {
                Self::deposit_event(Event::SideEffectConfirmed(
                    relayer_id.clone(),
                    xtx_id,
                    confirmed_side_effect,
                    0,
                ));
            }

            if xtx.is_completed() {
                // ToDo: Check whether xtx.side_effects_dfd is now completed before completing xtx
                Self::deposit_event(Event::XTransactionSuccessfullyCompleted(xtx_id.clone()));
            }

            Ok(().into())
        }

        // ToDo: Create and move higher to main Circuit pallet
        #[pallet::weight(< T as Config >::WeightInfo::register_gateway_default_polka())]
        pub fn register_gateway(
            origin: OriginFor<T>,
            url: Vec<u8>,
            gateway_id: bp_runtime::ChainId,
            gateway_abi: GatewayABIConfig,
            gateway_vendor: t3rn_primitives::GatewayVendor,
            gateway_type: t3rn_primitives::GatewayType,
            gateway_genesis: GatewayGenesisConfig,
            gateway_sys_props: GatewaySysProps,
            first_header: Vec<u8>,
            authorities: Option<Vec<<T as frame_system::Config>::AccountId>>,
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
                gateway_sys_props.clone(),
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
                gateway_sys_props,    // system properties - ss58 format, token symbol etc.
                allowed_side_effects, // allowed side effects / enabled methods
            ));

            Ok(res.into())
        }

        // ToDo: Create and move higher to main Circuit pallet
        #[pallet::weight(< T as Config >::WeightInfo::update_gateway())]
        pub fn update_gateway(
            _origin: OriginFor<T>,
            gateway_id: bp_runtime::ChainId,
            _url: Option<Vec<u8>>,
            _gateway_abi: Option<GatewayABIConfig>,
            _gateway_sys_props: Option<GatewaySysProps>,
            _authorities: Option<Vec<<T as frame_system::Config>::AccountId>>,
            allowed_side_effects: Option<Vec<AllowedSideEffect>>,
        ) -> DispatchResultWithPostInfo {
            // ToDo: Implement!
            Self::deposit_event(Event::GatewayUpdated(
                gateway_id,           // gateway id
                allowed_side_effects, // allowed side effects / enabled methods
            ));
            Ok(().into())
        }

        #[pallet::weight(< T as Config >::WeightInfo::confirm_side_effect())]
        pub fn confirm_side_effect(
            origin: OriginFor<T>,
            xtx_id: XtxId<T>,
            side_effect: SideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>,
            confirmed_side_effect: ConfirmedSideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>,
            _inclusion_proof: Option<Bytes>,
            // ToDo: Replace step_confirmation with inclusion_proof
            step_confirmation: StepConfirmation,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let relayer_id = ensure_signed(origin)?;
            // ToDo: parse events to discover their content and verify execution

            let mut xtx: Xtx<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>> =
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
                // Check if the side effect has been deposited with respect to the execution order
                if xtx.complete_side_effect::<bp_circuit::Hasher>(
                    confirmed_side_effect.clone(),
                    side_effect.clone(),
                )? {
                    Self::deposit_event(Event::SideEffectConfirmed(
                        relayer_id.clone(),
                        xtx_id,
                        confirmed_side_effect,
                        0,
                    ));
                }

                if xtx.is_completed() {
                    // ToDo: Check whether xtx.side_effects_dfd is now completed before completing xtx
                    Self::deposit_event(Event::XTransactionSuccessfullyCompleted(xtx_id.clone()));
                }

                Ok(().into())
            }
        }
    }

    /// Events for the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Listeners - users + SDK + UI to know whether their request has ended
        XTransactionSuccessfullyCompleted(XtxId<T>),
        // Listeners - users + SDK + UI to know whether their request is accepted for exec and pending
        XTransactionReceivedForExec(XtxId<T>, SideEffectsDFD),
        // Listeners - executioners/relayers to know new challenges and perform offline risk/reward calc
        //  of whether side effect is worth picking up
        NewSideEffectsAvailable(
            <T as frame_system::Config>::AccountId,
            XtxId<T>,
            Vec<SideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>>,
        ),
        // Listeners - executioners/relayers to know that certain SideEffects are no longer valid
        // ToDo: Implement Xtx timeout!
        CancelledSideEffects(
            <T as frame_system::Config>::AccountId,
            XtxId<T>,
            Vec<SideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>>,
        ),
        // Listeners - executioners/relayers to know whether they won the confirmation challenge
        SideEffectConfirmed(
            <T as frame_system::Config>::AccountId, // winner
            XtxId<T>,
            ConfirmedSideEffect<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber, BalanceOf<T>>,
            u64, // reward?
        ),
        // Listeners - remote targets integrators/registrants
        NewGatewayRegistered(
            bp_runtime::ChainId,    // gateway id
            GatewayType,            // type - external, programmable, tx-only
            GatewayVendor,          // vendor - substrate, eth etc.
            GatewaySysProps,        // system properties - ss58 format, token symbol etc.
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
        VendorUnknown,
        SideEffectTypeNotRecognized,
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
    fn account_id() -> <T as frame_system::Config>::AccountId {
        T::PalletId::get().into_account()
    }

    pub fn submit_side_effects(
        x_tx_id: XtxId<T>,
        requester: <T as frame_system::Config>::AccountId,
        side_effects: Vec<SideEffect<<T as frame_system::Config>::AccountId, T::BlockNumber, BalanceOf<T>>>,
        sequential: bool,
    ) {
        Self::deposit_event(Event::XTransactionReceivedForExec(
            x_tx_id.clone(),
            // ToDo: Emit side effects DFD
            sequential.encode(),
        ));

        Self::deposit_event(Event::NewSideEffectsAvailable(
            requester.clone(),
            x_tx_id.clone(),
            // ToDo: Emit circuit outbound messages -> side effects
            side_effects,
        ));
    }
}

/// Simple ensure origin from the exec delivery
pub struct EnsureExecDelivery<T>(sp_std::marker::PhantomData<T>);

impl<
        T: pallet::Config,
        O: Into<Result<RawOrigin<<T as frame_system::Config>::AccountId>, O>> + From<RawOrigin<<T as frame_system::Config>::AccountId>>,
    > EnsureOrigin<O> for EnsureExecDelivery<T>
{
    type Success = <T as frame_system::Config>::AccountId;

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
