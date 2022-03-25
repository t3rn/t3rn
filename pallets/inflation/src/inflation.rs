use crate::pallet::{Config, Pallet};
use codec::{Decode, Encode, MaxEncodedLen};
use fixed::transcendental::pow;
use fixed::types::{I32F32, I64F64};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{PerThing, Perbill, RuntimeDebug};

const SECONDS_PER_YEAR: u32 = 31557600;
const SECONDS_PER_BLOCK: u32 = 12;
pub const BLOCKS_PER_YEAR: u32 = SECONDS_PER_YEAR / SECONDS_PER_BLOCK;

fn rounds_per_year<T: Config>() -> u32 {
    let blocks_per_round = <Pallet<T>>::current_round().length;
    BLOCKS_PER_YEAR / blocks_per_round
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Eq, PartialEq, Clone, Copy, Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo,
)]
pub struct Range<T> {
    pub(crate) min: T,
    pub(crate) ideal: T,
    pub(crate) max: T,
}

impl<T: Ord> Range<T> {
    pub fn is_valid(&self) -> bool {
        self.min <= self.ideal && self.ideal <= self.max
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, Default, RuntimeDebug, TypeInfo)]
pub struct InflationInfo {
    pub(crate) annual: Range<Perbill>,
    pub(crate) per_round: Range<Perbill>,
}

impl InflationInfo {
    /// Set round inflation range according to input annual inflation range
    pub fn set_round_from_annual<T: Config>(&mut self, new: Range<Perbill>) {
        self.per_round = annual_to_round::<T>(new);
    }
}
/// Convert an annual inflation to a round inflation
/// round = (1+annual)^(1/rounds_per_year) - 1
pub fn perbill_annual_to_perbill_round(
    annual: Range<Perbill>,
    rounds_per_year: u32,
) -> Range<Perbill> {
    let exponent = I32F32::from_num(1) / I32F32::from_num(rounds_per_year);

    let annual_to_round = |annual: Perbill| -> Perbill {
        let x = I32F32::from_num(annual.deconstruct()) / I32F32::from_num(Perbill::ACCURACY);
        let y: I64F64 = pow(I32F32::from_num(1) + x, exponent)
            .expect("Cannot overflow since rounds_per_year is u32 so worst case 0; QED");
        Perbill::from_parts(
            ((y - I64F64::from_num(1)) * I64F64::from_num(Perbill::ACCURACY))
                .ceil()
                .to_num::<u32>(),
        )
    };
    Range {
        min: annual_to_round(annual.min),
        ideal: annual_to_round(annual.ideal),
        max: annual_to_round(annual.max),
    }
}

/// Convert annual inflation rate range to round inflation range
pub fn annual_to_round<T: Config>(annual: Range<Perbill>) -> Range<Perbill> {
    let periods = rounds_per_year::<T>();
    perbill_annual_to_perbill_round(annual, periods)
}

pub(crate) type RoundIndex = u32;

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
/// The current round index and transition information
pub struct RoundInfo<BlockNumber> {
    /// Current round index
    pub current: RoundIndex,
    /// The first block of the current round
    pub first_block: BlockNumber,
    /// The length of the current round in number of blocks
    pub length: u32,
}
impl<
        B: Copy + sp_std::ops::Add<Output = B> + sp_std::ops::Sub<Output = B> + From<u32> + PartialOrd,
    > RoundInfo<B>
{
    pub fn new(current: RoundIndex, first_block: B, length: u32) -> RoundInfo<B> {
        RoundInfo {
            current,
            first_block,
            length,
        }
    }
    /// Check if the round should be updated
    pub fn should_update(&self, now: B) -> bool {
        now - self.first_block >= self.length.into()
    }
    /// New round
    pub fn update(&mut self, now: B) {
        self.current = self.current.saturating_add(1u32);
        self.first_block = now;
    }
}
impl<
        B: Copy + sp_std::ops::Add<Output = B> + sp_std::ops::Sub<Output = B> + From<u32> + PartialOrd,
    > Default for RoundInfo<B>
{
    fn default() -> RoundInfo<B> {
        RoundInfo::new(1u32, 1u32.into(), 20u32)
    }
}
