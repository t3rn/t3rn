use crate::*;

use frame_support::{parameter_types, traits::ConstU32, PalletId};
use pallet_grandpa_finality_verifier::bridges::runtime as bp_runtime;
use sp_core::H256;
use sp_runtime::traits::Convert;
use t3rn_primitives::common::DEFAULT_ROUND_TERM;

use crate::xbi_config::XBIPortalRuntimeEntry;

impl t3rn_primitives::EscrowTrait<Runtime> for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

impl pallet_clock::Config for Runtime {
    type AccountManager = AccountManager;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type RoundDuration = ConstU32<500u32>;
    type Treasury = Treasury;
}

impl pallet_xdns::Config for Runtime {
    type Balances = Balances;
    type Escrowed = Self;
    type Event = Event;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<Runtime>;
}

impl pallet_contracts_registry::Config for Runtime {
    type Balances = Balances;
    type Escrowed = AccountManager;
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
    type DeletionQueueLimit = ConstU32<100u32>;
    type Escrowed = Self;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Portal = Portal;
    type SFXBiddingPeriod = ConstU32<3u32>;
    type SelfAccountId = CircuitAccountId;
    type SelfGatewayId = SelfGatewayId;
    type SelfParaId = ConstU32<3333u32>;
    type SignalQueueDepth = ConstU32<5u32>;
    type WeightInfo = ();
    type XBIPortal = XBIPortalRuntimeEntry;
    type XBIPromise = XBIPortal;
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

// MinRoundTerm plays a crucial role:
//  + must at least be the size of the active collator set
//  + is applied as default round term during genesis
//  + codetermines staking delays as they are measured in rounds
parameter_types! {
    pub const TreasuryAccount: AccountId = AccountId::new([0u8; 32]); // TODO
    pub const ReserveAccount: AccountId = AccountId::new([1u8; 32]); // TODO
    pub const AuctionFund: AccountId = AccountId::new([2u8; 32]); // TODO
    pub const ContractFund: AccountId = AccountId::new([3u8; 32]); // TODO
    pub const MinRoundTerm: u32 = 20; // TODO
    pub const DefaultRoundTerm: u32 = DEFAULT_ROUND_TERM; // TODO
    pub const GenesisIssuance: u32 = 20_000_000; // TODO
    pub const IdealPerpetualInflation: Perbill =Perbill::from_percent(1);
    pub const InflationRegressionMonths: u32 = 72;
}

impl pallet_treasury::Config for Runtime {
    type AuctionFund = AuctionFund;
    type ContractFund = ContractFund;
    type Currency = Balances;
    type DefaultRoundTerm = DefaultRoundTerm;
    type Event = Event;
    type GenesisIssuance = GenesisIssuance;
    type IdealPerpetualInflation = IdealPerpetualInflation;
    type InflationRegressionMonths = InflationRegressionMonths;
    type MinRoundTerm = MinRoundTerm;
    type ReserveAccount = ReserveAccount;
    type TreasuryAccount = TreasuryAccount;
    type WeightInfo = pallet_treasury::weights::TreasuryWeight<Runtime>;
}
