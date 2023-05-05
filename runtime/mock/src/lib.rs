//! Runtime utilities
use circuit_runtime_pallets::pallet_circuit::{self as pallet_circuit, GatewayABIConfig};

use codec::Encode;

use frame_support::pallet_prelude::GenesisBuild;

use frame_support::{pallet_prelude::Weight, traits::KeyOwnerProofSystem};
use sp_core::{crypto::KeyTypeId, H256};
use sp_runtime::impl_opaque_keys;
use sp_std::convert::{TryFrom, TryInto};
pub mod signed_extrinsics_config;
pub use circuit_runtime_pallets::*;
pub use circuit_runtime_types::*;

pub type AccountId = sp_runtime::AccountId32;
pub use crate::signed_extrinsics_config::*;
mod accounts_config;
mod circuit_config;
mod consensus_aura_config;
mod contracts_config;
mod system_no_version_config;
pub mod test_utils;
mod xbi_config;

pub type RococoLightClient = ();
pub type PolkadotLightClient = pallet_grandpa_finality_verifier::Instance1;
pub type KusamaLightClient = pallet_grandpa_finality_verifier::Instance2;

frame_support::construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Sudo: pallet_sudo,
        Utility: pallet_utility,
        ParachainSystem: cumulus_pallet_parachain_system::{
            Pallet, Call, Config, Storage, Inherent, Event<T>, ValidateUnsigned,
        },

        RandomnessCollectiveFlip: pallet_randomness_collective_flip,
        Timestamp: pallet_timestamp,

        Aura: pallet_aura,
        Grandpa: pallet_grandpa,

        // Monetary
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11,
        Assets: pallet_assets = 12,
        AssetTxPayment: pallet_asset_tx_payment = 13,
        Authorship: pallet_authorship = 14,

        // Circuit
        // t3rn pallets
        XDNS: pallet_xdns::{Pallet, Call, Config<T>, Storage, Event<T>} = 100,
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Config<T>, Storage, Event<T>} = 106,
        Circuit: pallet_circuit::{Pallet, Call, Storage, Event<T>} = 108,
        Treasury: pallet_treasury = 109,
        Clock: pallet_clock::{Pallet, Config<T>, Storage, Event<T>} = 110,
        Executors: pallet_executors = 111,

        // XCM helpers.
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 30,
        PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin, Config} = 31,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 32,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 33,
        XbiPortal: pallet_xbi_portal = 34,
        AssetRegistry: pallet_asset_registry = 35,

        // 3VM
        ThreeVm: pallet_3vm = 119,
        Contracts: pallet_3vm_contracts = 120,
        Evm: pallet_3vm_evm = 121,
        AccountManager: pallet_account_manager = 125,

        // Portal
        Portal: pallet_portal::{Pallet, Call, Storage, Event<T>} = 128,
        RococoBridge: pallet_grandpa_finality_verifier = 129,
        PolkadotBridge: pallet_grandpa_finality_verifier::<Instance1> = 130,
        KusamaBridge: pallet_grandpa_finality_verifier::<Instance2> = 131,
    }
);

use t3rn_types::gateway::{CryptoAlgo, HasherAlgo};

use t3rn_abi::SFXAbi;
use t3rn_primitives::{
    contracts_registry::RegistryContract,
    xdns::{GatewayRecord, Parachain, XdnsRecord},
    ExecutionVendor, GatewayType, GatewayVendor, SubstrateToken, TokenInfo,
};
use t3rn_types::sfx::Sfx4bId;

#[derive(Default)]
pub struct ExtBuilder {
    known_xdns_records: Vec<XdnsRecord<AccountId>>,
    known_gateway_records: Vec<GatewayRecord<AccountId>>,
    standard_sfx_abi: Vec<(Sfx4bId, SFXAbi)>,
    known_contracts: Vec<RegistryContract<H256, AccountId, Balance, BlockNumber>>,
}

impl ExtBuilder {
    pub fn with_default_xdns_records(mut self) -> ExtBuilder {
        let circuit_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            [3u8, 3u8, 3u8, 3u8],
            Some(Parachain {
                relay_chain_id: *b"circ",
                id: 3333,
            }),
            Default::default(),
            GatewayVendor::Rococo,
            GatewayType::OnCircuit(0),
            Default::default(),
            TokenInfo::Substrate(SubstrateToken {
                id: 1333,
                symbol: Encode::encode("TRN"),
                decimals: 12,
            }),
            vec![],
            t3rn_abi::standard::standard_sfx_abi_ids(),
        );

        let polka_like_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            [5u8, 5u8, 5u8, 5u8],
            Some(Parachain {
                relay_chain_id: *b"polk",
                id: 3333,
            }),
            Default::default(),
            GatewayVendor::Rococo,
            GatewayType::OnCircuit(0),
            Default::default(),
            TokenInfo::Substrate(SubstrateToken {
                id: 1333,
                symbol: Encode::encode("TRN"),
                decimals: 12,
            }),
            vec![],
            t3rn_abi::standard::standard_sfx_abi_ids(),
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
            },
            GatewayVendor::Rococo,
            GatewayType::ProgrammableInternal(0),
            Default::default(),
            TokenInfo::Substrate(SubstrateToken {
                id: 1333,
                symbol: Encode::encode("TRN"),
                decimals: 12,
            }),
            vec![],
            t3rn_abi::standard::standard_sfx_abi_ids(),
        );
        let zero_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            [0u8, 0u8, 0u8, 0u8],
            None,
            Default::default(),
            GatewayVendor::Rococo,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            TokenInfo::Substrate(SubstrateToken {
                id: 1333,
                symbol: Encode::encode("ZERO"),
                decimals: 0,
            }),
            vec![],
            t3rn_abi::standard::standard_sfx_abi_ids(),
        );
        let gateway_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"gate",
            None,
            Default::default(),
            GatewayVendor::Rococo,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            TokenInfo::Substrate(SubstrateToken {
                id: 1333,
                symbol: Encode::encode("TRN"),
                decimals: 12,
            }),
            vec![],
            t3rn_abi::standard::standard_sfx_abi_ids(),
        );
        let polkadot_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"pdot",
            None,
            Default::default(),
            GatewayVendor::Rococo,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            TokenInfo::Substrate(SubstrateToken {
                id: 0,
                symbol: Encode::encode("DOT"),
                decimals: 10,
            }),
            vec![],
            t3rn_abi::standard::standard_sfx_abi_ids(),
        );
        let kusama_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"ksma",
            None,
            Default::default(),
            GatewayVendor::Rococo,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            TokenInfo::Substrate(SubstrateToken {
                id: 2,
                symbol: Encode::encode("KSM"),
                decimals: 12,
            }),
            vec![],
            t3rn_abi::standard::standard_sfx_abi_ids(),
        );
        self.known_xdns_records = vec![
            zero_xdns_record,
            circuit_xdns_record,
            polka_like_xdns_record,
            evm_like_xdns_record,
            gateway_xdns_record,
            polkadot_xdns_record,
            kusama_xdns_record,
        ];

        self.known_gateway_records = vec![
            GatewayRecord {
                gateway_id: [3, 3, 3, 3],
                verification_vendor: GatewayVendor::Polkadot,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Scale,
                registrant: None,
                escrow_account: None,
                allowed_side_effects: vec![
                    (*b"tran", Some(2)),
                    (*b"tass", Some(4)),
                    (*b"swap", Some(3)),
                    (*b"aliq", Some(3)),
                    (*b"cevm", Some(10)),
                    (*b"wasm", Some(10)),
                    (*b"call", Some(10)),
                ],
            },
            GatewayRecord {
                gateway_id: [1, 1, 1, 1],
                verification_vendor: GatewayVendor::Polkadot,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Scale,
                registrant: None,
                escrow_account: None,
                allowed_side_effects: vec![
                    (*b"tran", Some(2)),
                    (*b"tass", Some(4)),
                    (*b"swap", Some(3)),
                    (*b"aliq", Some(3)),
                    (*b"cevm", Some(10)),
                    (*b"wasm", Some(10)),
                    (*b"call", Some(10)),
                ],
            },
            GatewayRecord {
                gateway_id: [5, 5, 5, 5],
                verification_vendor: GatewayVendor::Polkadot,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Scale,
                registrant: None,
                escrow_account: None,
                allowed_side_effects: vec![
                    (*b"tran", Some(2)),
                    (*b"tass", Some(4)),
                    (*b"swap", Some(3)),
                    (*b"aliq", Some(3)),
                    (*b"cevm", Some(10)),
                    (*b"wasm", Some(10)),
                    (*b"call", Some(10)),
                ],
            },
            GatewayRecord {
                gateway_id: *b"ksma",
                verification_vendor: GatewayVendor::Polkadot,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Scale,
                registrant: None,
                escrow_account: None,
                allowed_side_effects: vec![(*b"tran", Some(2)), (*b"tass", Some(4))],
            },
            GatewayRecord {
                gateway_id: *b"pdot",
                verification_vendor: GatewayVendor::Polkadot,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Scale,
                registrant: None,
                escrow_account: None,
                allowed_side_effects: vec![(*b"tran", Some(2)), (*b"tass", Some(4))],
            },
            GatewayRecord {
                gateway_id: *b"gate",
                verification_vendor: GatewayVendor::Rococo,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Scale,
                registrant: None,
                escrow_account: None,
                allowed_side_effects: vec![(*b"tran", Some(2))],
            },
            GatewayRecord {
                gateway_id: [0, 0, 0, 0],
                verification_vendor: GatewayVendor::Rococo,
                execution_vendor: ExecutionVendor::Substrate,
                codec: t3rn_abi::Codec::Scale,
                registrant: None,
                escrow_account: None,
                allowed_side_effects: vec![
                    (*b"tran", Some(2)),
                    (*b"tass", Some(4)),
                    (*b"swap", Some(3)),
                    (*b"aliq", Some(3)),
                    (*b"cevm", Some(10)),
                    (*b"wasm", Some(10)),
                    (*b"call", Some(10)),
                ],
            },
        ];
        self
    }

    pub fn with_standard_sfx_abi(mut self) -> ExtBuilder {
        // map side_effects to id, keeping lib.rs clean
        self.standard_sfx_abi = t3rn_abi::standard::standard_sfx_abi();

        self
    }

    pub fn with_contracts(
        mut self,
        contracts: Vec<RegistryContract<H256, AccountId, Balance, BlockNumber>>,
    ) -> ExtBuilder {
        self.known_contracts = contracts;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .expect("Frame system builds valid default genesis config");

        pallet_balances::GenesisConfig::<Runtime> { balances: vec![] }
            .assimilate_storage(&mut t)
            .expect("Pallet balances storage can be assimilated");

        pallet_xdns::GenesisConfig::<Runtime> {
            known_xdns_records: self.known_xdns_records,
            known_gateway_records: self.known_gateway_records,
            standard_sfx_abi: self.standard_sfx_abi,
        }
        .assimilate_storage(&mut t)
        .expect("Pallet xdns can be assimilated");
        // TODO: reenable
        // pallet_executors::GenesisConfig::<Runtime>::default()
        //     .assimilate_storage(&mut t)
        //     .expect("mock pallet-executors genesis storage assimilation");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub const ALICE: AccountId = AccountId::new([1u8; 32]);
pub const BOB: AccountId = AccountId::new([2u8; 32]);
pub const BOB_RELAYER: AccountId = AccountId::new([2u8; 32]);
pub const CHARLIE: AccountId = AccountId::new([3u8; 32]);
pub const DJANGO: AccountId = AccountId::new([4u8; 32]);
pub const CLI_DEFAULT: AccountId = AccountId::new([
    108, 81, 222, 3, 128, 118, 146, 25, 212, 131, 171, 210, 104, 110, 11, 63, 79, 235, 65, 99, 161,
    143, 230, 174, 109, 98, 47, 128, 20, 242, 27, 114,
]);
pub const EXECUTOR_DEFAULT: AccountId = AccountId::new([
    1, 119, 209, 36, 229, 1, 136, 124, 36, 112, 226, 96, 200, 240, 218, 96, 219, 158, 211, 219,
    168, 8, 166, 130, 240, 154, 251, 57, 239, 240, 197, 97,
]);
pub const EXECUTOR_SECOND: AccountId = AccountId::new([
    2, 119, 209, 36, 229, 1, 136, 124, 36, 112, 226, 96, 200, 240, 218, 96, 219, 158, 211, 219,
    168, 8, 166, 130, 240, 154, 251, 57, 239, 240, 197, 99,
]);
