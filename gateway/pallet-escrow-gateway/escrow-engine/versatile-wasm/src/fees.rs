use crate::gas::{Gas, GasMeter, GasMeterResult, Token};
use crate::VersatileWasm as Trait;
use crate::*;
use contracts::Schedule;
use frame_support::dispatch::DispatchError;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Copy, Clone)]
pub enum RuntimeToken {
    /// Explicit call to the `gas` function. Charge the gas meter
    /// with the value provided.
    Explicit(u32),
    /// The given number of bytes is read from the sandbox memory.
    ReadMemory(u32),
    /// The given number of bytes is written to the sandbox memory.
    WriteMemory(u32),
    /// The given number of bytes is read from the sandbox memory and
    /// is returned as the return data buffer of the call.
    ReturnData(u32),
    /// (topic_count, data_bytes): A buffer of the given size is posted as an event indexed with the
    /// given number of topics.
    DepositEvent(u32, u32),
}

impl<T: Trait> Token<T> for RuntimeToken {
    type Metadata = Schedule;

    fn calculate_amount(&self, metadata: &Schedule) -> Gas {
        use self::RuntimeToken::*;
        let value = match *self {
            Explicit(amount) => Some(amount.into()),
            ReadMemory(byte_count) => metadata
                .sandbox_data_read_cost
                .checked_mul(byte_count.into()),
            WriteMemory(byte_count) => metadata
                .sandbox_data_write_cost
                .checked_mul(byte_count.into()),
            ReturnData(byte_count) => metadata
                .return_data_per_byte_cost
                .checked_mul(byte_count.into()),
            DepositEvent(topic_count, data_byte_count) => {
                let data_cost = metadata
                    .event_data_per_byte_cost
                    .checked_mul(data_byte_count.into());

                let topics_cost = metadata
                    .event_per_topic_cost
                    .checked_mul(topic_count.into());

                data_cost
                    .and_then(|data_cost| {
                        topics_cost.and_then(|topics_cost| data_cost.checked_add(topics_cost))
                    })
                    .and_then(|data_and_topics_cost| {
                        data_and_topics_cost.checked_add(metadata.event_base_cost)
                    })
            }
        };

        value.unwrap_or_else(|| sp_runtime::traits::Bounded::max_value())
    }
}

/// Charge the gas meter with the specified token.
///
/// Returns `Err(HostError)` if there is not enough gas.
pub fn charge_gas<T: Trait, Tok: Token<T>>(
    gas_meter: &mut GasMeter<T>,
    metadata: &Tok::Metadata,
    trap_reason: &mut Option<TrapReason>,
    token: Tok,
) -> Result<(), sp_sandbox::HostError> {
    match gas_meter.charge(metadata, token) {
        GasMeterResult::Proceed => Ok(()),
        GasMeterResult::OutOfGas => {
            *trap_reason = Some(TrapReason::SupervisorError(DispatchError::Other(
                "Out of gas",
            )));
            Err(sp_sandbox::HostError)
        }
    }
}
