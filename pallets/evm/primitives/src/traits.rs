use sp_core::{H160, H256, U256};
use sp_std::vec::Vec;

pub trait Evm<Origin> {
    type Outcome;
    #[allow(clippy::too_many_arguments)] // Simply has a lot of args
    fn call(
        origin: Origin,
        target: H160,
        input: Vec<u8>,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: Option<U256>,
        nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
    ) -> Self::Outcome;
}

// FIXME; THIS IS CURRENTLY WORTHLESS SINCE WE REQUIRE POSTDISPATCHINFO NATIVE TO FRAME, do this later
impl<Origin> Evm<Origin> for () {
    type Outcome = ();

    fn call(
        _origin: Origin,
        _target: H160,
        _input: Vec<u8>,
        _value: U256,
        _gas_limit: u64,
        _max_fee_per_gas: U256,
        _max_priority_fee_per_gas: Option<U256>,
        _nonce: Option<U256>,
        _access_list: Vec<(H160, Vec<H256>)>,
    ) -> Self::Outcome {
    }
}
