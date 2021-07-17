#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchError;
use frame_support::traits::Time;

use t3rn_primitives::{
    transfers::TransferEntry, CircuitOutboundMessage, EscrowTrait, GatewayInboundProtocol,
    GatewayPointer,
};

use sp_std::boxed::Box;
use sp_std::vec::*;

use frame_system::Config as SystemTrait;

use versatile_wasm::VersatileWasm;

use volatile_vm::wasm::RunMode;

use crate::Config as CircuitTrait;

pub use crate::message_assembly::gateway_inbound_assembly::GatewayInboundAssembly;
pub use crate::message_assembly::substrate_gateway_assembly::SubstrateGatewayAssembly;
pub use crate::message_assembly::substrate_gateway_protocol::SubstrateGatewayProtocol;

pub struct CircuitVersatileWasmEnv<
    'a,
    T: EscrowTrait + SystemTrait + VersatileWasm + CircuitTrait,
    OM,
> {
    pub escrow_account: &'a T::AccountId,
    pub requester: &'a T::AccountId,
    pub block_number: <T as SystemTrait>::BlockNumber,
    pub timestamp: <<T as EscrowTrait>::Time as Time>::Moment,
    pub storage_trie_id: T::Hash,
    pub input_data: Option<Vec<u8>>,
    pub inner_exec_transfers: &'a mut Vec<TransferEntry>,
    pub constructed_outbound_messages: &'a mut Vec<CircuitOutboundMessage>,
    pub gateway_inbound_protocol: Box<dyn GatewayInboundProtocol>,
    pub gateway_pointer: GatewayPointer,
    pub output_mode: OM,
}

pub struct StuffedOutputMode {}

impl StuffedOutputMode {
    pub fn new() -> Self {
        StuffedOutputMode {}
    }
}

impl WasmEnvOutputMode for StuffedOutputMode {
    fn return_output(
        &self,
        _latest_messsage: CircuitOutboundMessage,
        _previous_ctx_messages: &mut Vec<CircuitOutboundMessage>,
    ) -> Option<Vec<u8>> {
        unimplemented!()
    }
    fn return_dispatch_output(
        &self,
        _latest_messsage: CircuitOutboundMessage,
        _previous_ctx_messages: &mut Vec<CircuitOutboundMessage>,
    ) -> Result<(), DispatchError> {
        unimplemented!()
    }

    fn get_run_mode() -> RunMode {
        RunMode::Post
    }
}
pub struct PessimisticOutputMode {}

impl PessimisticOutputMode {
    pub fn new() -> Self {
        PessimisticOutputMode {}
    }
}

impl WasmEnvOutputMode for PessimisticOutputMode {
    fn return_output(
        &self,
        _latest_messsage: CircuitOutboundMessage,
        _previous_ctx_messages: &mut Vec<CircuitOutboundMessage>,
    ) -> Option<Vec<u8>> {
        unimplemented!()
    }
    fn return_dispatch_output(
        &self,
        _latest_messsage: CircuitOutboundMessage,
        _previous_ctx_messages: &mut Vec<CircuitOutboundMessage>,
    ) -> Result<(), DispatchError> {
        unimplemented!()
    }

    fn get_run_mode() -> RunMode {
        RunMode::Pre
    }
}

pub struct OptimisticOutputMode {}

impl OptimisticOutputMode {
    pub fn new() -> Self {
        OptimisticOutputMode {}
    }
}

impl WasmEnvOutputMode for OptimisticOutputMode {
    fn return_output(
        &self,
        _latest_messsage: CircuitOutboundMessage,
        _previous_ctx_messages: &mut Vec<CircuitOutboundMessage>,
    ) -> Option<Vec<u8>> {
        unimplemented!()
    }
    fn return_dispatch_output(
        &self,
        _latest_messsage: CircuitOutboundMessage,
        _previous_ctx_messages: &mut Vec<CircuitOutboundMessage>,
    ) -> Result<(), DispatchError> {
        unimplemented!()
    }

    fn get_run_mode() -> RunMode {
        RunMode::Dry
    }
}

pub trait WasmEnvOutputMode {
    fn return_output(
        &self,
        latest_messsage: CircuitOutboundMessage,
        previous_ctx_messages: &mut Vec<CircuitOutboundMessage>,
    ) -> Option<Vec<u8>>;

    fn return_dispatch_output(
        &self,
        latest_messsage: CircuitOutboundMessage,
        previous_ctx_messages: &mut Vec<CircuitOutboundMessage>,
    ) -> Result<(), DispatchError>;

    fn get_run_mode() -> RunMode;
}
//
// impl<'a, T: EscrowTrait + SystemTrait, OM> ExtStandards for CircuitVersatileWasmEnv<'a, T, OM>
// where
//     T: EscrowTrait + SystemTrait + VersatileWasm + CircuitTrait,
//     // GII: GatewayInboundImplementer<GA>,
//     OM: WasmEnvOutputMode,
// {
//     type T = T;
//
//     fn get_storage(&mut self, key: &StorageKey) -> Result<Option<Vec<u8>>, DispatchError> {
//         // Could use a macro to:
//         // 1. access gateway_inbound_protocol
//         // 2. store outbound message in constructed_outbound_messages stack
//         // 3. return result using output_mode
//         let outbound_message = self
//             .gateway_inbound_protocol
//             .get_storage((*key).to_vec(), self.gateway_pointer.gateway_type.clone())?;
//
//         self.constructed_outbound_messages
//             .push(outbound_message.clone());
//
//         Ok(self
//             .output_mode
//             .return_output(outbound_message, self.constructed_outbound_messages)
//             .into())
//     }
//
//     fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>) -> DispatchResult {
//         let outbound_message = self.gateway_inbound_protocol.set_storage(
//             key.to_vec(),
//             value,
//             self.gateway_pointer.gateway_type.clone(),
//         )?;
//
//         self.constructed_outbound_messages
//             .push(outbound_message.clone());
//
//         self.output_mode
//             .return_output(outbound_message, self.constructed_outbound_messages);
//         Ok(())
//     }
//
//     fn get_raw_storage(&self, key: &StorageKey) -> Option<Vec<u8>> {
//         unhashed::get_raw(key)
//     }
//
//     fn set_raw_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>) -> DispatchResult {
//         Ok(match value {
//             Some(new_value) => unhashed::put_raw(&key, &new_value[..]),
//             None => unhashed::kill(&key),
//         }
//         .into())
//     }
//
//     fn get_child_storage(&self, child: ChildInfo, key: &StorageKey) -> Option<Vec<u8>> {
//         child::get_raw(&child, key)
//     }
//
//     fn set_child_storage(&mut self, child: ChildInfo, key: StorageKey, value: Option<Vec<u8>>) {
//         match value {
//             Some(new_value) => child::put_raw(&child, &key, &new_value[..]),
//             None => child::kill(&child, &key),
//         }
//     }
//
//     fn transfer(
//         &mut self,
//         to: &T::AccountId,
//         value: BalanceOf<T>,
//         _gas_meter: &mut GasMeter<T>,
//     ) -> Result<(), DispatchError> {
//         let outbound_message = self.gateway_inbound_protocol.transfer_escrow(
//             self.escrow_account.encode(),
//             self.requester.encode(),
//             to.clone().encode(),
//             <T as CircuitTrait>::ToStandardizedGatewayBalance::convert(value).encode(),
//             self.inner_exec_transfers,
//             self.gateway_pointer.gateway_type.clone(),
//         )?;
//
//         self.constructed_outbound_messages
//             .push(outbound_message.clone());
//
//         self.output_mode
//             .return_dispatch_output(outbound_message, self.constructed_outbound_messages)
//     }
//
//     fn call(
//         &mut self,
//         module_name: &str,
//         fn_name: &str,
//         to: &T::AccountId,
//         value: BalanceOf<T>,
//         gas_meter: &mut GasMeter<T>,
//         data: Vec<u8>,
//     ) -> Result<(), DispatchError> {
//         let outbound_message = self.gateway_inbound_protocol.call(
//             module_name.as_bytes().to_vec(),
//             fn_name.as_bytes().to_vec(),
//             data,
//             <T as CircuitTrait>::AccountId32Converter::convert(self.escrow_account.clone()),
//             <T as CircuitTrait>::AccountId32Converter::convert(self.requester.clone()),
//             <T as CircuitTrait>::AccountId32Converter::convert(to.clone()),
//             <T as CircuitTrait>::ToStandardizedGatewayBalance::convert(value).into(),
//             gas_meter.gas_left(),
//             self.gateway_pointer.gateway_type.clone(),
//             None,
//         )?;
//
//         self.constructed_outbound_messages
//             .push(outbound_message.clone());
//
//         self.output_mode
//             .return_dispatch_output(outbound_message, self.constructed_outbound_messages)
//     }
//
//     fn deposit_event(&mut self, topics: Vec<TopicOf<Self::T>>, data: Vec<u8>) {
//         <frame_system::Pallet<T>>::deposit_event_indexed(
//             &*topics,
//             <Self::T as VersatileWasm>::Event::from(
//                 versatile_wasm::pallet::Event::VersatileVMExecution(
//                     self.escrow_account.clone(),
//                     self.requester.clone(),
//                     data,
//                 ),
//             )
//             .into(),
//         )
//     }
// }
//
// impl<'a, T: EscrowTrait + SystemTrait, OM> CircuitVersatileWasmEnv<'a, T, OM>
// where
//     T: EscrowTrait + SystemTrait + VersatileWasm + CircuitTrait,
//     OM: WasmEnvOutputMode,
// {
//     pub fn new(
//         escrow_account: &'a T::AccountId,
//         requester: &'a T::AccountId,
//         block_number: <T as SystemTrait>::BlockNumber,
//         timestamp: <<T as EscrowTrait>::Time as Time>::Moment,
//         storage_trie_id: T::Hash,
//         input_data: Option<Vec<u8>>,
//         inner_exec_transfers: &'a mut Vec<TransferEntry>,
//         constructed_outbound_messages: &'a mut Vec<CircuitOutboundMessage>,
//         gateway_inbound_protocol: Box<dyn GatewayInboundProtocol>,
//         gateway_pointer: GatewayPointer,
//         output_mode: OM,
//     ) -> Self {
//         CircuitVersatileWasmEnv {
//             escrow_account,
//             requester,
//             block_number,
//             timestamp,
//             storage_trie_id,
//             input_data,
//             inner_exec_transfers,
//             constructed_outbound_messages,
//             gateway_inbound_protocol,
//             gateway_pointer,
//             output_mode,
//         }
//     }
// }
