#![cfg_attr(not(feature = "std"), no_std)]

use codec::Decode;
use sp_std::boxed::Box;
use sp_std::vec;
use sp_std::vec::*;

use pallet_contracts_registry::RegistryContract;

use t3rn_primitives::abi::GatewayABIConfig;
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::transfers::TransferEntry;

use frame_support::traits::Time;

use t3rn_primitives::{Compose, EscrowTrait};
use t3rn_primitives::{GatewayPointer, GatewayType, GatewayVendor};

use versatile_wasm::gas::GasMeter;
use versatile_wasm::runtime::run_code_on_versatile_wm;
use versatile_wasm::runtime::{CallStamp, DeferredStorageWrite};

use crate::message_assembly::circuit_outbound::CircuitOutboundMessage;

pub mod versatile_vm_impl;

use crate::exec_composer::versatile_vm_impl::*;
use crate::AuthorityId;

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
        escrow_account: T::AccountId,
        submitter: AuthorityId,
        requester: T::AccountId,
        target_dest: T::AccountId,
        value: BalanceOf<T>,
        input: Vec<u8>,
        gateway_id: bp_runtime::ChainId,
        gateway_abi_config: GatewayABIConfig,
        _confirmed_outputs: Vec<u8>,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        let output_mode = StuffedOutputMode::new();

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

        Self::run_single_contract::<T, StuffedOutputMode>(
            compose,
            escrow_account,
            submitter,
            requester,
            gateway_id,
            gateway_abi_config,
            output_mode,
        )
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
        gateway_abi_config: GatewayABIConfig,
        output_mode: OM,
    ) -> Result<Vec<CircuitOutboundMessage>, &'static str> {
        // TODO: this is hardcoded to the first returned value until a proper implementation
        let gateway_pointer = t3rn_primitives::retrieve_gateway_pointers(*&gateway_id)?.first();
        let gateway_protocol = Self::retrieve_gateway_protocol(submitter, *gateway_pointer)?;

        let (
            block_number,
            timestamp,
            contract_trie_id,
            input_data,
            code,
            value,
            gas_limit,
            target_account,
        ) = (
            <frame_system::Pallet<T>>::block_number(),
            <T as EscrowTrait>::Time::now(),
            // get_child_storage_for_current_execution::<T>(&escrow_account, T::Hash::decode(&mut &sp_io::storage::root()[..]).expect("storage root should be there")),
            T::Hash::decode(&mut &sp_io::storage::root()[..])
                .expect("storage root should be there"),
            compose.input_data,
            compose.bytes,
            BalanceOf::<T>::from(
                sp_std::convert::TryInto::<u32>::try_into(compose.value)
                    .map_err(|_e| "Can't cast value in dry_run_single_contract")?,
            ),
            u64::MAX,
            compose.dest,
        );

        let mut deferred_transfers = Vec::<TransferEntry>::new();
        let mut constructed_outbound_messages = Vec::<CircuitOutboundMessage>::new();

        // Utilise Rust specialisation
        let env_circuit_run = CircuitVersatileWasmEnv::<T, OM>::new(
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

        // ToDo: Implement as env_circuit_run::run()
        let _res = run_code_on_versatile_wm::<T, CircuitVersatileWasmEnv<T, OM>>(
            env_circuit_run.escrow_account,
            &env_circuit_run.requester,
            &target_account, // dest
            value,
            gas_meter,
            env_circuit_run.input_data.clone().unwrap(),
            &mut env_circuit_run.inner_exec_transfers.clone(),
            &mut deferred_storage_writes,
            &mut call_stamps,
            code,
            env_circuit_run.storage_trie_id,
            trace_stack,
            gateway_abi_config,
            env_circuit_run,
        );

        Ok(constructed_outbound_messages.to_vec())
    }

    /// Given a Gateway Pointer and an Authority, it returns the respective Gateway Protocol
    fn retrieve_gateway_protocol<T: crate::Config>(
        submitter_id: AuthorityId,
        gateway_pointer: &GatewayPointer,
    ) -> Result<Box<dyn GatewayInboundProtocol>, &'static str> {
        let best_gateway = pallet_xdns::Pallet::<T>::best_available(gateway_pointer.id)?;

        let genesis_hash = T::Hash::decode(
            best_gateway
                .gateway_genesis
                .genesis_hash
                .clone()
                .as_mut_slice(),
        )?;
        let runtime_version = best_gateway.gateway_genesis.runtime_version;

        Ok(Box::new(SubstrateGatewayProtocol::<
            AuthorityId,
            bp_polkadot_core::Hash,
        >::new(
            Default::default(),
            runtime_version,
            genesis_hash,
            submitter_id,
        )))
    }
}

#[cfg(test)]
mod tests {}
