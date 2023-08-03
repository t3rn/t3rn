#![recursion_limit = "256"]
//! Runtime utilities

use circuit_runtime_pallets::pallet_circuit::{self as pallet_circuit};

use frame_support::{
    pallet_prelude::{GenesisBuild, Weight},
    traits::KeyOwnerProofSystem,
    weights::{constants::ExtrinsicBaseWeight, WeightToFeeCoefficients, WeightToFeePolynomial},
};
use hex_literal::hex;
use pallet_sudo::GenesisConfig as SudoGenesisConfig;
use sp_core::{crypto::KeyTypeId, H256};
use sp_runtime::impl_opaque_keys;
use sp_std::convert::{TryFrom, TryInto};
pub mod signed_extrinsics_config;
use circuit_runtime_pallets::pallet_attesters::TargetId;
pub use circuit_runtime_pallets::*;
pub use circuit_runtime_types::*;
pub type AccountId = sp_runtime::AccountId32;
pub use crate::signed_extrinsics_config::*;
mod accounts_config;
mod circuit_config;
mod consensus_aura_config;
mod contracts_config;
mod hooks;
mod system_no_version_config;
pub mod test_utils;
mod treasuries_config;
mod xbi_config;
pub type RococoLightClient = ();
pub type PolkadotLightClient = pallet_grandpa_finality_verifier::Instance1;
pub type KusamaLightClient = pallet_grandpa_finality_verifier::Instance2;
pub use crate::circuit_config::GlobalOnInitQueues;
pub use pallet_3vm_evm::Config as ConfigEvm;
use smallvec::smallvec;

frame_support::construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Sudo: pallet_sudo,
        Utility: pallet_utility,
        ParachainSystem: cumulus_pallet_parachain_system,
        ParachainInfo: parachain_info,

        RandomnessCollectiveFlip: pallet_randomness_collective_flip,
        Timestamp: pallet_timestamp,

        Aura: pallet_aura,
        Grandpa: pallet_grandpa,

        // Monetary
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11,
        Assets: pallet_assets = 12,
        AssetTxPayment: pallet_asset_tx_payment = 14,
        Authorship: pallet_authorship = 15,

        // Treasuries
        Treasury: pallet_treasury = 13, // Keep old treasury index for backwards compatibility
        EscrowTreasury: pallet_treasury::<Instance1> = 16,
        FeeTreasury: pallet_treasury::<Instance2> = 17,
        ParachainTreasury: pallet_treasury::<Instance3> = 18,
        SlashTreasury: pallet_treasury::<Instance4> = 19,

        // Circuit
        // t3rn pallets
        XDNS: pallet_xdns = 100,
        Attesters: pallet_attesters = 101,
        Rewards: pallet_rewards = 102,

        ContractsRegistry: pallet_contracts_registry = 106,
        Circuit: pallet_circuit = 108,
        Clock: pallet_clock = 110,
        Vacuum: pallet_vacuum = 111,

        // XCM helpers.
        XcmpQueue: cumulus_pallet_xcmp_queue = 30,
        PolkadotXcm: pallet_xcm = 31,
        CumulusXcm: cumulus_pallet_xcm = 32,
        DmpQueue: cumulus_pallet_dmp_queue = 33,
        // XbiPortal: pallet_xbi_portal = 34,
        AssetRegistry: pallet_asset_registry = 35,

        // 3VM
        ThreeVm: pallet_3vm = 119,
        Contracts: pallet_3vm_contracts = 120,
        Evm: pallet_3vm_evm = 121,
        AccountManager: pallet_account_manager = 125,

        // Portal
        Portal: pallet_portal = 128,
        RococoBridge: pallet_grandpa_finality_verifier = 129,
        PolkadotBridge: pallet_grandpa_finality_verifier::<Instance1> = 130,
        KusamaBridge: pallet_grandpa_finality_verifier::<Instance2> = 131,
        EthereumBridge: pallet_eth2_finality_verifier = 132,
        SepoliaBridge: pallet_sepolia_finality_verifier = 133,
    }
);
use frame_support::weights::WeightToFeeCoefficient;
use sp_runtime::Perbill;
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
    type Balance = Balance;

    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        // in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
        // in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
        let p = MILLIUNIT / 10;
        let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
        smallvec![WeightToFeeCoefficient {
            degree: 1,
            negative: false,
            coeff_frac: Perbill::from_rational(p % q, q),
            coeff_integer: p / q,
        }]
    }
}

use t3rn_abi::SFXAbi;
use t3rn_primitives::{
    contracts_registry::RegistryContract,
    xdns::{GatewayRecord, XdnsRecord},
    ExecutionVendor, GatewayVendor,
};
use t3rn_types::sfx::Sfx4bId;

#[derive(Default)]
pub struct ExtBuilder {
    known_xdns_records: Vec<XdnsRecord<AccountId>>,
    known_gateway_records: Vec<GatewayRecord<AccountId>>,
    standard_sfx_abi: Vec<(Sfx4bId, SFXAbi)>,
    known_contracts: Vec<RegistryContract<H256, AccountId, Balance, BlockNumber>>,
    attestation_targets: Vec<TargetId>,
}

impl ExtBuilder {
    pub fn with_default_attestation_targets(mut self) -> ExtBuilder {
        self.attestation_targets = vec![[0, 0, 0, 0], [1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3]];
        self
    }

    pub fn with_default_xdns_records(mut self) -> ExtBuilder {
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
                ],
            },
            GatewayRecord {
                gateway_id: *b"ksma",
                verification_vendor: GatewayVendor::Kusama,
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
                ],
            },
            GatewayRecord {
                gateway_id: *b"eth2",
                verification_vendor: GatewayVendor::Ethereum,
                execution_vendor: ExecutionVendor::EVM,
                codec: t3rn_abi::Codec::Rlp,
                registrant: None,
                escrow_account: None,
                allowed_side_effects: vec![(*b"tran", Some(2))],
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

    fn make_all_light_clients_move_2_times_by(move_by: u32) {
        use circuit_runtime_pallets::pallet_eth2_finality_verifier::LightClientAsyncAPI;
        use t3rn_primitives::portal::Portal as PortalT;
        let starting_height = System::block_number();
        for vendor in GatewayVendor::iterator() {
            let mut latest_heartbeat = Portal::get_latest_heartbeat_by_vendor(vendor.clone());
            latest_heartbeat.last_finalized_height += move_by;
            latest_heartbeat.last_rational_height += move_by;
            latest_heartbeat.last_fast_height += move_by;

            System::set_block_number(starting_height + move_by);

            XDNS::on_new_epoch(
                vendor.clone(),
                latest_heartbeat.last_finalized_height + 1,
                latest_heartbeat.clone(),
            );

            latest_heartbeat.last_finalized_height += 2 * move_by;
            latest_heartbeat.last_rational_height += 2 * move_by;
            latest_heartbeat.last_fast_height += 2 * move_by;

            System::set_block_number(starting_height + move_by * 2);

            XDNS::on_new_epoch(
                vendor.clone(),
                latest_heartbeat.last_finalized_height + 2,
                latest_heartbeat,
            );
        }
    }

    fn activate_all_light_clients() {
        use t3rn_primitives::portal::Portal as PortalT;
        for &gateway in XDNS::all_gateway_ids().iter() {
            Portal::turn_on(RuntimeOrigin::root(), gateway).unwrap();
        }
        XDNS::process_all_verifier_overviews(System::block_number());
        XDNS::process_overview(System::block_number());

        Self::make_all_light_clients_move_2_times_by(8);
        XDNS::process_overview(System::block_number());
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .expect("Frame system builds valid default genesis config");

        let sudo_genesis_config = SudoGenesisConfig::<Runtime> {
            key: Some(AccountId::new(hex!(
                "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
            ))), // the actual key with //Alice seed
        };
        sudo_genesis_config.assimilate_storage(&mut t).unwrap();

        pallet_balances::GenesisConfig::<Runtime> { balances: vec![] }
            .assimilate_storage(&mut t)
            .expect("Pallet balances storage can be assimilated");

        pallet_attesters::GenesisConfig::<Runtime> {
            phantom: Default::default(),
            attestation_targets: self.attestation_targets,
        }
        .assimilate_storage(&mut t)
        .expect("Pallet attesters can be assimilated");
        pallet_xdns::GenesisConfig::<Runtime> {
            known_gateway_records: self.known_gateway_records,
            standard_sfx_abi: self.standard_sfx_abi,
        }
        .assimilate_storage(&mut t)
        .expect("Pallet xdns can be assimilated");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext.execute_with(Self::activate_all_light_clients);
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
