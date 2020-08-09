#![cfg_attr(not(feature = "std"), no_std)]

pub enum Phase {
    Execute,
    Commit,
    Revert
}

/**
CALL SCHEMA:
	origin: AccountId,
	dest: AccountId,
	value: Balance,
	gas_limit: u64,
	input_data: Vec<u8>,
	exec_ctx: Vec<u8>,

GET STORAGE SCHEMA
	address: AccountId,
	key: H256,
	at: Option<BlockHash>,
	exec_ctx: Vec<u8>,

RENT PROJECTION SCHEMA
	address: AccountId, // Here should read the size taken as a branch of this address on escrow storage.
	at: Option<BlockHash>,
	exec_ctx: Vec<u8>,

struct CallSchema {
	origin: AccountId,
	dest: AccountId,
	value: Balance,
	gas_limit: u64,
	input_data: Vec<u8>,
	// All of the relevant info for Gatway will be encoded into input_data:
	/// exec_step_ctx: Vec<u8>
}
**/

/// ToDo: Package as an argument means every time follow do: instantiate -> execute -> destroy
/// ToDo: Target Accounts as the original callee vs the destination account for example for transfers.
/// 	- for manual transfers -- original callee needs then to be set as origin (check whether its signature is required). !BARE_CALL VS CALL MAKES THE DIFFERENCE.
/// 	- for auto transfers -- origin will stay as escrow account.
/// 	- api.call -> bare_call -> execute_wasm -> ExecutionContext::top_level => loads origin as self account and charges for execution.
pub struct StepInput {
	phase: Phase,
	// input_args: Option<Bytes>,
	// target_accounts: Vec<AccountId>,
	// package: Bytes,
}
