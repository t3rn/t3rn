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

use frame_support::sp_runtime::traits::Saturating;
use t3rn_primitives::reexport_currency_types;
pub use t3rn_primitives::{ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use crate::pallet::*;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

use weights::WeightInfo;
reexport_currency_types!();

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use crate::WeightInfo;
    use circuit_runtime_types::AssetId;
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
        attesters::AttestersReadApi,
        light_client::LightClientHeartbeat,
        portal::Portal,
        xdns::{FullGatewayRecord, GatewayRecord, PalletAssetsOverlay, TokenRecord, Xdns},
        Bytes, ChainId, ExecutionVendor, GatewayType, GatewayVendor, TokenInfo, TreasuryAccount,
        TreasuryAccountProvider,
    };
    use t3rn_types::{fsx::TargetId, sfx::Sfx4bId};

    use t3rn_types::{fsx::FullSideEffect, sfx::SecurityLvl};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Type representing the weight of this pallet
        type WeightInfo: weights::WeightInfo;

        /// A type that provides inspection and mutation to some fungible assets
        type Balances: Inspect<Self::AccountId> + Mutate<Self::AccountId>;

        type Currency: Currency<Self::AccountId>;

        type AssetsOverlay: PalletAssetsOverlay<Self, BalanceOf<Self>>;

        type Portal: Portal<Self>;

        type AttestersRead: AttestersReadApi<Self::AccountId, BalanceOf<Self>>;

        type TreasuryAccounts: TreasuryAccountProvider<Self::AccountId>;

        type SelfTokenId: Get<AssetId>;

        type SelfGatewayId: Get<ChainId>;

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
        /// Re-adds the self-gateway if was present before. Inserts if wasn't. Root only access.
        #[pallet::weight(< T as Config >::WeightInfo::reboot_self_gateway())]
        pub fn reboot_self_gateway(
            origin: OriginFor<T>,
            vendor: GatewayVendor,
        ) -> DispatchResultWithPostInfo {
            Self::do_reboot_self_gateway(origin, vendor)?;

            Ok(().into())
        }

        /// Removes a gateway from the onchain registry. Root only access.
        #[pallet::weight(< T as Config >::WeightInfo::purge_gateway())]
        pub fn purge_gateway_record(
            origin: OriginFor<T>,
            requester: T::AccountId,
            gateway_id: TargetId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            if !<Gateways<T>>::contains_key(gateway_id) {
                Err(Error::<T>::XdnsRecordNotFound.into())
            } else {
                <Gateways<T>>::remove(gateway_id);

                let token_ids = GatewayTokens::<T>::get(gateway_id);

                token_ids.iter().for_each(|token_id| {
                    <Tokens<T>>::remove(token_id, gateway_id);
                    if gateway_id == T::SelfGatewayId::get() {
                        <AllTokenIds<T>>::mutate(|all_token_ids| {
                            all_token_ids.retain(|id| id != token_id);
                        });
                    }
                });

                <GatewayTokens<T>>::remove(gateway_id);

                <AllGatewayIds<T>>::mutate(|all_gateway_ids| {
                    all_gateway_ids.retain(|&id| id != gateway_id);
                });

                Self::deposit_event(Event::<T>::GatewayRecordPurged(requester, gateway_id));
                Ok(().into())
            }
        }

        #[pallet::weight(< T as Config >::WeightInfo::purge_gateway())]
        pub fn unlink_token(
            origin: OriginFor<T>,
            gateway_id: TargetId,
            token_id: AssetId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            <Tokens<T>>::remove(token_id, gateway_id);

            <GatewayTokens<T>>::mutate(gateway_id, |token_ids| {
                token_ids.retain(|&x_token_id| x_token_id != token_id);
            });

            Ok(().into())
        }

        /// Removes from all of the registered destinations + the onchain registry. Root only access.
        #[pallet::weight(< T as Config >::WeightInfo::purge_gateway())]
        pub fn purge_token_record(
            origin: OriginFor<T>,
            token_id: AssetId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin.clone())?;

            T::AssetsOverlay::destroy(origin, &token_id)?;

            // Remove from all destinations
            let destinations = <Tokens<T>>::iter_prefix(token_id)
                .map(|(dest, _)| dest)
                .collect::<Vec<_>>();
            destinations.iter().for_each(|dest| {
                <Tokens<T>>::remove(token_id, *dest);
            });

            <AllTokenIds<T>>::mutate(|all_token_ids| {
                all_token_ids.retain(|&id| id != token_id);
            });

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// \[gateway_4b_id\]
        GatewayRecordStored(TargetId),
        /// \[asset_id, gateway_4b_id\]
        NewTokenLinkedToGateway(AssetId, TargetId),
        /// \[asset_id, gateway_4b_id\]
        NewTokenAssetRegistered(AssetId, TargetId),
        /// \[requester, gateway_record_id\]
        GatewayRecordPurged(T::AccountId, TargetId),
        /// \[requester, xdns_record_id\]
        XdnsRecordPurged(T::AccountId, TargetId),
        /// \[xdns_record_id\]
        XdnsRecordUpdated(TargetId),
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
        /// XDNS Token not found in assets overlay
        TokenRecordNotFoundInAssetsOverlay,
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
        /// Gateway verified as inactive
        GatewayNotActive,
    }

    // Deprecated storage entry -- StandardSideEffects
    // Storage Migration: StandardSideEffects -> StandardSFXABIs
    // Storage Migration Details: 16-03-2023; v1.4.0-rc -> v1.5.0-rc
    #[pallet::storage]
    pub type StandardSideEffects<T: Config> = StorageMap<_, Identity, TargetId, Vec<u8>>; // SideEffectInterface

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
        StorageMap<_, Identity, TargetId, GatewayRecord<T::AccountId>, OptionQuery>;

    // Token can be stored in multiple gateways and on each Gateway be mapped to a different TokenRecord (Substrate, Eth etc.)
    #[pallet::storage]
    #[pallet::getter(fn tokens)]
    pub type Tokens<T: Config> =
        StorageDoubleMap<_, Identity, AssetId, Identity, TargetId, TokenRecord, OptionQuery>;

    // Recover TokenRecords stored per gateway, to be able to iterate over all tokens stored on a gateway
    #[pallet::storage]
    #[pallet::getter(fn gateway_tokens)]
    pub type GatewayTokens<T: Config> = StorageMap<_, Identity, TargetId, Vec<AssetId>, ValueQuery>;

    // All known TokenIds to t3rn
    #[pallet::storage]
    #[pallet::getter(fn all_token_ids)]
    pub type AllTokenIds<T: Config> = StorageValue<_, Vec<AssetId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn all_gateway_ids)]
    pub type AllGatewayIds<T: Config> = StorageValue<_, Vec<TargetId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn per_target_asset_estimates)]
    pub type PerTargetAssetEstimates<T: Config> = StorageDoubleMap<
        _,
        Identity,
        TargetId,
        Identity,
        (AssetId, AssetId),
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn asset_estimates)]
    pub type AssetEstimates<T: Config> =
        StorageMap<_, Identity, (AssetId, AssetId), BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn asset_estimates_in_native)]
    pub type AssetEstimatesInNative<T: Config> =
        StorageMap<_, Identity, AssetId, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn asset_cost_estimates_in_native)]
    pub type AssetCostEstimatesInNative<T: Config> =
        StorageMap<_, Identity, AssetId, BalanceOf<T>, ValueQuery>;

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

    impl<T: Config> Pallet<T> {
        pub fn do_reboot_self_gateway(
            origin: OriginFor<T>,
            vendor: GatewayVendor,
        ) -> DispatchResult {
            let admin: T::AccountId = ensure_signed_or_root(origin)?.unwrap_or(
                T::TreasuryAccounts::get_treasury_account(TreasuryAccount::Escrow),
            );

            let target_id = T::SelfGatewayId::get();

            const BALANCES_INDEX: u8 = 10;
            const ASSETS_INDEX: u8 = 12;
            const EVM_INDEX: u8 = 120;
            const WASM_INDEX: u8 = 121;

            let mut allowed_side_effects = vec![];

            if <StandardSFXABIs<T>>::contains_key(*b"tran") {
                allowed_side_effects.push((*b"tran", Some(BALANCES_INDEX)));
            }
            if <StandardSFXABIs<T>>::contains_key(*b"tass") {
                allowed_side_effects.push((*b"tran", Some(ASSETS_INDEX)));
            }
            if <StandardSFXABIs<T>>::contains_key(*b"swap") {
                allowed_side_effects.push((*b"swap", Some(BALANCES_INDEX)));
            }
            if <StandardSFXABIs<T>>::contains_key(*b"aliq") {
                allowed_side_effects.push((*b"aliq", Some(BALANCES_INDEX)));
            }
            if <StandardSFXABIs<T>>::contains_key(*b"rliq") {
                allowed_side_effects.push((*b"rliq", Some(BALANCES_INDEX)));
            }
            if <StandardSFXABIs<T>>::contains_key(*b"cevm") {
                allowed_side_effects.push((*b"cevm", Some(EVM_INDEX)));
            }
            if <StandardSFXABIs<T>>::contains_key(*b"wasm") {
                allowed_side_effects.push((*b"wasm", Some(WASM_INDEX)));
            }

            Pallet::<T>::override_gateway(
                target_id,
                vendor,
                ExecutionVendor::Substrate,
                Codec::Scale,
                Some(admin),
                None,
                allowed_side_effects,
            )
        }
    }
    impl<T: Config> Xdns<T, BalanceOf<T>> for Pallet<T> {
        /// Fetches all known Gateway records
        fn fetch_gateways() -> Vec<GatewayRecord<T::AccountId>> {
            Gateways::<T>::iter_values().collect()
        }

        /// Register new token assuming self::SelfGatewayIdOptimistic as a base chain
        fn register_new_token(
            origin: &OriginFor<T>,
            token_id: AssetId,
            token_props: TokenInfo,
        ) -> DispatchResult {
            if T::AssetsOverlay::contains_asset(&token_id) {
                return Err(Error::<T>::TokenRecordAlreadyExists.into())
            }

            let admin: T::AccountId = ensure_signed_or_root(origin.clone())?.unwrap_or(
                T::TreasuryAccounts::get_treasury_account(TreasuryAccount::Escrow),
            );

            T::AssetsOverlay::force_create_asset(
                origin.clone(),
                token_id,
                admin,
                true,
                T::Currency::minimum_balance(),
            )?;

            let gateway_id = T::SelfGatewayId::get();

            Self::link_token_to_gateway(token_id, gateway_id, token_props)?;

            Self::deposit_event(Event::<T>::NewTokenAssetRegistered(token_id, gateway_id));

            Ok(())
        }

        // Link existing token to a gateway. Assume that the token is already registered in the assets overlay via register_new_token
        fn link_token_to_gateway(
            token_id: AssetId,
            gateway_id: TargetId,
            token_props: TokenInfo,
        ) -> DispatchResult {
            // fetch record and ensure it exists
            let record = <Gateways<T>>::get(gateway_id).ok_or(Error::<T>::GatewayRecordNotFound)?;

            // early exit if record already exists in storage
            if <Tokens<T>>::contains_key(token_id, gateway_id) {
                return Err(Error::<T>::TokenRecordAlreadyExists.into())
            }

            ensure!(
                T::AssetsOverlay::contains_asset(&token_id),
                Error::<T>::TokenRecordNotFoundInAssetsOverlay
            );

            Self::override_token(token_id, gateway_id, token_props)
        }

        fn override_token(
            token_id: AssetId,
            gateway_id: TargetId,
            token_props: TokenInfo,
        ) -> DispatchResult {
            <Tokens<T>>::insert(
                token_id,
                gateway_id,
                TokenRecord {
                    token_id,
                    gateway_id,
                    token_props,
                },
            );

            <GatewayTokens<T>>::mutate(gateway_id, |tokens| {
                if !tokens.contains(&token_id) {
                    tokens.push(token_id);
                }
            });

            Self::deposit_event(Event::<T>::NewTokenLinkedToGateway(token_id, gateway_id));
            Ok(())
        }

        fn add_new_gateway(
            gateway_id: TargetId,
            verification_vendor: GatewayVendor,
            execution_vendor: ExecutionVendor,
            codec: Codec,
            registrant: Option<T::AccountId>,
            escrow_account: Option<T::AccountId>,
            allowed_side_effects: Vec<(TargetId, Option<u8>)>,
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
            gateway_id: TargetId,
            verification_vendor: GatewayVendor,
            execution_vendor: ExecutionVendor,
            codec: Codec,
            registrant: Option<T::AccountId>,
            escrow_account: Option<T::AccountId>,
            allowed_side_effects: Vec<(TargetId, Option<u8>)>,
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
            <AllGatewayIds<T>>::mutate(|ids| ids.push(gateway_id));

            Self::deposit_event(Event::<T>::GatewayRecordStored(gateway_id));

            Ok(())
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

        /// returns a mapping of all allowed side_effects of a gateway.
        fn allowed_side_effects(gateway_id: &ChainId) -> Vec<(Sfx4bId, Option<u8>)> {
            match <Gateways<T>>::get(gateway_id) {
                Some(gateway) => gateway.allowed_side_effects,
                None => Vec::new(),
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

        fn verify_active(
            gateway_id: &ChainId,
            max_acceptable_heartbeat_offset: T::BlockNumber,
            security_lvl: &SecurityLvl,
        ) -> Result<LightClientHeartbeat<T>, DispatchError> {
            let heartbeat = T::Portal::get_latest_heartbeat(gateway_id)
                .map_err(|_| Error::<T>::GatewayNotActive)?;

            if heartbeat.is_halted
                || !heartbeat.ever_initialized
                || max_acceptable_heartbeat_offset
                    > frame_system::Pallet::<T>::block_number()
                        .saturating_sub(heartbeat.last_heartbeat)
            {
                return Err(Error::<T>::GatewayNotActive.into())
            }

            if security_lvl == &SecurityLvl::Escrow {
                T::AttestersRead::get_activated_targets()
                    .iter()
                    .find(|target| target == &gateway_id)
                    .ok_or(Error::<T>::GatewayNotActive)?;
            }

            Ok(heartbeat)
        }

        fn estimate_costs(_fsx: &Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>) {
            todo!("estimate costs")
        }
    }
}
