#![cfg_attr(not(feature = "std"), no_std)]

use codec::Decode;
use sp_std::boxed::Box;
use sp_std::vec;
use sp_std::vec::*;

use pallet_contracts_registry::RegistryContract;

use t3rn_primitives::abi::GatewayABIConfig;
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::CircuitOutboundMessage;

use frame_support::traits::Currency;
use frame_support::traits::Time;

use t3rn_primitives::{Compose, EscrowTrait};
use t3rn_primitives::{GatewayInboundProtocol, GatewayPointer, GatewayType, GatewayVendor};

use volatile_vm::exec::StackExtension;
use volatile_vm::{CallStamp, DeferredStorageWrite, ExecReturnValue};

pub mod volatile_vm_impl;

use crate::exec_composer::volatile_vm_impl::*;
use crate::AuthorityId;

use volatile_vm::exec::FrameArgs;
use volatile_vm::wasm::RunMode;

pub struct ExecComposer {}

impl ExecComposer {
    pub fn pre_run_single_contract<T: crate::Config>(
        contract: RegistryContract<T::AccountId>,
        escrow_account: T::AccountId,
        submitter: AuthorityId,
        _requester: T::AccountId,
        target_dest: T::AccountId,
        value: BalanceOf<T>,
        input: Vec<u8>,
        gateway_id: bp_runtime::ChainId,
        gateway_abi_config: GatewayABIConfig,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        let output_mode = PessimisticOutputMode::new();
        let requester = T::AccountId::default(); // In dry run don't use a requester to check whether the code is correct

        let (name, code_txt, gateway_id, exec_type, dest, value, bytes, input_data) = (
            vec![],
            contract.code_txt,
            gateway_id,
            vec![],
            target_dest,
            value,
            contract.bytes,
            input,
        );

        let compose = Compose {
            name,
            code_txt,
            gateway_id,
            exec_type,
            dest,
            value,
            bytes,
            input_data,
        };

        Self::run_single_contract::<T, PessimisticOutputMode>(
            compose,
            escrow_account,
            submitter,
            requester,
            gateway_id,
            gateway_abi_config,
            output_mode,
        )
    }

    pub fn post_run_single_contract<T: crate::Config>(
        contract: RegistryContract<T::AccountId>,
        _escrow_account: T::AccountId,
        _submitter: AuthorityId,
        _requester: T::AccountId,
        target_dest: T::AccountId,
        value: BalanceOf<T>,
        input: Vec<u8>,
        gateway_id: bp_runtime::ChainId,
        _gateway_abi_config: GatewayABIConfig,
        _confirmed_outputs: Vec<u8>,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        let _output_mode = StuffedOutputMode::new();

        let (name, code_txt, gateway_id, exec_type, dest, value, bytes, input_data) = (
            vec![],
            contract.code_txt,
            gateway_id,
            vec![],
            target_dest,
            value,
            contract.bytes,
            input,
        );
        let _compose = Compose {
            name,
            code_txt,
            gateway_id,
            exec_type,
            dest,
            value,
            bytes,
            input_data,
        };

        // confirmed_outputs =? collected_artifacts

        Ok(vec![])
    }

    pub fn dry_run_single_contract<T: crate::Config>(
        compose: Compose<T::AccountId, BalanceOf<T>>,
        escrow_account: T::AccountId,
        submitter: AuthorityId,
        gateway_id: bp_runtime::ChainId,
        gateway_abi_config: GatewayABIConfig,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        let output_mode = OptimisticOutputMode::new();
        let requester = T::AccountId::default(); // In dry run don't use a requester to check whether the code is correct

        Self::run_single_contract::<T, OptimisticOutputMode>(
            compose,
            escrow_account,
            submitter,
            requester,
            gateway_id,
            gateway_abi_config,
            output_mode,
        )
    }

    pub fn run_single_contract<T: crate::Config, OM: WasmEnvOutputMode>(
        compose: Compose<T::AccountId, BalanceOf<T>>,
        escrow_account: T::AccountId,
        submitter: AuthorityId,
        requester: T::AccountId,
        gateway_id: bp_runtime::ChainId,
        gateway_abi: GatewayABIConfig,
        _output_mode: OM,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        let gateway_pointer = Self::retrieve_gateway_pointer(gateway_id)?;
        let gateway_inbound_protocol =
            Self::retrieve_gateway_protocol::<T>(submitter, gateway_pointer.clone())?;

        let (
            _block_number,
            _timestamp,
            storage_trie_id,
            input_data,
            code,
            _value,
            gas_limit,
            _target_account,
        ) = (
            <frame_system::Pallet<T>>::block_number(),
            <T as EscrowTrait>::Time::now(),
            // Replaces get_child_storage_for_current_execution::<T>(&escrow_account, T::Hash::decode(&mut &sp_io::storage::root()[..]).expect("storage root should be there")),
            T::Hash::decode(&mut &sp_io::storage::root()[..])
                .expect("storage root should be there"),
            compose.input_data,
            compose.bytes,
            BalanceOf::<T>::from(
                sp_std::convert::TryInto::<u32>::try_into(compose.value)
                    .map_err(|_e| "Can't cast value in dry_run_single_contract")?,
            ),
            u64::max_value(),
            compose.dest,
        );

        let maybe_input_data = match input_data.len() {
            0 => None,
            _ => Some(input_data.clone()),
        };

        let inner_exec_transfers = &mut Vec::<TransferEntry>::new();
        let constructed_outbound_messages = &mut Vec::<CircuitOutboundMessage>::new();

        let _trace_stack = true;
        use frame_support::traits::Get;
        use volatile_vm::exec::Stack;
        use volatile_vm::gas::GasMeter as VVMGasMeter;
        use volatile_vm::wasm::PrefabWasmModule;
        use volatile_vm::VolatileVM;

        let gas_meter = &mut VVMGasMeter::<T>::new(gas_limit);
        let _deferred_storage_writes = &mut Vec::<DeferredStorageWrite>::new();
        let _call_stamps = &mut Vec::<CallStamp>::new();

        let schedule = <T as VolatileVM>::Schedule::get();

        // Create stack for multiple level of sub-calls

        // ToDo: Change to submitter
        let origin = requester.clone();
        // ToDo: Frame value equal to requested args
        let value = <T as EscrowTrait>::Currency::minimum_balance();
        let debug_message = None;

        // Here could also access and pre-load code to lazy storage of VVM
        let executable =
            PrefabWasmModule::<T>::from_code(code, &schedule, OM::get_run_mode(), Some(gateway_id))
                .map_err(|_e| "Can't decode WASM code")?;

        // For now finish dry run here - if the code passing static analysis in the previous step,
        // add it to the candidates queue if new.
        if OM::get_run_mode() == RunMode::Dry {
            <volatile_vm::Pallet<T>>::add_contract_code_lazy(executable.code_hash, executable);
            return Ok(vec![]);
        }

        let stack_extension = &mut StackExtension {
            escrow_account,
            requester: requester.clone(),
            storage_trie_id,
            input_data: maybe_input_data,
            inner_exec_transfers,
            constructed_outbound_messages,
            round_breakpoints: vec![],
            gateway_inbound_protocol,
            gateway_pointer,
            gateway_abi,
        };

        let (mut stack, executable) = Stack::<T, PrefabWasmModule<T>>::new(
            FrameArgs::Call {
                dest: requester.clone(),
                cached_info: None,
            },
            origin,
            gas_meter,
            &schedule,
            value,
            debug_message,
            stack_extension,
        )
        .map_err(|_e| "Can't create VVM call stack")?;

        let _ret_out: ExecReturnValue =
            stack.run(executable, input_data).map_err(|err| err.error)?;

        Ok(constructed_outbound_messages.to_vec())
    }

    fn retrieve_gateway_pointer(
        gateway_id: bp_runtime::ChainId,
    ) -> Result<GatewayPointer, &'static str> {
        Ok(GatewayPointer {
            id: gateway_id,
            gateway_type: GatewayType::ProgrammableExternal,
            vendor: GatewayVendor::Substrate,
        })
    }

    /// Given a Gateway Pointer and an Authority, it returns the respective Gateway Protocol
    fn retrieve_gateway_protocol<T: crate::Config>(
        submitter_id: AuthorityId,
        _gateway_pointer: GatewayPointer,
    ) -> Result<Box<dyn GatewayInboundProtocol>, &'static str> {
        // ToDo: Communicate with pallet_xdns in order to retrieve latest data about
        // let (metadata, runtime_version, genesis_hash) = pallet_xdns::Pallet<T>::get_gateway_protocol_meta(gateway_pointer.id)
        Ok(Box::new(SubstrateGatewayProtocol::<
            AuthorityId,
            bp_polkadot_core::Hash,
        >::new(
            Default::default(),
            Default::default(),
            Default::default(),
            submitter_id,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::Test;
    use crate::*;

    use sp_core::{crypto::Pair, sr25519};

    use frame_support::assert_ok;

    use sp_io::TestExternalities;
    use sp_keystore::testing::KeyStore;
    use sp_keystore::{KeystoreExt, SyncCryptoStore};
    use sp_runtime::AccountId32;
    use std::str::FromStr;

    fn make_compose_out_of_raw_wat_code<T: Config>(
        wat: &str,
        input_data: Vec<u8>,
        dest: T::AccountId,
        value: T::Balance,
        gateway_id: [u8; 4],
    ) -> Compose<T::AccountId, T::Balance> {
        let wasm = wat::parse_str(wat.clone()).unwrap();
        Compose {
            name: b"component1".to_vec(),
            code_txt: wat.encode(),
            gateway_id,
            exec_type: b"exec_escrow".to_vec(),
            dest,
            value,
            bytes: wasm,
            input_data,
        }
    }

    fn setup_test_escrow_as_tx_signer(ext: &mut TestExternalities) -> AccountId32 {
        let keystore = KeyStore::new();
        // Insert Alice's keys
        const SURI_ALICE: &str = "//Alice";

        let key_pair_alice =
            sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
        SyncCryptoStore::insert_unknown(
            &keystore,
            KEY_TYPE,
            SURI_ALICE,
            key_pair_alice.public().as_ref(),
        )
        .expect("Inserts unknown key");

        ext.register_extension(KeystoreExt(keystore.into()));
        // Alice's account
        hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()
    }

    const CODE_CALL: &str = r#"
(module
	;; seal_call(
	;;    callee_ptr: u32,
	;;    callee_len: u32,
	;;    gas: u64,
	;;    value_ptr: u32,
	;;    value_len: u32,
	;;    input_data_ptr: u32,
	;;    input_data_len: u32,
	;;    output_ptr: u32,
	;;    output_len_ptr: u32
	;;) -> u32
	(import "seal0" "seal_call" (func $seal_call (param i32 i32 i64 i32 i32 i32 i32 i32 i32) (result i32)))
	(import "env" "memory" (memory 1 1))
	(func (export "call")
		(drop
			(call $seal_call
				(i32.const 4)  ;; Pointer to "callee" address.
				(i32.const 32)  ;; Length of "callee" address.
				(i64.const 0)  ;; How much gas to devote for the execution. 0 = all.
				(i32.const 36) ;; Pointer to the buffer with value to transfer
				(i32.const 8)  ;; Length of the buffer with value to transfer.
				(i32.const 44) ;; Pointer to input data buffer address
				(i32.const 4)  ;; Length of input data buffer
				(i32.const 4294967295) ;; u32 max value is the sentinel value: do not copy output
				(i32.const 0) ;; Length is ignored in this case
			)
		)
	)
	(func (export "deploy"))

	;; Destination AccountId (ALICE)
	(data (i32.const 4)
		"\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
		"\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
	)

	;; Amount of value to transfer.
	;; Represented by u64 (8 bytes long) in little endian.
	(data (i32.const 36) "\06\00\00\00\00\00\00\00")

	(data (i32.const 44) "\01\02\03\04")
)
"#;

    #[test]
    fn exec_composer_dry_run_succeeds_for_valid_call_contract() {
        // Bob - dest
        let dest =
            AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
        let value = BalanceOf::<Test>::from(0u32);
        let gateway_id = [0 as u8; 4];
        let compose = make_compose_out_of_raw_wat_code::<Test>(
            CODE_CALL,
            vec![],
            dest,
            value,
            gateway_id.clone(),
        );

        let mut ext = TestExternalities::new_empty();
        let escrow_account = setup_test_escrow_as_tx_signer(&mut ext);
        let gateway_abi_config: GatewayABIConfig = Default::default();

        ext.execute_with(|| {
            let submitter = crate::Pallet::<Test>::select_authority(escrow_account.clone())
                .unwrap_or_else(|_| panic!("failed to select_authority"));
            assert_ok!(ExecComposer::dry_run_single_contract::<Test>(
                compose,
                escrow_account,
                submitter,
                gateway_id,
                gateway_abi_config,
            ));
        });
    }

    #[test]
    fn exec_composer_dry_run_fails_for_invalid_call_contract() {
        // Bob - dest
        let dest =
            AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
        let value = BalanceOf::<Test>::from(0u32);
        let gateway_id = [0 as u8; 4];

        let compose = Compose {
            name: b"component1".to_vec(),
            code_txt: " invalid code str ".encode(),
            gateway_id,
            exec_type: b"exec_escrow".to_vec(),
            dest,
            value,
            bytes: " invalid code str ".encode(),
            input_data: vec![],
        };

        let mut ext = TestExternalities::new_empty();
        let escrow_account = setup_test_escrow_as_tx_signer(&mut ext);
        let gateway_abi_config: GatewayABIConfig = Default::default();

        ext.execute_with(|| {
            let submitter = crate::Pallet::<Test>::select_authority(escrow_account.clone())
                .unwrap_or_else(|_| panic!("failed to select_authority"));
            let res = ExecComposer::dry_run_single_contract::<Test>(
                compose,
                escrow_account,
                submitter,
                gateway_id,
                gateway_abi_config,
            );
            assert_eq!(res, Err("Can't decode WASM code"))
        });
    }
}
