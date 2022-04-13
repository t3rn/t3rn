use crate::Bytes;
use codec::{Decode, Encode};

use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::vec::Vec;

#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

pub mod interface;
pub use interface::*;
pub mod parser;

pub type SideEffectId<T> = <T as frame_system::Config>::Hash;
pub type TargetId = [u8; 4];
pub type EventSignature = Vec<u8>;
pub type SideEffectName = Vec<u8>;

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

impl<
        AccountId: Encode,
        BlockNumber: Ord + Copy + Zero + Encode,
        BalanceOf: Copy + Zero + Encode + Decode,
    > SideEffect<AccountId, BlockNumber, BalanceOf>
{
    pub fn generate_id<Hasher: sp_core::Hasher>(&self) -> <Hasher as sp_core::Hasher>::Out {
        Hasher::hash(Encode::encode(self).as_ref())
    }

    pub fn id_as_bytes<Hasher: sp_core::Hasher>(id: <Hasher as sp_core::Hasher>::Out) -> Bytes {
        id.as_ref().to_vec()
    }
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub err: Option<Bytes>,
    pub output: Option<Bytes>,
    pub encoded_effect: Bytes,
    pub inclusion_proof: Option<Bytes>,
    pub executioner: AccountId,
    pub received_at: BlockNumber,
    pub cost: Option<BalanceOf>,
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct FullSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub input: SideEffect<AccountId, BlockNumber, BalanceOf>,
    pub confirmed: Option<ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sp_runtime::testing::H256;

    type BlockNumber = u64;
    type BalanceOf = u64;
    type AccountId = u64;
    type Hashing = sp_runtime::traits::BlakeTwo256;

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
    fn successfully_generates_id_for_side_empty_effect() {
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
            empty_side_effect.generate_id::<Hashing>(),
            H256::from_slice(&hex!(
                "19ea4a516c66775ea1f648d71f6b8fa227e8b0c1a0c9203f82c33b89c4e759b5"
            ))
        );
    }

    #[test]
    fn successfully_defaults_side_effect_to_an_empty_one() {
        let empty_side_effect = SideEffect::<AccountId, BlockNumber, BalanceOf> {
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
}
