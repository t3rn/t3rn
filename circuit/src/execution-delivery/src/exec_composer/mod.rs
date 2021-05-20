use codec::Decode;


use pallet_contracts_registry::RegistryContract;

use substrate_api_client::sp_core::Pair;
use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::transfers::BalanceOf;

use frame_support::traits::Time;

use t3rn_primitives::{Compose};
use t3rn_primitives::{GatewayPointer, GatewayType, GatewayVendor};

use versatile_wasm::runtime::{DeferredStorageWrite, CallStamp};
use versatile_wasm::runtime::run_code_on_versatile_wm;
use versatile_wasm::gas::GasMeter;

use crate::message_assembly::circuit_outbound::{CircuitOutboundMessage};

pub mod versatile_vm_impl;

use crate::exec_composer::versatile_vm_impl::*;

pub struct ExecComposer { }

impl ExecComposer {

    pub fn pre_run_single_contract<T: crate::Config>(
        contract: RegistryContract<T::AccountId>,
        escrow_account: T::AccountId,
        _requester: T::AccountId,
        target_dest: T::AccountId,
        value: BalanceOf<T>,
        input: Vec<u8>,
        gateway_id: bp_runtime::InstanceId,
    ) -> anyhow::Result<Vec<CircuitOutboundMessage>, &'static str> {

        let output_mode = PessimisticOutputMode::new();
        let requester = T::AccountId::default();  // In dry run don't use a requester to check whether the code is correct

        let (name, code_txt, gateway_id, exec_type, dest, value, bytes, input_data) =
            (vec![], contract.code_txt, gateway_id, vec![], target_dest, value, contract.bytes, input);
        let compose = Compose { name, code_txt, gateway_id, exec_type, dest, value, bytes, input_data };

        Self::run_single_contract::<T, PessimisticOutputMode>(compose, escrow_account, requester, output_mode)
    }

    pub fn post_run_single_contract<T: crate::Config>(
        contract: RegistryContract<T::AccountId>,
        escrow_account: T::AccountId,
        requester: T::AccountId,
        target_dest: T::AccountId,
        value: BalanceOf<T>,
        input: Vec<u8>,
        gateway_id: bp_runtime::InstanceId,
        _confirmed_outputs: Vec<u8>,
    ) -> anyhow::Result<Vec<CircuitOutboundMessage>, &'static str> {

        let output_mode = StuffedOutputMode::new();

        let (name, code_txt, gateway_id, exec_type, dest, value, bytes, input_data) =
            (vec![], contract.code_txt, gateway_id, vec![], target_dest, value, contract.bytes, input);
        let compose = Compose { name, code_txt, gateway_id, exec_type, dest, value, bytes, input_data };

        Self::run_single_contract::<T, StuffedOutputMode>(compose, escrow_account, requester, output_mode)
    }

    pub fn dry_run_single_contract<T: crate::Config>(
        compose: Compose<T::AccountId, BalanceOf<T>>,
        escrow_account: T::AccountId,
    ) -> anyhow::Result<Vec<CircuitOutboundMessage>, &'static str> {

        let output_mode = OptimisticOutputMode::new();
        let requester = T::AccountId::default();  // In dry run don't use a requester to check whether the code is correct

        Self::run_single_contract::<T, OptimisticOutputMode>(compose, escrow_account, requester, output_mode)
    }

    pub fn run_single_contract<T: crate::Config, OM: WasmEnvOutputMode>(
        compose: Compose<T::AccountId, BalanceOf<T>>,
        escrow_account: T::AccountId,
        requester: T::AccountId,
        output_mode: OM,
    ) -> anyhow::Result<Vec<CircuitOutboundMessage>, &'static str> {

        let gateway_pointer = Self::retrieve_gateway_pointer(compose.gateway_id)?;
        let gateway_protocol = Self::retrieve_gateway_protocol(gateway_pointer.clone())?;

        let (block_number, timestamp, contract_trie_id, input_data, code, value, gas_limit, target_account) = (
            <frame_system::Pallet<T>>::block_number(),
            T::Time::now(),
            // get_child_storage_for_current_execution::<T>(&escrow_account, T::Hash::decode(&mut &sp_io::storage::root()[..]).expect("storage root should be there")),
            T::Hash::decode(&mut &sp_io::storage::root()[..]).expect("storage root should be there"),
            compose.input_data,
            compose.bytes,
            BalanceOf::<T>::from(
                sp_std::convert::TryInto::<u32>::try_into(compose.value).map_err(|_e| "Can't cast value in dry_run_single_contract")?,
            ),
            u64::max_value(),
            compose.dest,
        );

        let mut deferred_transfers = Vec::<TransferEntry>::new();
        let mut constructed_outbound_messages = Vec::<CircuitOutboundMessage>::new();

        // Utilise Rust specialisation
        let env_optimistic_dry = CircuitVersatileWasmEnv::<T, OM>::new(
            &escrow_account,
            &requester,
            block_number,
            timestamp,
            contract_trie_id,
            Some(input_data),
            &mut deferred_transfers,
            &mut constructed_outbound_messages,
            gateway_protocol,
            gateway_pointer,
            output_mode,
        );

        let trace_stack = true;
        let gas_meter = &mut GasMeter::new(gas_limit);
        let mut deferred_storage_writes = Vec::<DeferredStorageWrite>::new();
        let mut call_stamps = Vec::<CallStamp>::new();

        // ToDo: Implement as env_optimistic_dry::run()
        run_code_on_versatile_wm::<T, CircuitVersatileWasmEnv<T, OM>>(
            env_optimistic_dry.escrow_account,
            &env_optimistic_dry.requester,
            &target_account, // dest
            value,
            gas_meter,
            env_optimistic_dry.input_data.clone().unwrap(),
            &mut env_optimistic_dry.inner_exec_transfers.clone(),
            &mut deferred_storage_writes,
            &mut call_stamps,
            code,
            env_optimistic_dry.storage_trie_id,
            trace_stack,
            env_optimistic_dry,
        );

        Ok(constructed_outbound_messages.to_vec())
    }

    fn retrieve_gateway_pointer(
        gateway_id: bp_runtime::InstanceId,
    ) -> anyhow::Result<GatewayPointer, &'static str> {

        // let (gateway_vendor (Substrate, Eth), gateway_type (EXT, INT, TX-ONLY) = xdns::Pallet<T>::best_available(gateway_id);
        //
        // let gateway_pointer = exec_composer::xdns_decoder::decode(encoded_gateway_params)
        //
        // let encoded_gateway_params = xdns::Pallet<T>::best_available(gateway_id);
        // the logic here should be dynamically decoding the requested type without match (based on maybe another sandboxed VM wasm eval?
        // - implement a mini-vm for that purpose


        /// Parachain validation
        ///
        /// Sandbox - contract specific methods
        /// On parachain - new blcok - eval in runtime
        /// Relay chain runs parchain new in wasm enironment of parachain
        /// POV block - Block + Subtree of state which are necessary to validate the blocks's validity
        ///
        Ok(
            GatewayPointer {
                id: gateway_id,
                gateway_type: GatewayType::ProgrammableExternal,
                vendor: GatewayVendor::Substrate,
            }
        )
    }

    fn retrieve_gateway_protocol(
        _gateway_pointer: GatewayPointer,
    ) -> anyhow::Result<Box<dyn GatewayInboundProtocol>, &'static str> {
        // ToDo: Communicate with pallet_xdns in order to retrieve latest data about
        /// let (metadata, runtime_version, genesis_hash) = pallet_xdns::Pallet<T>::get_gateway_protocol_meta(gateway_pointer.id)
        Ok(Box::new(SubstrateGatewayProtocol::<substrate_api_client::sp_core::sr25519::Pair>::new(
            Default::default(),
            Default::default(),
            Default::default(),
            substrate_api_client::sp_core::sr25519::Pair::from_string("//Alice", None).unwrap(),
            substrate_api_client::sp_core::sr25519::Pair::from_string("//Alice", None).unwrap(),
        )))
    }
}

#[cfg(test)]
mod tests {
    
    
    
    
    
    


}