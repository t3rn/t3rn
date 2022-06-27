use crate::{
    pallet::{Config, Error, Pallet},
    BalanceOf,
};
use codec::{Decode, Encode, MaxEncodedLen};
use fixed::{
    transcendental::pow,
    types::{I32F32, I64F64},
};
use frame_support::{
    ensure,
    traits::{Currency, Get},
};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::CheckedAdd, PerThing, Perbill, RuntimeDebug};
use t3rn_primitives::{common::{BLOCKS_PER_YEAR, Range}, monetary::{InflationAllocation, BeneficiaryRole}};

// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// #[derive(
//     Eq, PartialEq, Clone, Copy, Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo,
// )]
// pub struct Range<T> {
//     pub(crate) min: T,
//     pub(crate) ideal: T,
//     pub(crate) max: T,
// }

// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
// pub enum BeneficiaryRole {
//     Developer,
//     Executor,
// }

// impl<T: Ord> Range<T> {
//     pub fn is_valid(&self) -> bool {
//         self.min <= self.ideal && self.ideal <= self.max
//     }
// }

// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// #[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, Default, RuntimeDebug, TypeInfo)]
// pub struct InflationAllocation {
//     pub(crate) developer: Perbill,
//     pub(crate) executor: Perbill,
// }

// impl InflationAllocation {
//     pub fn is_valid(&self) -> bool {
//         match self.developer.checked_add(&self.executor) {
//             Some(perbill) => perbill == Perbill::one(),
//             None => false,
//         }
//     }
// }

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, Default, RuntimeDebug, TypeInfo)]
pub struct InflationInfo {
    pub annual: Range<Perbill>,
    pub round: Range<Perbill>,
}

// impl Default for InflationInfo {
//     fn default() -> Self {
//         InflationInfo {
//             annual: {
//                 min: Perbill::from_parts(0),
//                 ideal: Perbill::from_parts(0),
//                 max: Perbill::from_parts(0)
//             },
//             round: Range {
//                 min: Perbill::from_parts(0),
//                 ideal: Perbill::from_parts(0),
//                 max: Perbill::from_parts(0)
//             }
//         }
//     }
// }

impl InflationInfo {
    /// Reset round inflation rate based on changes to round term.
    pub fn update_from_round_term<T: Config>(
        &mut self,
        new_round_term: u32,
    ) -> Result<(), Error<T>> {
        ensure!(
            new_round_term > 0 && new_round_term >= T::MinBlocksPerRound::get(),
            <Error<T>>::RoundTermTooShort
        );

        let rounds_per_year = BLOCKS_PER_YEAR / new_round_term;
        self.round = perbill_annual_to_perbill_round(self.annual, rounds_per_year);

        Ok(())
    }

    /// Updates the annual and round inflation config given the annual value.
    pub fn update_from_annual<T: Config>(
        &mut self,
        new_annual_inflation: Range<Perbill>,
    ) -> Result<(), Error<T>> {
        ensure!(
            new_annual_inflation.is_valid(),
            <Error<T>>::InvalidInflationConfig
        );

        self.annual = new_annual_inflation;
        self.round = annual_to_round_inflation::<T>(new_annual_inflation)?;

        Ok(())
    }
}

/// Convert annual inflation rate range to round inflation range
pub fn annual_to_round_inflation<T: Config>(
    annual_inflation: Range<Perbill>,
) -> Result<Range<Perbill>, Error<T>> {
    let rounds_per_year = rounds_per_year::<T>()?;
    let round_inflation = perbill_annual_to_perbill_round(annual_inflation, rounds_per_year);
    Ok(round_inflation)
}

/// Computes the number of rounds per year given a fixed bock time of 12s.
pub fn rounds_per_year<T: Config>() -> Result<u32, Error<T>> {
    let round_term = <Pallet<T>>::current_round().term;

    ensure!(
        round_term > 0 && round_term >= T::MinBlocksPerRound::get(),
        <Error<T>>::RoundTermTooShort
    );

    Ok(BLOCKS_PER_YEAR / round_term)
}

/// Compute round issuance range from round inflation range and current total issuance
pub fn round_issuance_range<T: Config>(round_inflation: Range<Perbill>) -> Range<BalanceOf<T>> {
    let circulating = T::Currency::total_issuance();
    Range {
        min: round_inflation.min * circulating,
        ideal: round_inflation.ideal * circulating,
        max: round_inflation.max * circulating,
    }
}

/// Convert an annual inflation to a round inflation
/// round = (1+annual)^(1/rounds_per_year) - 1
pub fn perbill_annual_to_perbill_round(
    annual_inflation: Range<Perbill>,
    rounds_per_year: u32,
) -> Range<Perbill> {
    let exponent = I32F32::from_num(1) / I32F32::from_num(rounds_per_year);

    let annual_to_round_inflation = |annual_inflation: Perbill| -> Perbill {
        let x =
            I32F32::from_num(annual_inflation.deconstruct()) / I32F32::from_num(Perbill::ACCURACY);
        let y: I64F64 = pow(I32F32::from_num(1) + x, exponent)
            .expect("Cannot overflow since rounds_per_year is u32 so worst case 0; QED");
        Perbill::from_parts(
            ((y - I64F64::from_num(1)) * I64F64::from_num(Perbill::ACCURACY))
                .ceil()
                .to_num::<u32>(),
        )
    };

    Range {
        min: annual_to_round_inflation(annual_inflation.min),
        ideal: annual_to_round_inflation(annual_inflation.ideal),
        max: annual_to_round_inflation(annual_inflation.max),
    }
}

// pub(crate) type RoundIndex = u32;

// #[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
// /// The current round index and transition information
// pub struct RoundInfo<BlockNumber> {
//     /// Current round index.
//     pub index: RoundIndex,
//     /// The first block of the current round.
//     pub head: BlockNumber,
//     /// The length of the current round in number of blocks.
//     pub term: u32,
// }

// impl<
//         B: Copy + sp_std::ops::Add<Output = B> + sp_std::ops::Sub<Output = B> + From<u32> + PartialOrd,
//     > RoundInfo<B>
// {
//     pub fn new(index: RoundIndex, head: B, term: u32) -> RoundInfo<B> {
//         RoundInfo { index, head, term }
//     }

//     /// Check if the round should be updated
//     pub fn should_update(&self, now: B) -> bool {
//         now - self.head >= self.term.into()
//     }

//     /// New round
//     pub fn update(&mut self, now: B) {
//         self.index = self.index.saturating_add(1u32);
//         self.head = now;
//     }
// }

// impl<
//         B: Copy + sp_std::ops::Add<Output = B> + sp_std::ops::Sub<Output = B> + From<u32> + PartialOrd,
//     > Default for RoundInfo<B>
// {
//     fn default() -> RoundInfo<B> {
//         RoundInfo::new(1u32, 1u32.into(), 20u32)
//     }
// }
