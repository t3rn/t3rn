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
use sp_runtime::{PerThing, Perbill, RuntimeDebug};
use t3rn_primitives::common::{Range, BLOCKS_PER_YEAR};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, Default, RuntimeDebug, TypeInfo)]
pub struct InflationInfo {
    pub annual: Range<Perbill>,
    pub round: Range<Perbill>,
}

impl InflationInfo {
    /// Reset round inflation rate based on changes to round term.
    pub fn update_from_round_term<T: Config>(
        &mut self,
        new_round_term: u32,
    ) -> Result<(), Error<T>> {
        ensure!(
            new_round_term > 0 && new_round_term >= T::MinRoundTerm::get(),
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

/// Computes the number of rounds per year given a fixed bock time of 12s.
pub fn rounds_per_year<T: Config>() -> Result<u32, Error<T>> {
    let round_term = <Pallet<T>>::current_round().term;

    ensure!(
        round_term > 0 && round_term >= T::MinRoundTerm::get(),
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
