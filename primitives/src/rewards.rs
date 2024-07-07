#![cfg_attr(not(feature = "std"), no_std)]
use crate::circuit::CircuitStatus;
use sp_core::H256;
use t3rn_types::fsx::FullSideEffect;

pub trait RewardsWriteApi<Account, Balance, BlockNumber> {
    fn repatriate_for_faulty_or_missing_attestation(
        sfx_id: &H256,
        fsx: &FullSideEffect<Account, BlockNumber, Balance>,
        status: &CircuitStatus,
        requester: Option<Account>,
    ) -> bool;
}
