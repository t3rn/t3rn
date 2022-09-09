//! Runtime utilities
use crate::{self as pallet_circuit, GatewayABIConfig};

use codec::Encode;

use frame_support::pallet_prelude::GenesisBuild;

use frame_support::{pallet_prelude::Weight, traits::KeyOwnerProofSystem};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    impl_opaque_keys,
    traits::{BlakeTwo256, Convert, Keccak256, OpaqueKeys},
    Perbill,
};
use sp_std::convert::{TryFrom, TryInto};

pub use circuit_runtime_types::*;

pub type AccountId = sp_runtime::AccountId32;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
pub type Block = frame_system::mocking::MockBlock<Runtime>;

mod accounts_config;
mod circuit_config;
mod consensus_aura_config;
mod contracts_config;
mod orml_config;
mod system_no_version_config;
mod xbi_config;

use circuit_runtime_pallets::*;

frame_support::construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip,
        Timestamp: pallet_timestamp,

        Aura: pallet_aura,
        Grandpa: pallet_grandpa,

        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,

        Sudo: pallet_sudo,
        Utility: pallet_utility,


        ORMLTokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},

        // Circuit
        // t3rn pallets
        XDNS: pallet_xdns::{Pallet, Call, Config<T>, Storage, Event<T>} = 100,
        MultiFinalityVerifierPolkadotLike: pallet_mfv::<Instance1>::{
            Pallet, Call, Storage, Config<T, I>, Event<T, I>
        } = 101,
        MultiFinalityVerifierSubstrateLike: pallet_mfv::<Instance2>::{
            Pallet, Call, Storage, Config<T, I>, Event<T, I>
        } = 102,
        MultiFinalityVerifierEthereumLike: pallet_mfv::<Instance3>::{
            Pallet, Call, Storage, Config<T, I>, Event<T, I>
        } = 103,
        MultiFinalityVerifierGenericLike: pallet_mfv::<Instance4>::{
            Pallet, Call, Storage, Config<T, I>, Event<T, I>
        } = 104,
        MultiFinalityVerifierDefault: pallet_mfv::{
            Pallet, Call, Storage, Config<T, I>, Event<T, I>
        } = 105,
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Config<T>, Storage, Event<T>} = 106,
        CircuitPortal: pallet_circuit_portal::{Pallet, Call, Storage, Event<T>} = 107,
        Circuit: pallet_circuit::{Pallet, Call, Storage, Event<T>} = 108,
        Treasury: pallet_treasury = 109,
        Clock: pallet_clock::{Pallet, Storage, Event<T>} = 110,

        XBIPortal: pallet_xbi_portal::{Pallet, Call, Storage, Event<T>} = 111,
        XBIPortalEnter: pallet_xbi_portal_enter::{Pallet, Call, Event<T>} = 112,

        // 3VM
        ThreeVm: pallet_3vm = 119,
        Contracts: pallet_3vm_contracts = 120,
        Evm: pallet_3vm_evm = 121,
        AccountManager: pallet_account_manager = 125,
    }
);

use t3rn_primitives::{
    abi::{CryptoAlgo, HasherAlgo},
    side_effect::interface::SideEffectInterface,
    xdns::{Parachain, XdnsRecord},
    GatewaySysProps, GatewayType, GatewayVendor,
};

#[derive(Default)]
pub struct ExtBuilder {
    known_xdns_records: Vec<XdnsRecord<AccountId>>,
    standard_side_effects: Vec<SideEffectInterface>,
}

impl ExtBuilder {
    pub(crate) fn with_default_xdns_records(mut self) -> ExtBuilder {
        let circuit_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            [3u8, 3u8, 3u8, 3u8],
            Some(Parachain {
                relay_chain_id: *b"pdot",
                id: 3333,
            }),
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::OnCircuit(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            },
            vec![],
            t3rn_protocol::side_effects::standards::standard_side_effects_ids(),
        );

        let evm_like_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            [1u8, 1u8, 1u8, 1u8],
            Some(Parachain {
                relay_chain_id: *b"evmb",
                id: 1111,
            }),
            GatewayABIConfig {
                block_number_type_size: 32,
                hash_size: 32,
                hasher: HasherAlgo::Keccak256,
                crypto: CryptoAlgo::Ed25519,
                address_length: 20,
                value_type_size: 32,
                decimals: 12,
                structs: vec![],
            },
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableInternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            },
            vec![],
            t3rn_protocol::side_effects::standards::standard_side_effects_ids(),
        );
        let zero_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            [0u8, 0u8, 0u8, 0u8],
            None,
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("ZERO"),
                token_decimals: 0,
            },
            vec![],
            t3rn_protocol::side_effects::standards::standard_side_effects_ids(),
        );
        let gateway_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"gate",
            None,
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            },
            vec![],
            t3rn_protocol::side_effects::standards::standard_side_effects_ids(),
        );
        let polkadot_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"pdot",
            None,
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 0,
                token_symbol: Encode::encode("DOT"),
                token_decimals: 10,
            },
            vec![],
            t3rn_protocol::side_effects::standards::standard_side_effects_ids(),
        );
        let kusama_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"ksma",
            None,
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 2,
                token_symbol: Encode::encode("KSM"),
                token_decimals: 12,
            },
            vec![],
            t3rn_protocol::side_effects::standards::standard_side_effects_ids(),
        );
        self.known_xdns_records = vec![
            zero_xdns_record,
            circuit_xdns_record,
            evm_like_xdns_record,
            gateway_xdns_record,
            polkadot_xdns_record,
            kusama_xdns_record,
        ];
        self
    }

    pub(crate) fn with_standard_side_effects(mut self) -> ExtBuilder {
        // map side_effects to id, keeping lib.rs clean
        self.standard_side_effects =
            t3rn_protocol::side_effects::standards::standard_side_effects();

        self
    }

    pub(crate) fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .expect("Frame system builds valid default genesis config");

        pallet_balances::GenesisConfig::<Runtime> { balances: vec![] }
            .assimilate_storage(&mut t)
            .expect("Pallet balances storage can be assimilated");

        pallet_xdns::GenesisConfig::<Runtime> {
            known_xdns_records: self.known_xdns_records,
            standard_side_effects: self.standard_side_effects,
        }
        .assimilate_storage(&mut t)
        .expect("Pallet xdns can be assimilated");

        // pallet_executors::GenesisConfig::<Runtime>::default()
        //     .assimilate_storage(&mut t)
        //     .expect("mock pallet-staking genesis storage assimilation");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub const ALICE: AccountId = AccountId::new([1u8; 32]);
pub const BOB_RELAYER: AccountId = AccountId::new([2u8; 32]);
pub const CHARLIE: AccountId = AccountId::new([3u8; 32]);
pub const DJANGO: AccountId = AccountId::new([4u8; 32]);
