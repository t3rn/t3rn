//! <!-- markdown-link-check-disable -->
//! # X-DNS Pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use codec::Encode;

use sp_std::prelude::*;
pub use t3rn_types::{
    gateway::GatewayABIConfig,
    sfx::{EventSignature, SideEffectId, SideEffectName},
};

pub use t3rn_primitives::{ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use crate::pallet::*;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

use weights::WeightInfo;

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use crate::WeightInfo;
    use frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Inspect, Mutate},
            Currency, Time,
        },
    };
    use frame_system::pallet_prelude::*;
    use sp_std::convert::TryInto;
    use t3rn_abi::{sfx_abi::SFXAbi, Codec};
    use t3rn_primitives::{
        xdns::{FullGatewayRecord, GatewayRecord, TokenRecord, Xdns},
        Bytes, ChainId, ExecutionVendor, GatewayType, GatewayVendor, TokenInfo,
    };
    use t3rn_types::{fsx::TargetId, sfx::Sfx4bId};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Type representing the weight of this pallet
        type WeightInfo: weights::WeightInfo;

        /// A type that provides inspection and mutation to some fungible assets
        type Balances: Inspect<Self::AccountId> + Mutate<Self::AccountId>;

        type Currency: Currency<Self::AccountId>;

        type Time: Time;
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
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

        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(_n: T::BlockNumber) {
            // Perform necessary data/state clean up here.
        }

        // A runtime code run after every block and have access to extended set of APIs.
        //
        // For instance you can generate extrinsics for the upcoming produced block.
        fn offchain_worker(_n: T::BlockNumber) {
            // We don't do anything here.
            // but we could dispatch extrinsic (transaction/unsigned/inherent) using
            // sp_io::submit_extrinsic.
            // To see example on offchain worker, please refer to example-offchain-worker pallet
            // accompanied in this repository.
        }

        fn on_runtime_upgrade() -> Weight {
            // Define the maximum weight of this migration.
            let max_weight = T::DbWeight::get().reads_writes(10, 10);
            // Define the current storage migration version.
            const CURRENT_STORAGE_VERSION: u32 = 1;
            // Migrate the storage entries.
            StorageMigrations::<T>::try_mutate(|current_version| {
                match *current_version {
                    0 => {
                        // Storage Migration: StandardSideEffects -> StandardSFXABIs
                        // Storage Migration Details: 16-03-2023; v1.4.0-rc -> v1.5.0-rc
                        // Iterate through the old storage entries and migrate them.
                        for (key, _value) in StandardSideEffects::<T>::drain() {
                            let sfx4b_id = key;
                            match SFXAbi::get_standard_interface(sfx4b_id) {
                                Some(sfx_abi) => {
                                    StandardSFXABIs::<T>::insert(sfx4b_id, sfx_abi);
                                }
                                None => {
                                    log::error!(
                                "Failed to migrate StandardSideEffects to StandardSFXABIs for sfx4b_id: {:?}",
                                sfx4b_id
                            );
                                }
                            }
                        }

                        // Set migrations_done to true
                        *current_version = CURRENT_STORAGE_VERSION;

                        // Return the weight consumed by the migration.
                        Ok::<Weight, DispatchError>(max_weight)
                    }
                    // Add more migration cases here, if needed in the future
                    _ => {
                        // No migration needed.
                        Ok::<Weight, DispatchError>(0 as Weight)
                    }
                }
            })
            .unwrap_or(0)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Removes a gateway from the onchain registry. Root only access.
        #[pallet::weight(< T as Config >::WeightInfo::purge_gateway())]
        pub fn purge_gateway_record(
            origin: OriginFor<T>,
            requester: T::AccountId,
            gateway_id: [u8; 4],
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            if !<Gateways<T>>::contains_key(gateway_id) {
                Err(Error::<T>::XdnsRecordNotFound.into())
            } else {
                <Gateways<T>>::remove(gateway_id);
                // remove all tokens associated with this gateway
                Tokens::<T>::iter_values()
                    .filter(|token| token.gateway_id == gateway_id)
                    .for_each(|token| {
                        <Tokens<T>>::remove(token.token_id);
                    });

                Self::deposit_event(Event::<T>::GatewayRecordPurged(requester, gateway_id));
                Ok(().into())
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// \[gateway_4b_id\]
        GatewayRecordStored([u8; 4]),
        /// \[token_4b_id, gateway_4b_id\]
        TokenRecordStored([u8; 4], [u8; 4]),
        /// \[requester, gateway_record_id\]
        GatewayRecordPurged(T::AccountId, [u8; 4]),
        /// \[requester, xdns_record_id\]
        XdnsRecordPurged(T::AccountId, [u8; 4]),
        /// \[xdns_record_id\]
        XdnsRecordUpdated([u8; 4]),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Stored gateway has already been added before
        GatewayRecordAlreadyExists,
        /// XDNS Record not found
        XdnsRecordNotFound,
        /// Stored token has already been added before
        TokenRecordAlreadyExists,
        /// Gateway Record not found
        GatewayRecordNotFound,
        /// SideEffectABI already exists
        SideEffectABIAlreadyExists,
        /// SideEffectABI not found
        SideEffectABINotFound,
        /// the xdns entry does not contain parachain information
        NoParachainInfoFound,
        /// A token is not compatible with the gateways execution layer
        TokenExecutionVendorMismatch,
    }

    // Deprecated storage entry -- StandardSideEffects
    // Storage Migration: StandardSideEffects -> StandardSFXABIs
    // Storage Migration Details: 16-03-2023; v1.4.0-rc -> v1.5.0-rc
    #[pallet::storage]
    pub type StandardSideEffects<T: Config> = StorageMap<_, Identity, [u8; 4], Vec<u8>>; // SideEffectInterface

    // Deprecated storage entry -- CustomSideEffects
    // Storage Migration: CustomSideEffects -> !dropped and replaced by SFXABIRegistry
    // Storage Migration Details: 16-03-2023; v1.4.0-rc -> v1.5.0-rc
    #[pallet::storage]
    pub type CustomSideEffects<T: Config> = StorageMap<_, Identity, SideEffectId<T>, Vec<u8>>;

    #[pallet::storage]
    #[pallet::getter(fn storage_migrations_done)]
    pub type StorageMigrations<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    pub type StandardSFXABIs<T: Config> = StorageMap<_, Identity, Sfx4bId, SFXAbi>;

    #[pallet::storage]
    pub type SFXABIRegistry<T: Config> =
        StorageDoubleMap<_, Identity, TargetId, Identity, Sfx4bId, SFXAbi>;

    #[pallet::storage]
    #[pallet::getter(fn gateways)]
    pub type Gateways<T: Config> =
        StorageMap<_, Identity, [u8; 4], GatewayRecord<T::AccountId>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn tokens)]
    pub type Tokens<T: Config> = StorageMap<_, Identity, [u8; 4], TokenRecord, OptionQuery>;

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub known_gateway_records: Vec<GatewayRecord<T::AccountId>>,
        pub standard_sfx_abi: Vec<(Sfx4bId, SFXAbi)>,
    }

    /// The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                known_gateway_records: Default::default(),
                standard_sfx_abi: Default::default(),
            }
        }
    }

    /// The build of genesis for the pallet.
    /// Populates storage with the known XDNS Records
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (sfx_4b_id, sfx_abi) in self.standard_sfx_abi.iter() {
                let sfx_4b_str = sp_std::str::from_utf8(sfx_4b_id.as_slice())
                    .unwrap_or("invalid utf8 4b sfx id format");
                log::info!("XDNS -- on-genesis: add standard SFX ABI: {:?}", sfx_4b_str);
                <StandardSFXABIs<T>>::insert(sfx_4b_id, sfx_abi);
            }

            for gateway_record in self.known_gateway_records.clone() {
                <Gateways<T>>::insert(gateway_record.gateway_id, gateway_record.clone());
                // Populate standard side effect ABI registry
                for (sfx_4b_id, memo_prefix) in gateway_record.allowed_side_effects.iter() {
                    match <StandardSFXABIs<T>>::get(sfx_4b_id) {
                        Some(mut abi) => {
                            abi.maybe_prefix_memo = *memo_prefix;
                            <SFXABIRegistry<T>>::insert(gateway_record.gateway_id, sfx_4b_id, abi)
                        },
                        None => {
                            let sfx_4b_str = sp_std::str::from_utf8(sfx_4b_id.as_slice())
                                .unwrap_or("invalid utf8 4b sfx id format");
                            log::error!(
                                "XDNS -- on-genesis: standard SFX ABI not found: {:?}",
                                sfx_4b_str
                            )
                        },
                    }
                }
            }
        }
    }

    impl<T: Config> Xdns<T> for Pallet<T> {
        /// Fetches all known Gateway records
        fn fetch_gateways() -> Vec<GatewayRecord<T::AccountId>> {
            Gateways::<T>::iter_values().collect()
        }

        fn fetch_full_gateway_records() -> Vec<FullGatewayRecord<T::AccountId>> {
            Gateways::<T>::iter_values()
                .map(|gateway| {
                    let tokens = Tokens::<T>::iter_values()
                        .filter(|token| token.gateway_id == gateway.gateway_id)
                        .collect();
                    FullGatewayRecord {
                        gateway_record: gateway,
                        tokens,
                    }
                })
                .collect()
        }

        fn add_new_gateway(
            gateway_id: [u8; 4],
            verification_vendor: GatewayVendor,
            execution_vendor: ExecutionVendor,
            codec: Codec,
            registrant: Option<T::AccountId>,
            escrow_account: Option<T::AccountId>,
            allowed_side_effects: Vec<([u8; 4], Option<u8>)>,
        ) -> DispatchResult {
            // early exit if record already exists in storage
            if <Gateways<T>>::contains_key(gateway_id) {
                return Err(Error::<T>::GatewayRecordAlreadyExists.into())
            }

            Self::override_gateway(
                gateway_id,
                verification_vendor,
                execution_vendor,
                codec,
                registrant,
                escrow_account,
                allowed_side_effects,
            )
        }

        fn override_gateway(
            gateway_id: [u8; 4],
            verification_vendor: GatewayVendor,
            execution_vendor: ExecutionVendor,
            codec: Codec,
            registrant: Option<T::AccountId>,
            escrow_account: Option<T::AccountId>,
            allowed_side_effects: Vec<([u8; 4], Option<u8>)>,
        ) -> DispatchResult {
            // Populate standard side effect ABI registry
            for (sfx_4b_id, maybe_event_memo_prefix) in allowed_side_effects.iter() {
                match <StandardSFXABIs<T>>::get(sfx_4b_id) {
                    Some(mut abi) => {
                        abi.maybe_prefix_memo = *maybe_event_memo_prefix;
                        <SFXABIRegistry<T>>::insert(gateway_id, sfx_4b_id, abi)
                    },
                    None => return Err(Error::<T>::SideEffectABINotFound.into()),
                }
            }
            <Gateways<T>>::insert(
                gateway_id,
                GatewayRecord {
                    gateway_id,
                    verification_vendor,
                    execution_vendor,
                    codec,
                    registrant,
                    escrow_account,
                    allowed_side_effects,
                },
            );
            Self::deposit_event(Event::<T>::GatewayRecordStored(gateway_id));

            Ok(())
        }

        fn add_new_token(
            token_id: [u8; 4],
            gateway_id: [u8; 4],
            token_props: TokenInfo,
        ) -> DispatchResult {
            // early exit if record already exists in storage
            if <Tokens<T>>::contains_key(token_id) {
                return Err(Error::<T>::TokenRecordAlreadyExists.into())
            }

            // fetch record and ensure it exists
            let record = <Gateways<T>>::get(gateway_id).ok_or(Error::<T>::GatewayRecordNotFound)?;

            // ensure that the token's execution vendor matches the gateway's execution vendor
            ensure!(
                token_props.match_execution_vendor() == record.execution_vendor,
                Error::<T>::TokenExecutionVendorMismatch
            );

            Self::override_token(token_id, gateway_id, token_props)
        }

        fn override_token(
            token_id: [u8; 4],
            gateway_id: [u8; 4],
            token_props: TokenInfo,
        ) -> DispatchResult {
            <Tokens<T>>::insert(
                token_id,
                TokenRecord {
                    token_id,
                    gateway_id,
                    token_props,
                },
            );
            Self::deposit_event(Event::<T>::TokenRecordStored(token_id, gateway_id));
            Ok(())
        }

        /// returns a mapping of all allowed side_effects of a gateway.
        fn allowed_side_effects(gateway_id: &ChainId) -> Vec<(Sfx4bId, Option<u8>)> {
            match <Gateways<T>>::get(gateway_id) {
                Some(gateway) => gateway.allowed_side_effects,
                None => Vec::new(),
            }
        }

        /// returns the gateway vendor of a gateway if its available
        fn get_verification_vendor(chain_id: &ChainId) -> Result<GatewayVendor, DispatchError> {
            match <Gateways<T>>::get(chain_id) {
                Some(rec) => Ok(rec.verification_vendor),
                None => Err(Error::<T>::XdnsRecordNotFound.into()),
            }
        }

        fn get_target_codec(chain_id: &ChainId) -> Result<Codec, DispatchError> {
            match <Gateways<T>>::get(chain_id) {
                Some(rec) => Ok(rec.codec),
                None => Err(Error::<T>::XdnsRecordNotFound.into()),
            }
        }

        fn get_escrow_account(chain_id: &ChainId) -> Result<Bytes, DispatchError> {
            match <Gateways<T>>::get(chain_id) {
                Some(rec) => Ok(rec.escrow_account.encode()),
                None => Err(Error::<T>::XdnsRecordNotFound.into()),
            }
        }

        // todo: this must be removed and functionality replaced
        fn get_gateway_type_unsafe(chain_id: &ChainId) -> GatewayType {
            if chain_id == &[3u8; 4] {
                return GatewayType::OnCircuit(0)
            }
            match <Gateways<T>>::get(chain_id) {
                Some(rec) => match rec.escrow_account {
                    Some(_) => GatewayType::ProgrammableExternal(0),
                    None => GatewayType::TxOnly(0),
                },
                None => panic!("Gateway record not found"),
            }
        }

        fn extend_sfx_abi(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            sfx_4b_id: Sfx4bId,
            sfx_expected_abi: SFXAbi,
        ) -> DispatchResult {
            ensure_root(origin)?;
            if !<Gateways<T>>::contains_key(gateway_id) {
                return Err(Error::<T>::XdnsRecordNotFound.into())
            }

            <SFXABIRegistry<T>>::mutate(gateway_id, sfx_4b_id, |sfx_abi| match sfx_abi {
                Some(_) => Err(Error::<T>::SideEffectABIAlreadyExists),
                None => {
                    *sfx_abi = Some(sfx_expected_abi);
                    Ok(())
                },
            })?;

            Ok(())
        }

        fn override_sfx_abi(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            new_sfx_abis: Vec<(Sfx4bId, SFXAbi)>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            if !<Gateways<T>>::contains_key(gateway_id) {
                return Err(Error::<T>::XdnsRecordNotFound.into())
            }

            for (sfx_4b_id, sfx_expected_abi) in new_sfx_abis {
                <SFXABIRegistry<T>>::mutate(gateway_id, sfx_4b_id, |sfx_abi| {
                    *sfx_abi = Some(sfx_expected_abi);
                });
            }

            Ok(())
        }

        fn get_all_sfx_abi(gateway_id: &ChainId) -> Vec<(Sfx4bId, SFXAbi)> {
            <SFXABIRegistry<T>>::iter_prefix(gateway_id)
                .map(|(sfx_4b_id, sfx_abi)| (sfx_4b_id, sfx_abi))
                .collect()
        }

        fn get_sfx_abi(gateway_id: &ChainId, sfx_4b_id: Sfx4bId) -> Option<SFXAbi> {
            <SFXABIRegistry<T>>::get(gateway_id, sfx_4b_id)
        }

        fn add_escrow_account(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            escrow_account: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Gateways::<T>::mutate(gateway_id, |gateway| match gateway {
                None => Err(Error::<T>::GatewayRecordNotFound),
                Some(record) => {
                    record.escrow_account = Some(escrow_account);
                    Ok(())
                },
            })?;

            Ok(())
        }
    }
}
