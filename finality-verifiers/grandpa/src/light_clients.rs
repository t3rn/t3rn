#![cfg_attr(not(feature = "std"), no_std)]

use crate::{to_local_block_number, BridgedBlockHash, Config, Pallet};
use codec::Decode;
use frame_support::traits::Get;
use frame_system::pallet_prelude::OriginFor;
pub use t3rn_primitives::light_client::{LightClient, LightClientHeartbeat};

use crate::pallet::ImportedHeaders;
use sp_runtime::{traits::Header, DispatchError};
use sp_std::marker::PhantomData;
use t3rn_abi::types::Bytes;
use t3rn_primitives::{
    light_client::{HeaderResult, HeightResult, InclusionReceipt},
    GatewayVendor, SpeedMode,
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

    fn get_latest_finalized_height(&self) -> HeightResult<T::BlockNumber> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_latest_finalized_height(),
            PalletInstance::Kusama(pallet) => pallet.get_latest_finalized_height(),
            PalletInstance::Polkadot(pallet) => pallet.get_latest_finalized_height(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_latest_updated_height(&self) -> HeightResult<T::BlockNumber> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_latest_updated_height(),
            PalletInstance::Kusama(pallet) => pallet.get_latest_updated_height(),
            PalletInstance::Polkadot(pallet) => pallet.get_latest_updated_height(),
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

    fn read_fast_confirmation_offset(&self) -> T::BlockNumber {
        match self {
            PalletInstance::Rococo(pallet) => pallet.read_fast_confirmation_offset(),
            PalletInstance::Kusama(pallet) => pallet.read_fast_confirmation_offset(),
            PalletInstance::Polkadot(pallet) => pallet.read_fast_confirmation_offset(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn read_rational_confirmation_offset(&self) -> T::BlockNumber {
        match self {
            PalletInstance::Rococo(pallet) => pallet.read_rational_confirmation_offset(),
            PalletInstance::Kusama(pallet) => pallet.read_rational_confirmation_offset(),
            PalletInstance::Polkadot(pallet) => pallet.read_rational_confirmation_offset(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn read_finalized_confirmation_offset(&self) -> T::BlockNumber {
        match self {
            PalletInstance::Rococo(pallet) => pallet.read_finalized_confirmation_offset(),
            PalletInstance::Kusama(pallet) => pallet.read_finalized_confirmation_offset(),
            PalletInstance::Polkadot(pallet) => pallet.read_finalized_confirmation_offset(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn get_current_epoch(&self) -> HeightResult<T::BlockNumber> {
        match self {
            PalletInstance::Rococo(pallet) => pallet.get_current_epoch(),
            PalletInstance::Kusama(pallet) => pallet.get_current_epoch(),
            PalletInstance::Polkadot(pallet) => pallet.get_current_epoch(),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn read_epoch_offset(&self) -> T::BlockNumber {
        match self {
            PalletInstance::Rococo(pallet) => pallet.read_epoch_offset(),
            PalletInstance::Kusama(pallet) => pallet.read_epoch_offset(),
            PalletInstance::Polkadot(pallet) => pallet.read_epoch_offset(),
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

    fn submit_finality_header(
        &self,
        origin: OriginFor<T>,
        encoded_header_data: Bytes,
    ) -> Result<bool, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.submit_finality_header(origin, encoded_header_data),
            PalletInstance::Kusama(pallet) =>
                pallet.submit_finality_header(origin, encoded_header_data),
            PalletInstance::Polkadot(pallet) =>
                pallet.submit_finality_header(origin, encoded_header_data),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn header_speed_mode_satisfied(&self, header: Bytes, speed_mode: SpeedMode) -> bool {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.header_speed_mode_satisfied(header, speed_mode),
            PalletInstance::Kusama(pallet) =>
                pallet.header_speed_mode_satisfied(header, speed_mode),
            PalletInstance::Polkadot(pallet) =>
                pallet.header_speed_mode_satisfied(header, speed_mode),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_event_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_event_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_event_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_event_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_state_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_state_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_state_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_state_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }

    fn verify_tx_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        match self {
            PalletInstance::Rococo(pallet) =>
                pallet.verify_tx_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Kusama(pallet) =>
                pallet.verify_tx_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Polkadot(pallet) =>
                pallet.verify_tx_inclusion(gateway_id, message, submission_target_height),
            PalletInstance::Phantom(_) => unreachable!("Phantom variant should not be used"),
        }
    }
}

impl<T: Config<I>, I: 'static> LightClient<T> for Pallet<T, I> {
    fn get_latest_finalized_header(&self) -> HeaderResult {
        match Pallet::<T, I>::get_latest_finalized_header() {
            Some(header) => HeaderResult::Header(header),
            None => HeaderResult::NotActive,
        }
    }

    fn get_latest_finalized_height(&self) -> HeightResult<T::BlockNumber> {
        let header = Pallet::<T, I>::best_finalized_map();
        let local_number = match to_local_block_number::<T, I>(*header.number()) {
            Ok(number) => number,
            Err(_) => return HeightResult::NotActive,
        };
        HeightResult::Height(local_number)
    }

    fn get_latest_updated_height(&self) -> HeightResult<T::BlockNumber> {
        self.get_latest_finalized_height()
    }

    fn get_latest_heartbeat(&self) -> Result<LightClientHeartbeat<T>, DispatchError> {
        let header = Pallet::<T, I>::best_finalized_map();
        let last_finalized_height = to_local_block_number::<T, I>(*header.number())?;
        Ok(LightClientHeartbeat {
            last_heartbeat: frame_system::Pallet::<T>::block_number(),
            last_finalized_height,
            last_updated_height: last_finalized_height,
            is_halted: Pallet::<T, I>::is_halted(),
            ever_initialized: Pallet::<T, I>::ever_initialized(),
        })
    }

    fn read_fast_confirmation_offset(&self) -> T::BlockNumber {
        T::FastConfirmationOffset::get()
    }

    fn read_rational_confirmation_offset(&self) -> T::BlockNumber {
        T::RationalConfirmationOffset::get()
    }

    fn read_finalized_confirmation_offset(&self) -> T::BlockNumber {
        T::FinalizedConfirmationOffset::get()
    }

    fn get_current_epoch(&self) -> HeightResult<T::BlockNumber> {
        HeightResult::NotActive
    }

    fn read_epoch_offset(&self) -> T::BlockNumber {
        T::EpochOffset::get()
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

    fn header_speed_mode_satisfied(&self, including_header: Bytes, speed_mode: SpeedMode) -> bool {
        let as_target_header: BridgedBlockHash<T, I> =
            match BridgedBlockHash::<T, I>::decode(&mut &*including_header) {
                Ok(header) => header,
                Err(_) => return false,
            };

        let inclusion_block_number = match <ImportedHeaders<T, I>>::get(as_target_header) {
            Some(header) => match to_local_block_number::<T, I>(*header.number()) {
                Ok(block_number) => block_number,
                Err(_) => return false,
            },
            None => return false,
        };

        let current_target_height = match self.get_latest_updated_height() {
            HeightResult::Height(height) => height,
            HeightResult::NotActive => return false,
        };

        let offset = match speed_mode {
            SpeedMode::Fast => self.read_fast_confirmation_offset(),
            SpeedMode::Rational => self.read_rational_confirmation_offset(),
            SpeedMode::Finalized => self.read_finalized_confirmation_offset(),
        };

        inclusion_block_number <= current_target_height + offset
    }

    fn submit_finality_header(
        &self,
        _origin: OriginFor<T>,
        encoded_header_data: Bytes,
    ) -> Result<bool, DispatchError> {
        self.submit_encoded_headers(encoded_header_data)
    }

    fn verify_event_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        Pallet::<T, I>::confirm_event_inclusion(gateway_id, message, submission_target_height)
    }

    fn verify_state_inclusion(
        &self,
        _gateway_id: [u8; 4],
        _message: Bytes,
        _submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        unimplemented!("GrandpaFV::verify_storage_inclusion not implemented yet")
    }

    fn verify_tx_inclusion(
        &self,
        _gateway_id: [u8; 4],
        _message: Bytes,
        _submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        unimplemented!("GrandpaFV::verify_tx_inclusion not implemented yet")
    }
}

#[cfg(test)]
pub mod grandpa_light_clients_test {
    use super::*;
    use codec::Encode;

    use crate::{
        bridges::test_utils::authorities, mock::*, tests::produce_mock_headers_range,
        types::RelaychainRegistrationData,
    };
    use frame_support::{assert_ok, traits::OriginTrait};

    use crate::mock::{Origin, TestRuntime};
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
                        last_updated_height: 5,
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
}
