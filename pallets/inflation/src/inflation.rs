use crate::pallet::Config;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{Perbill, RuntimeDebug};

const SECONDS_PER_YEAR: u32 = 31557600;
const SECONDS_PER_BLOCK: u32 = 12;
pub const BLOCKS_PER_YEAR: u32 = SECONDS_PER_YEAR / SECONDS_PER_BLOCK;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Eq, PartialEq, Clone, Copy, Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo,
)]
pub struct Range<T> {
    min: T,
    ideal: T,
    max: T,
}

impl<T> Default for Range<T> {
    fn default() -> Self {
        Range {
            min: Perbill::from_percent(0),
            ideal: Perbill::from_percent(0),
            max: Perbill::from_percent(0),
        }
    }
}

pub struct InflationInfo<T: Config> {
    annual: Range<Perbill>,
    per_round: Range<T::Balance>,
}
