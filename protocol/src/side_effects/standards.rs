#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::side_effects::confirm::protocol::SideEffectConfirmationProtocol;

pub use crate::side_effects::protocol::{EventSignature, SideEffectProtocol, String};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::abi::Type;

/// ## The main idea would be to give a possibility of define the side effects dynamically
/// We'd have the "standard" side effects in the codebase, but for the sake of extensions,
/// the side effects should be made serialized, stored and pre-loaded before the exec pallet starts.
///
///
/// Regular transfers in the native to target currency (Eth @ Ethereum, Dot @ Polkadot etc.)
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct TransferSideEffectProtocol {}

/// Multi-asset transfers in the different to native currency (ERC-20s @ Ethereum, ACA @ Polkadot etc.)
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct MultiTransferSideEffectProtocol {}

/// Get data for a given key and confirm with storage proof from target chain
/// Use mainly to off-chain proof data before the on-chain execution
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct GetDataSideEffectProtocol {}

/// Multi-asset swaps
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct SwapSideEffectProtocol {}

/// Multi-asset liquidity provision
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct AddLiquiditySideEffectProtocol {}

/// Call EVM on the target chain.
/// Will be made Reversible only of addressed locally on-circuit, otherwise are dirty - irreversible.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct CallEVMSideEffectProtocol {}

/// Call Pallet Contracts (WASM Contracts) on the target chain.
/// Will be made Reversible only of addressed locally on-circuit, otherwise are dirty - irreversible.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct CallWASMSideEffectProtocol {}

/// Call Composable Contracts from the on-chain Contracts Registry
/// Only local calls allowed.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct CallComposableSideEffectProtocol {}

/// Generic call - doesn't do much checks on the arguments
/// Use mainly for calling Runtime Pallets.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct CallSideEffectProtocol {}

/// Possibility to implement a custom reversible logic on the target chain.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct CallCustomSideEffectProtocol {}

/// ### Implement standard confirmation protocol for all standard Side Effects
impl SideEffectConfirmationProtocol for TransferSideEffectProtocol {}
// impl SideEffectConfirmationProtocol for MultiTransferSideEffectProtocol {}
impl SideEffectConfirmationProtocol for GetDataSideEffectProtocol {}
// impl SideEffectConfirmationProtocol for SwapSideEffectProtocol {}
// impl SideEffectConfirmationProtocol for AddLiquiditySideEffectProtocol {}
impl SideEffectConfirmationProtocol for CallEVMSideEffectProtocol {}
// impl SideEffectConfirmationProtocol for CallWASMSideEffectProtocol {}
impl SideEffectConfirmationProtocol for CallSideEffectProtocol {}
// impl SideEffectConfirmationProtocol for CallCustomSideEffectProtocol {}

/// ### Assign 4-bytes ID for each Side Effect
/// - TransferSideEffectProtocol = b"tran",
/// - MultiTransferSideEffectProtocol = b"mult",
/// - GetDataSideEffectProtocol = b"data",
/// - SwapTransferSideEffectProtocol = b"swap",
/// - AddLiquiditySideEffectProtocol = b"aliq",
/// - CallEVMTransferSideEffectProtocol = b"evmc",
/// - CallWASMTransferSideEffectProtocol = b"wasm",
/// - CallSideEffectProtocol = b"call",
/// - CallCustomSideEffectProtocol = b"cust",

/// Return the ones that are working for now :)
pub fn select_side_effect_by_id(id: [u8; 4]) -> Result<Box<dyn SideEffectProtocol>, &'static str> {
    match &id {
        b"tran" => Ok(Box::new(TransferSideEffectProtocol {})),
        b"data" => Ok(Box::new(GetDataSideEffectProtocol {})),
        _ => Err("Side Effect Selection: Unknown ID"),
    }
}

/// Return the ones that are working for now :)
pub fn get_all_standard_side_effects() -> Vec<Box<dyn SideEffectProtocol>> {
    vec![
        Box::new(TransferSideEffectProtocol {}),
        Box::new(GetDataSideEffectProtocol {}),
    ]
}

/// Return the ones that are working for now :)
pub fn get_all_experimental_side_effects(_id: [u8; 4]) -> Vec<Box<dyn SideEffectProtocol>> {
    vec![
        Box::new(TransferSideEffectProtocol {}),
        Box::new(GetDataSideEffectProtocol {}),
        Box::new(CallSideEffectProtocol {}),
        Box::new(CallEVMSideEffectProtocol {}),
    ]
}

impl SideEffectProtocol for TransferSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "transfer:dirty"
    }
    fn get_id(&self) -> [u8; 4] {
        *b"tran"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress,    // argument_0: from
            Type::DynamicAddress,    // argument_1: to
            Type::Value,             // argument_2: value
            Type::OptionalInsurance, // argument_3: insurance
        ]
    }
    /// ToDo: Protocol::Reversible x-t3rn#69 - !Inspect if from is doable here - the original transfer is from a user,
    ///     whereas the transfers on targets are made by relayers/executers.
    ///     Prefer to only inspect the the target
    /// ToDo: Protocol::Reversible - Support optional parameters like insurance. - must be hardcoded name
    ///         // vec!["from", "to", "value", "Option<insurance>"]
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec!["from", "to", "value", "insurance"]
    }
    fn get_confirming_events(&self) -> Vec<&'static str> {
        vec!["Transfer(from,to,value)"]
    }
    /// This event must be emitted by Escrow Contracts
    fn get_escrowed_events(&self) -> Vec<&'static str> {
        vec!["EscrowTransfer(from,to,value)"]
    }
    /// This event must be emitted by Insurance Submodule on pallet-circuit x-t3rn#44
    /// ToDo: Protocol::Reversible x-t3rn#69 - get_reversible_exec should also populate additional parameters to the LocalState
    ///     e.g here executer that insures transfer wasn't known before the execution.
    fn get_reversible_exec(&self) -> Vec<&'static str> {
        vec!["InsuredTransfer(Append<executer>,to,insurance)"]
    }
    fn get_reversible_commit(&self) -> Vec<&'static str> {
        vec!["Transfer(executer,to,value)"]
    }
    /// ToDo: Protocol::Reversible x-t3rn#69 - If executers wants to avoid loosing their insurance deposit, must return the funds to the original user
    ///     - that's problematic since we don't know user's address on target
    ///     Temp. solution before the locks in wrapped tokens on Circuit is to leave it empty
    ///     and Commit relayer to perform the transfer for 100% or they loose insured deposit
    fn get_reversible_revert(&self) -> Vec<&'static str> {
        vec!["Transfer(executer,from,value)"]
    }
}

impl SideEffectProtocol for CallSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "call:generic"
    }
    fn get_id(&self) -> [u8; 4] {
        *b"call"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress, // argument_0: from
            Type::DynamicAddress, // argument_1: dest
            Type::Value,          // argument_2: value
            Type::DynamicBytes,   // argument_2: value
        ]
    }
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec!["from", "dest", "value", "data"]
    }
    fn get_confirming_events(&self) -> Vec<&'static str> {
        // ToDo: Protocol:GenericEvents - support unknown event name and random order of args??
        vec!["<Unknown>(from,to,value,data)"]
    }
}

impl SideEffectProtocol for CallEVMSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "call:generic"
    }
    fn get_id(&self) -> [u8; 4] {
        *b"call"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress, // argument_0: source
            Type::DynamicAddress, // argument_1: target
            Type::DynamicBytes,   // argument_2: target
            Type::Value,          // argument_3: value
            Type::Uint(64),       // argument_4: gas_limit
            Type::Value,          // argument_5: max_fee_per_gas
            Type::Value,          // argument_6: max_priority_fee_per_gas
            Type::Value,          // argument_7: nonce
            Type::DynamicBytes,   // argument_8: access_list (since HF Berlin?)
        ]
    }
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec![
            "source",
            "target",
            "input",
            "value",
            "gas_limit",
            "max_fee_per_gas",
            "max_priority_fee_per_gas",
            "nonce",
            "access_list",
        ]
    }

    fn get_confirming_events(&self) -> Vec<&'static str> {
        // ToDo: Protocol:DynamicParams - support unknown/dynamic argument
        vec!["TransactCall(Append<caller>,source,value,input,gas_limit)"]
    }
}

/// ToDo: Protocol:GetData - support storage reads for the sake of data read - must all come with storage_proof,
/// the execution is unknown before so it's enough for ExecDelivery to only verify the inclusion and skip Execution Verify for the "<InclusionOnly>" Key
impl SideEffectProtocol for GetDataSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "data:get"
    }
    fn get_id(&self) -> [u8; 4] {
        *b"data"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicBytes, // argument_0: key
        ]
    }
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec!["key"]
    }
    fn get_confirming_events(&self) -> Vec<&'static str> {
        vec!["<InclusionOnly>"]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn selector_successfully_selects_get_data_by_id() {
        let valid_and_known_action_id = *b"data";
        let res = select_side_effect_by_id(valid_and_known_action_id).unwrap();

        assert_eq!(res.get_id(), *b"data")
    }
}
