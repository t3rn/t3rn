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

use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResultWithPostInfo,
    traits::{EnsureOrigin, Get},
};
use frame_system::{
    offchain::{SignedPayload, SigningTypes},
    RawOrigin,
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    traits::{AccountIdConversion, Convert},
    RuntimeDebug,
};
use sp_std::vec::*;
pub use t3rn_primitives::{
    abi::{GatewayABIConfig, HasherAlgo},
    bridges::{chain_circuit as bp_circuit, runtime as bp_runtime},
    side_effect::{ConfirmedSideEffect, FullSideEffect, SideEffect},
    transfers::BalanceOf,
    volatile::LocalState,
    xtx::{Xtx, XtxId},
    GatewayType, *,
};
pub use t3rn_protocol::{circuit_inbound::StepConfirmation, merklize::*};

pub use pallet::*;
use t3rn_primitives::{circuit_portal::CircuitPortal, xdns::Xdns};

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

use sp_finality_grandpa::SetId;
pub type AllowedSideEffect = [u8; 4];

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

const LOG_TARGET: &str = "circuit-portal";

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::fungible::{Inspect, Mutate},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use snowbridge_core::Verifier;
    use t3rn_primitives::xdns::Xdns;

    use super::*;
    use crate::WeightInfo;

    /// This pallet's configuration trait
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_multi_finality_verifier::Config<DefaultPolkadotLikeGateway>
        + pallet_multi_finality_verifier::Config<PolkadotLikeValU64Gateway>
        + pallet_multi_finality_verifier::Config<EthLikeKeccak256ValU64Gateway>
        + pallet_multi_finality_verifier::Config<EthLikeKeccak256ValU32Gateway>
    // + snowbridge_basic_channel::outbound::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The overarching dispatch call type.
        type Call: From<Call<Self>>;

        type AccountId32Converter: Convert<<Self as frame_system::Config>::AccountId, [u8; 32]>;

        // TODO: removed since its better to have an account manager for this and is not used atm
        // type ToStandardizedGatewayBalance: Convert<
        //     EscrowedBalanceOf<Self, <Self as Config>::Escrowed>,
        //     u128,
        // >;

        type WeightInfo: weights::WeightInfo;

        type PalletId: Get<PalletId>;

        type EthVerifier: Verifier;

        /// A type that provides inspection and mutation to some fungible assets
        type Balances: Inspect<Self::AccountId> + Mutate<Self::AccountId>;

        /// A type that manages escrow, and therefore balances
        type Escrowed: EscrowTrait<Self>;

        /// A type that provides access to Xdns
        type Xdns: Xdns<Self>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
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
        // ToDo: Create and move higher to main Circuit pallet
        #[pallet::weight(< T as Config >::WeightInfo::register_gateway_default_polka())]
        pub fn register_gateway(
            origin: OriginFor<T>,
            url: Vec<u8>,
            gateway_id: ChainId,
            gateway_abi: GatewayABIConfig,
            gateway_vendor: t3rn_primitives::GatewayVendor,
            gateway_type: t3rn_primitives::GatewayType,
            gateway_genesis: GatewayGenesisConfig,
            gateway_sys_props: GatewaySysProps,
            first_header: Vec<u8>,
            authorities: Option<Vec<<T as frame_system::Config>::AccountId>>,
            authority_set_id: Option<SetId>,
            allowed_side_effects: Vec<AllowedSideEffect>,
        ) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            <T as Config>::Xdns::add_new_xdns_record(
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
                (HasherAlgo::Blake2, 32) => init_bridge_instance::<T, DefaultPolkadotLikeGateway>(
                    origin,
                    first_header,
                    authorities,
                    authority_set_id,
                    gateway_id,
                )?,
                (HasherAlgo::Blake2, 64) => init_bridge_instance::<T, PolkadotLikeValU64Gateway>(
                    origin,
                    first_header,
                    authorities,
                    authority_set_id,
                    gateway_id,
                )?,
                (HasherAlgo::Keccak256, 32) =>
                    init_bridge_instance::<T, EthLikeKeccak256ValU32Gateway>(
                        origin,
                        first_header,
                        authorities,
                        authority_set_id,
                        gateway_id,
                    )?,
                (HasherAlgo::Keccak256, 64) =>
                    init_bridge_instance::<T, EthLikeKeccak256ValU64Gateway>(
                        origin,
                        first_header,
                        authorities,
                        authority_set_id,
                        gateway_id,
                    )?,
                (_, _) => init_bridge_instance::<T, DefaultPolkadotLikeGateway>(
                    origin,
                    first_header,
                    authorities,
                    authority_set_id,
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

            Ok(res)
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
    }

    /// Events for the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Listeners - remote targets integrators/registrants
        NewGatewayRegistered(
            bp_runtime::ChainId,    // gateway id
            GatewayType,            // type - external, programmable, tx-only
            GatewayVendor,          // vendor - substrate, eth etc.
            GatewaySysProps,        // system properties - ss58 format, token symbol etc.
            Vec<AllowedSideEffect>, // allowed side effects / enabled methods
        ),
        GatewayUpdated(
            bp_runtime::ChainId,            // gateway id
            Option<Vec<AllowedSideEffect>>, // allowed side effects / enabled methods
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

impl<T: Config> CircuitPortal<T> for Pallet<T> {
    type EthVerifier = T::EthVerifier;

    fn confirm_inclusion(
        gateway_id: [u8; 4],
        _encoded_message: Vec<u8>,
        trie_type: ProofTriePointer,
        maybe_block_hash: Option<Vec<u8>>,
        maybe_proof: Option<Vec<Vec<u8>>>,
    ) -> Result<(), &'static str> {
        let gateway_xdns_record = <T as Config>::Xdns::best_available(gateway_id)?;

        match gateway_xdns_record.gateway_vendor {
            GatewayVendor::Ethereum => {
                unimplemented!()
            },
            GatewayVendor::Substrate => {
                // For Substrate bridges block hash is required
                let block_hash = if let Some(x) = maybe_block_hash {
                    Ok(x)
                } else {
                    Err(
                        "Must provide a valid read proof when proving inclusion with Substrate Bridge"
                    )
                }?;
                let proof = if let Some(x) = maybe_proof {
                    Ok(x)
                } else {
                    Err(
                        "Must provide a valid read proof when proving inclusion with Substrate Bridge"
                    )
                }?;
                // Check inclusion relying on data in pallet-multi-verifier
                let (extrinsics_root_h256, storage_root_h256) = match (
                    gateway_xdns_record.gateway_abi.hasher.clone(),
                    gateway_xdns_record.gateway_abi.block_number_type_size,
                ) {
                    (HasherAlgo::Blake2, 32) => get_roots_from_bridge::<
                        T,
                        DefaultPolkadotLikeGateway,
                    >(block_hash, gateway_id)?,
                    (HasherAlgo::Blake2, 64) => get_roots_from_bridge::<
                        T,
                        PolkadotLikeValU64Gateway,
                    >(block_hash, gateway_id)?,
                    (HasherAlgo::Keccak256, 32) => get_roots_from_bridge::<
                        T,
                        EthLikeKeccak256ValU32Gateway,
                    >(block_hash, gateway_id)?,
                    (HasherAlgo::Keccak256, 64) => get_roots_from_bridge::<
                        T,
                        EthLikeKeccak256ValU64Gateway,
                    >(block_hash, gateway_id)?,
                    (_, _) => get_roots_from_bridge::<T, DefaultPolkadotLikeGateway>(
                        block_hash, gateway_id,
                    )?,
                };

                // let expected_root = match step_confirmation.proof.proof_trie_pointer {
                let expected_root = match trie_type {
                    ProofTriePointer::State => storage_root_h256,
                    ProofTriePointer::Transaction => extrinsics_root_h256,
                    ProofTriePointer::Receipts => storage_root_h256,
                };

                return if let Err(computed_root) = check_merkle_proof(
                    expected_root,
                    // step_confirmation.proof.proof_data.into_iter(),
                    proof.into_iter(),
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
                    Ok(())
                }
            },
        }
    }
}
impl<T: Config> Pallet<T> {
    pub fn account_id() -> <T as frame_system::Config>::AccountId {
        T::PalletId::get().into_account()
    }
}

/// Simple ensure origin from the portal
pub struct EnsureCircuitPortal<T>(sp_std::marker::PhantomData<T>);

impl<
        T: pallet::Config,
        O: Into<Result<RawOrigin<<T as frame_system::Config>::AccountId>, O>>
            + From<RawOrigin<<T as frame_system::Config>::AccountId>>,
    > EnsureOrigin<O> for EnsureCircuitPortal<T>
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
