use frame_support::pallet_prelude::*;
use sp_core::H256;
use sp_std::prelude::*;

pub trait RewardsWriteApi<Account, Error> {
    fn repatriate_executors_from_slash_treasury(sfx_id: &H256) -> bool;
    fn repatriate_requesters_from_slash_treasury(sfx_id: &H256) -> bool;
}
