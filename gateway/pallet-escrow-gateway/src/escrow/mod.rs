use crate::{CodeHash, Trait};
use codec::{Decode, Encode};
use contracts::Schedule;
use sp_runtime::traits::Block;
use sp_std::prelude::*;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode)]
pub struct ContractsEscrowEngine {}

pub type Error = ();

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct EscrowExecuteResult {
    result: Vec<u8>,
}

impl ContractsEscrowEngine {
    pub fn new() -> Self {
        ContractsEscrowEngine {}
    }

    // Executes the wasm code and copies all of the changes made to the temporary account created for the contract.
    pub fn execute(&self, input: Vec<u8>) -> Result<EscrowExecuteResult, Error> {
        Ok(EscrowExecuteResult { result: input })
    }

    pub fn revert(&self) -> u32 {
        99 as u32
    }

    pub fn feed_contract_from_escrow(&self) -> u32 {
        33 as u32
    }

    pub fn feed_escrow_from_contract(&self) -> u32 {
        22 as u32
    }
}
