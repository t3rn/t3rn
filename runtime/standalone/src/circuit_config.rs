use super::*;
use frame_support::parameter_types;
use frame_support::PalletId;
use sp_core::H256;
use sp_runtime::traits::*;
use t3rn_primitives::bridges::runtime as bp_runtime;

// impl pallet_randomness_collective_flip::Config for Runtime {}

// t3rn pallets
impl t3rn_primitives::EscrowTrait for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

impl pallet_xdns::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<Runtime>;
}

impl pallet_contracts_registry::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_contracts_registry::weights::SubstrateWeight<Runtime>;
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

pub struct CircuitToGateway;
impl Convert<Balance, u128> for CircuitToGateway {
    fn convert(val: Balance) -> u128 {
        val
    }
}

impl pallet_circuit_portal::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type AccountId32Converter = AccountId32Converter;
    type ToStandardizedGatewayBalance = CircuitToGateway;
    type WeightInfo = pallet_circuit_portal::weights::SubstrateWeight<Runtime>;
    type PalletId = PortalPalletId;
    // type EthVerifier = ethereum_light_client::Pallet<Runtime>;
    type EthVerifier = t3rn_protocol::side_effects::confirm::ethereum::EthereumMockVerifier;
}

parameter_types! {
    pub const CircuitPalletId: PalletId = PalletId(*b"pal/circ");
}

impl pallet_circuit::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type WeightInfo = ();
    type PalletId = CircuitPalletId;
}

parameter_types! {
    pub const MaxRequests: u32 = 2;
    pub const HeadersToKeep: u32 = 5;
}

type DefaultPolkadotBridgeInstance = ();
type Blake2ValU32BridgeInstance = pallet_mfv::Instance1;
type Blake2ValU64BridgeInstance = pallet_mfv::Instance2;
type Keccak256ValU64BridgeInstance = pallet_mfv::Instance3;
type Keccak256ValU32BridgeInstance = pallet_mfv::Instance4;

#[derive(Debug)]
pub struct Blake2ValU64Chain;
impl bp_runtime::Chain for Blake2ValU64Chain {
    type BlockNumber = <Runtime as frame_system::Config>::BlockNumber;
    type Hash = <Runtime as frame_system::Config>::Hash;
    type Hasher = <Runtime as frame_system::Config>::Hashing;
    type Header = <Runtime as frame_system::Config>::Header;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl bp_runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

#[derive(Debug)]
pub struct Keccak256ValU64Chain;
impl bp_runtime::Chain for Keccak256ValU64Chain {
    type BlockNumber = u64;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u64, Keccak256>;
}

#[derive(Debug)]
pub struct Keccak256ValU32Chain;
impl bp_runtime::Chain for Keccak256ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u32, Keccak256>;
}

impl pallet_mfv::Config<Blake2ValU64BridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU64Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_mfv::Config<Blake2ValU32BridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_mfv::Config<Keccak256ValU64BridgeInstance> for Runtime {
    type BridgedChain = Keccak256ValU64Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_mfv::Config<Keccak256ValU32BridgeInstance> for Runtime {
    type BridgedChain = Keccak256ValU32Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_mfv::Config<DefaultPolkadotBridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}
