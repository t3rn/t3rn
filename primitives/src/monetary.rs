use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::CheckedAdd, Perbill, RuntimeDebug};

pub const DECIMALS: u8 = 18;
pub const MILLIT3RN: u64 = 1_000_000_000_000_000;
pub const TRN: u64 = 1_000_000_000_000_000_000;
pub const EXISTENTIAL_DEPOSIT: u64 = 1;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum BeneficiaryRole {
    Developer,
    Executor,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, Default, RuntimeDebug, TypeInfo)]
pub struct InflationAllocation {
    pub developer: Perbill,
    pub executor: Perbill,
}

impl InflationAllocation {
    pub fn is_valid(&self) -> bool {
        match self.developer.checked_add(&self.executor) {
            Some(perbill) => perbill == Perbill::one(),
            None => false,
        }
    }
}
