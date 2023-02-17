use crate::abi::{decode_buf2val, GatewayABIConfig, Type};
pub use crate::{
    bid::SFXBid,
    fsx::{FullSideEffect, SideEffectId, SideEffectInterface},
};
use bytes::Buf;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::{
    prelude::{fmt::Debug, vec, vec::Vec},
    TypeInfo,
};
use sp_runtime::DispatchError;
use sp_std::borrow::ToOwned;

#[cfg(feature = "runtime")]
use num::Zero;
#[cfg(feature = "runtime")]
use scale_info::prelude::collections::VecDeque;

pub type TargetId = [u8; 4];
pub type Bytes = Vec<u8>;
pub type EventSignature = Bytes;
pub type SideEffectName = Bytes;

pub const COMPOSABLE_CALL_SIDE_EFFECT_ID: &[u8; 4] = b"comp";
pub const WASM_CALL_SIDE_EFFECT_ID: &[u8; 4] = b"wasm";
pub const EVM_CALL_SIDE_EFFECT_ID: &[u8; 4] = b"cevm";
pub const CALL_SIDE_EFFECT_ID: &[u8; 4] = b"call";
pub const ORML_TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"orml";
pub const ASSETS_TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"tass";
pub const MULTI_TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"mult";
pub const TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"tran";
pub const ADD_LIQUIDITY_SIDE_EFFECT_ID: &[u8; 4] = b"aliq";
pub const SWAP_SIDE_EFFECT_ID: &[u8; 4] = b"swap";
pub const DATA_SIDE_EFFECT_ID: &[u8; 4] = b"data";

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
pub struct SideEffect<AccountId, BalanceOf> {
    pub target: TargetId,
    pub max_reward: BalanceOf,
    pub insurance: BalanceOf,
    pub encoded_action: Bytes,
    pub encoded_args: Vec<Bytes>,
    pub signature: Bytes,
    pub enforce_executor: Option<AccountId>,
    pub reward_asset_id: Option<u32>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
pub struct HardenedSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub target: TargetId,
    pub prize: BalanceOf,
    pub encoded_action: [u8; 4],
    pub encoded_args: Vec<Bytes>,
    pub encoded_args_abi: Vec<Type>,
    pub security_lvl: SecurityLvl,
    pub confirmation_outcome: Option<ConfirmationOutcome>,
    pub confirmed_executioner: Option<AccountId>,
    pub confirmed_received_at: Option<BlockNumber>,
    pub confirmed_cost: Option<BalanceOf>,
    pub index: u32,
}

impl<AccountId, BlockNumber, BalanceOf> Default
    for HardenedSideEffect<AccountId, BlockNumber, BalanceOf>
where
    AccountId: From<[u8; 32]>,
    BlockNumber: Default,
    BalanceOf: Default,
{
    fn default() -> Self {
        HardenedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: BalanceOf::default(),
            encoded_action: [0, 0, 0, 0],
            encoded_args: vec![],
            encoded_args_abi: vec![],
            security_lvl: SecurityLvl::Optimistic,
            confirmation_outcome: None,
            confirmed_executioner: None,
            confirmed_received_at: None,
            confirmed_cost: None,
            index: 0,
        }
    }
}

#[cfg(feature = "runtime")]
impl<AccountId, BalanceOf> SideEffect<AccountId, BalanceOf>
where
    AccountId: Encode,
    BalanceOf: Copy + Zero + Encode + Decode,
{
    pub fn generate_id<Hasher: sp_core::Hasher>(
        &self,
        xtx_id: &[u8], // would a slice also be fine here for XBI?
        sfx_index: u32,
    ) -> <Hasher as sp_core::Hasher>::Out {
        let mut sfx_id = xtx_id.to_vec();
        sfx_id.extend_from_slice(&sfx_index.to_be_bytes());
        // sfx_id.extend_from_slice(&self.encoded_args.to_be_bytes());

        Hasher::hash(sfx_id.as_slice())
    }

    pub fn id_as_bytes<Hasher: sp_core::Hasher>(id: <Hasher as sp_core::Hasher>::Out) -> Bytes {
        id.as_ref().to_vec()
    }

    pub fn read_interface(&self) -> Result<SideEffectInterface, DispatchError> {
        let decoded_action: [u8; 4] = decode_buf2val(self.encoded_action.to_owned())?;

        let sfx_interface: SideEffectInterface = match &decoded_action {
            DATA_SIDE_EFFECT_ID => crate::standard::get_data_interface(),
            TRANSFER_SIDE_EFFECT_ID => crate::standard::get_transfer_interface(),
            ORML_TRANSFER_SIDE_EFFECT_ID
            | ASSETS_TRANSFER_SIDE_EFFECT_ID
            | MULTI_TRANSFER_SIDE_EFFECT_ID => crate::standard::get_transfer_assets_interface(),
            SWAP_SIDE_EFFECT_ID => crate::standard::get_swap_interface(),
            // todo: add remove liquidity interface
            ADD_LIQUIDITY_SIDE_EFFECT_ID => crate::standard::get_add_liquidity_interface(),
            EVM_CALL_SIDE_EFFECT_ID => crate::standard::get_call_evm_interface(),
            WASM_CALL_SIDE_EFFECT_ID => crate::standard::get_call_wasm_interface(),
            COMPOSABLE_CALL_SIDE_EFFECT_ID | CALL_SIDE_EFFECT_ID =>
                crate::standard::get_call_composable_interface(),
            &_ => return Err("SFX::validate - type not recognized".into()),
        };

        Ok(sfx_interface)
    }

    pub fn validate(&self, abi: GatewayABIConfig) -> Result<TargetId, DispatchError> {
        // Validate optional insurance as the required last encoded argument for each SFX
        let last_arg = self
            .encoded_args
            .last()
            .ok_or("SFX::validate - Empty encoded args")?;

        let arg_insurance_and_reward: [u128; 2] = decode_buf2val(last_arg.to_owned())?;

        if arg_insurance_and_reward[0].encode() != self.insurance.encode() {
            return Err("SFX::validate - Invalid insurance".into())
        }
        if arg_insurance_and_reward[1].encode() != self.max_reward.encode() {
            return Err("SFX::validate - Invalid reward".into())
        }

        let decoded_action: [u8; 4] = decode_buf2val(self.encoded_action.to_owned())?;

        let sfx_interface: SideEffectInterface = self.read_interface()?;

        // Validate the encoded args against the ABI
        for (index, arg) in self.encoded_args.iter().enumerate() {
            let abi_type: &Type = sfx_interface
                .argument_abi
                .get(index)
                .ok_or("SFX::validate - Invalid number of encoded args")?;
            abi_type.eval_abi(arg.to_owned(), &abi)?;
        }

        Ok(decoded_action)
    }

    pub fn match_event_property_name_with_input_argument_name(
        &self,
        event_property_name: &Vec<u8>,
        sfx_interface: &SideEffectInterface,
    ) -> Result<Bytes, DispatchError> {
        match sfx_interface.argument_to_state_mapper.iter().position(|arg_name| {
            arg_name == event_property_name
        }) {
            Some(index) => {
                match self.encoded_args.get(index) {
                    Some(encoded_bytes) => Ok(encoded_bytes.to_owned()),
                    _ => Err("SFX::match_event_property_name_with_input_argument_name - Invalid argument index".into()),
                }
            },
            None => Err("SFX::match_event_property_name_with_input_argument_name - Argument name doesn't match input parameter".into()),
        }
    }

    pub fn confirm(
        &self,
        output_payload_arguments: Vec<Vec<u8>>,
        security_lvl: SecurityLvl,
        security_coordinates: Bytes,
        source: Bytes,
    ) -> Result<(), DispatchError> {
        let sfx_interface: SideEffectInterface = self.read_interface()?;

        let mut was_source_validated: bool = false;
        let expected_event_signature = &sfx_interface.get_confirming_events()[0];
        // Handle special case for vec!["<InclusionOnly>"]
        //  where the execution confirmation isn't required hence the side effect doesn't produce
        //  any event or event's arguments can't be known beforehand - like GetDataSideEffect
        if *expected_event_signature == b"<InclusionOnly>".encode() {
            return Ok(())
        }

        let (_, property_names) = crate::abi::extract_property_names_from_signature_as_bytes(
            expected_event_signature.encode(),
        )?;

        for (output_argument_index, property_name) in property_names.iter().enumerate() {
            //  Check each argument of decoded "encoded_remote_events" against the values from State
            //      Don't check arguments starts with "_" - (95u8). In case of dirty actions we don't know / care who the author is
            if property_name.is_empty() || property_name[0] == 95u8 {
                if property_name == b"_source" {
                    if security_coordinates != source.clone() {
                        return Err(
                            "Confirmation Failed - received event confirmed by different entity than selected Executor".into(),
                        )
                    }
                    was_source_validated = true;
                }
                continue
            }

            let encoded_input_argument = self.match_event_property_name_with_input_argument_name(
                property_name,
                &sfx_interface,
            )?;

            if *output_payload_arguments
                .get(output_argument_index)
                .ok_or("Confirmation Failed - expected payload args order not out of bounds")?
                != encoded_input_argument
            {
                return Err(
                    "Confirmation Failed - received event arguments differ from expected input"
                        .into(),
                )
            }
        }

        if security_lvl == SecurityLvl::Escrow && !was_source_validated {
            return Err(
                "Confirmation Failed - Escrowed SFX require source to be correctly derived from incoming events".into(),
            )
        }

        Ok(())
    }
}

#[cfg(feature = "runtime")]
/// Decode the side effect from encoded Chain.
impl<AccountId, BalanceOf> TryFrom<Vec<u8>> for SideEffect<AccountId, BalanceOf>
where
    AccountId: Encode + MaxEncodedLen,
    BalanceOf: Copy + Zero + Encode + Decode + MaxEncodedLen,
{
    type Error = &'static str;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut bytes: VecDeque<u8> = bytes.into();
        let mut take_next = || bytes.pop_front().ok_or("no more bytes");

        let target: TargetId = TargetByte(take_next()?).try_into()?;
        let action = Action::try_from(take_next()?)?;

        let bytes: Vec<u8> = bytes.into();
        let args = extract_args::<AccountId, BalanceOf, [u8; 32], Insurance<BalanceOf>>(
            &action,
            &mut bytes::Bytes::from(bytes),
        )?;
        let action_bytes: [u8; 4] = action.into();
        let action_bytes = action_bytes.encode();

        Ok(SideEffect::<AccountId, BalanceOf> {
            target,
            max_reward: Zero::zero(),
            encoded_action: action_bytes,
            encoded_args: args,
            signature: vec![],
            insurance: Zero::zero(),
            enforce_executor: None,
            reward_asset_id: None,
        })
    }
}

struct TargetByte(u8);

impl TryInto<TargetId> for TargetByte {
    type Error = &'static str;

    fn try_into(self) -> Result<TargetId, Self::Error> {
        match self.0 {
            0 => Ok(*b"ksma"),
            1 => Ok(*b"pdot"),
            2 => Ok(*b"karu"),
            3 => Ok(*b"t3rn"),
            _ => Err("Invalid target Id"),
        }
    }
}

enum Action {
    Transfer,
    TransferMulti,
    AddLiquidity,
    Swap,
    Call,
    CallEvm,
    CallWasm,
    CallComposable,
    Data,
}

impl TryFrom<u8> for Action {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Action::Transfer),
            1 => Ok(Action::TransferMulti),
            2 => Ok(Action::AddLiquidity),
            3 => Ok(Action::Swap),
            4 => Ok(Action::Call), // This needs to be structured nicer
            5 => Ok(Action::Data),
            _ => Err("Invalid action id"),
        }
    }
}

impl From<Action> for [u8; 4] {
    fn from(val: Action) -> Self {
        match val {
            Action::Transfer => *TRANSFER_SIDE_EFFECT_ID,
            Action::AddLiquidity => *ADD_LIQUIDITY_SIDE_EFFECT_ID,
            Action::Swap => *SWAP_SIDE_EFFECT_ID,
            Action::Call => *CALL_SIDE_EFFECT_ID,
            Action::CallEvm => *EVM_CALL_SIDE_EFFECT_ID,
            Action::CallWasm => *WASM_CALL_SIDE_EFFECT_ID,
            Action::CallComposable => *COMPOSABLE_CALL_SIDE_EFFECT_ID,
            Action::Data => *DATA_SIDE_EFFECT_ID,
            Action::TransferMulti => *MULTI_TRANSFER_SIDE_EFFECT_ID,
        }
    }
}

#[derive(Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, Debug)]
pub struct Insurance<Balance> {
    insurance: Balance,
    reward: Balance,
}

fn extract_args<
    AccountId: MaxEncodedLen,
    BalanceOf: MaxEncodedLen,
    Hash: MaxEncodedLen,
    Insurance: MaxEncodedLen,
>(
    action: &Action,
    bytes: &mut bytes::Bytes,
) -> Result<Vec<Bytes>, &'static str> {
    let mut args = Vec::new();

    match *action {
        Action::Transfer => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to

            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt

            take_insurance::<BalanceOf>(bytes, &mut args);

            Ok(args)
        },
        Action::AddLiquidity => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset_left
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset_right
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // liquidity_token
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_left
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_right
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_liquidity_token
            take_insurance::<BalanceOf>(bytes, &mut args);

            Ok(args)
        },
        Action::Swap => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_left
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_right
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset_left
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset_right
            take_insurance::<BalanceOf>(bytes, &mut args);

            Ok(args)
        },
        Action::Call => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // caller

            // now we check the VM
            match bytes.first() {
                Some(byte) if byte == &0_u8 => {
                    // its an evm op, get rid of that byte
                    bytes.advance(1);
                    args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // dest
                    args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec());
                    // value
                },
                Some(byte) if byte == &1_u8 => {
                    // its a wasm op, get rid of that byte
                    bytes.advance(1);
                    args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // dest
                    args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // value
                    args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // gas_limit

                    match bytes.first() {
                        Some(byte) if byte == &0_u8 => args.push(vec![*byte]),
                        Some(byte) if byte == &1_u8 => {
                            match bytes.first() {
                                Some(byte) if byte == &0_u8 => args.push(vec![*byte]),
                                Some(byte) if byte == &1_u8 => {
                                    bytes.advance(1);
                                    args.push(
                                        bytes.split_to(BalanceOf::max_encoded_len()).to_vec(),
                                    ); // storage_limit
                                },
                                _ => {},
                            }
                        },
                        _ => {},
                    }
                },
                _ => {
                    // should die
                },
            }

            // remove the length of the input
            bytes.advance(1);

            args.push(bytes.to_vec()); // data

            Ok(args)
        },
        Action::CallEvm => Ok(args),
        Action::CallWasm => Ok(args),
        Action::CallComposable => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // dest
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // value
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // gas
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // storage
            args.push(bytes.to_vec()); // data

            Ok(args)
        },
        Action::TransferMulti => {
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset id
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt
            take_insurance::<BalanceOf>(bytes, &mut args);

            Ok(args)
        },
        Action::Data => {
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // key

            Ok(args)
        },
    }
}

fn take_insurance<Balance: MaxEncodedLen>(bytes: &mut bytes::Bytes, args: &mut Vec<Vec<u8>>) {
    match bytes.first() {
        Some(byte) if byte == &0_u8 => args.push(vec![]),
        Some(byte) if byte == &1_u8 => {
            args.push(
                bytes
                    .split_to(Option::<Insurance<Balance>>::max_encoded_len())
                    .to_vec(),
            ); // opt insurance
        },
        _ => {},
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo, Default)]
pub enum ConfirmationOutcome {
    #[default]
    Success,
    MisbehaviourMalformedValues {
        key: Bytes,
        expected: Bytes,
        received: Bytes,
    },
    TimedOut,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
pub struct ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub err: Option<ConfirmationOutcome>,
    pub output: Option<Bytes>,
    pub inclusion_data: Vec<u8>,
    pub executioner: AccountId,
    pub received_at: BlockNumber,
    pub cost: Option<BalanceOf>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, Debug, TypeInfo, Default)]
pub enum SecurityLvl {
    #[default]
    Optimistic,
    Escrow,
}

// Side effects conversion error.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Expected confirmation to FSX wasn't there while hardening.
    HardeningMissingConfirmationError,
}

#[cfg(test)]
mod tests {
    use super::*;

    use sp_core::crypto::AccountId32;

    type BalanceOf = u128;
    type AccountId = AccountId32;

    #[test]
    fn successfully_creates_empty_side_effect() {
        let empty_side_effect = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            insurance: 0,
            enforce_executor: None,
            reward_asset_id: None,
        };

        assert_eq!(
            empty_side_effect,
            SideEffect {
                target: [0, 0, 0, 0],
                max_reward: 0,
                encoded_action: vec![],
                encoded_args: vec![],
                signature: vec![],
                insurance: 0,
                enforce_executor: None,
                reward_asset_id: None,
            }
        );
    }

    #[test]
    fn successfully_encodes_transfer_full_side_effect_with_confirmation() {
        let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let optional_insurance = 2u128;
        let optional_reward = 3u128;

        let tsfx_input = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 0,
            encoded_action: vec![],
            encoded_args: vec![
                from.encode(),
                to.encode(),
                value.encode(),
                [optional_insurance.encode(), optional_reward.encode()].concat(),
            ],
            signature: vec![],
            insurance: 0,
            enforce_executor: None,
            reward_asset_id: None,
        };

        assert_eq!(
            tsfx_input,
            SideEffect {
                target: [0, 0, 0, 0],
                max_reward: 0,
                insurance: 0,
                encoded_action: vec![],
                encoded_args: vec![
                    vec![
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1
                    ],
                    vec![
                        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                        2, 2, 2, 2, 2, 2, 2
                    ],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![
                        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ]
                ],
                signature: vec![],
                enforce_executor: None,
                reward_asset_id: None,
            }
        );
    }

    // fixme: Revisit t3rn_sdk_primitives and update TryFrom SideEffect new interface changed
    //  after Executors Bidding (t3rn/t3rn#477)
    // use t3rn_sdk_primitives::{
    //     storage::BoundedVec,
    //     xc::{Call, Chain, Operation, VM},
    // };
    // #[test]
    // fn encoded_evm_call_to_side_effect() {
    //     let se =
    //         Chain::<AccountId, BalanceOf, Hash>::Polkadot(
    //             Operation::<AccountId, BalanceOf, Hash>::Call(box Call {
    //                 caller: ALICE,
    //                 call: VM::<AccountId, BalanceOf>::Evm {
    //                     dest: BOB,
    //                     value: 50,
    //                 },
    //                 data: t3rn_sdk_primitives::storage::BoundedVec::<u8, 1024>::from_iter(vec![
    //                     0_u8, 1_u8, 2_u8,
    //                 ]),
    //             }),
    //         );
    //     let bytes = se.encode();
    //     let s = SideEffect::<AccountId, BalanceOf>::try_from(bytes).unwrap();
    //
    //     assert_eq!(s.target, *b"pdot");
    //     assert_eq!(s.encoded_action, *CALL_SIDE_EFFECT_ID);
    //     assert_eq!(
    //         s.encoded_args,
    //         vec![
    //             [1_u8; 32].to_vec(),
    //             [2_u8; 32].to_vec(),
    //             50_u128.encode(),
    //             vec![0, 1, 2]
    //         ]
    //     );
    // }
    //
    // #[test]
    // fn encoded_wasm_call_to_side_effect() {
    //     let se =
    //         Chain::<AccountId, BalanceOf, Hash>::Polkadot(
    //             Operation::<AccountId, BalanceOf, Hash>::Call(box Call {
    //                 caller: ALICE,
    //                 call: VM::<AccountId, BalanceOf>::Wasm {
    //                     dest: BOB,
    //                     value: 50,
    //                     gas_limit: 60,
    //                     storage_limit: Some(70),
    //                 },
    //                 data: BoundedVec::<u8, 1024>::from_iter(vec![0_u8, 1_u8, 2_u8]),
    //             }),
    //         );
    //     let bytes = se.encode();
    //     let s = SideEffect::<AccountId, BalanceOf>::try_from(bytes).unwrap();
    //
    //     assert_eq!(s.target, *b"pdot");
    //     assert_eq!(s.encoded_action, *CALL_SIDE_EFFECT_ID);
    //     assert_eq!(
    //         s.encoded_args,
    //         vec![
    //             [1_u8; 32].to_vec(),
    //             [2_u8; 32].to_vec(),
    //             50_u128.encode(),
    //             60_u128.encode(),
    //             70_u128.encode(),
    //             vec![0, 1, 2]
    //         ]
    //     );
    // }
    //
    // #[test]
    // fn encoded_transfer_to_side_effect() {
    //     let se =
    //         Chain::<AccountId, BalanceOf, Hash>::Polkadot(
    //             Operation::<AccountId, BalanceOf, Hash>::Transfer {
    //                 caller: ALICE,
    //                 to: BOB,
    //                 amount: 50,
    //                 insurance: None,
    //             },
    //         );
    //     let bytes = se.encode();
    //     let s = SideEffect::<AccountId, BalanceOf>::try_from(bytes).unwrap();
    //
    //     assert_eq!(s.target, *b"pdot");
    //     assert_eq!(s.encoded_action, *TRANSFER_SIDE_EFFECT_ID);
    //     assert_eq!(
    //         s.encoded_args,
    //         vec![
    //             [1_u8; 32].to_vec(),
    //             [2_u8; 32].to_vec(),
    //             50_u128.encode(),
    //             vec![]
    //         ]
    //     );
    // }
    //
    // #[test]
    // fn encoded_multi_transfer_to_side_effect() {
    //     let asset = [5_u8; 32];
    //     let se =
    //         Chain::<AccountId, BalanceOf, Hash>::Polkadot(
    //             Operation::<AccountId, BalanceOf, Hash>::TransferMulti {
    //                 asset,
    //                 caller: ALICE,
    //                 to: BOB,
    //                 amount: 50,
    //                 insurance: None,
    //             },
    //         );
    //     let bytes = se.encode();
    //     let s = SideEffect::<AccountId, BalanceOf>::try_from(bytes).unwrap();
    //
    //     assert_eq!(s.target, *b"pdot");
    //     assert_eq!(s.encoded_action, *MULTI_TRANSFER_SIDE_EFFECT_ID);
    //     assert_eq!(
    //         s.encoded_args,
    //         vec![
    //             asset.to_vec(),
    //             [1_u8; 32].to_vec(),
    //             [2_u8; 32].to_vec(),
    //             50_u128.encode(),
    //             vec![]
    //         ]
    //     );
    // }
    //
    // #[test]
    // fn encoded_aliq_to_side_effect() {
    //     let amount_left = 1_u128;
    //     let amount_right = 2_u128;
    //     let amount_liquidity_token = 3_u128;
    //     let liquidity_token = [4_u8; 32];
    //     let asset_right = [3_u8; 32];
    //     let asset_left = [2_u8; 32];
    //     let se =
    //         Chain::<AccountId, BalanceOf, Hash>::Polkadot(
    //             Operation::<AccountId, BalanceOf, Hash>::AddLiquidity {
    //                 caller: ALICE,
    //                 to: BOB,
    //                 asset_left,
    //                 asset_right,
    //                 liquidity_token,
    //                 amount_left,
    //                 amount_right,
    //                 amount_liquidity_token,
    //                 insurance: None,
    //             },
    //         );
    //     let bytes = se.encode();
    //     let s = SideEffect::<AccountId, BalanceOf>::try_from(bytes).unwrap();
    //
    //     assert_eq!(s.target, *b"pdot");
    //     assert_eq!(s.encoded_action, *ADD_LIQUIDITY_SIDE_EFFECT_ID);
    //     assert_eq!(
    //         s.encoded_args,
    //         vec![
    //             [1_u8; 32].to_vec(),
    //             [2_u8; 32].to_vec(),
    //             asset_left.to_vec(),
    //             asset_right.to_vec(),
    //             liquidity_token.to_vec(),
    //             amount_left.encode(),
    //             amount_right.encode(),
    //             amount_liquidity_token.encode(),
    //             vec![]
    //         ]
    //     );
    // }
    //
    // #[test]
    // fn encoded_swap_to_side_effect() {
    //     let amount_from = 1_u128;
    //     let amount_to = 2_u128;
    //     let asset_from = [3_u8; 32];
    //     let asset_to = [2_u8; 32];
    //     let se =
    //         Chain::<AccountId, BalanceOf, Hash>::Polkadot(
    //             Operation::<AccountId, BalanceOf, Hash>::Swap {
    //                 caller: ALICE,
    //                 to: BOB,
    //                 amount_from,
    //                 amount_to,
    //                 asset_from,
    //                 asset_to,
    //                 insurance: None,
    //             },
    //         );
    //     let bytes = se.encode();
    //     let s = SideEffect::<AccountId, BalanceOf>::try_from(bytes).unwrap();
    //
    //     assert_eq!(s.target, *b"pdot");
    //     assert_eq!(s.encoded_action, *SWAP_SIDE_EFFECT_ID);
    //     assert_eq!(
    //         s.encoded_args,
    //         vec![
    //             [1_u8; 32].to_vec(),
    //             [2_u8; 32].to_vec(),
    //             amount_from.encode(),
    //             amount_to.encode(),
    //             asset_from.to_vec(),
    //             asset_to.to_vec(),
    //             vec![]
    //         ]
    //     );
    // }
    // #[test]
    // fn encoded_data_to_side_effect() {
    //     let index = [3_u8; 32];
    //     let se =
    //         Chain::<AccountId, BalanceOf, Hash>::Polkadot(
    //             Operation::<AccountId, BalanceOf, Hash>::Data { index },
    //         );
    //     let bytes = se.encode();
    //     let s = SideEffect::<AccountId, BalanceOf>::try_from(bytes).unwrap();
    //
    //     assert_eq!(s.target, *b"pdot");
    //     assert_eq!(s.encoded_action, *DATA_SIDE_EFFECT_ID);
    //     assert_eq!(s.encoded_args, vec![index.to_vec(),]);
    // }
    //
    // #[test]
    // fn from_encoded_chain_to_side_effect() {
    //     let v: Vec<u8> = vec![
    //         1, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    //         5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    //         6, 6, 6, 6, 6, 6, 6, 6, 100, 0, 0, 0,
    //     ];
    //     let s = SideEffect::<[u8; 32], u32, u32>::try_from(v).unwrap();
    //
    //     assert_eq!(
    //         s,
    //         SideEffect {
    //             target: [112, 100, 111, 116],
    //             max_reward: 0,
    //             encoded_action: vec![116, 114, 97, 110],
    //             encoded_args: vec![
    //                 vec![
    //                     5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    //                     5, 5, 5, 5, 5, 5, 5
    //                 ],
    //                 vec![
    //                     6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    //                     6, 6, 6, 6, 6, 6, 6
    //                 ],
    //                 vec![100, 0, 0, 0],
    //             ],
    //             signature: vec![],
    //             insurance: 0,
    //             nonce: 0,
    //             enforce_executor: None
    //         }
    //     );
    // }
}
