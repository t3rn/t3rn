use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::{
    prelude::{collections::VecDeque, fmt::Debug, vec, vec::Vec},
    TypeInfo,
};

#[cfg(feature = "runtime")]
use num::Zero;

pub type TargetId = [u8; 4];
pub type EventSignature = Vec<u8>;
pub type SideEffectName = Vec<u8>;
type Bytes = Vec<u8>;

pub const COMPOSABLE_CALL_SIDE_EFFECT_ID: &[u8; 4] = b"comp";
pub const WASM_CALL_SIDE_EFFECT_ID: &[u8; 4] = b"wasm";
pub const EVM_CALL_SIDE_EFFECT_ID: &[u8; 4] = b"cevm";
pub const CALL_SIDE_EFFECT_ID: &[u8; 4] = b"call";
pub const ORML_TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"orml";
pub const TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"tran";
pub const ASSETS_TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"tass";
pub const ADD_LIQUIDITY_SIDE_EFFECT_ID: &[u8; 4] = b"aliq";
pub const SWAP_SIDE_EFFECT_ID: &[u8; 4] = b"swap";
pub const DATA_SIDE_EFFECT_ID: &[u8; 4] = b"data";

#[derive(Clone, Eq, PartialEq, Encode, Default, Decode, Debug, TypeInfo)]
pub struct SideEffect<AccountId, BlockNumber, BalanceOf> {
    pub target: TargetId,
    pub prize: BalanceOf,
    pub ordered_at: BlockNumber,
    pub encoded_action: Bytes,
    pub encoded_args: Vec<Bytes>,
    pub signature: Bytes,
    pub enforce_executioner: Option<AccountId>,
}

#[cfg(feature = "runtime")]
impl<AccountId, BlockNumber, BalanceOf> SideEffect<AccountId, BlockNumber, BalanceOf>
where
    AccountId: Encode,
    BlockNumber: Ord + Copy + sp_runtime::traits::Zero + Encode,
    BalanceOf: Copy + sp_runtime::traits::Zero + Encode + Decode,
{
    pub fn generate_id<Hasher: sp_core::Hasher>(&self) -> <Hasher as sp_core::Hasher>::Out {
        Hasher::hash(Encode::encode(self).as_ref())
    }

    pub fn id_as_bytes<Hasher: sp_core::Hasher>(id: <Hasher as sp_core::Hasher>::Out) -> Bytes {
        id.as_ref().to_vec()
    }
}

#[cfg(feature = "runtime")]
/// Decode the side effect from encoded Chain.
impl<AccountId, BlockNumber, BalanceOf> TryFrom<Vec<u8>>
    for SideEffect<AccountId, BlockNumber, BalanceOf>
where
    AccountId: Encode + MaxEncodedLen,
    BlockNumber: Ord + Copy + Zero + Encode,
    BalanceOf: Copy + Zero + Encode + Decode + MaxEncodedLen,
{
    type Error = &'static str;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut bytes: VecDeque<u8> = bytes.into();
        let mut take_next = || bytes.pop_front().ok_or("no more bytes");

        let target: TargetId = TargetByte(take_next()?).try_into()?;
        let action = Action::try_from(take_next()?)?;

        let bytes: Vec<u8> = bytes.into();
        let args = extract_args::<AccountId, BalanceOf, [u8; 32]>(
            &action,
            &mut bytes::Bytes::from(bytes),
        )?;
        let action_bytes: [u8; 4] = action.into();
        let action_bytes = action_bytes.encode();

        Ok(SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target,
            prize: Zero::zero(),
            ordered_at: Zero::zero(),
            encoded_action: action_bytes.into(),
            encoded_args: args,
            signature: vec![],
            enforce_executioner: None,
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
    TransferAssets,
    TransferOrml,
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

    // TODO: currently these enums are out of order since they don't fully map together yet
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Action::Transfer),
            2 => Ok(Action::AddLiquidity),
            3 => Ok(Action::Swap),
            4 => Ok(Action::Call),
            5 => Ok(Action::TransferAssets),
            6 => Ok(Action::TransferOrml),
            7 => Ok(Action::CallEvm),
            8 => Ok(Action::CallWasm),
            9 => Ok(Action::CallComposable),
            10 => Ok(Action::Data),
            _ => Err("Invalid action id"),
        }
    }
}

impl Into<[u8; 4]> for Action {
    fn into(self) -> [u8; 4] {
        match self {
            Action::Transfer => *TRANSFER_SIDE_EFFECT_ID,
            Action::AddLiquidity => *ADD_LIQUIDITY_SIDE_EFFECT_ID,
            Action::Swap => *SWAP_SIDE_EFFECT_ID,
            Action::Call => *CALL_SIDE_EFFECT_ID,
            Action::CallEvm => *EVM_CALL_SIDE_EFFECT_ID,
            Action::CallWasm => *WASM_CALL_SIDE_EFFECT_ID,
            Action::CallComposable => *COMPOSABLE_CALL_SIDE_EFFECT_ID,
            Action::Data => *DATA_SIDE_EFFECT_ID,
            Action::TransferAssets => *ASSETS_TRANSFER_SIDE_EFFECT_ID,
            Action::TransferOrml => *ORML_TRANSFER_SIDE_EFFECT_ID,
        }
    }
}

fn extract_args<AccountId: MaxEncodedLen, BalanceOf: MaxEncodedLen, Hash: MaxEncodedLen>(
    action: &Action,
    bytes: &mut bytes::Bytes,
) -> Result<Vec<Bytes>, &'static str> {
    let mut args = Vec::new();

    match *action {
        Action::Transfer => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt

            // TODO: support insurance

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
            args.push(bytes.split_to(0).to_vec()); // insurance

            // TODO: support insurance
            Ok(args)
        },
        Action::Swap => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_left
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_right
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset_left
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset_right

            // TODO: support insurance

            Ok(args)
        },
        Action::Call => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // caller
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // dest
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // value

            // TODO: This is tricky since we define input as a dynamic size so we don't know what is next
            args.push(bytes.to_vec()); // args

            Ok(args)
        },
        Action::CallEvm => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // caller
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // dest
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // value

            // TODO: This is tricky since we define input as a dynamic size so we don't know what is next
            args.push(bytes.to_vec()); // args

            Ok(args)
        },
        Action::CallWasm => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // dest
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // value
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // gas
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // storage

            args.push(bytes.to_vec()); // data

            Ok(args)
        },
        Action::CallComposable => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // dest
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // value
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // gas
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // storage

            args.push(bytes.to_vec()); // data

            Ok(args)
        },
        Action::TransferAssets => {
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset id
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt
            args.push(bytes.to_vec());
            // TODO: support insurance

            Ok(args)
        },
        Action::TransferOrml => {
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset id
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt
            args.push(bytes.to_vec());
            // TODO: support insurance

            Ok(args)
        },
        Action::Data => {
            args.push(bytes.to_vec()); // key

            Ok(args)
        },
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
pub enum ConfirmationOutcome {
    Success,
    MisbehaviourMalformedValues {
        key: Bytes,
        expected: Bytes,
        received: Bytes,
    },
    TimedOut,
}

impl Default for ConfirmationOutcome {
    fn default() -> Self {
        ConfirmationOutcome::Success
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
pub struct ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub err: Option<ConfirmationOutcome>,
    pub output: Option<Bytes>,
    pub encoded_effect: Bytes,
    pub inclusion_proof: Option<Bytes>,
    pub executioner: AccountId,
    pub received_at: BlockNumber,
    pub cost: Option<BalanceOf>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, Debug, TypeInfo)]
pub enum SecurityLvl {
    Dirty,
    Optimistic,
    Escrowed,
}

impl Default for SecurityLvl {
    fn default() -> Self {
        SecurityLvl::Dirty
    }
}

// Side effects conversion error.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Failed to decode a property while hardening.
    HardeningDecodeError,
    /// Expected confirmation to FSX wasn't there while hardening.
    HardeningMissingConfirmationError,
}

#[cfg(test)]
mod tests {
    use super::*;

    use scale_info::prelude::vec::Vec;
    use sp_core::crypto::AccountId32;

    type BlockNumber = u64;
    type BalanceOf = u128;
    type AccountId = AccountId32;

    #[test]
    fn successfully_creates_empty_side_effect() {
        let empty_side_effect = SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(
            empty_side_effect,
            SideEffect {
                target: [0, 0, 0, 0],
                prize: 0,
                ordered_at: 0,
                encoded_action: vec![],
                encoded_args: vec![],
                signature: vec![],
                enforce_executioner: None,
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

        let tsfx_input = SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![
                from.encode(),
                to.encode(),
                value.encode(),
                [optional_insurance.encode(), optional_reward.encode()].concat(),
            ],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(
            tsfx_input,
            SideEffect {
                target: [0, 0, 0, 0],
                prize: 0,
                ordered_at: 0,
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
                enforce_executioner: None,
            }
        );
    }

    #[test]
    fn successfully_defaults_side_effect_to_an_empty_one() {
        let empty_side_effect = SideEffect::<u64, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(empty_side_effect, SideEffect::default(),);
    }

    #[test]
    fn from_encoded_chain_to_side_effect() {
        let v: Vec<u8> = vec![
            1, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 100, 0, 0, 0,
        ];
        let s = SideEffect::<[u8; 32], u32, u32>::try_from(v).unwrap();

        assert_eq!(
            s,
            SideEffect {
                target: [112, 100, 111, 116],
                prize: 0,
                ordered_at: 0,
                encoded_action: vec![116, 114, 97, 110],
                encoded_args: vec![
                    vec![
                        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                        5, 5, 5, 5, 5, 5, 5
                    ],
                    vec![
                        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                        6, 6, 6, 6, 6, 6, 6
                    ],
                    vec![100, 0, 0, 0],
                ],
                signature: vec![],
                enforce_executioner: None,
            }
        );
    }
}
