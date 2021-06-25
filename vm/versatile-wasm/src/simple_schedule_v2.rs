#![cfg_attr(not(feature = "std"), no_std)]

use crate::gas::Gas;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
/// Definition of the cost schedule and other parameterizations for wasm vm.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct Schedule {
    /// Version of the schedule.
    pub version: u32,

    /// Cost of putting a byte of code into storage.
    pub put_code_per_byte_cost: Gas,

    /// Gas cost of a growing memory by single page.
    pub grow_mem_cost: Gas,

    /// Gas cost of a regular operation.
    pub regular_op_cost: Gas,

    /// Gas cost per one byte returned.
    pub return_data_per_byte_cost: Gas,

    /// Gas cost to deposit an event; the per-byte portion.
    pub event_data_per_byte_cost: Gas,

    /// Gas cost to deposit an event; the cost per topic.
    pub event_per_topic_cost: Gas,

    /// Gas cost to deposit an event; the base.
    pub event_base_cost: Gas,

    /// Base gas cost to call into a contract.
    pub call_base_cost: Gas,

    /// Base gas cost to instantiate a contract.
    pub instantiate_base_cost: Gas,

    /// Base gas cost to dispatch a runtime call.
    pub dispatch_base_cost: Gas,

    /// Gas cost per one byte read from the sandbox memory.
    pub sandbox_data_read_cost: Gas,

    /// Gas cost per one byte written to the sandbox memory.
    pub sandbox_data_write_cost: Gas,

    /// Cost for a simple balance transfer.
    pub transfer_cost: Gas,

    /// Cost for instantiating a new contract.
    pub instantiate_cost: Gas,

    /// The maximum number of topics supported by an event.
    pub max_event_topics: u32,

    /// Maximum allowed stack height.
    ///
    /// See https://wiki.parity.io/WebAssembly-StackHeight to find out
    /// how the stack frame cost is calculated.
    pub max_stack_height: u32,

    /// Maximum number of memory pages allowed for a contract.
    pub max_memory_pages: u32,

    /// Maximum allowed size of a declared table.
    pub max_table_size: u32,

    /// Whether the `seal_println` function is allowed to be used contracts.
    /// MUST only be enabled for `dev` chains, NOT for production chains
    pub enable_println: bool,

    /// The maximum length of a subject used for PRNG generation.
    pub max_subject_len: u32,

    /// The maximum length of a contract code in bytes. This limit applies to the uninstrumented
    // and pristine form of the code as supplied to `put_code`.
    pub max_code_size: u32,
}

// 500 (2 instructions per nano second on 2GHZ) * 1000x slowdown through wasmi
// This is a wild guess and should be viewed as a rough estimation.
// Proper benchmarks are needed before this value and its derivatives can be used in production.
const WASM_INSTRUCTION_COST: Gas = 500_000;

impl Default for Schedule {
    fn default() -> Schedule {
        Schedule {
            version: 0,
            put_code_per_byte_cost: WASM_INSTRUCTION_COST,
            grow_mem_cost: WASM_INSTRUCTION_COST,
            regular_op_cost: WASM_INSTRUCTION_COST,
            return_data_per_byte_cost: WASM_INSTRUCTION_COST,
            event_data_per_byte_cost: WASM_INSTRUCTION_COST,
            event_per_topic_cost: WASM_INSTRUCTION_COST,
            event_base_cost: WASM_INSTRUCTION_COST,
            call_base_cost: 135 * WASM_INSTRUCTION_COST,
            dispatch_base_cost: 135 * WASM_INSTRUCTION_COST,
            instantiate_base_cost: 175 * WASM_INSTRUCTION_COST,
            sandbox_data_read_cost: WASM_INSTRUCTION_COST,
            sandbox_data_write_cost: WASM_INSTRUCTION_COST,
            transfer_cost: 100 * WASM_INSTRUCTION_COST,
            instantiate_cost: 200 * WASM_INSTRUCTION_COST,
            max_event_topics: 4,
            max_stack_height: 64 * 1024,
            max_memory_pages: 16,
            max_table_size: 16 * 1024,
            enable_println: false,
            max_subject_len: 32,
            max_code_size: 512 * 1024,
        }
    }
}
