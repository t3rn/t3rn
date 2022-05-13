use crate::{
    bridges::chain_circuit::{
        AccountId, Balance as CircuitBalance, BlockNumber as CircuitBlockNumber,
    },
    Bytes,
};
use codec::{Decode, Encode};
use std::convert::TryFrom;

use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::vec::Vec;

use sp_core::crypto::AccountId32;
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

pub mod interface;
use crate::abi::Type;
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

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
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

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub err: Option<ConfirmationOutcome>,
    pub output: Option<Bytes>,
    pub encoded_effect: Bytes,
    pub inclusion_proof: Option<Bytes>,
    pub executioner: AccountId,
    pub received_at: BlockNumber,
    pub cost: Option<BalanceOf>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum SecurityLvl {
    Dirty = 0,
    Optimistic = 1,
    Escrowed = 2,
}

impl Default for SecurityLvl {
    fn default() -> Self {
        SecurityLvl::Dirty
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct FullSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub input: SideEffect<AccountId, BlockNumber, BalanceOf>,
    pub confirmed: Option<ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf>>,
    pub security_lvl: SecurityLvl,
}

impl<
        AccountId: Encode,
        BlockNumber: Ord + Copy + Zero + Encode,
        BalanceOf: Copy + Zero + Encode + Decode,
    > FullSideEffect<AccountId, BlockNumber, BalanceOf>
{
    pub fn harden(
        &self,
    ) -> Result<
        (
            SecurityLvl,
            ConfirmationOutcome,
            CircuitBalance,
            AccountId32,
            CircuitBlockNumber,
            CircuitBalance,
            [u8; 4],
            Vec<Bytes>,
            [u8; 4],
        ),
        &'static str,
    > {
        let confirmed = if let Some(ref confirmed) = self.confirmed {
            Ok(confirmed.clone())
        } else {
            Err("At harden() expect FSX confirmation part to be there")
        }?;

        let confirmation_outcome = if let Some(outcome) = &confirmed.err {
            outcome.clone()
        } else {
            ConfirmationOutcome::Success
        };

        let confirmed_cost: CircuitBalance = Decode::decode(&mut &confirmed.cost.encode()[..])
            .map_err(|_| "harden() decoding error: confirmed_cost")?;

        let confirmed_executioner: AccountId32 =
            Decode::decode(&mut &confirmed.executioner.encode()[..])
                .map_err(|_| "harden() decoding error: confirmed_executioner")?;

        let confirmed_received_at: CircuitBlockNumber =
            Decode::decode(&mut &confirmed.received_at.encode()[..])
                .map_err(|_| "harden() decoding error: confirmed_received_at")?;

        let prize: CircuitBalance = Decode::decode(&mut &self.input.prize.encode()[..])
            .map_err(|_| "harden() decoding error: confirmed_cost")?;

        return Ok((
            self.security_lvl.clone(),
            confirmation_outcome,
            confirmed_cost,
            confirmed_executioner,
            confirmed_received_at,
            prize,
            self.input.target.clone(),
            self.input.encoded_args.clone(),
            <[u8; 4]>::try_from(self.input.encoded_action.clone()).unwrap_or_default(),
        ))
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct HardenedSideEffect {
    target: [u8; 4],
    prize: CircuitBalance,
    encoded_action: [u8; 4],
    encoded_args: Vec<Bytes>,
    encoded_args_abi: Vec<crate::abi::Type>,

    security_lvl: SecurityLvl,

    confirmation_outcome: ConfirmationOutcome,
    confirmed_executioner: AccountId,
    confirmed_received_at: CircuitBlockNumber,
    confirmed_cost: CircuitBalance,
}

impl Default for HardenedSideEffect {
    fn default() -> Self {
        HardenedSideEffect {
            target: [0, 0, 0, 0],
            prize: 0u128,
            encoded_action: [0, 0, 0, 0],
            encoded_args: vec![],
            encoded_args_abi: vec![],
            security_lvl: SecurityLvl::Dirty,
            confirmation_outcome: ConfirmationOutcome::Success,
            confirmed_executioner: AccountId32::new([0u8; 32]),
            confirmed_received_at: 0,
            confirmed_cost: 0,
        }
    }
}

impl
    sp_runtime::traits::Convert<
        (
            SecurityLvl,
            ConfirmationOutcome,
            CircuitBalance,
            AccountId32,
            CircuitBlockNumber,
            CircuitBalance,
            [u8; 4],
            Vec<Bytes>,
            [u8; 4],
        ),
        HardenedSideEffect,
    > for HardenedSideEffect
{
    fn convert(
        hardened_args: (
            SecurityLvl,
            ConfirmationOutcome,
            CircuitBalance,
            AccountId32,
            CircuitBlockNumber,
            CircuitBalance,
            [u8; 4],
            Vec<Bytes>,
            [u8; 4],
        ),
    ) -> HardenedSideEffect {
        let (
            security_lvl,
            confirmation_outcome,
            confirmed_cost,
            confirmed_executioner,
            confirmed_received_at,
            prize,
            target,
            encoded_args,
            encoded_action,
        ) = hardened_args;
        HardenedSideEffect {
            target,
            prize,
            encoded_action,
            encoded_args,
            encoded_args_abi: vec![],
            security_lvl,
            confirmation_outcome,
            confirmed_executioner,
            confirmed_received_at,
            confirmed_cost,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sp_core::crypto::AccountId32;
    use sp_runtime::{testing::H256, traits::Convert};

    type BlockNumber = CircuitBlockNumber;
    type BalanceOf = CircuitBalance;
    type AccountId = AccountId32;
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

        let tfsfx = FullSideEffect::<AccountId, BlockNumber, BalanceOf> {
            input: tsfx_input.clone(),
            security_lvl: SecurityLvl::Dirty,
            confirmed: Some(ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
                err: Some(ConfirmationOutcome::Success),
                output: Some(vec![]),
                encoded_effect: vec![],
                inclusion_proof: None,
                executioner: from,
                received_at: 1u64 as BlockNumber,
                cost: Some(2u64 as BalanceOf),
            }),
        };

        let h = HardenedSideEffect::default();
        let b = HardenedSideEffect::convert(tfsfx.harden().unwrap());

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
                "89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3"
            ))
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
}
