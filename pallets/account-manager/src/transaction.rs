use crate::{BalanceOf, Config};
use frame_support::traits::{Currency, Imbalance, OnUnbalanced, SameOrOther, TryDrop};

pub type NegativeImbalanceOf<T, C> =
    <C as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::PositiveImbalance;

pub struct ImbalanceToBeneficiary<T>(sp_std::marker::PhantomData<T>);

pub struct ImbalanceBeneficiary<T: Config>(
    Option<<T as frame_system::Config>::AccountId>,
    NegativeImbalanceOf<T, <T as Config>::Currency>,
);

impl<T: Config> Default for ImbalanceBeneficiary<T> {
    fn default() -> Self {
        Self(None, Default::default())
    }
}

impl<T: Config> TryDrop for ImbalanceBeneficiary<T> {
    fn try_drop(self) -> Result<(), Self> {
        self.drop_zero()
    }
}

impl<T: Config> Imbalance<BalanceOf<T>> for ImbalanceBeneficiary<T> {
    type Opposite = PositiveImbalanceOf<T>;

    fn zero() -> Self {
        Self(
            None,
            NegativeImbalanceOf::<T, <T as Config>::Currency>::zero(),
        )
    }

    fn drop_zero(self) -> Result<(), Self> {
        self.1.drop_zero().map_err(|e| Self(self.0, e))
    }

    fn split(self, amount: BalanceOf<T>) -> (Self, Self) {
        let (first, second) = self.1.split(amount);
        (Self(self.0.clone(), first), Self(self.0.clone(), second))
    }

    fn merge(mut self, other: Self) -> Self {
        Self(self.0, self.1.merge(other.1))
    }

    fn subsume(&mut self, other: Self) {
        self.1.subsume(other.1);
    }

    fn offset(
        self,
        other: Self::Opposite,
    ) -> frame_support::traits::SameOrOther<Self, Self::Opposite> {
        let x = self.1.offset(other);
        match x.try_same() {
            Ok(same) | Err(SameOrOther::Same(same)) => SameOrOther::Same(Self(self.0, same)),
            Err(SameOrOther::Other(err)) => SameOrOther::Other(err),
            Err(SameOrOther::None) => SameOrOther::None,
        }
    }

    fn peek(&self) -> BalanceOf<T> {
        self.1.peek()
    }
}

// For a negative imbalance, some account decreased due to either slashing or paying for a tx
impl<T> OnUnbalanced<ImbalanceBeneficiary<T>> for ImbalanceToBeneficiary<T>
where
    T: Config,
{
    // TODO: fix unwraps, these should never be None anyhow
    // this seems to be called for substrate-based transactions when there is a difference between pre dispatch
    // and post dispatch balances.
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = ImbalanceBeneficiary<T>>) {
        if let Some(beneficiary) = fees_then_tips.next() {
            let (beneficiary, fees) = (beneficiary.0, beneficiary.1);
            // Balances pallet automatically burns dropped Negative Imbalances by decreasing
            // total_supply accordingly
            // TODO: these fees need to be both configurable and used
            // let (_, to_treasury) = fees.ration(80, 20);
            <T as Config>::Currency::resolve_creating(&beneficiary.unwrap(), fees);
        }
    }

    // this is called from pallet_evm for Ethereum-based transactions
    // (technically, it calls on_unbalanced, which calls this when non-zero)
    fn on_nonzero_unbalanced(amount: ImbalanceBeneficiary<T>) {
        let (beneficiary, fees) = (amount.0, amount.1);
        <T as Config>::Currency::resolve_creating(&beneficiary.unwrap(), fees);
    }
}

// pub struct AccountManagerCurrencyAdapter<C, OU>(sp_std::marker::PhantomData<(C, OU)>);
//
// impl<T, C, OU> pallet_transaction_payment::OnChargeTransaction<T>
//     for AccountManagerCurrencyAdapter<C, OU>
// where
//     T: Config,
//     C: Currency<<T as frame_system::Config>::AccountId>,
//     C::PositiveImbalance: frame_support::traits::Imbalance<
//         <C as Currency<<T as frame_system::Config>::AccountId>>::Balance,
//         Opposite = C::NegativeImbalance,
//     >,
//     C::NegativeImbalance: frame_support::traits::Imbalance<
//         <C as Currency<<T as frame_system::Config>::AccountId>>::Balance,
//         Opposite = C::PositiveImbalance,
//     >,
// {
//     fn withdraw_fee(
//         who: &T::AccountId,
//         _call: &T::Call,
//         _info: &sp_runtime::traits::DispatchInfoOf<T::Call>,
//         fee: Self::Balance,
//         tip: Self::Balance,
//     ) -> Result<Self::LiquidityInfo, frame_support::pallet_prelude::TransactionValidityError> {
//         // TODO: here we just call through to CurrencyAdapter::withdraw_fee
//     }
//
//     fn correct_and_deposit_fee(
//         who: &T::AccountId,
//         _dispatch_info: &sp_runtime::traits::DispatchInfoOf<T::Call>,
//         _post_info: &sp_runtime::traits::PostDispatchInfoOf<T::Call>,
//         corrected_fee: Self::Balance,
//         tip: Self::Balance,
//         already_withdrawn: Self::LiquidityInfo,
//     ) -> Result<(), frame_support::pallet_prelude::TransactionValidityError> {
//         if let Some(paid) = already_withdrawn {
//             // Calculate how much refund we should return
//             let refund_amount = paid.peek().saturating_sub(corrected_fee);
//             // refund to the the account that paid the fees. If this fails, the
//             // account might have dropped below the existential balance. In
//             // that case we don't refund anything.
//             let refund_imbalance = C::deposit_into_existing(who, refund_amount)
//                 .unwrap_or_else(|_| C::PositiveImbalance::zero());
//             // merge the imbalance caused by paying the fees and refunding parts of it again.
//             let adjusted_paid = paid.offset(refund_imbalance).same().map_err(|_| {
//                 frame_support::pallet_prelude::TransactionValidityError::Invalid(
//                     frame_support::pallet_prelude::InvalidTransaction::Payment,
//                 )
//             })?;
//             // Call someone else to handle the imbalance (fee and tip separately)
//             let (tip, fee) = adjusted_paid.split(tip);
//             ImbalanceBeneficiary::on_unbalanceds(Some(fee).into_iter().chain(Some(tip)));
//         }
//         Ok(())
//     }
// }
