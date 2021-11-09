#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchError;

use t3rn_primitives::*;

use sp_std::vec::*;

use volatile_vm::wasm::RunMode;

pub use t3rn_protocol::gateway_inbound_assembly::GatewayInboundAssembly;
pub use t3rn_protocol::substrate_gateway_assembly::SubstrateGatewayAssembly;
pub use t3rn_protocol::substrate_gateway_protocol::SubstrateGatewayProtocol;

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
