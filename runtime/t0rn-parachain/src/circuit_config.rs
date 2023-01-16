use crate::{AccountId, AccountManager, Balances, Call, Event, Portal, Runtime, Timestamp, XDNS};

use frame_support::{parameter_types, traits::ConstU32, PalletId};
use pallet_grandpa_finality_verifier::bridges::runtime as bp_runtime;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Convert};
use t3rn_primitives::common::DEFAULT_ROUND_TERM;

impl t3rn_primitives::EscrowTrait<Runtime> for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

impl pallet_clock::Config for Runtime {
    type AccountManager = AccountManager;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
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
    // type XBIPortal = XBIPortalRuntimeEntry;
    // type XBIPromise = XBIPortal;
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
