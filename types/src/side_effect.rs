use codec::{Decode, Encode};
use num::Zero;
use scale_info::{
    prelude::{fmt::Debug, vec, vec::Vec},
    TypeInfo,
};

pub type TargetId = [u8; 4];
pub type EventSignature = Vec<u8>;
pub type SideEffectName = Vec<u8>;
type Bytes = Vec<u8>;

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
impl<
        AccountId: Encode,
        BlockNumber: Ord + Copy + sp_runtime::traits::Zero + Encode,
        BalanceOf: Copy + sp_runtime::traits::Zero + Encode + Decode,
    > SideEffect<AccountId, BlockNumber, BalanceOf>
{
    pub fn generate_id<Hasher: sp_core::Hasher>(&self) -> <Hasher as sp_core::Hasher>::Out {
        Hasher::hash(Encode::encode(self).as_ref())
    }

    pub fn id_as_bytes<Hasher: sp_core::Hasher>(id: <Hasher as sp_core::Hasher>::Out) -> Bytes {
        id.as_ref().to_vec()
    }
}

// Decode the side effect from encoded Chain.
impl<
        AccountId: Encode,
        BlockNumber: Ord + Copy + Zero + Encode,
        BalanceOf: Copy + Zero + Encode + Decode,
    > From<&Vec<u8>> for SideEffect<AccountId, BlockNumber, BalanceOf>
{
    fn from(bytes: &Vec<u8>) -> Self {
        let (action, args) = match_action(bytes[1], &bytes[2..]).unwrap();
        SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: match_target(bytes[0]).unwrap(),
            prize: Zero::zero(),
            ordered_at: Zero::zero(),
            encoded_action: action.into(),
            encoded_args: args,
            signature: vec![],
            enforce_executioner: None,
        }
    }
}

fn match_target(id: u8) -> Result<[u8; 4], &'static str> {
    match id {
        0 => Ok(*b"ksma"),
        1 => Ok(*b"pdot"),
        2 => Ok(*b"karu"),
        3 => Ok(*b"t3rn"),
        _ => Err("Invalid target Id"),
    }
}

fn match_action(id: u8, bytes: &[u8]) -> Result<(Bytes, Vec<Bytes>), &'static str> {
    match id {
        0 => {
            let mut args: Vec<Bytes> = bytes[0..64]
                .chunks(32)
                .map(|chunk| chunk.to_vec())
                .collect(); //from, to
            args.push(bytes[64..].to_vec()); //amount
            Ok((b"tran".encode(), args))
        },
        1 => {
            let mut args: Vec<Bytes> = bytes[0..64]
                .chunks(32)
                .map(|chunk| chunk.to_vec())
                .collect(); //from, to
            args.push(bytes[64..80].to_vec()); //amount
            args.push(bytes[80..].to_vec()); //asset; not sure maybe 2 bytes, maybe 4

            Ok((b"mult".encode(), args))
        },
        2 => {
            let mut args: Vec<Bytes> = bytes[0..160]
                .chunks(32)
                .map(|chunk| chunk.to_vec())
                .collect(); //from, to, asset_lef, asset_right
            args.push(bytes[160..176].to_vec()); //amount_left
            args.push(bytes[176..192].to_vec()); //amount_right
            args.push(bytes[192..].to_vec()); //amount liquidity token

            Ok((b"aliq".encode(), args))
        },
        3 => {
            let mut args: Vec<Bytes> = bytes[0..64]
                .chunks(32)
                .map(|chunk| chunk.to_vec())
                .collect(); //from, to
            args.push(bytes[64..80].to_vec()); //amount_from
            args.push(bytes[80..96].to_vec()); //amount_to
            args.push(bytes[96..128].to_vec()); //asset_from
            args.push(bytes[128..].to_vec()); //asset_to

            Ok((b"swap".encode(), args))
        },
        4 => {
            let mut args: Vec<Bytes> = Vec::new();
            args.push(bytes[0..32].to_vec()); //caller
            args.push(bytes[32..].to_vec()); //vm

            Ok((b"call".encode(), args))
        },
        _ => Err("Invalid action Id"),
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
        let s = SideEffect::<[u8; 2], u32, u32>::from(&v);

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
                    vec![100, 0, 0, 0]
                ],
                signature: vec![],
                enforce_executioner: None,
            }
        );
    }
}
