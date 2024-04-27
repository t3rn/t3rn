use circuit_runtime_types::{
    Balance, MICROUNIT as BASE_MICROUNIT, MILLIUNIT as BASE_MILLIUNIT, UNIT as BASE_UNIT,
};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::CheckedAdd, Perbill, RuntimeDebug};

pub const DECIMALS: u8 = 12;
// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = BASE_UNIT;
pub const MILLIUNIT: Balance = BASE_MILLIUNIT;
pub const MICROUNIT: Balance = BASE_MICROUNIT;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

// Existential deposit for testing purposes
pub const MOCK_EXISTENTIAL_DEPOSIT: Balance = 1;

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
