use crate::side_effects::protocol::SideEffectProtocol;
use crate::volatile::LocalState;
use t3rn_primitives::EscrowTrait;

use sp_std::{prelude::*, vec::Vec};

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;

// pub fn just_transfer<'a, T: EscrowTrait>(
//     transactor: &T::AccountId,
//     dest: &T::AccountId,
//     value: BalanceOf<T>,
// ) -> DispatchResult {
//     <T as EscrowTrait>::Currency::transfer(transactor, dest, value, ExistenceRequirement::KeepAlive)
// }

pub fn try_commit_insurance_deposit<T: EscrowTrait>(
    _side_effect_protocol: Box<dyn SideEffectProtocol>,
    _args: &Arguments,
    _local_state: &mut LocalState,
    _requester: &T::AccountId,
) -> Result<(), &'static str> {
    unimplemented!();
}

// ... side_effect_id: H256, requester: AccountId, requested_insurance: Option<Balance> , promised_reward: Balance
pub fn lock_requester<T: EscrowTrait>() {
    // T::Balances::transfer(requester, VAULT, promised_reward)
    // T::RequestedLocks::insert(side_effect_id, requester: AccountId, requested_insurance: Option<Balance> , promised_reward: Balance, bonded_by: None);
}

pub fn bond_relayer() {
    // let (... promised_reward, requested_insurance ) = T::RequestedLocks::get(side_effect_id)
    // if !requested_insurance.is_none()
    // if T::Balances::transfer(relayer, VAULT, requested_insurance) == SUCCESS {
    //     T::RequestedLocks::insert_mut(side_effect_id, (... bonded_by: relayer)
    //                                   T::DepositEvent(BondedRelayer(side_effect_id, relayer_id, requested_insurance)
    // }
}

pub fn enact_on_insurance() {
    // let xtx = T::ExecDelivery::match_side_effect_with_xtx(side_effect_id);
    // let (... promised_reward, requested_insurance, promised_reward, bonded_by, requester )  =. T::RequestedLocks::get(side_effect_id)
    // // commit - success - release reward + insurance back to relayer -> user already got his requested assets on remote chain
    // if xtx.is_succesfull {
    //     T::Balances::transfer(VAULT, relayer, requested_insurance + promised_reward)
    // }
    // // else Revert - refund the locked reward + insurance back to user
    // else if xtx.is_completed {
    //     T::Balances::transfer(VAULT, user, requested_insurance)
    // }
}
