use crate::*;

use frame_support::{parameter_types, traits::ConstU32, weights::Weight, PalletId};
use pallet_grandpa_finality_verifier::bridges::runtime as bp_runtime;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::Perbill;

use sp_runtime::traits::One;

use sp_runtime::traits::Zero;

impl t3rn_primitives::EscrowTrait<Runtime> for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

struct GlobalOnInitQueues;

impl pallet_clock::traits::OnHookQueues<Runtime> for GlobalOnInitQueues {
    fn process(n: BlockNumber, on_init_weight_limit: Weight) -> Weight {
        // Iterate over all pre-init hooks implemented by pallets and return aggregated weight
        #[cfg_attr(rustfmt, rustfmt_skip)] // prefer to keep hooks declaration in one line
        let hooks = vec![
            (Circuit::process_signal_queue, BlockNumber::one(), Perbill::from_percent(15), "Circuit::Signals", ),
            (Circuit::process_xtx_tick_queue, BlockNumber::one(), Perbill::from_percent(35), "Circuit::XtxTicks", ),
            (Circuit::process_revert_xtx_queue, Circuit::XtxTimeoutCheckInterval::get(), Perbill::from_percent(35), "Circuit::XtxRevert", ),
            (Clock::check_bump_round, BlockNumber::one(), Perbill::from_percent(5), "Clock::BumpRound", ),
            (Clock::calculate_claimable_for_round, BlockNumber::one(), Perbill::from_percent(10), "Clock::CalcClaimable", ),
        ];

        let mut weight: Weight = 0;
        for (hook, interval, percentage_share, dbg_queue_name) in hooks {
            if n % interval == BlockNumber::zero() {
                weight = weight.saturating_add(hook(
                    n,
                    interval,
                    percentage_share * on_init_weight_limit,
                ));
                if weight > on_init_weight_limit {
                    log::error!(
                        "GlobalOnInitQueues::on_init_weight_limit exceeded for queue: {}",
                        dbg_queue_name
                    );
                }
            }
        }
        // weight = weight.saturating_add(Circuit::process_signal_queue(n, BlockNumber::one(), Perbill::from_percent(25) * on_init_weight_limit));
        // weight = weight.saturating_add(Circuit::process_xtx_tick_queue(n, BlockNumber::one(), on_init_weight_limit * Percent::from_percent(30)));
        // weight = weight.saturating_add(Circuit::process_revert_xtx_queue(n, BlockNumber::one(), on_init_weight_limit * Percent::from_percent(30)));
        // weight = weight.saturating_add(Self::process_revert_xtx_queue(n, Circuit::XtxTimeoutCheckInterval::get(), BlockExecutionWeight::get() / 10));
        weight
    }
}

impl pallet_clock::Config for Runtime {
    type AccountManager = AccountManager;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type OnFinalizeQueues = pallet_clock::traits::EmptyOnHookQueues<Self>;
    type OnInitializeQueues = pallet_clock::traits::EmptyOnHookQueues<Self>;
    type RoundDuration = ConstU32<500u32>;
}

impl pallet_xdns::Config for Runtime {
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type Time = Timestamp;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<Runtime>;
}

impl pallet_contracts_registry::Config for Runtime {
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type WeightInfo = pallet_contracts_registry::weights::SubstrateWeight<Runtime>;
}

impl pallet_portal::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_portal::weights::SubstrateWeight<Runtime>;
    type Xdns = XDNS;
}

parameter_types! {
    pub const PortalPalletId: PalletId = PalletId(*b"pal/port");
}
pub struct AccountId32Converter;
impl Convert<AccountId, [u8; 32]> for AccountId32Converter {
    fn convert(account_id: AccountId) -> [u8; 32] {
        account_id.into()
    }
}

parameter_types! {
    pub const CircuitAccountId: AccountId = AccountId::new([51u8; 32]); // 0x333...3
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
}

impl pallet_circuit::Config for Runtime {
    type AccountManager = AccountManager;
    type Balances = Balances;
    type Call = Call;
    type Currency = Balances;
    type DeletionQueueLimit = ConstU32<100u32>;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Portal = Portal;
    type SFXBiddingPeriod = ConstU32<3u32>;
    type SelfAccountId = CircuitAccountId;
    type SelfGatewayId = SelfGatewayId;
    type SelfParaId = ConstU32<3333u32>;
    type SignalQueueDepth = ConstU32<5u32>;
    type WeightInfo = ();
    type Xdns = XDNS;
    type XtxTimeoutCheckInterval = ConstU32<10u32>;
    type XtxTimeoutDefault = ConstU32<400u32>;
}

parameter_types! {
    pub const HeadersToStore: u32 = 100;
}

type RococoBridgeInstance = ();

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl bp_runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

impl pallet_grandpa_finality_verifier::Config<RococoBridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type HeadersToStore = HeadersToStore;
    type WeightInfo = ();
}
