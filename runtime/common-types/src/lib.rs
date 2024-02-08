#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::{DispatchClass, Weight},
    parameter_types,
    weights::constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_REF_TIME_PER_SECOND},
};
use frame_system::limits::{BlockLength, BlockWeights};
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, CheckedDiv, IdentifyAccount, Verify, Zero},
    MultiSignature, Perbill, Saturating
};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Balance of an account.
pub type Amount = i128;

/// Asset Id.
pub type AssetId = u32;

/// Index of a transaction in the chain.
pub type Index = u32;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// EVM Address
pub type EvmAddress = sp_core::H160;

/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

// Prints debug output of the `contracts` pallet to stdout if the node is
// started with `-lruntime::contracts=debug`.
pub const CONTRACTS_DEBUG_OUTPUT: bool = true;

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12_000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We assume that ~25% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(25);

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

pub const DECIMALS: u8 = 12;
pub const MILLIT3RN: u64 = 1_000_000_000;
pub const MT3RN: Balance = MILLIT3RN as Balance;
pub const TRN: u64 = 1_000_000_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

/// We allow for 0.5 of a second of compute with a 12 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
    WEIGHT_REF_TIME_PER_SECOND.saturating_div(2),
    cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

parameter_types! {

    // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
    //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
    // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
    // the lazy contract deletion.
    pub const BlockHashCount: BlockNumber = 4096;
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    // Allows for t3 prefix in addresses
    pub const SS58Prefix: u16 = 9935;
    pub const SS58PrefixT1rn: u16 = 4815;
}

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

pub fn base_tx_fee() -> Balance {
    MILLIUNIT / 10
}

pub fn default_fee_per_second() -> u128 {
    let base_weight = Balance::from(ExtrinsicBaseWeight::get().ref_time());
    let base_tx_per_second = (WEIGHT_REF_TIME_PER_SECOND as u128) / base_weight;
    base_tx_per_second * base_tx_fee()
}

/// Convert decimal between native(12) and EVM(18) and therefore the 1_000_000 conversion.
const DECIMALS_VALUE: u32 = 1_000_000u32;

/// Convert decimal from native(TRN 12) to EVM(18).
pub fn convert_decimals_from_evm(b: Balance) -> Balance {
    if b.is_zero() {
        return b;
    }
    b.saturating_mul(DECIMALS_VALUE.into())
}

/// Convert decimal from native EVM(18) to TRN(12).
pub fn convert_decimals_to_evm(b: Balance) -> Option<Balance> {
    if b.is_zero() {
        return Some(b);
    }
    let res = b
        .checked_div(Into::<Balance>::into(DECIMALS_VALUE))
        .expect("divisor is non-zero; qed");

    if res.saturating_mul(DECIMALS_VALUE.into()) == b {
        Some(res)
    } else {
        None
    }
}

#[test]
fn fixed_block_time_12s() {
    assert_eq!(MILLISECS_PER_BLOCK, 12_000);
    assert_eq!(SLOT_DURATION, 12_000);
}
