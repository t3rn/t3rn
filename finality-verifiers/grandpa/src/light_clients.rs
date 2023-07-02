#![cfg_attr(not(feature = "std"), no_std)]

use crate::{to_local_block_number, Config, Pallet};
use codec::Encode;

use frame_system::pallet_prelude::OriginFor;
use num_traits::Zero;
use sp_runtime::{traits::Header, DispatchError};
use sp_std::{marker::PhantomData, vec};
use t3rn_abi::types::Bytes;
pub use t3rn_primitives::light_client::{LightClient, LightClientHeartbeat};
use t3rn_primitives::{
    light_client::{HeaderResult, HeightResult, InclusionReceipt},
    ExecutionSource, GatewayVendor, SpeedMode,
};

pub type RococoInstance = ();
pub type KusamaInstance = crate::pallet::Instance1;
pub type PolkadotInstance = crate::pallet::Instance2;

pub type RococoPallet<T> = Pallet<T, RococoInstance>;
pub type KusamaPallet<T> = Pallet<T, KusamaInstance>;
pub type PolkadotPallet<T> = Pallet<T, PolkadotInstance>;

pub enum PalletInstance<T, I: 'static = ()> {
    Rococo(RococoPallet<T>),
    Kusama(KusamaPallet<T>),
    Polkadot(PolkadotPallet<T>),
    Phantom(PhantomData<I>),
}

pub fn grab_lc_instance_unsafe<T, I: 'static>(vendor: GatewayVendor) -> PalletInstance<T, I>
where
    T: Config<RococoInstance> + Config<KusamaInstance> + Config<PolkadotInstance>,
{
    select_grandpa_light_client_instance(vendor).unwrap()
}

pub fn select_grandpa_light_client_instance<T, I: 'static>(
    vendor: GatewayVendor,
) -> Option<PalletInstance<T, I>>
where
    T: Config<RococoInstance> + Config<KusamaInstance> + Config<PolkadotInstance>,
{
    match vendor {
        GatewayVendor::Rococo => Some(PalletInstance::Rococo(Pallet::<T, ()>(PhantomData))),
        GatewayVendor::Kusama => Some(PalletInstance::Kusama(Pallet::<T, KusamaInstance>(
            PhantomData,
        ))),
        GatewayVendor::Polkadot => Some(PalletInstance::Polkadot(Pallet::<T, PolkadotInstance>(
            PhantomData,
        ))),
        _ => None,
    }
}

impl<T, I: 'static> LightClient<T> for PalletInstance<T, I>
where
    T: Config<RococoInstance> + Config<KusamaInstance> + Config<PolkadotInstance> + Config<I>,
{
    fn get_latest_finalized_header(&self) -> HeaderResult {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_latest_finalized_header(),
            PalletInstance::Kusama(pallet) => pallet.get_latest_finalized_header(),
            PalletInstance::Polkadot(pallet) => pallet.get_latest_finalized_header(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_finalized_height(&self) -> HeightResult<T::BlockNumber> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_finalized_height(),
            PalletInstance::Kusama(pallet) => pallet.get_finalized_height(),
            PalletInstance::Polkadot(pallet) => pallet.get_finalized_height(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_rational_height(&self) -> HeightResult<T::BlockNumber> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_finalized_height(),
            PalletInstance::Kusama(pallet) => pallet.get_finalized_height(),
            PalletInstance::Polkadot(pallet) => pallet.get_finalized_height(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_fast_height(&self) -> HeightResult<T::BlockNumber> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_finalized_height(),
            PalletInstance::Kusama(pallet) => pallet.get_finalized_height(),
            PalletInstance::Polkadot(pallet) => pallet.get_finalized_height(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_latest_heartbeat(&self) -> Result<LightClientHeartbeat<T>, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_latest_heartbeat(),
            PalletInstance::Kusama(pallet) => pallet.get_latest_heartbeat(),
            PalletInstance::Polkadot(pallet) => pallet.get_latest_heartbeat(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_latest_finalized_header_precompile(&self) -> Bytes {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_latest_finalized_header_precompile(),
            PalletInstance::Kusama(pallet) => pallet.get_latest_finalized_header_precompile(),
            PalletInstance::Polkadot(pallet) => pallet.get_latest_finalized_header_precompile(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_finalized_height_precompile(&self) -> T::BlockNumber {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Kusama(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Polkadot(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_rational_height_precompile(&self) -> T::BlockNumber {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Kusama(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Polkadot(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_fast_height_precompile(&self) -> T::BlockNumber {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Kusama(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Polkadot(pallet) => pallet.get_finalized_height_precompile(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn initialize(
        &self,
        origin: OriginFor<T>,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.initialize(origin, gateway_id, encoded_registration_data),
            PalletInstance::Kusama(pallet) =>
                pallet.initialize(origin, gateway_id, encoded_registration_data),
            PalletInstance::Polkadot(pallet) =>
                pallet.initialize(origin, gateway_id, encoded_registration_data),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn turn_on(&self, origin: OriginFor<T>) -> Result<bool, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.turn_on(origin),
            PalletInstance::Kusama(pallet) => pallet.turn_on(origin),
            PalletInstance::Polkadot(pallet) => pallet.turn_on(origin),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn turn_off(&self, origin: OriginFor<T>) -> Result<bool, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.turn_off(origin),
            PalletInstance::Kusama(pallet) => pallet.turn_off(origin),
            PalletInstance::Polkadot(pallet) => pallet.turn_off(origin),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn submit_encoded_headers(&self, encoded_headers_data: Bytes) -> Result<bool, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.submit_encoded_headers(encoded_headers_data),
            PalletInstance::Kusama(pallet) => pallet.submit_encoded_headers(encoded_headers_data),
            PalletInstance::Polkadot(pallet) => pallet.submit_encoded_headers(encoded_headers_data),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_event_inclusion(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: Option<ExecutionSource>,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_event_inclusion(gateway_id, speed_mode, source, message),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_event_inclusion(gateway_id, speed_mode, source, message),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_event_inclusion(gateway_id, speed_mode, source, message),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_state_inclusion(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_state_inclusion(gateway_id, speed_mode, message),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_state_inclusion(gateway_id, speed_mode, message),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_state_inclusion(gateway_id, speed_mode, message),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_tx_inclusion(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_tx_inclusion(gateway_id, speed_mode, message),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_tx_inclusion(gateway_id, speed_mode, message),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_tx_inclusion(gateway_id, speed_mode, message),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_event_inclusion_precompile(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: ExecutionSource,
        message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_event_inclusion_precompile(gateway_id, speed_mode, source, message),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_event_inclusion_precompile(gateway_id, speed_mode, source, message),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_event_inclusion_precompile(gateway_id, speed_mode, source, message),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_state_inclusion_precompile(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_state_inclusion_precompile(gateway_id, speed_mode, message),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_state_inclusion_precompile(gateway_id, speed_mode, message),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_state_inclusion_precompile(gateway_id, speed_mode, message),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_tx_inclusion_precompile(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_tx_inclusion_precompile(gateway_id, speed_mode, message),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_tx_inclusion_precompile(gateway_id, speed_mode, message),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_tx_inclusion_precompile(gateway_id, speed_mode, message),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }
}

impl<T: Config<I>, I: 'static> LightClient<T> for Pallet<T, I> {
    fn get_latest_finalized_header(&self) -> HeaderResult {
        match Pallet::<T, I>::get_best_block_hash() {
            Some(header) => HeaderResult::Header(header.encode()),
            None => HeaderResult::NotActive,
        }
    }

    fn get_fast_height(&self) -> HeightResult<T::BlockNumber> {
        self.get_finalized_height()
    }

    fn get_rational_height(&self) -> HeightResult<T::BlockNumber> {
        self.get_finalized_height()
    }

    fn get_finalized_height(&self) -> HeightResult<T::BlockNumber> {
        let header = Pallet::<T, I>::best_finalized_map();
        let local_number = match to_local_block_number::<T, I>(*header.number()) {
            Ok(number) => number,
            Err(_) => return HeightResult::NotActive,
        };
        HeightResult::Height(local_number)
    }

    fn get_latest_finalized_header_precompile(&self) -> Bytes {
        match Pallet::<T, I>::get_best_block_hash() {
            Some(header) => header.encode(),
            None => vec![],
        }
    }

    fn get_fast_height_precompile(&self) -> T::BlockNumber {
        match self.get_finalized_height() {
            HeightResult::Height(height) => height,
            HeightResult::NotActive => T::BlockNumber::zero(),
        }
    }

    fn get_rational_height_precompile(&self) -> T::BlockNumber {
        match self.get_finalized_height() {
            HeightResult::Height(height) => height,
            HeightResult::NotActive => T::BlockNumber::zero(),
        }
    }

    fn get_finalized_height_precompile(&self) -> T::BlockNumber {
        match self.get_finalized_height() {
            HeightResult::Height(height) => height,
            HeightResult::NotActive => T::BlockNumber::zero(),
        }
    }

    fn get_latest_heartbeat(&self) -> Result<LightClientHeartbeat<T>, DispatchError> {
        let header = Pallet::<T, I>::best_finalized_map();
        let last_finalized_height = to_local_block_number::<T, I>(*header.number())?;
        Ok(LightClientHeartbeat {
            last_heartbeat: frame_system::Pallet::<T>::block_number(),
            last_finalized_height,
            last_rational_height: last_finalized_height,
            last_fast_height: last_finalized_height,
            is_halted: Pallet::<T, I>::is_halted(),
            ever_initialized: Pallet::<T, I>::ever_initialized(),
        })
    }

    fn initialize(
        &self,
        origin: OriginFor<T>,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError> {
        Pallet::<T, I>::initialize(origin, gateway_id, encoded_registration_data)
            .map_err(|str_err| str_err.into())
    }

    fn turn_on(&self, origin: OriginFor<T>) -> Result<bool, DispatchError> {
        Pallet::<T, I>::set_operational(origin, true)?;
        Ok(!Pallet::<T, I>::is_halted())
    }

    fn turn_off(&self, origin: OriginFor<T>) -> Result<bool, DispatchError> {
        Pallet::<T, I>::set_operational(origin, false)?;
        Ok(!Pallet::<T, I>::is_halted())
    }

    fn submit_encoded_headers(&self, headers: Bytes) -> Result<bool, DispatchError> {
        Pallet::<T, I>::submit_encoded_headers(headers)?;
        Ok(true)
    }

    fn verify_event_inclusion(
        &self,
        gateway_id: [u8; 4],
        _speed_mode: SpeedMode,
        _source: Option<ExecutionSource>,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        // todo: handle ExecutionSource in grandpa
        Pallet::<T, I>::confirm_event_inclusion(gateway_id, message)
    }

    fn verify_state_inclusion(
        &self,
        _gateway_id: [u8; 4],
        _speed_mode: SpeedMode,
        _message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        Err("GrandpaFV::verify_storage_inclusion not implemented yet".into())
    }

    fn verify_tx_inclusion(
        &self,
        _gateway_id: [u8; 4],
        _speed_mode: SpeedMode,
        _message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        Err("GrandpaFV::verify_tx_inclusion not implemented yet".into())
    }

    fn verify_event_inclusion_precompile(
        &self,
        gateway_id: [u8; 4],
        _speed_mode: SpeedMode,
        _source: ExecutionSource,
        message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        match Pallet::<T, I>::confirm_event_inclusion(gateway_id, message) {
            Ok(receipt) => Ok(receipt.message.encode()),
            Err(err) => Err(err),
        }
    }

    fn verify_state_inclusion_precompile(
        &self,
        _gateway_id: [u8; 4],
        _speed_mode: SpeedMode,
        _message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        Err("GrandpaFV::verify_storage_inclusion not implemented yet".into())
    }

    fn verify_tx_inclusion_precompile(
        &self,
        _gateway_id: [u8; 4],
        _speed_mode: SpeedMode,
        _message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        Err("GrandpaFV::verify_tx_inclusion not implemented yet".into())
    }
}

#[cfg(all(feature = "testing"))]
pub mod grandpa_light_clients_test {
    use super::*;
    use codec::Encode;

    use crate::{bridges::test_utils::authorities, mock::*, types::RelaychainRegistrationData};
    use frame_support::{assert_ok, traits::OriginTrait};

    use crate::{mock::TestRuntime, types::GrandpaHeaderData};
    use hex_literal::hex;

    fn prep_init_data() -> RelaychainRegistrationData<AccountId> {
        let genesis = test_header_with_correct_parent(0, None);

        RelaychainRegistrationData::<AccountId> {
            authorities: authorities(),
            first_header: genesis.encode(),
            authority_set_id: 1,
            owner: 1u64,
        }
    }

    pub fn stage_test_and_init_instance<
        T: frame_system::Config
            + Config
            + Config<I>
            + Config<KusamaInstance>
            + Config<PolkadotInstance>
            + Config<RococoInstance>,
        I: 'static,
    >(
        test: fn(),
        vendor: GatewayVendor,
        light_client_4b_id: [u8; 4],
    ) {
        run_test(|| {
            let BLOCK_ZERO: T::BlockNumber = T::BlockNumber::from(0u8);
            let BLOCK_ONE: T::BlockNumber = T::BlockNumber::from(1u8);

            frame_system::Pallet::<T>::set_block_number(BLOCK_ONE);
            let lc_instance = select_grandpa_light_client_instance::<T, I>(vendor).expect(
                "GatewayVendor::Rococo must be covered by select_grandpa_light_client_instance",
            );

            let heartbeat_before = lc_instance.get_latest_heartbeat().unwrap();
            assert_eq!(heartbeat_before.last_finalized_height, BLOCK_ZERO);
            assert_eq!(heartbeat_before.ever_initialized, false);
            assert_eq!(heartbeat_before.is_halted, false);

            let initialization_result = lc_instance.initialize(
                <T as frame_system::Config>::Origin::root(),
                light_client_4b_id,
                prep_init_data().encode(),
            );

            assert_ok!(initialization_result);

            let heartbeat_after = lc_instance.get_latest_heartbeat().unwrap();
            assert_eq!(heartbeat_after.last_finalized_height, BLOCK_ZERO);
            assert_eq!(heartbeat_after.ever_initialized, true);
            assert_eq!(heartbeat_after.is_halted, false);

            let latest_finalized = lc_instance.get_latest_finalized_header();

            assert_eq!(
                latest_finalized,
                HeaderResult::Header(
                    hex!("dcdd89927d8a348e00257e1ecc8617f45edb5118efff3ea2f9961b2ad9b7690a").into()
                )
            );

            test();
        });
    }

    #[test]
    fn given_rococo_instance_inits_light_client_with_mock_data() {
        stage_test_and_init_instance::<TestRuntime, RococoInstance>(
            || {},
            GatewayVendor::Rococo,
            [0, 0, 0, 0],
        );
    }

    #[test]
    fn given_kusama_instance_inits_light_client_with_mock_data() {
        stage_test_and_init_instance::<TestRuntime, KusamaInstance>(
            || {},
            GatewayVendor::Kusama,
            [0, 0, 0, 1],
        );
    }

    #[test]
    fn given_polkadot_instance_inits_light_client_with_mock_data() {
        stage_test_and_init_instance::<TestRuntime, PolkadotInstance>(
            || {},
            GatewayVendor::Polkadot,
            [0, 0, 0, 2],
        );
    }

    #[test]
    fn given_three_instances_initialized_returns_correct_heartbeats() {
        stage_test_and_init_instance::<TestRuntime, RococoInstance>(
            || {
                let roco_light_client =
                    grab_lc_instance_unsafe::<TestRuntime, RococoInstance>(GatewayVendor::Rococo);
                let ksm_light_client =
                    grab_lc_instance_unsafe::<TestRuntime, KusamaInstance>(GatewayVendor::Kusama);
                let dot_light_client = grab_lc_instance_unsafe::<TestRuntime, PolkadotInstance>(
                    GatewayVendor::Polkadot,
                );

                let roco_heartbeat = roco_light_client.get_latest_heartbeat().unwrap();
                assert_eq!(roco_heartbeat.ever_initialized, true);
                let ksm_heartbeat = ksm_light_client.get_latest_heartbeat().unwrap();
                assert_eq!(ksm_heartbeat.ever_initialized, false);
                let dot_heartbeat = dot_light_client.get_latest_heartbeat().unwrap();
                assert_eq!(dot_heartbeat.ever_initialized, false);

                stage_test_and_init_instance::<TestRuntime, RococoInstance>(
                    || {
                        let roco_light_client = grab_lc_instance_unsafe::<
                            TestRuntime,
                            RococoInstance,
                        >(GatewayVendor::Rococo);
                        let ksm_light_client = grab_lc_instance_unsafe::<TestRuntime, KusamaInstance>(
                            GatewayVendor::Kusama,
                        );
                        let dot_light_client = grab_lc_instance_unsafe::<
                            TestRuntime,
                            PolkadotInstance,
                        >(GatewayVendor::Polkadot);

                        let roco_heartbeat = roco_light_client.get_latest_heartbeat().unwrap();
                        assert_eq!(roco_heartbeat.ever_initialized, false);
                        let ksm_heartbeat = ksm_light_client.get_latest_heartbeat().unwrap();
                        assert_eq!(ksm_heartbeat.ever_initialized, true);
                        let dot_heartbeat = dot_light_client.get_latest_heartbeat().unwrap();
                        assert_eq!(dot_heartbeat.ever_initialized, false);

                        stage_test_and_init_instance::<TestRuntime, PolkadotInstance>(
                            || {
                                let roco_light_client =
                                    grab_lc_instance_unsafe::<TestRuntime, RococoInstance>(
                                        GatewayVendor::Rococo,
                                    );
                                let ksm_light_client =
                                    grab_lc_instance_unsafe::<TestRuntime, KusamaInstance>(
                                        GatewayVendor::Kusama,
                                    );
                                let dot_light_client =
                                    grab_lc_instance_unsafe::<TestRuntime, PolkadotInstance>(
                                        GatewayVendor::Polkadot,
                                    );

                                let roco_heartbeat =
                                    roco_light_client.get_latest_heartbeat().unwrap();
                                assert_eq!(roco_heartbeat.ever_initialized, false);
                                let ksm_heartbeat =
                                    ksm_light_client.get_latest_heartbeat().unwrap();
                                assert_eq!(ksm_heartbeat.ever_initialized, false);
                                let dot_heartbeat =
                                    dot_light_client.get_latest_heartbeat().unwrap();
                                assert_eq!(dot_heartbeat.ever_initialized, true);
                            },
                            GatewayVendor::Polkadot,
                            [0, 0, 0, 2],
                        );
                    },
                    GatewayVendor::Kusama,
                    [0, 0, 0, 1],
                );
            },
            GatewayVendor::Rococo,
            [0, 0, 0, 0],
        );
    }

    #[test]
    fn given_rococo_instance_can_submit_headers_range_1_5() {
        stage_test_and_init_instance::<TestRuntime, RococoInstance>(
            || {
                let roco_light_client =
                    grab_lc_instance_unsafe::<TestRuntime, RococoInstance>(GatewayVendor::Rococo);

                let headers_range = produce_mock_headers_range(1, 5);

                let submit_res = roco_light_client.submit_encoded_headers(headers_range.encode());

                assert_ok!(submit_res);

                let expected_header =
                    HeaderResult::Header(headers_range.signed_header.hash().encode());

                assert_eq!(
                    roco_light_client.get_latest_heartbeat().unwrap(),
                    LightClientHeartbeat {
                        last_heartbeat: 1,
                        last_finalized_height: 5,
                        last_rational_height: 5,
                        last_fast_height: 5,
                        is_halted: false,
                        ever_initialized: true
                    }
                );

                let actual_header = roco_light_client.get_latest_finalized_header();
                assert_eq!(actual_header, expected_header);
            },
            GatewayVendor::Rococo,
            [0, 0, 0, 0],
        );
    }

    #[test]
    fn initialize_works_for_default_rococo_instance_between_direct_access() {
        run_test(|| {
            frame_system::Pallet::<TestRuntime>::set_block_number(1);

            let rococo_instance =
                select_grandpa_light_client_instance::<TestRuntime, ()>(GatewayVendor::Rococo)
                    .expect(
                    "GatewayVendor::Rococo must be covered by select_grandpa_light_client_instance",
                );

            let heartbeat_before = rococo_instance.get_latest_heartbeat().unwrap();

            assert_eq!(heartbeat_before.last_finalized_height, 0);

            Pallet::<TestRuntime, ()>::initialize(
                Origin::root(),
                [0, 0, 0, 0],
                prep_init_data().encode(),
            )
            .unwrap();

            let expected_ever_initialized_direct_storage_read =
                crate::EverInitialized::<TestRuntime, ()>::get();
            let heartbeat_after = rococo_instance.get_latest_heartbeat().unwrap();

            assert_eq!(
                heartbeat_after.ever_initialized,
                expected_ever_initialized_direct_storage_read
            );
        });
    }

    #[test]
    fn turn_on_off_works_between_default_instance_and_direct_access_via_heartbeats() {
        run_test(|| {
            let rococo_light_client =
                select_grandpa_light_client_instance::<TestRuntime, ()>(GatewayVendor::Rococo)
                    .expect(
                    "GatewayVendor::Rococo must be covered by select_grandpa_light_client_instance",
                );

            let heartbeat = rococo_light_client.get_latest_heartbeat().unwrap();

            assert_eq!(heartbeat.is_halted, false);

            Pallet::<TestRuntime, ()>::set_operational(Origin::root(), false);

            let heartbeat = rococo_light_client.get_latest_heartbeat().unwrap();

            assert_eq!(heartbeat.is_halted, true);
        });
    }

    // returns the last header in encoded form
    pub fn insert_headers_range_to_roco(headers_range: GrandpaHeaderData<TestHeader>) -> Bytes {
        let roco_light_client =
            grab_lc_instance_unsafe::<TestRuntime, RococoInstance>(GatewayVendor::Rococo);

        let submit_res = roco_light_client.submit_encoded_headers(headers_range.encode());

        assert_ok!(submit_res);

        headers_range.signed_header.hash().encode()
    }
}
