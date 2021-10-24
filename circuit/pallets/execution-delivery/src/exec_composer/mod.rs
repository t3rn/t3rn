#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use sp_std::boxed::Box;
use sp_std::vec;
use sp_std::vec::*;

use pallet_contracts_registry::RegistryContract;

use t3rn_primitives::abi::{ContractActionDesc, GatewayABIConfig};
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::CircuitOutboundMessage;

use frame_support::{traits::Get, weights::Weight};

use t3rn_primitives::Compose;
use t3rn_primitives::{GatewayInboundProtocol, GatewayPointer, GatewayType, GatewayVendor};

use volatile_vm::exec::Stack;
use volatile_vm::exec::StackExtension;
use volatile_vm::gas::GasMeter as VVMGasMeter;
use volatile_vm::storage::RawAliveContractInfo;
use volatile_vm::wasm::PrefabWasmModule;
use volatile_vm::VolatileVM;
use volatile_vm::{CallStamp, DeferredStorageWrite, ExecReturnValue};
pub mod volatile_vm_impl;

#[cfg(test)]
pub mod tests;

use crate::exec_composer::volatile_vm_impl::*;
use crate::AuthorityId;
use crate::Config;

use sp_core::Hasher;
use volatile_vm::exec::FrameArgs;
use volatile_vm::wasm::RunMode;

type ChainId = [u8; 4];

use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;

pub const TEST_RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("test-runtime"),
    impl_name: create_runtime_str!("test-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: sp_version::create_apis_vec!([]),
    transaction_version: 1,
};
pub struct ExecComposer {}

impl ExecComposer {
    pub fn post_run_single_contract<T: Config>(
        contract: RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber>,
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

        let (name, code_txt, _gateway_id, exec_type, dest, value, bytes, input_data) = (
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
            exec_type,
            dest,
            value,
            bytes,
            input_data,
        };

        // confirmed_outputs =? collected_artifacts
        Ok(vec![])
    }

    pub fn dry_run_single_contract<T: Config>(
        compose: Compose<T::AccountId, BalanceOf<T>>,
    ) -> Result<RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber>, &'static str>
    {
        let contract_action_descriptions: Vec<ContractActionDesc<T::Hash, ChainId, T::AccountId>> =
            vec![];

        let mut temp_contract = RegistryContract::from_compose(
            compose.clone(),
            contract_action_descriptions,
            Default::default(),
            None,
            None,
            Some(RawAliveContractInfo {
                trie_id: Default::default(),
                storage_size: Default::default(),
                pair_count: Default::default(),
                code_hash: T::Hashing::hash(&compose.bytes),
                rent_allowance: Default::default(),
                rent_paid: Default::default(),
                deduct_block: Default::default(),
                last_write: Default::default(),
                _reserved: Default::default(),
            }),
            Default::default(),
        );

        Self::preload_bunch_of_contracts::<T>(vec![temp_contract.clone()], Default::default())?;

        Self::run_single_contract::<T, OptimisticOutputMode>(
            &mut temp_contract,
            Default::default(),
            Weight::MAX,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )?;

        Ok(temp_contract)
    }

    pub fn run_single_contract<T: Config, OM: WasmEnvOutputMode>(
        contract: &mut RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber>,
        input_data: Vec<u8>,
        gas_limit: Weight,
        value: BalanceOf<T>,
        escrow_account: T::AccountId,
        submitter: AuthorityId,
        requester: T::AccountId,
        gateway_id: Option<bp_runtime::ChainId>,
        gateway_abi: GatewayABIConfig,
    ) -> Result<(Vec<CircuitOutboundMessage>, Vec<u32>), &'static str> {
        let gateway_pointer = Self::retrieve_gateway_pointer::<T>(gateway_id.clone())?;
        let gateway_inbound_protocol =
            Self::retrieve_gateway_protocol::<T>(submitter, &gateway_pointer)?;

        let maybe_input_data = match input_data.len() {
            0 => None,
            _ => Some(input_data.clone()),
        };

        let inner_exec_transfers = &mut Vec::<TransferEntry>::new();
        let constructed_outbound_messages = &mut Vec::<CircuitOutboundMessage>::new();
        let round_breakpoints = &mut Vec::<u32>::new();

        let gas_meter = &mut VVMGasMeter::<T>::new(gas_limit);
        let _deferred_storage_writes = &mut Vec::<DeferredStorageWrite>::new();
        let _call_stamps = &mut Vec::<CallStamp>::new();

        let schedule = <T as VolatileVM>::Schedule::get();

        // Create stack for multiple level of sub-calls
        // ToDo: Change to submitter
        let origin = requester.clone();
        // ToDo: Frame value equal to requested args
        let debug_message = None;

        // Here could also access and pre-load code to lazy storage of VVM
        let _executable = PrefabWasmModule::<T>::from_code(
            contract.bytes.clone(),
            &schedule,
            OM::get_run_mode(),
            gateway_id.clone(),
        )
        .map_err(|_e| "Can't decode WASM code")?;

        // if target is None if in the contracts-repository
        let target_id = gateway_id;
        let run_mode = OM::get_run_mode();

        let stack_extension = &mut StackExtension {
            escrow_account,
            requester: requester.clone(),
            storage_trie_id: contract.info.clone().unwrap().child_trie_info(),
            input_data: maybe_input_data,
            inner_exec_transfers,
            constructed_outbound_messages,
            round_breakpoints,
            gateway_inbound_protocol,
            gateway_pointer,
            gateway_abi,
            preloaded_action_descriptions: &mut contract.action_descriptions,
            target_id,
            run_mode,
        };

        let (mut stack, executable) = Stack::<T, PrefabWasmModule<T>>::new(
            FrameArgs::Call {
                dest: requester.clone(),
                cached_info: None, //  If no lazy load set Some(contract.info.clone())
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

        // External caller should respond and if still executing locally carry on with executing next contract
        Ok((
            constructed_outbound_messages.to_vec(),
            round_breakpoints.to_vec(),
        ))
    }

    /// Returns - all messages created at this round which are immediately available to relay to foreign consensus systems.
    pub fn pre_run_bunch_until_break<T: Config>(
        contracts: Vec<RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber>>,
        escrow_account: T::AccountId,
        submitter: AuthorityId,
        requester: T::AccountId,
        value: BalanceOf<T>,
        input_data: Vec<u8>,
        gas_limit: Weight,
        gateway_id: Option<bp_runtime::ChainId>,
        gateway_abi_config: GatewayABIConfig,
    ) -> Result<(Vec<CircuitOutboundMessage>, u16), &'static str> {
        Self::preload_bunch_of_contracts::<T>(contracts.clone(), requester.clone())?;

        let constructed_outbound_messages = &mut Vec::<CircuitOutboundMessage>::new();

        let mut counter: u16 = 0;

        for mut contract in contracts {
            // ToDo: Input data can change with next loop iteration from output of the last contracts if within the same round
            let (outbound_messages_current, round_breakpoints_current) =
                Self::run_single_contract::<T, PessimisticOutputMode>(
                    &mut contract,
                    input_data.clone(),
                    gas_limit.clone(),
                    value.clone(),
                    escrow_account.clone(),
                    submitter.clone(),
                    requester.clone(),
                    gateway_id.clone(),
                    gateway_abi_config.clone(),
                )?;

            constructed_outbound_messages.extend(outbound_messages_current);

            // If the round finished return only messages produced until now
            if !round_breakpoints_current.is_empty() {
                return Ok((constructed_outbound_messages.to_vec(), counter));
            }

            counter += 1;
        }

        // All messages in that round
        Ok((constructed_outbound_messages.to_vec(), counter))
    }

    /// Pre-load is called before pre-run in order to load contracts code and info (about used space and active rent)
    /// into volatile VM (as cache).
    /// Pre-run accesses the contract code and info from the contracts-cache of VVM aka fake-storage
    pub fn preload_bunch_of_contracts<T: Config>(
        contracts: Vec<RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber>>,
        account_id: T::AccountId, // purchaser
    ) -> Result<(), &'static str> {
        let schedule = <T as VolatileVM>::Schedule::get();
        // Assume contracts from on-chain repo set None as a foreign target.
        let gateway_id = None;
        // Perform syntax check and convert raw code into executables
        let executables_map = contracts
            .iter()
            .map(|contract| {
                PrefabWasmModule::<T>::from_code(
                    contract.bytes.clone(),
                    &schedule,
                    RunMode::Dry,
                    gateway_id,
                )
                .map_err(|_e| "Can't decode WASM code")
            })
            .collect::<Result<Vec<PrefabWasmModule<T>>, &'static str>>()?;

        for i in 0..contracts.len() {
            let curr_executable = executables_map[i.clone()].clone();
            let contract_info = contracts[i].info.clone();
            volatile_vm::Pallet::<T>::add_contract_code_lazy(
                curr_executable.code_hash,
                curr_executable,
                contract_info.unwrap(),
                account_id.clone(),
            )
        }

        Ok(())
    }

    fn retrieve_gateway_pointer<T: Config>(
        gateway_id: Option<t3rn_primitives::ChainId>,
    ) -> Result<GatewayPointer, &'static str> {
        match gateway_id {
            None => Ok(GatewayPointer {
                // ToDo: Setup default for Circuit equivalent to None
                id: Default::default(),
                gateway_type: GatewayType::ProgrammableExternal(0),
                vendor: GatewayVendor::Substrate,
            }),
            Some(gateway_id) => {
                let xdns_record_id = T::Hashing::hash(Encode::encode(&gateway_id).as_ref());

                let xdns_record = pallet_xdns::Pallet::<T>::xdns_registry(xdns_record_id).unwrap();
                Ok(GatewayPointer {
                    id: xdns_record.gateway_id,
                    gateway_type: xdns_record.gateway_type,
                    vendor: xdns_record.gateway_vendor,
                })
            }
        }
    }

    /// Given a Gateway Pointer and an Authority, it returns the respective Gateway Protocol
    fn retrieve_gateway_protocol<T: crate::Config>(
        submitter_id: AuthorityId,
        gateway_pointer: &GatewayPointer,
    ) -> Result<Box<dyn GatewayInboundProtocol>, &'static str> {
        // Very dummy - replace asap with https://github.com/t3rn/t3rn/pull/87
        use crate::message_assembly::chain_generic_metadata::Metadata;
        use frame_metadata::decode_different::DecodeDifferent;
        use frame_metadata::v13::{
            ExtrinsicMetadata, FunctionMetadata, ModuleMetadata, RuntimeMetadataV13,
        };
        pub fn get_dummy_modules_with_functions() -> Vec<(&'static str, Vec<&'static str>)> {
            vec![
                ("state", vec!["call"]),
                ("state", vec!["getStorage"]),
                ("state", vec!["setStorage"]),
                ("ModuleName", vec!["FnName"]),
                ("ModuleName", vec!["FnName1"]),
                ("ModuleName", vec!["FnName2"]),
                ("ModuleName", vec!["FnName3"]),
                ("author", vec!["submitExtrinsic"]),
                ("utility", vec!["batchAll"]),
                ("system", vec!["remark"]),
                ("gateway", vec!["call"]),
                ("balances", vec!["transfer"]),
                ("gateway", vec!["getStorage"]),
                ("gateway", vec!["transfer"]),
                ("gateway", vec!["emitEvent"]),
                ("gateway", vec!["custom"]),
                ("gatewayEscrowed", vec!["callStatic"]),
                ("gatewayEscrowed", vec!["callEscrowed"]),
            ]
        }
        // Very dummy - replace asap with https://github.com/t3rn/t3rn/pull/87
        fn create_test_metadata(
            modules_with_functions: Vec<(&'static str, Vec<&'static str>)>,
        ) -> Metadata {
            let mut module_index = 0;
            let mut modules: Vec<ModuleMetadata> = vec![];

            let fn_metadata_generator = |name: &'static str| -> FunctionMetadata {
                FunctionMetadata {
                    name: DecodeDifferent::Encode(name),
                    arguments: DecodeDifferent::Decoded(vec![]),
                    documentation: DecodeDifferent::Decoded(vec![]),
                }
            };

            let module_metadata_generator = |mod_name: &'static str,
                                             mod_index: u8,
                                             functions: Vec<FunctionMetadata>|
             -> ModuleMetadata {
                ModuleMetadata {
                    index: mod_index,
                    name: DecodeDifferent::Encode(mod_name),
                    storage: None,
                    calls: Some(DecodeDifferent::Decoded(functions)),
                    event: None,
                    constants: DecodeDifferent::Decoded(vec![]),
                    errors: DecodeDifferent::Decoded(vec![]),
                }
            };

            for module in modules_with_functions {
                let (module_name, fn_names) = module;
                let functions = fn_names.into_iter().map(fn_metadata_generator).collect();
                modules.push(module_metadata_generator(
                    module_name,
                    module_index.clone(),
                    functions,
                ));
                module_index = module_index + 1;
            }

            let runtime_metadata = RuntimeMetadataV13 {
                extrinsic: ExtrinsicMetadata {
                    version: 1,
                    signed_extensions: vec![DecodeDifferent::Encode("test")],
                },
                modules: DecodeDifferent::Decoded(modules),
            };
            Metadata::new(runtime_metadata)
        }

        let mut best_gateway =
            pallet_xdns::Pallet::<T>::best_available(gateway_pointer.clone().id)?;

        let genesis_hash = T::Hashing::hash(&mut best_gateway.gateway_genesis.genesis_hash);
        let runtime_version = best_gateway.gateway_genesis.runtime_version;

        Ok(Box::new(
            SubstrateGatewayProtocol::<AuthorityId, T::Hash>::new(
                // FixMe: Very dummy - replace asap with https://github.com/t3rn/t3rn/pull/87
                create_test_metadata(get_dummy_modules_with_functions()),
                runtime_version,
                genesis_hash,
                submitter_id,
            ),
        ))
    }
}
