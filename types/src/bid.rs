use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// All Executors from the active set can bid for SFX executions in order to claim the rewards (max_fee) set by users,
///     ultimately competing against one another on the open market rules.
/// In case bid goes on Optimistic SFX, Executor will also have their bonded stake reserve to insure
///     other Optimistic Executors co-executing given Xtx with their bonded collateral (reserved_bond)
/// Their balance
#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct SFXBid<AccountId, BalanceOf, AssetId> {
    /// Bid amount - always below SFX::max_fee requested by a user
    pub amount: BalanceOf,
    /// Insurance in case of optimistic FSX
    pub insurance: BalanceOf,
    /// Optional reserved bond in case of optimistic FSX
    pub reserved_bond: Option<BalanceOf>,
    /// Optional reserved asset id in case execution on foreign assets
    pub reward_asset_id: Option<AssetId>,
    /// Bidding Executor belonging to the active set
    pub executor: AccountId,
    /// Requester - subject ordering SFX
    pub requester: AccountId,
}

impl<AccountId: Encode, BalanceOf, AssetId> SFXBid<AccountId, BalanceOf, AssetId> {
    pub fn new_none_optimistic(
        bid: BalanceOf,
        insurance: BalanceOf,
        executor: AccountId,
        requester: AccountId,
        reward_asset_id: Option<AssetId>,
    ) -> Self {
        SFXBid {
            amount: bid,
            insurance,
            reserved_bond: None,
            executor,
            requester,
            reward_asset_id,
        }
    }

    pub fn expect_reserved_bond(&self) -> &BalanceOf {
        self.reserved_bond
            .as_ref()
            .expect("Accessed reserved_bond and expected it to be a part of SFXBid")
    }

    pub fn get_reserved_bond(&self) -> &Option<BalanceOf> {
        &self.reserved_bond
    }

    pub fn get_insurance(&self) -> &BalanceOf {
        &self.insurance
    }

    /// Generate BID id as a hash of the SFX id and the executor account id bytes
    pub fn generate_id<Hasher: sp_core::Hasher, T: frame_system::Config>(
        &self,
        sfx_id: T::Hash,
    ) -> <Hasher as sp_core::Hasher>::Out {
        let mut sfx_id_and_index = sfx_id.encode();
        sfx_id_and_index.extend(&self.executor.encode());
        Hasher::hash(sfx_id_and_index.as_ref())
    }
}
