use crate::*;

pub trait Contracts<AccountId, Balance> {
    type Outcome;
    fn call(
        origin: AccountId,
        dest: AccountId,
        value: Balance,
        gas_limit: sp_weights::Weight,
        storage_deposit_limit: Option<Balance>,
        data: Vec<u8>,
        debug: bool,
    ) -> Self::Outcome;
}

impl<AccountId, Balance: Default> Contracts<AccountId, Balance> for () {
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
            gas_consumed: sp_weights::Weight::zero(),
            gas_required: sp_weights::Weight::zero(),
            debug_message: Vec::new(),
            storage_deposit: StorageDeposit::Refund(Default::default()),
            result: Ok(ExecReturnValue {
                flags: ReturnFlags::empty(),
                data: Vec::default(),
            }),
        }
    }
}
