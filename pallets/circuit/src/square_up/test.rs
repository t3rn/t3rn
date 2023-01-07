#[cfg(test)]
pub mod test {

    use circuit_mock_runtime::{ExtBuilder, Runtime, System};
    use circuit_runtime_pallets::pallet_circuit::{
        square_up::{SquareUp},
        state::{Cause, CircuitStatus},
    };

    // use crate::square_up::test_extra::*;

    #[test]
    fn square_up_locks_up_requester_with_enough_native_currency() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                assert!(false);
            });
    }
}
