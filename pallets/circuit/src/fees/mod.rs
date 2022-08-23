use crate::{pallet::Error, *};

use sp_std::marker::PhantomData;
use t3rn_primitives::transfers::EscrowedBalanceOf;

pub struct Fees<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> Fees<T> {
    pub fn burn_validation_fee(
        _requester: &T::AccountId,
        _sfx_batch: &[SideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
        >],
    ) -> Result<(), Error<T>> {
        // let total_charge = sfx_batch.iter().reduce(|sfx, mut sfx_acc| {
        //     sfx_acc.prize += T::Xdns::get_sfx_validation_fee(sfx.action_type);
        //
        //     sfx_acc
        // });
        //
        // T::Currency::burn(requester, total_charge)?;

        Ok(())
    }

    pub fn charge_deposit(
        _requester: T::AccountId,
        _sfx_batch: &Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, <T as Config>::Escrowed>,
            >,
        >,
    ) -> Result<(), Error<T>> {
        // for sfx in sfx_batch.iter() {
        //     let total_fee =
        //         T::Xdns::get_sfx_execution_and_delivery_fee(sfx.action_type, sfx.target);
        //
        //     T::AccountManager::deposit(requester.clone(), total_fee, sfx.rward)
        // }

        Ok(())
    }

    // Assert infallible
    pub fn finalize_revert(
        _fsx_step: &Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, <T as Config>::Escrowed>,
            >,
        >,
    ) {
    }

    pub fn finalize_commit(
        _fsx_step: &Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, <T as Config>::Escrowed>,
            >,
        >,
    ) -> Result<(), Error<T>> {
        Ok(())
    }
}
