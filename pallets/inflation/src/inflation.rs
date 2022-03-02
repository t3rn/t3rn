use crate::pallet::{Config, Pallet};
use codec::{Decode, Encode, MaxEncodedLen};
use fixed::{
    transcendental::pow,
    types::{I32F32, I64F64},
};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{PerThing, Perbill, RuntimeDebug};

const SECONDS_PER_YEAR: u32 = 31557600;
const SECONDS_PER_BLOCK: u32 = 12;
pub const BLOCKS_PER_YEAR: u32 = SECONDS_PER_YEAR / SECONDS_PER_BLOCK;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Eq, PartialEq, Clone, Copy, Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo,
)]
pub struct Range<T> {
    pub(crate) min: T,
    pub(crate) ideal: T,
    pub(crate) max: T,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum CandidateRole {
    Developer,
    Executor,
}

impl<T: Ord> Range<T> {
    pub fn is_valid(&self) -> bool {
        self.min <= self.ideal && self.ideal <= self.max
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, Default, RuntimeDebug, TypeInfo)]
pub struct RewardsAllocationConfig {
    pub(crate) developer: Perbill,
    pub(crate) executor: Perbill,
}

impl RewardsAllocationConfig {
    pub fn validate(&self) -> bool {
        self.developer + self.executor == Perbill::one()
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, Default, RuntimeDebug, TypeInfo)]
pub struct InflationInfo {
    pub(crate) annual: Range<Perbill>,
    pub(crate) round: Range<Perbill>,
    pub(crate) rewards_alloc: RewardsAllocationConfig,
}

impl InflationInfo {
    /// Reset round inflation rate based on changes to round term.
    pub fn update_round_term(&mut self, new_round_term: u32) {
        let periods = BLOCKS_PER_YEAR / new_round_term;
        self.round = perbill_annual_to_perbill_round(self.annual, periods);
    }

    /// Updates the annual and round inflation config given the annual value.
    pub fn update_from_annual<T: Config>(&mut self, new_annual_inflation_config: Range<Perbill>) {
        self.annual = new_annual_inflation_config;
        self.round = annual_to_round::<T>(new_annual_inflation_config);
    }
}

/// Convert annual inflation rate range to round inflation range
pub fn annual_to_round<T: Config>(annual: Range<Perbill>) -> Range<Perbill> {
    let periods = rounds_per_year::<T>();
    perbill_annual_to_perbill_round(annual, periods)
}

fn rounds_per_year<T: Config>() -> u32 {
    let blocks_per_round = <Pallet<T>>::current_round().term;
    BLOCKS_PER_YEAR / blocks_per_round
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

pub(crate) type RoundIndex = u32;

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
/// The current round index and transition information
pub struct RoundInfo<BlockNumber> {
    /// Current round index.
    pub index: RoundIndex,
    /// The first block of the current round.
    pub head: BlockNumber,
    /// The length of the current round in number of blocks.
    pub term: u32,
}

impl<
        B: Copy + sp_std::ops::Add<Output = B> + sp_std::ops::Sub<Output = B> + From<u32> + PartialOrd,
    > RoundInfo<B>
{
    pub fn new(index: RoundIndex, head: B, term: u32) -> RoundInfo<B> {
        RoundInfo { index, head, term }
    }

    /// Check if the round should be updated
    pub fn should_update(&self, now: B) -> bool {
        now - self.head >= self.term.into()
    }

    /// New round
    pub fn update(&mut self, now: B) {
        self.index = self.index.saturating_add(1u32);
        self.head = now;
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
