use crate::{BalanceOf, Config, Error};
use frame_support::{dispatch::DispatchResult, sp_runtime::traits::Zero};
use t3rn_primitives::{
    account_manager::{AccountManager, Outcome, RequestCharge},
    claimable::{BenefitSource, CircuitRole},
    contracts_registry::{AuthorInfo, KindValidator},
    threevm::{ModuleOperations, Remunerated},
};

const LOG_TARGET: &str = "3vm::remuneration";

pub(crate) fn try_remunerate<T: Config, Module: ModuleOperations<T, BalanceOf<T>>>(
    payee: &T::AccountId,
    module: &Module,
) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError> {
    if let Some(author) = module.get_author() {
        let amount = author.fees_per_single_use.unwrap_or_default();
        handle_remuneration(payee, module, author, amount)
    } else {
        Ok(Remunerated::default())
    }
}

pub(crate) fn try_remunerate_exact<T: Config, Module: ModuleOperations<T, BalanceOf<T>>>(
    payee: &T::AccountId,
    amount: BalanceOf<T>,
    module: &Module,
) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError> {
    if let Some(author) = module.get_author() {
        handle_remuneration(payee, module, author, amount)
    } else {
        Ok(Remunerated::default())
    }
}

pub(crate) fn try_finalize<T: Config>(ledger_id: T::Hash, outcome: Outcome) -> DispatchResult {
    T::AccountManager::finalize(ledger_id, outcome, None, Option::<BalanceOf<T>>::None)
}

fn handle_remuneration<T: Config, Module: ModuleOperations<T, BalanceOf<T>>>(
    payee: &T::AccountId,
    module: &Module,
    author: &AuthorInfo<T::AccountId, BalanceOf<T>>,
    amount: BalanceOf<T>,
) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError> {
    if amount > BalanceOf::<T>::zero() {
        let kind = module.get_type();
        log::trace!(
            target: LOG_TARGET,
            "Checking module {:?}, can remunerate to author: {:?}",
            kind,
            author.clone()
        );
        if kind.can_remunerate() {
            let next_charge_id = T::AccountManager::bump_contracts_registry_nonce()?;
            T::AccountManager::deposit(
                next_charge_id,
                RequestCharge {
                    payee: payee.clone(),
                    offered_reward: amount,
                    charge_fee: Zero::zero(),
                    source: BenefitSource::TrafficRewards,
                    role: CircuitRole::ContractAuthor,
                    recipient: Some(author.account.clone()),
                    maybe_asset_id: None,
                },
            )?;

            Ok(Remunerated::<T::Hash>::new(Some(next_charge_id)))
        } else {
            Err(Error::<T>::ContractCannotRemunerate.into())
        }
    } else {
        Ok(Remunerated::<T::Hash>::default())
    }
}
