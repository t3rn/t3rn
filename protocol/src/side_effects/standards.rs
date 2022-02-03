#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::side_effects::confirm::protocol::SideEffectConfirmationProtocol;

pub use crate::side_effects::protocol::{EventSignature, SideEffectProtocol, String};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::boxed::Box;
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
impl SideEffectConfirmationProtocol for SwapSideEffectProtocol {}
impl SideEffectConfirmationProtocol for AddLiquiditySideEffectProtocol {}
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
        b"swap" => Ok(Box::new(SwapSideEffectProtocol {})),
        b"aliq" => Ok(Box::new(AddLiquiditySideEffectProtocol {})),
        _ => Err("Side Effect Selection: Unknown ID"),
    }
}

/// Return the ones that are working for now :)
pub fn get_all_standard_side_effects() -> Vec<Box<dyn SideEffectProtocol>> {
    vec![
        Box::new(TransferSideEffectProtocol {}),
        Box::new(GetDataSideEffectProtocol {}),
        Box::new(SwapSideEffectProtocol {}),
        Box::new(AddLiquiditySideEffectProtocol {}),
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
/// !Insurance is enacted on in the upper layer Circuit, where the Executor and XtxID + SideEffect are matched.
impl SideEffectProtocol for SwapSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "swap"
    }
    fn get_id(&self) -> [u8; 4] {
        *b"swap"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress,    // argument_0: caller
            Type::DynamicAddress,    // argument_1: to
            Type::Value,             // argument_2: amount_from
            Type::Value,             // argument_3: amount_to
            Type::DynamicBytes,      // argument_4: asset_from
            Type::DynamicBytes,      // argument_5: asset_to
            Type::OptionalInsurance, // argument_6: insurance
        ]
    }
    /// Supports optional parameters - insurance.
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec![
            "caller",
            "to",
            "amount_from",
            "amount_to",
            "asset_from",
            "asset_to",
            "insurance",
        ]
    }

    fn get_confirming_events(&self) -> Vec<&'static str> {
        vec!["ExecuteToken(_executor,to,asset_to,amount_to)"]
    }
    /// This event must be emitted by Escrow Contracts
    /// Info: XtxID is checked in the upper context of Circuit, where the Side Effect + XtxId are matched.
    fn get_escrowed_events(&self) -> Vec<&'static str> {
        vec!["ExecuteToken(_executor,to,asset_to,amount_to)"]
    }

    fn get_reversible_commit(&self) -> Vec<&'static str> {
        vec!["MultiTransfer(executor,to,asset_to,amount_to)"]
    }
    /// ToDo: Protocol::Reversible x-t3rn#69 - If executors wants to avoid loosing their insurance deposit, must return the funds to the original user
    ///     - that's problematic since we don't know user's address on target
    ///     Temp. solution before the locks in wrapped tokens on Circuit is to leave it empty
    ///     and Commit relayer to perform the transfer for 100% or they loose insured deposit
    fn get_reversible_revert(&self) -> Vec<&'static str> {
        // Or introducing swap_value:
        //      vec!["Transfer(executor,caller,swap_value"]
        vec!["MultiTransfer(executor,caller,asset_from,amount_from)"]
    }
}

impl SideEffectProtocol for AddLiquiditySideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "add_liquidity"
    }
    fn get_id(&self) -> [u8; 4] {
        *b"aliq"
    }

    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress,    // argument_0: caller
            Type::DynamicAddress,    // argument_1: to
            Type::DynamicBytes,      // argument_2: asset_left
            Type::DynamicBytes,      // argument_3: asset_right
            Type::DynamicBytes,      // argument_4: liquidity_token
            Type::Value,             // argument_5: amount_left
            Type::Value,             // argument_6: amount_right
            Type::Value,             // argument_7: amount_liquidity_token
            Type::OptionalInsurance, // argument_8: insurance
        ]
    }
    /// Supports optional parameters - insurance.
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec![
            "caller",
            "to",
            "asset_left",
            "asset_right",
            "liquidity_token",
            "amount_left",
            "amount_right",
            "amount_liquidity_token",
            "insurance",
        ]
    }

    fn get_confirming_events(&self) -> Vec<&'static str> {
        vec!["ExecuteToken(executor,to,liquidity_token,amount_liquidity_token)"]
    }
    /// This event must be emitted by Escrow Contracts
    /// Info: XtxID is checked in the upper context of Circuit, where the Side Effect + XtxId are matched.
    fn get_escrowed_events(&self) -> Vec<&'static str> {
        vec!["ExecuteToken(xtx_id,to,liquidity_token,amount_liquidity_token)"]
    }

    fn get_reversible_commit(&self) -> Vec<&'static str> {
        vec!["MultiTransfer(executor,to,liquidity_token,amount_liquidity_token)"]
    }
    /// ToDo: Protocol::Reversible x-t3rn#69 - If executors wants to avoid loosing their insurance deposit, must return the funds to the original user
    ///     - that's problematic since we don't know user's address on target
    ///     Temp. solution before the locks in wrapped tokens on Circuit is to leave it empty
    ///     and Commit relayer to perform the transfer for 100% or they loose insured deposit
    fn get_reversible_revert(&self) -> Vec<&'static str> {
        // a) Could be either returning LP-token back to the caller: altough the caller doesn't
        // necessarily has the address on the target + doesn't really set back the effects of
        // adding liquidity. Discard for now.
        //      vec!["MultiTransfer(executor,caller,amount_from)"]
        // b) Or return both wrapped assets back to the caller. !Warning! Assumes caller providing
        // wrapper assets into Circuit.
        // Provide both ways b) and c) -> with wrapped tokens and automatic liqudity out of pool
        // value
        vec![
            "MultiTransfer(executor,caller,asset_left,amount_left)",
            "MultiTransfer(executor,caller,asset_right,amount_right)",
        ]
        // c) Or return the total value in TRN token back to caller - requires a different type of
        // adding liquidity accepting the total value as locked value parameter that will be
        // converted equally into left + right assets of liquidity pool
        //      vec!["Transfer(executor,to,pool_value)"]
    }
}

impl SideEffectProtocol for TransferSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "transfer"
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
    ///     whereas the transfers on targets are made by relayers/executors.
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
    fn get_reversible_commit(&self) -> Vec<&'static str> {
        vec!["Transfer(executor,to,value)"]
    }
    /// ToDo: Protocol::Reversible x-t3rn#69 - If executors wants to avoid loosing their insurance deposit, must return the funds to the original user
    ///     - that's problematic since we don't know user's address on target
    ///     Temp. solution before the locks in wrapped tokens on Circuit is to leave it empty
    ///     and Commit relayer to perform the transfer for 100% or they loose insured deposit
    fn get_reversible_revert(&self) -> Vec<&'static str> {
        vec!["Transfer(executor,from,value)"]
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
            Type::DynamicBytes,   // argument_3: input
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
