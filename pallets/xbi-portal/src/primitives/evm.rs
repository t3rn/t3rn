use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_system::pallet_prelude::OriginFor;

use sp_core::{H160, H256, U256};
use sp_std::vec::Vec;

pub trait Evm<T: frame_system::Config> {
    fn call(
        origin: OriginFor<T>,
        source: H160,
        target: H160,
        input: Vec<u8>,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: Option<U256>,
        nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
    ) -> DispatchResultWithPostInfo;
}
