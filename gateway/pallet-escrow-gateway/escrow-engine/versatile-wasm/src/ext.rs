use crate::gas::GasMeter;
use crate::*;
use frame_support::dispatch::DispatchError;
use frame_support::traits::Randomness;
use frame_support::weights::Weight;
use frame_support::{
    storage::child,
    storage::child::{get_raw, ChildInfo},
};
use gateway_escrow_engine::{
    transfers::{escrow_transfer, BalanceOf, TransferEntry},
    DispatchRuntimeCall, EscrowTrait, ExtendedWasm,
};
use sp_std::vec::Vec;
use system::Trait as SystemTrait;

pub struct DefaultRuntimeEnv<'a, T: EscrowTrait + SystemTrait + ExtendedWasm> {
    pub escrow_account: &'a T::AccountId,
    pub requester: &'a T::AccountId,
    pub block_number: <T as SystemTrait>::BlockNumber,
    pub timestamp: <<T as EscrowTrait>::Time as Time>::Moment,
    pub escrow_account_trie_id: ChildInfo,
    pub input_data: Option<Vec<u8>>,
    pub inner_exec_transfers: &'a mut Vec<TransferEntry>,
}

impl<'a, T: EscrowTrait + SystemTrait> ExtStandards for DefaultRuntimeEnv<'a, T>
where
    T: EscrowTrait + SystemTrait + ExtendedWasm,
{
    type T = T;

    fn get_storage(&self, key: &StorageKey) -> Option<Vec<u8>> {
        get_raw(&self.escrow_account_trie_id, key)
    }

    fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>) {
        match value {
            Some(new_value) => child::put_raw(&self.escrow_account_trie_id, &key, &new_value[..]),
            None => child::kill(&self.escrow_account_trie_id, &key),
        }
    }

    fn transfer(
        &mut self,
        to: &T::AccountId,
        value: BalanceOf<T>,
        _gas_meter: &mut GasMeter<T>,
    ) -> Result<(), DispatchError> {
        escrow_transfer::<T>(
            &self.escrow_account.clone(),
            &self.requester.clone(),
            to,
            value,
            self.inner_exec_transfers,
        )
    }

    fn call(
        &mut self,
        module_name: &str,
        fn_name: &str,
        to: &T::AccountId,
        value: BalanceOf<T>,
        gas_meter: &mut GasMeter<T>,
        input_data: Vec<u8>,
    ) -> Result<(), DispatchError> {
        T::DispatchRuntimeCall::dispatch_runtime_call(
            module_name,
            fn_name,
            &input_data[..],
            self.escrow_account,
            self.requester,
            to,
            value,
            gas_meter.gas_spent(),
        )
    }

    fn deposit_event(&mut self, topics: Vec<TopicOf<Self::T>>, data: Vec<u8>) {
        // deposit_event::<Self::T>(
        //     topics,
        //     RawEvent::VersatileVMExecution(self.ctx.self_account.clone(), data),
        // );

        <system::Module<T>>::deposit_event_indexed(
            &*topics,
            <Self::T as ExtendedWasm>::Event::from(
                gateway_escrow_engine::RawEvent::VersatileVMExecution(
                    self.escrow_account.clone(),
                    self.requester.clone(),
                    data,
                ),
            )
            .into(),
        )

        // <system::Module<Self::T>>::deposit_event_indexed(
        //     &*topics,
        //     gateway_escrow_engine::RawEvent::VersatileVMExecution(
        //         self.escrow_account.clone(),
        //         self.requester.clone(),
        //         data,
        //     )
        //     .into(),
        // )
    }
}

// fn deposit_event<T: Trait>(topics: Vec<T::Hash>, event: Event<T>) {
//     <frame_system::Module<T>>::deposit_event_indexed(
//         &*topics,
//         <T as Trait>::Event::from(event).into(),
//     )
// }

pub trait ExtStandards {
    type T: EscrowTrait + SystemTrait + ExtendedWasm;

    /// Returns the storage entry of the executing account by the given `key`.
    ///
    /// Returns `None` if the `key` wasn't previously set by `set_storage` or
    /// was deleted.
    fn get_storage(&self, key: &StorageKey) -> Option<Vec<u8>>;

    /// Sets the storage entry by the given key to the specified value. If `value` is `None` then
    /// the storage entry is deleted.
    fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>);

    /// Transfer some amount of funds into the specified account.
    fn transfer(
        &mut self,
        to: &AccountIdOf<Self::T>,
        value: BalanceOf<Self::T>,
        gas_meter: &mut GasMeter<Self::T>,
    ) -> Result<(), DispatchError>;

    /// Call (possibly transferring some amount of funds) into the specified account.
    fn call(
        &mut self,
        module_name: &str,
        fn_name: &str,
        to: &AccountIdOf<Self::T>,
        value: BalanceOf<Self::T>,
        gas_meter: &mut GasMeter<Self::T>,
        input_data: Vec<u8>,
    ) -> Result<(), DispatchError>;

    /// There should not be any duplicates in `topics`.
    fn deposit_event(&mut self, topics: Vec<TopicOf<Self::T>>, data: Vec<u8>);
    /// Default implementations based on configured trait.

    /// Returns a reference to the timestamp of the current block
    fn now(&self) -> MomentOf<Self::T> {
        <Self::T as EscrowTrait>::Time::now()
    }

    /// Deposit an event with the given topics.
    ///

    /// Returns the current block number.
    fn block_number(&self) -> BlockNumberOf<Self::T> {
        system::Module::<Self::T>::block_number()
    }

    /// Returns the price for the specified amount of weight.
    fn get_weight_price(&self, _weight: Weight) -> BalanceOf<Self::T> {
        unimplemented!()
    }

    /// Returns a random number for the current block with the given subject.
    fn random(&self, subject: &[u8]) -> SeedOf<Self::T> {
        <Self::T as ExtendedWasm>::Randomness::random(subject)
    }

    /// <h2>Config</h2>
    ///
    /// Returns the minimum balance that is required for creating an account.
    fn minimum_balance(&self) -> BalanceOf<Self::T> {
        <Self::T as EscrowTrait>::Currency::minimum_balance()
    }

    /// Returns the deposit required to create a tombstone upon contract eviction.
    fn tombstone_deposit(&self) -> BalanceOf<Self::T> {
        <Self::T as EscrowTrait>::Currency::minimum_balance()
    }
    /// Returns the maximum allowed size of a storage item.
    fn max_value_size(&self) -> u32 {
        16_384
    }

    /// <h2>Builtins</h2>
    /// All of builtins are by default implemented by Versatile VM and can be used to execute attached code
    /// on that chain without implementing it. Some of builtins rely on the configuration which is passed as a module argument or derived from a system trait.

    /// Returns the value transferred along with this call or as endowment.
    fn seal_value_transferred(&self) -> BalanceOf<Self::T> {
        unimplemented!("Builtin")
    }

    /// Returns a reference to the account id of the original execution requester.
    fn seal_requester(&self) -> &AccountIdOf<Self::T> {
        unimplemented!("Builtin")
    }

    /// Returns a reference to the account id of the current contract.
    fn seal_escrow_address(&self) -> &AccountIdOf<Self::T> {
        unimplemented!("Builtin")
    }

    /// Returns the balance of the escrow account.
    ///
    /// The `value_transferred` is already added.
    fn seal_balance(&self) -> BalanceOf<Self::T> {
        unimplemented!("Builtin")
    }
}
