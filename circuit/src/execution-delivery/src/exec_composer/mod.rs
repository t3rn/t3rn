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
                .unwrap();

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
mod tests {}
