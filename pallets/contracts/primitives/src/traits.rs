use crate::*;

pub trait Contracts<AccountId, Balance, Weight> {
    type Outcome;
    fn call(
        origin: AccountId,
        dest: AccountId,
        value: Balance,
        gas_limit: Weight,
        storage_deposit_limit: Option<Balance>,
        data: Vec<u8>,
        debug: bool,
    ) -> Self::Outcome;
}

impl<AccountId, Balance: Default, Weight> Contracts<AccountId, Balance, Weight> for () {
    type Outcome = ContractExecResult<Balance>;

    fn call(
        _origin: AccountId,
        _dest: AccountId,
        _value: Balance,
        _gas_limit: Weight,
        _storage_deposit_limit: Option<Balance>,
        _data: Vec<u8>,
        _debug: bool,
    ) -> Self::Outcome {
        ContractExecResult {
            gas_consumed: 0,
            gas_required: 0,
            debug_message: Vec::new(),
            storage_deposit: StorageDeposit::Refund(Default::default()),
            result: Ok(ComposableExecReturnValue {
                flags: ReturnFlags::empty(),
                data: sp_core::Bytes(Vec::new()),
                side_effects: Vec::new(),
            }),
        }
    }
}
