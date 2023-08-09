// use codec::Encode;
// use primitive_types::H256;
// use sha3::{Digest, Keccak256};
// use sp_core::crypto::AccountId32;
// use sp_runtime::traits::Hash;
// use t3rn_primitives::{
//     contract_metadata::ContractMetadata,
//     contracts_registry::{AuthorInfo, RegistryContract},
//     storage::CodeHash,
// };
//
// pub const GAS_LIMIT: u64 = 5_000_000;
// pub const CHARLIE: AccountId32 = AccountId32::new([3u8; 32]);
// pub const _DJANGO: AccountId32 = AccountId32::new([4u8; 32]);
// pub const ESCROW: AccountId32 = AccountId32::new([15u8; 32]);
//
// /// Extracts a contract id from an event, if $addr is some, it takes the event, otherwise it takes
// /// the last event from the system.
// #[cfg(test)]
// #[macro_export]
// macro_rules! take_created_contract_id_from_event {
//     ($addr:expr) => {{
//         use pallet_contracts_registry::Event as ContractsRegistryEvent;
//         use $crate::mock::System;
//
//         let last_event = match $addr {
//             Some(addr) => addr,
//             None => {
//                 let events = System::events();
//                 events.last().unwrap().clone()
//             },
//         };
//
//         if let <Test as frame_system::Config>::RuntimeEvent::ContractsRegistry(
//             ContractsRegistryEvent::ContractStored(_, addr),
//         ) = last_event.event
//         {
//             addr.clone()
//         } else {
//             panic!("Unexpected event: {:?}", last_event);
//         }
//     }};
// }
//
// pub(crate) fn create_test_registry_contract<T: frame_system::Config>(
//     wasm: Vec<u8>,
//     code_hash: &<T::Hashing as Hash>::Output,
//     author: AccountId32,
//     author_fees: Option<u64>,
//     meta: Option<ContractMetadata>,
// ) -> RegistryContract<CodeHash<T>, AccountId32, u64, BlockNumberFor<T>> {
//     RegistryContract::new(
//         code_hash.clone().encode(),
//         wasm,
//         AuthorInfo::new(author, author_fees),
//         None,
//         vec![],
//         None,
//         meta.unwrap_or_default(),
//     )
// }
//
// pub fn compile_module<T>(fixture_name: &str) -> (Vec<u8>, H256)
// where
//     T: frame_system::Config,
// {
//     let fixture_path = ["fixtures/", fixture_name, ".json"].concat();
//     let blob = std::fs::read(fixture_path).unwrap();
//     let solidity: solidity::SolidityMetadata = serde_json::from_slice(&blob).unwrap();
//     let code_hash = H256::from_slice(Keccak256::digest(&blob).as_slice());
//
//     let contract = solidity
//         .contracts
//         .get(&format!("{fixture_name}.sol:{fixture_name}"))
//         .unwrap();
//     (contract.bin.clone(), code_hash)
// }
//
// pub fn initialize_block(number: u32) {
//     crate::tests::System::reset_events();
//     crate::tests::System::initialize(&number, &[0u8; 32].into(), &Default::default());
// }
//
// mod solidity {
//     use serde::{Deserialize, Serialize};
//     use std::collections::HashMap;
//
//     type Contracts = HashMap<String, Contract>;
//     #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct SolidityMetadata {
//         pub contracts: Contracts,
//         pub version: String,
//     }
//
//     #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct Contract {
//         pub abi: Vec<Abi>,
//         #[serde(with = "hex::serde")]
//         pub bin: Vec<u8>,
//         #[serde(rename = "bin-runtime", with = "hex::serde")]
//         pub bin_runtime: Vec<u8>,
//         pub hashes: Hashes,
//         pub metadata: serde_json::Value,
//     }
//
//     #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct Abi {
//         pub inputs: Vec<Input>,
//         pub name: String,
//         pub outputs: Vec<Output>,
//         pub state_mutability: String,
//         #[serde(rename = "type")]
//         pub type_field: String,
//     }
//
//     #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct Input {
//         pub internal_type: String,
//         pub name: String,
//         #[serde(rename = "type")]
//         pub type_field: String,
//     }
//
//     #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct Output {
//         pub internal_type: String,
//         pub name: String,
//         #[serde(rename = "type")]
//         pub type_field: String,
//     }
//
//     #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct Hashes {
//         #[serde(rename = "retrieve()")]
//         pub retrieve: String,
//         #[serde(rename = "store(uint256)")]
//         pub store_uint256: String,
//     }
// }
