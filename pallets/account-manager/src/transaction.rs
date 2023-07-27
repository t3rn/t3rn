use crate::Config;
use frame_support::traits::{Currency, Imbalance, OnUnbalanced, SameOrOther, TryDrop};

pub type NegativeImbalanceOf<T, C> =
    <C as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

pub type PositiveImbalanceOf<T, C> =
    <C as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;

pub type BalanceForCurrency<T, C> =
    <C as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub struct ImbalanceToBeneficiary<T>(sp_std::marker::PhantomData<T>);

pub struct ImbalanceBeneficiary<T: frame_system::Config, C: Currency<T::AccountId>>(
    pub Option<T::AccountId>,
    pub NegativeImbalanceOf<T, C>,
);

impl<T: frame_system::Config, C: Currency<T::AccountId>> Default for ImbalanceBeneficiary<T, C> {
    fn default() -> Self {
        Self(None, Default::default())
    }
}

impl<T: frame_system::Config, C: Currency<T::AccountId>> TryDrop for ImbalanceBeneficiary<T, C> {
    fn try_drop(self) -> Result<(), Self> {
        self.drop_zero()
    }
}

impl<T: frame_system::Config, C: Currency<T::AccountId>> Imbalance<BalanceForCurrency<T, C>>
    for ImbalanceBeneficiary<T, C>
{
    type Opposite = PositiveImbalanceOf<T, C>;

    fn zero() -> Self {
        Self(None, NegativeImbalanceOf::<T, C>::zero())
    }

    fn drop_zero(self) -> Result<(), Self> {
        self.1.drop_zero().map_err(|e| Self(self.0, e))
    }

    fn split(self, amount: BalanceForCurrency<T, C>) -> (Self, Self) {
        let (first, second) = self.1.split(amount);
        (Self(self.0.clone(), first), Self(self.0, second))
    }

    fn merge(self, other: Self) -> Self {
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

    fn peek(&self) -> BalanceForCurrency<T, C> {
        self.1.peek()
    }
}

// For a negative imbalance, some account decreased due to either slashing or paying for a tx
impl<T, C> OnUnbalanced<ImbalanceBeneficiary<T, C>> for ImbalanceToBeneficiary<T>
where
    T: Config<Currency = C>,
    C: Currency<T::AccountId>,
{
    // this seems to be called for substrate-based transactions when there is a difference between pre dispatch
    // and post dispatch balances.
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = ImbalanceBeneficiary<T, C>>) {
        if let Some(ImbalanceBeneficiary(Some(beneficiary), fees)) = fees_then_tips.next() {
            // Balances pallet automatically burns dropped Negative Imbalances by decreasing
            // total_supply when the fee is dropped
            // TODO[Feature]: at some point these may want to be configured by the pallet
            <T as Config>::Currency::resolve_creating(&beneficiary, fees);
        }
        // Otherwise drop scope
    }

    // this is called from pallet_evm for Ethereum-based transactions
    // (technically, it calls on_unbalanced, which calls this when non-zero)
    fn on_nonzero_unbalanced(amount: ImbalanceBeneficiary<T, C>) {
        let (beneficiary, fees) = (amount.0, amount.1);
        if let Some(beneficiary) = beneficiary {
            <T as Config>::Currency::resolve_creating(&beneficiary, fees);
        }
        // Otherwise drop scope
    }
}

// TODO[https://github.com/t3rn/3vm/issues/54]: this will need to be integration tested since it is coupled to the runtime
/// The purpose of this macro is to setup the currency adapter for a runtime so that it can be
/// author-aware. This needs to be a macro since the injection site can only be at the runtime, we don't want to import
/// pallet-balances or pallet-transaction-payment, since these traits are not available in frame_support and
/// only in their respective pallets.
#[macro_export]
macro_rules! setup_currency_adapter {
    () => {
        use sp_runtime::traits::Saturating;
        use frame_support::traits::IsSubType;
        use pallet_transaction_payment::CurrencyAdapter;
        use codec::Decode;

        pub struct AccountManagerCurrencyAdapter<C, OU>(sp_std::marker::PhantomData<(C, OU)>);
        type AccountImbalanceLiquidityInfo<T, C> = (
            Option<<T as frame_system::Config>::AccountId>,
            Option<pallet_account_manager::transaction::NegativeImbalanceOf<T, C>>,
        );

        impl<T, C, OU> pallet_transaction_payment::OnChargeTransaction<T>
            for AccountManagerCurrencyAdapter<C, OU>
        where
            T: pallet_transaction_payment::Config + pallet_account_manager::Config<Currency = C>,
            <T as frame_system::Config>::RuntimeCall: IsSubType<pallet_3vm_contracts::Call<Runtime>>,
            C: frame_support::traits::Currency<<T as frame_system::Config>::AccountId>,
            C::PositiveImbalance: Imbalance<
                <C as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance,
                Opposite = C::NegativeImbalance,
            >,
            C::NegativeImbalance: Imbalance<
                <C as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance,
                Opposite = C::PositiveImbalance,
            >,
            OU: OnUnbalanced<<C as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance>,
        {
            type Balance =
                <C as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance;
            type LiquidityInfo = AccountImbalanceLiquidityInfo<T, C>;

            fn withdraw_fee(
                who: &T::AccountId,
                call: &<T as frame_system::Config>::RuntimeCall,
                info: &sp_runtime::traits::DispatchInfoOf<<T as frame_system::Config>::RuntimeCall>,
                fee: Self::Balance,
                tip: Self::Balance,
            ) -> Result<Self::LiquidityInfo, frame_support::pallet_prelude::TransactionValidityError> {
                let call: &<T as frame_system::Config>::RuntimeCall = call;

                let result = <CurrencyAdapter<C, OU> as pallet_transaction_payment::OnChargeTransaction<T>>::withdraw_fee(
                    who, call, info, fee, tip,
                );

                if let Some(pallet_3vm_contracts::Call::call { dest, .. }) = call.is_sub_type() {
                    if let Some(author) = ThreeVm::get_author(dest) {
                        return result.map(|info| {
                            let opaque_author = T::AccountId::decode(&mut author.as_ref()).ok();
                            (opaque_author, info)
                        });
                    }
                }

                result.map(|info| (None, info))
            }

            // Largely, this is a copy of pallet-tx-payment except the part at the end where we check
            // for some beneficiary, and peg that metadata to the call to OnUnbalanced
            fn correct_and_deposit_fee(
                who: &T::AccountId,
                _dispatch_info: &sp_runtime::traits::DispatchInfoOf<<T as frame_system::Config>::RuntimeCall>,
                _post_info: &sp_runtime::traits::PostDispatchInfoOf<<T as frame_system::Config>::RuntimeCall>,
                corrected_fee: Self::Balance,
                tip: Self::Balance,
                already_withdrawn: Self::LiquidityInfo,
            ) -> Result<(), frame_support::pallet_prelude::TransactionValidityError> {
                if let (beneficiary, Some(paid)) = already_withdrawn {
                    // Calculate how much refund we should return
                    let refund_amount = paid.peek().saturating_sub(corrected_fee);
                    // refund to the the account that paid the fees. If this fails, the
                    // account might have dropped below the existential balance. In
                    // that case we don't refund anything.
                    let refund_imbalance = C::deposit_into_existing(who, refund_amount)
                        .unwrap_or_else(|_| C::PositiveImbalance::zero());
                    // merge the imbalance caused by paying the fees and refunding parts of it again.
                    let adjusted_paid = paid.offset(refund_imbalance).same().map_err(|_| {
                        frame_support::pallet_prelude::TransactionValidityError::Invalid(
                            frame_support::pallet_prelude::InvalidTransaction::Payment,
                        )
                    })?;

                    // Call someone else to handle the imbalance (fee and tip separately)
                    let (tip, fee) = adjusted_paid.split(tip);

                    // Since there is a beneficiary, account-manager handles it
                    if let Some(beneficiary) = beneficiary {
                        let fee_with_tip = Some(fee).into_iter().chain(Some(tip));
                        let as_beneficiaries = fee_with_tip
                            .map(|i| {
                                pallet_account_manager::transaction::ImbalanceBeneficiary::<T, C>(Some(beneficiary.clone()), i)
                            });
                        <pallet_account_manager::transaction::ImbalanceToBeneficiary<T> as OnUnbalanced<pallet_account_manager::transaction::ImbalanceBeneficiary<T, C>>>::on_unbalanceds(as_beneficiaries);
                    } else {
                        <OU as OnUnbalanced<<C as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance>>::on_unbalanceds(Some(fee).into_iter().chain(Some(tip)));
                    }
                }
                Ok(())
            }
        }
    };
}
