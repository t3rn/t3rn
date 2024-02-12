use crate::*;

use crate::{
    accounts_config::EscrowAccount, pallet_3vm_contracts::NoopMigration,
    pallet_3vm_ethereum::Transaction, AccountId, AccountManager, Aura, Balance, Balances, Circuit,
    ContractsRegistry, Portal, RandomnessCollectiveFlip, RuntimeCall, RuntimeEvent, ThreeVm,
    Timestamp, Weight, AVERAGE_ON_INITIALIZE_RATIO,
};
use circuit_runtime_pallets::{
    evm_precompile_util, pallet_3vm, pallet_3vm_contracts, pallet_3vm_evm,
    pallet_3vm_evm::HashedAddressMapping, pallet_3vm_evm_primitives,
};
use frame_support::{pallet_prelude::ConstU32, parameter_types, traits::FindAuthor, PalletId};

use crate::pallet_3vm_ethereum::PostLogContent;
use circuit_runtime_types::{AssetId, EvmAddress};
use ethereum::{TransactionAction, TransactionSignature};
pub use pallet_3vm_account_mapping::EvmAddressMapping;
use pallet_3vm_evm::{EnsureAddressTruncated, SubstrateBlockHashMapping};
use pallet_3vm_evm_primitives::FeeCalculator;
#[cfg(feature = "std")]
pub use pallet_3vm_evm_primitives::GenesisAccount as EvmGenesisAccount;
use rlp::RlpStream;
use sp_core::{H160, U256};
use sp_runtime::{
    traits::{AccountIdConversion, DispatchInfoOf, Dispatchable, Keccak256},
    transaction_validity::{TransactionValidity, TransactionValidityError},
    ConsensusEngineId, RuntimeAppPublic,
};
use t3rn_primitives::threevm::{Erc20Mapping, H160_POSITION_ASSET_ID_TYPE};

// Unit = the base number of indivisible units for balances
const UNIT: Balance = 1_000_000_000_000;
const MILLIUNIT: Balance = 1_000_000_000;
const _EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

const fn deposit(items: u32, bytes: u32) -> Balance {
    (items as Balance * UNIT + (bytes as Balance) * (5 * MILLIUNIT / 100)) / 10
}

parameter_types! {
    pub const CreateSideEffectsPrecompileDest: AccountId = AccountId::new([51u8; 32]); // 0x333...3
    pub const CircuitTargetId: t3rn_primitives::ChainId = [3, 3, 3, 3];

    pub const MaxValueSize: u32 = 16_384;
    // The lazy deletion runs inside on_initialize.
    pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
        RuntimeBlockWeights::get().max_block;
    pub Schedule: pallet_3vm_contracts::Schedule<Runtime> = Default::default();
    pub const MaxCodeSize: u32 = 2 * 1024;
    pub const DepositPerItem: Balance = deposit(1, 0);
    pub const DepositPerByte: Balance = deposit(0, 1);
}

impl pallet_3vm::Config for Runtime {
    type AccountManager = AccountManager;
    type AssetId = AssetId;
    type CircuitTargetId = CircuitTargetId;
    type ContractsRegistry = ContractsRegistry;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type OnLocalTrigger = Circuit;
    type Portal = Portal;
    type RuntimeEvent = RuntimeEvent;
    type SignalBounceThreshold = ConstU32<2>;
}

parameter_types! {
    pub static UnstableInterface: bool = true;
    pub static DefaultDepositLimit: Balance = 10_000_000;
}

impl pallet_3vm_contracts::Config for Runtime {
    type AddressGenerator = pallet_3vm_contracts::DefaultAddressGenerator;
    /// The safest default is to allow no calls at all.
    ///
    /// Runtimes should whitelist dispatchables that are allowed to be called from contracts
    /// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
    /// change because that would break already deployed contracts. The `Call` structure itself
    /// is not allowed to change the indices of existing pallets, too.
    type CallFilter = frame_support::traits::Nothing;
    type CallStack = [pallet_3vm_contracts::Frame<Self>; 5];
    type ChainExtension = ();
    type Currency = Balances;
    type DefaultDepositLimit = DefaultDepositLimit;
    type DepositPerByte = DepositPerByte;
    type DepositPerItem = DepositPerItem;
    type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
    type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
    type MaxStorageKeyLen = ConstU32<128>;
    type Migrations = (NoopMigration<1>, NoopMigration<2>);
    type Randomness = RandomnessCollectiveFlip;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Schedule = Schedule;
    type ThreeVm = ThreeVm;
    type Time = Timestamp;
    type UnsafeUnstableInterface = UnstableInterface;
    type WeightInfo = pallet_3vm_contracts::weights::SubstrateWeight<Self>;
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
}

pub struct FindAuthorTruncated<F>(sp_std::marker::PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
    fn find_author<'a, I>(digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        if let Some(author_index) = F::find_author(digests) {
            let authority_id = Aura::authorities()[author_index as usize].clone();
            return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]))
        }
        None
    }
}

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> (U256, Weight) {
        // Return some meaningful gas price and weight
        (1_000_000_000u128.into(), Weight::from_parts(7u64, 0))
    }
}

const BLOCK_GAS_LIMIT: u64 = 150_000_000;
const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;

parameter_types! {
    pub BlockGasLimit: U256 = U256::from(BLOCK_GAS_LIMIT);
    pub const GasLimitPovSizeRatio: u64 = BLOCK_GAS_LIMIT.saturating_div(MAX_POV_SIZE);
    pub const ChainId: u64 = 42;
    pub PrecompilesValue: evm_precompile_util::Precompiles<Runtime> = evm_precompile_util::Precompiles::<Runtime>::new(sp_std::vec![
         (sp_core::H160([0u8; 20]), evm_precompile_util::KnownPrecompile::ECRecover),
         (sp_core::H160([1u8; 20]), evm_precompile_util::KnownPrecompile::Sha256),
         (sp_core::H160([2u8; 20]), evm_precompile_util::KnownPrecompile::Ripemd160),
         (sp_core::H160([3u8; 20]), evm_precompile_util::KnownPrecompile::Identity),
         (sp_core::H160([4u8; 20]), evm_precompile_util::KnownPrecompile::Modexp),
         (sp_core::H160([5u8; 20]), evm_precompile_util::KnownPrecompile::Sha3FIPS256),
         (sp_core::H160([6u8; 20]), evm_precompile_util::KnownPrecompile::Sha3FIPS512),
         (sp_core::H160([7u8; 20]), evm_precompile_util::KnownPrecompile::ECRecoverPublicKey),
         (sp_core::H160([8u8; 20]), evm_precompile_util::KnownPrecompile::Portal)
    ].into_iter().collect());
    // pub MockPrecompiles: MockPrecompiles = MockPrecompileSet;
    pub WeightPerGas: Weight = Weight::from_parts(20_000, 0);
}

// TODO[https://github.com/t3rn/3vm/issues/102]: configure this appropriately
impl pallet_3vm_evm::Config for Runtime {
    type AddressMapping = HashedAddressMapping<Keccak256>;
    type BlockGasLimit = BlockGasLimit;
    type BlockHashMapping = SubstrateBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressTruncated;
    type ChainId = ChainId;
    type Currency = Balances;
    // BaseFee pallet may be better from frontier TODO
    type FeeCalculator = FixedGasPrice;
    type FindAuthor = FindAuthorTruncated<Aura>;
    type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
    type GasWeightMapping = pallet_3vm_evm::FixedGasWeightMapping<Runtime>;
    type OnChargeTransaction = ();
    type OnCreate = ();
    type PrecompilesType = evm_precompile_util::Precompiles<Runtime>;
    type PrecompilesValue = PrecompilesValue;
    type Runner = pallet_3vm_evm::runner::stack::Runner<Self>;
    type RuntimeEvent = RuntimeEvent;
    type ThreeVm = ThreeVm;
    type Timestamp = Timestamp;
    type WeightInfo = ();
    type WeightPerGas = WeightPerGas;
    type WithdrawOrigin = EnsureAddressTruncated;
}

parameter_types! {
    pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_3vm_ethereum::Config for Runtime {
    type ExtraDataLength = ConstU32<30>;
    type PostLogContent = PostBlockAndTxnHashes;
    type RuntimeEvent = RuntimeEvent;
    type StateRoot = pallet_3vm_ethereum::IntermediateStateRoot<Self>;
}

//     type AddressMapping = IdentityAddressMapping;
//     type BlockGasLimit = BlockGasLimit;
//     type BlockHashMapping = crate::SubstrateBlockHashMapping<Self>;
//     type CallOrigin = EnsureAddressRoot<Self::AccountId>;
//     type ChainId = ();
//     type Currency = Balances;
//     type FeeCalculator = FixedGasPrice;
//     type FindAuthor = FindAuthorTruncated;
//     type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
//     type GasWeightMapping = crate::FixedGasWeightMapping<Self>;
//     type OnChargeTransaction = ();
//     type OnCreate = ();
//     type PrecompilesType = MockPrecompileSet;
//     type PrecompilesValue = MockPrecompiles;
//     type Runner = crate::runner::stack::Runner<Self>;
//     type RuntimeEvent = RuntimeEvent;
//     type ThreeVm = t3rn_primitives::threevm::NoopThreeVm;
//     type Timestamp = Timestamp;
//     type WeightInfo = ();
//     type WeightPerGas = WeightPerGas;
//     type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;

parameter_types! {
    pub const T3rnPalletId: PalletId = PalletId(*b"trn/trsy");
    pub TreasuryModuleAccount: AccountId = T3rnPalletId::get().into_account_truncating();
    pub const StorageDepositFee: Balance = 1;
}

impl pallet_3vm_account_mapping::Config for Runtime {
    type AddressMapping = EvmAddressMapping<Runtime>;
    type ChainId = ChainId;
    type Currency = Balances;
    type NetworkTreasuryAccount = TreasuryModuleAccount;
    type RuntimeEvent = RuntimeEvent;
    type StorageDepositFee = StorageDepositFee;
}

// AssetId to EvmAddress mapping
impl Erc20Mapping for Runtime {
    fn encode_evm_address(v: AssetId) -> Option<EvmAddress> {
        let mut address = [0u8; 20];
        let asset_id_bytes: Vec<u8> = v.to_be_bytes().to_vec();

        for byte_index in 0..asset_id_bytes.len() {
            address[byte_index + H160_POSITION_ASSET_ID_TYPE] =
                asset_id_bytes.as_slice()[byte_index];
        }

        Some(EvmAddress::from_slice(&asset_id_bytes.as_slice()))
    }

    fn decode_evm_address(v: EvmAddress) -> Option<AssetId> {
        let address = v.as_bytes();
        let mut asset_id_bytes = [0u8; 4];
        for byte_index in H160_POSITION_ASSET_ID_TYPE..20 {
            asset_id_bytes[byte_index - H160_POSITION_ASSET_ID_TYPE] = address[byte_index];
        }
        let asset_id = u32::from_be_bytes(asset_id_bytes);
        Some(asset_id)
    }
}

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    fp_self_contained::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

impl fp_self_contained::SelfContainedCall for RuntimeCall {
    type SignedInfo = H160;

    fn is_self_contained(&self) -> bool {
        match self {
            RuntimeCall::Ethereum(call) => call.is_self_contained(),
            _ => false,
        }
    }

    fn check_self_contained(&self) -> Option<Result<Self::SignedInfo, TransactionValidityError>> {
        match self {
            RuntimeCall::Ethereum(call) => call.check_self_contained(),
            _ => None,
        }
    }

    fn validate_self_contained(
        &self,
        info: &Self::SignedInfo,
        dispatch_info: &DispatchInfoOf<RuntimeCall>,
        len: usize,
    ) -> Option<TransactionValidity> {
        match self {
            RuntimeCall::Ethereum(call) => call.validate_self_contained(info, dispatch_info, len),
            _ => None,
        }
    }

    fn pre_dispatch_self_contained(
        &self,
        info: &Self::SignedInfo,
        dispatch_info: &DispatchInfoOf<RuntimeCall>,
        len: usize,
    ) -> Option<Result<(), TransactionValidityError>> {
        match self {
            RuntimeCall::Ethereum(call) =>
                call.pre_dispatch_self_contained(info, dispatch_info, len),
            _ => None,
        }
    }

    fn apply_self_contained(
        self,
        info: Self::SignedInfo,
    ) -> Option<sp_runtime::DispatchResultWithInfo<sp_runtime::traits::PostDispatchInfoOf<Self>>>
    {
        match self {
            call @ RuntimeCall::Ethereum(crate::pallet_3vm_ethereum::Call::transact { .. }) =>
                Some(call.dispatch(RuntimeOrigin::from(
                    crate::pallet_3vm_ethereum::RawOrigin::EthereumTransaction(info),
                ))),
            _ => None,
        }
    }
}

pub fn contract_address(sender: H160, nonce: u64) -> H160 {
    let mut rlp = RlpStream::new_list(2);
    rlp.append(&sender);
    rlp.append(&nonce);

    H160::from_slice(&keccak_256(&rlp.out())[12..])
}

pub fn storage_address(sender: H160, slot: H256) -> H256 {
    H256::from(keccak_256(
        [&H256::from(sender)[..], &slot[..]].concat().as_slice(),
    ))
}

pub struct LegacyUnsignedTransaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub action: TransactionAction,
    pub value: U256,
    pub input: Vec<u8>,
}

impl LegacyUnsignedTransaction {
    fn signing_rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        s.append(&self.action);
        s.append(&self.value);
        s.append(&self.input);
        s.append(&ChainId::get());
        s.append(&0u8);
        s.append(&0u8);
    }

    fn signing_hash(&self) -> H256 {
        let mut stream = RlpStream::new();
        self.signing_rlp_append(&mut stream);
        H256::from(keccak_256(&stream.out()))
    }

    pub fn sign(&self, key: &H256) -> Transaction {
        self.sign_with_chain_id(key, ChainId::get())
    }

    pub fn sign_with_chain_id(&self, key: &H256, chain_id: u64) -> Transaction {
        let hash = self.signing_hash();
        let msg = libsecp256k1::Message::parse(hash.as_fixed_bytes());
        let s = libsecp256k1::sign(
            &msg,
            &libsecp256k1::SecretKey::parse_slice(&key[..]).unwrap(),
        );
        let sig = s.0.serialize();

        let sig = TransactionSignature::new(
            s.1.serialize() as u64 % 2 + chain_id * 2 + 35,
            H256::from_slice(&sig[0..32]),
            H256::from_slice(&sig[32..64]),
        )
        .unwrap();

        Transaction::Legacy(ethereum::LegacyTransaction {
            nonce: self.nonce,
            gas_price: self.gas_price,
            gas_limit: self.gas_limit,
            action: self.action,
            value: self.value,
            input: self.input.clone(),
            signature: sig,
        })
    }
}

pub struct EIP2930UnsignedTransaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub action: TransactionAction,
    pub value: U256,
    pub input: Vec<u8>,
}

impl EIP2930UnsignedTransaction {
    pub fn sign(&self, secret: &H256, chain_id: Option<u64>) -> Transaction {
        let secret = {
            let mut sk: [u8; 32] = [0u8; 32];
            sk.copy_from_slice(&secret[0..]);
            libsecp256k1::SecretKey::parse(&sk).unwrap()
        };
        let chain_id = chain_id.unwrap_or(ChainId::get());
        let msg = ethereum::EIP2930TransactionMessage {
            chain_id,
            nonce: self.nonce,
            gas_price: self.gas_price,
            gas_limit: self.gas_limit,
            action: self.action,
            value: self.value,
            input: self.input.clone(),
            access_list: vec![],
        };
        let signing_message = libsecp256k1::Message::parse_slice(&msg.hash()[..]).unwrap();

        let (signature, recid) = libsecp256k1::sign(&signing_message, &secret);
        let rs = signature.serialize();
        let r = H256::from_slice(&rs[0..32]);
        let s = H256::from_slice(&rs[32..64]);
        Transaction::EIP2930(ethereum::EIP2930Transaction {
            chain_id: msg.chain_id,
            nonce: msg.nonce,
            gas_price: msg.gas_price,
            gas_limit: msg.gas_limit,
            action: msg.action,
            value: msg.value,
            input: msg.input.clone(),
            access_list: msg.access_list,
            odd_y_parity: recid.serialize() != 0,
            r,
            s,
        })
    }
}

pub struct EIP1559UnsignedTransaction {
    pub nonce: U256,
    pub max_priority_fee_per_gas: U256,
    pub max_fee_per_gas: U256,
    pub gas_limit: U256,
    pub action: TransactionAction,
    pub value: U256,
    pub input: Vec<u8>,
}

impl EIP1559UnsignedTransaction {
    pub fn sign(&self, secret: &H256, chain_id: Option<u64>) -> Transaction {
        let secret = {
            let mut sk: [u8; 32] = [0u8; 32];
            sk.copy_from_slice(&secret[0..]);
            libsecp256k1::SecretKey::parse(&sk).unwrap()
        };
        let chain_id = chain_id.unwrap_or(ChainId::get());
        let msg = ethereum::EIP1559TransactionMessage {
            chain_id,
            nonce: self.nonce,
            max_priority_fee_per_gas: self.max_priority_fee_per_gas,
            max_fee_per_gas: self.max_fee_per_gas,
            gas_limit: self.gas_limit,
            action: self.action,
            value: self.value,
            input: self.input.clone(),
            access_list: vec![],
        };
        let signing_message = libsecp256k1::Message::parse_slice(&msg.hash()[..]).unwrap();

        let (signature, recid) = libsecp256k1::sign(&signing_message, &secret);
        let rs = signature.serialize();
        let r = H256::from_slice(&rs[0..32]);
        let s = H256::from_slice(&rs[32..64]);
        Transaction::EIP1559(ethereum::EIP1559Transaction {
            chain_id: msg.chain_id,
            nonce: msg.nonce,
            max_priority_fee_per_gas: msg.max_priority_fee_per_gas,
            max_fee_per_gas: msg.max_fee_per_gas,
            gas_limit: msg.gas_limit,
            action: msg.action,
            value: msg.value,
            input: msg.input.clone(),
            access_list: msg.access_list,
            odd_y_parity: recid.serialize() != 0,
            r,
            s,
        })
    }
}
