pub use circuit_mock_runtime::*;
use frame_support::traits::{GenesisBuild, OnFinalize, OnInitialize};

pub(crate) fn last_event() -> Event {
    System::events().pop().expect("event expected").event
}

pub(crate) fn last_n_events(n: usize) -> Vec<pallet_executors::Event<Runtime>> {
    let events = System::events();
    let len = events.len();
    if len > 0 {
        events[len - n..]
            .iter()
            .map(|r| r.event.clone())
            .filter_map(|e| {
                if let Event::Executors(inner) = e {
                    Some(inner)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    }
}

/// Assert input equal to the last event emitted
#[macro_export]
macro_rules! assert_last_event {
    ($event:expr) => {
        match &$event {
            e => assert_eq!($crate::mock::last_event(), *e),
        }
    };
}

/// Assert input equal to the last n events emitted
#[macro_export]
macro_rules! assert_last_n_events {
    ($n:expr, $event:expr) => {
        match &$event {
            e => similar_asserts::assert_eq!($crate::mock::last_n_events($n), *e),
        }
    };
}

pub(crate) fn fast_forward_to(n: u32) {
    while System::block_number() < n {
        Executors::on_finalize(System::block_number());
        Clock::on_finalize(System::block_number());
        Balances::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(if let Some(v) = System::block_number().checked_add(1) {
            v
        } else {
            System::block_number()
        });
        System::on_initialize(System::block_number());
        Balances::on_initialize(System::block_number());
        Executors::on_initialize(System::block_number());
        Clock::on_initialize(System::block_number());
    }
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .expect("mock pallet-staking genesis storage");

    circuit_runtime_pallets::pallet_executors::GenesisConfig::<Runtime>::default()
        .assimilate_storage(&mut storage)
        .expect("mock pallet-staking genesis storage assimilation");

    let mut ext = sp_io::TestExternalities::from(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
