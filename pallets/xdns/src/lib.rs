//! <!-- markdown-link-check-disable -->
//! # X-DNS Pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use codec::Encode;
use sp_runtime::traits::Hash;
use sp_std::prelude::*;
pub use t3rn_types::{
    abi::{GatewayABIConfig, Type},
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
    use t3rn_primitives::{
        xdns::{AllowedSideEffect, Parachain, Xdns, XdnsRecord},
        Bytes, ChainId, GatewaySysProps, GatewayType, GatewayVendor,
    };
    use t3rn_types::{
        fsx::{SecurityLvl, TargetId},
        interface::SideEffectInterface,
        sfx::{Sfx4bId, SfxExpectedDescriptor},
    };

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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(< T as Config >::WeightInfo::add_new_xdns_record())]
        pub fn add_side_effect(
            origin: OriginFor<T>,
            id: [u8; 4],
            name: SideEffectName,
            argument_abi: Vec<Type>,
            argument_to_state_mapper: Vec<EventSignature>,
            confirm_events: Vec<EventSignature>,
            escrowed_events: Vec<EventSignature>,
            commit_events: Vec<EventSignature>,
            revert_events: Vec<EventSignature>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            let side_effect_id: SideEffectId<T> = T::Hashing::hash(&id.encode());

            if <CustomSideEffects<T>>::contains_key(side_effect_id)
                | <StandardSideEffects<T>>::contains_key(id)
            {
                return Err(Error::<T>::SideEffectInterfaceAlreadyExists.into())
            }

            let side_effect = SideEffectInterface {
                id,
                name,
                argument_abi,
                argument_to_state_mapper,
                confirm_events,
                escrowed_events,
                commit_events,
                revert_events,
            };

            <CustomSideEffects<T>>::insert(side_effect_id, side_effect);

            Ok(().into())
        }

        /// Updates the last_finalized field for an xdns_record from the onchain registry. Root only access.
        #[pallet::weight(< T as Config >::WeightInfo::update_ttl())]
        pub fn update_ttl(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            last_finalized: u64,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            Self::update_gateway_ttl(gateway_id, last_finalized)
        }

        /// Removes a xdns_record from the onchain registry. Root only access.
        #[pallet::weight(< T as Config >::WeightInfo::purge_xdns_record())]
        pub fn purge_xdns_record(
            origin: OriginFor<T>,
            requester: T::AccountId,
            xdns_record_id: [u8; 4],
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            if !<XDNSRegistry<T>>::contains_key(xdns_record_id) {
                Err(Error::<T>::UnknownXdnsRecord.into())
            } else {
                <XDNSRegistry<T>>::remove(xdns_record_id);
                Self::deposit_event(Event::<T>::XdnsRecordPurged(requester, xdns_record_id));
                Ok(().into())
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// \[xdns_record_id\]
        XdnsRecordStored([u8; 4]),
        /// \[requester, xdns_record_id\]
        XdnsRecordPurged(T::AccountId, [u8; 4]),
        /// \[xdns_record_id\]
        XdnsRecordUpdated([u8; 4]),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Stored xdns_record has already been added before
        XdnsRecordAlreadyExists,
        /// Access of unknown xdns_record
        UnknownXdnsRecord,
        /// Xdns Record not found
        XdnsRecordNotFound,
        /// Xdns Record not found
        SideEffectDescriptorAlreadyExists,
        /// SideEffect already stored
        SideEffectInterfaceAlreadyExists,
        /// SideEffect interface was not found in storage
        SideEffectInterfaceNotFound,
        /// the xdns entry does not contain parachain information
        NoParachainInfoFound,
    }

    #[pallet::storage]
    pub type StandardSideEffects<T: Config> =
        StorageMap<_, Identity, TargetId, SideEffectInterface>;

    #[pallet::storage]
    pub type SeenSideEffects<T: Config> =
        StorageDoubleMap<_, Identity, TargetId, Identity, Sfx4bId, SfxExpectedDescriptor>;

    #[pallet::storage]
    #[pallet::getter(fn side_effect_registry)]
    pub type CustomSideEffects<T> = StorageMap<_, Identity, SideEffectId<T>, SideEffectInterface>;

    /// The pre-validated composable xdns_records on-chain registry.
    #[pallet::storage]
    #[pallet::getter(fn xdns_registry)]
    pub type XDNSRegistry<T: Config> =
        StorageMap<_, Identity, [u8; 4], XdnsRecord<T::AccountId>, OptionQuery>;

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub known_xdns_records: Vec<XdnsRecord<T::AccountId>>,
        pub standard_side_effects: Vec<SideEffectInterface>,
    }

    /// The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                known_xdns_records: Default::default(),
                standard_side_effects: Default::default(),
            }
        }
    }

    /// The build of genesis for the pallet.
    /// Populates storage with the known XDNS Records
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let _standard_enabled_side_effects: Vec<AllowedSideEffect> = self
                .standard_side_effects
                .iter()
                .map(|s| s.get_id())
                .collect();
            for xdns_record in self.known_xdns_records.clone() {
                <XDNSRegistry<T>>::insert(xdns_record.gateway_id, xdns_record);
            }

            for side_effect in self.standard_side_effects.clone() {
                <StandardSideEffects<T>>::insert(side_effect.get_id(), side_effect);
            }
        }
    }

    impl<T: Config> Xdns<T> for Pallet<T> {
        /// Fetches all known XDNS records
        fn fetch_records() -> Vec<XdnsRecord<T::AccountId>> {
            pallet::XDNSRegistry::<T>::iter_values().collect()
        }

        fn add_new_xdns_record(
            origin: OriginFor<T>,
            url: Vec<u8>,
            gateway_id: ChainId,
            parachain: Option<Parachain>,
            gateway_abi: GatewayABIConfig,
            gateway_vendor: GatewayVendor,
            gateway_type: GatewayType,
            gateway_genesis: GatewayGenesisConfig,
            gateway_sys_props: GatewaySysProps,
            security_coordinates: Vec<u8>,
            allowed_side_effects: Vec<AllowedSideEffect>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // early exit if record already exists in storage
            if <XDNSRegistry<T>>::contains_key(gateway_id) {
                return Err(Error::<T>::XdnsRecordAlreadyExists.into())
            }

            // TODO: check if side_effect exists
            let mut xdns_record = XdnsRecord::<T::AccountId>::new(
                url,
                gateway_id,
                parachain,
                gateway_abi,
                gateway_vendor,
                gateway_type,
                gateway_genesis,
                gateway_sys_props,
                security_coordinates,
                allowed_side_effects,
            );

            // ToDo: Uncomment when switching into a model with open registration. Sudo access for now.
            // xdns_record.assign_registrant(registrant.clone());
            let now = TryInto::<u64>::try_into(<T as Config>::Time::now())
                .map_err(|_| "Unable to compute current timestamp")?;

            xdns_record.set_last_finalized(now);
            <XDNSRegistry<T>>::insert(gateway_id, xdns_record);
            Self::deposit_event(Event::<T>::XdnsRecordStored(gateway_id));
            Ok(())
        }

        /// returns a mapping of all allowed side_effects of a gateway.
        fn allowed_side_effects(gateway_id: &ChainId) -> Vec<AllowedSideEffect> {
            match <XDNSRegistry<T>>::get(gateway_id) {
                Some(xdns_record) => xdns_record.allowed_side_effects,
                None => Vec::new(),
            }
        }

        fn update_gateway_ttl(
            gateway_id: ChainId,
            last_finalized: u64,
        ) -> DispatchResultWithPostInfo {
            if !XDNSRegistry::<T>::contains_key(gateway_id) {
                Err(Error::<T>::XdnsRecordNotFound.into())
            } else {
                XDNSRegistry::<T>::mutate(gateway_id, |xdns_record| match xdns_record {
                    None => Err(Error::<T>::XdnsRecordNotFound),
                    Some(record) => {
                        record.set_last_finalized(last_finalized);
                        Ok(())
                    },
                })?;

                Self::deposit_event(Event::<T>::XdnsRecordUpdated(gateway_id));
                Ok(().into())
            }
        }

        // Fetches the GatewayABIConfig for a given XDNS record
        fn get_abi(chain_id: ChainId) -> Result<GatewayABIConfig, DispatchError> {
            if !<XDNSRegistry<T>>::contains_key(chain_id) {
                return Err(Error::<T>::XdnsRecordNotFound.into())
            }
            Ok(<XDNSRegistry<T>>::get(chain_id).unwrap().gateway_abi) //safe because checked
        }

        fn get_gateway_value_unsigned_type_unsafe(chain_id: &ChainId) -> Type {
            Type::Uint(
                if let Some(v) = <XDNSRegistry<T>>::get(chain_id)
                    .unwrap()
                    .gateway_abi
                    .value_type_size
                    .checked_mul(8)
                {
                    v
                } else {
                    // return default value
                    0
                },
            )
        }

        /// returns the gateway vendor of a gateway if its available
        fn get_gateway_vendor(chain_id: &ChainId) -> Result<GatewayVendor, DispatchError> {
            match <XDNSRegistry<T>>::get(chain_id) {
                Some(rec) => Ok(rec.gateway_vendor),
                None => Err(Error::<T>::XdnsRecordNotFound.into()),
            }
        }

        fn get_gateway_security_coordinates(chain_id: &ChainId) -> Result<Bytes, DispatchError> {
            match <XDNSRegistry<T>>::get(chain_id) {
                Some(rec) => Ok(rec.security_coordinates),
                None => Err(Error::<T>::XdnsRecordNotFound.into()),
            }
        }

        fn get_gateway_para_id(chain_id: &ChainId) -> Result<u32, DispatchError> {
            match <XDNSRegistry<T>>::get(chain_id) {
                Some(rec) => match rec.parachain {
                    Some(entry) => Ok(entry.id),
                    None => Err(Error::<T>::NoParachainInfoFound.into()),
                },
                None => Err(Error::<T>::XdnsRecordNotFound.into()),
            }
        }

        fn get_gateway_type_unsafe(chain_id: &ChainId) -> GatewayType {
            <XDNSRegistry<T>>::get(chain_id).unwrap().gateway_type
        }

        fn extend_optimistic_sfx_descriptor(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            sfx_4b_id: Sfx4bId,
            sfx_expected_descriptor: SfxExpectedDescriptor,
        ) -> DispatchResult {
            ensure_root(origin)?;
            if !<XDNSRegistry<T>>::contains_key(gateway_id) {
                return Err(Error::<T>::XdnsRecordNotFound.into())
            }

            <SeenSideEffects<T>>::mutate(gateway_id, sfx_4b_id, |sfx_descriptor| {
                match sfx_descriptor {
                    Some(_) => Err(Error::<T>::SideEffectDescriptorAlreadyExists),
                    None => {
                        *sfx_descriptor = Some(sfx_expected_descriptor);
                        Ok(())
                    },
                }
            })?;

            Ok(())
        }

        fn override_optimistic_sfx_descriptor(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            new_sfx_descriptors: Vec<(Sfx4bId, SfxExpectedDescriptor)>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            if !<XDNSRegistry<T>>::contains_key(gateway_id) {
                return Err(Error::<T>::XdnsRecordNotFound.into())
            }

            for (sfx_4b_id, sfx_expected_descriptor) in new_sfx_descriptors {
                <SeenSideEffects<T>>::mutate(gateway_id, sfx_4b_id, |sfx_descriptor| {
                    *sfx_descriptor = Some(sfx_expected_descriptor);
                });
            }

            Ok(())
        }

        fn get_optimistic_sfx_descriptors(
            gateway_id: &ChainId,
        ) -> Vec<(Sfx4bId, SfxExpectedDescriptor)> {
            <SeenSideEffects<T>>::iter_prefix(gateway_id)
                .map(|(sfx_4b_id, sfx_descriptor)| (sfx_4b_id, sfx_descriptor))
                .collect()
        }

        fn get_optimistic_sfx_descriptor(
            gateway_id: &ChainId,
            sfx_4b_id: Sfx4bId,
        ) -> Option<SfxExpectedDescriptor> {
            <SeenSideEffects<T>>::get(gateway_id, sfx_4b_id)
        }

        fn modify_security_level(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            _security_level: SecurityLvl,
            security_coordinates: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            XDNSRegistry::<T>::mutate(gateway_id, |xdns_record| match xdns_record {
                None => Err(Error::<T>::XdnsRecordNotFound),
                Some(record) => {
                    record.security_coordinates = security_coordinates;
                    Ok(())
                },
            })?;

            Ok(())
        }
    }
}
