use super::*;
use frame_support::{parameter_types, traits::ConstU32, PalletId};
use pallet_grandpa_finality_verifier::bridges::runtime as bp_runtime;
use sp_core::H256;
use sp_runtime::traits::*;
use t3rn_primitives::portal::RococoBridge;

// impl pallet_randomness_collective_flip::Config for Runtime {}

// t3rn pallets
impl t3rn_primitives::EscrowTrait<Runtime> for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

impl pallet_xdns::Config for Runtime {
    type Balances = Balances;
    type Escrowed = Self;
    type Event = Event;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<Runtime>;
}

impl pallet_contracts_registry::Config for Runtime {
    type Balances = Balances;
    type Escrowed = Self;
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
    pub const CircuitPalletId: PalletId = PalletId(*b"pal/circ");
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
}

impl pallet_circuit::Config for Runtime {
    type Balances = Balances;
    type Call = Call;
    type DeletionQueueLimit = ConstU32<100>;
    type Escrowed = Self;
    type Event = Event;
    type MultiCurrency = ORMLTokens;
    type PalletId = CircuitPalletId;
    type Portal = Portal;
    type SelfGatewayId = SelfGatewayId;
    type SignalQueueDepth = ConstU32<64>;
    type WeightInfo = ();
    type Xdns = XDNS;
    type XtxTimeoutCheckInterval = ConstU32<50>;
    type XtxTimeoutDefault = ConstU32<400>;
}

parameter_types! {
    pub const MaxRequests: u32 = 2;
    pub const HeadersToKeep: u32 = 100;
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
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
}
