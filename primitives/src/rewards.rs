#![cfg_attr(not(feature = "std"), no_std)]
use sp_core::H256;
use t3rn_types::fsx::FullSideEffect;

pub trait RewardsWriteApi<Account, Balance, BlockNumber> {
    fn repatriate_executor_from_slash_treasury(
        sfx_id: &H256,
        fsx: &FullSideEffect<Account, BlockNumber, Balance>,
    ) -> bool;
}
