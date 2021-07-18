#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchError;
use frame_support::traits::Time;

use t3rn_primitives::{
    abi::GatewayABIConfig, transfers::TransferEntry, EscrowTrait, GatewayPointer, *,
};

use sp_std::boxed::Box;
use sp_std::vec::*;

use frame_system::Config as SystemTrait;

use volatile_vm::{wasm::RunMode, VolatileVM};

use crate::Config as CircuitTrait;

pub use crate::message_assembly::gateway_inbound_assembly::GatewayInboundAssembly;
pub use crate::message_assembly::substrate_gateway_assembly::SubstrateGatewayAssembly;
pub use crate::message_assembly::substrate_gateway_protocol::SubstrateGatewayProtocol;

pub struct CircuitVolatileWasmEnv<'a, T: EscrowTrait + SystemTrait + VolatileVM + CircuitTrait, OM>
{
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
    pub gateway_abi: GatewayABIConfig,
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
