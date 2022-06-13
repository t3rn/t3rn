use fixed::{
    transcendental::pow,
    types::{I32F32, I64F64},
};
use sp_runtime::{PerThing, Perbill};

#[derive(Debug)]
pub struct Range<T> {
    pub(crate) min: T,
    pub(crate) ideal: T,
    pub(crate) max: T,
}

const SECONDS_PER_YEAR: u32 = 31557600;
const SECONDS_PER_BLOCK: u32 = 12;
const BLOCKS_PER_YEAR: u32 = SECONDS_PER_YEAR / SECONDS_PER_BLOCK;

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

fn main() {
    let new_annual_inflation = Range {
        min: Perbill::from_percent(3),
        ideal: Perbill::from_percent(4),
        max: Perbill::from_percent(5),
    };

    let rounds = BLOCKS_PER_YEAR / 20;
    let round_inflation = perbill_annual_to_perbill_round(new_annual_inflation, rounds);

    println!("{:?}", round_inflation);
}
