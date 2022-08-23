//! RPC interface for the contracts registry pallet.

use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
pub use pallet_staking_rpc_runtime_api::{RpcBalance, StakingRuntimeApi};
use sp_api::{codec::Codec, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT, MaybeDisplay},
};
use std::sync::Arc;
use t3rn_primitives::{
    common::{OrderedSet, RoundIndex},
    staking::{
        Bond, CandidateMetadataFormat, ExecutorInfo, ExecutorSnapshot, Fixtures,
        StakerMetadataFormat,
    },
};

const RUNTIME_ERROR: i64 = 1;

#[rpc]
pub trait StakingApi<AccountId, Balance>
where
    Balance: std::str::FromStr + core::fmt::Display,
{
    // #[rpc(name = "staking_getFixtures")]
    // fn get_fixtures(&self) -> Result<Fixtures<RpcBalance<Balance>>>;

    #[rpc(name = "staking_getTotalValueLocked")]
    fn get_total_value_locked(&self) -> Result<RpcBalance<Balance>>;

    #[rpc(name = "staking_getActiveStake")]
    fn get_active_stake(&self, round: RoundIndex) -> Result<RpcBalance<Balance>>;

    // #[rpc(name = "staking_getExecutorConfig")]
    // fn get_executor_config(&self, who: AccountId) -> Result<Option<ExecutorInfo>>;

    // #[rpc(name = "staking_getExecutorSnapshot")]
    // fn get_executor_snapshot(
    //     &self,
    //     round: RoundIndex,
    //     who: AccountId,
    // ) -> Result<Option<ExecutorSnapshot<AccountId, RpcBalance<Balance>>>>;

    // #[rpc(name = "staking_getCandidateInfo")]
    // fn get_candidate_info(
    //     &self,
    //     who: AccountId,
    // ) -> Result<Option<CandidateMetadataFormat<RpcBalance<Balance>>>>;

    // #[rpc(name = "staking_getStakerInfo")]
    // fn get_staker_info(
    //     &self,
    //     who: AccountId,
    // ) -> Result<Option<StakerMetadataFormat<AccountId, RpcBalance<Balance>>>>;

    // #[rpc(name = "staking_listCandidates")]
    // fn list_candidates(&self) -> Result<OrderedSet<Bond<AccountId, RpcBalance<Balance>>>>;

    // #[rpc(name = "staking_listActiveSet")]
    // fn list_active_set(&self) -> Result<Vec<AccountId>>;
}

/// A struct that implements the [StakingApi].
pub struct Staking<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> Staking<C, B> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, Balance> StakingApi<AccountId, Balance> for Staking<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: StakingRuntimeApi<Block, AccountId, Balance>,
    AccountId: Codec + Clone + Send + Sync + 'static,
    Balance: Codec + Clone + Send + Sync + 'static + core::str::FromStr + core::fmt::Display,
{
    // fn get_fixtures(&self) -> Result<Fixtures<RpcBalance<Balance>>> {
    //     let api = self.client.runtime_api();
    //     let at = BlockId::hash(self.client.info().best_hash);
    //     api.get_fixtures(&at)
    //         .map_err(|e| runtime_error_into_rpc_err(e))
    // }

    fn get_total_value_locked(&self) -> Result<RpcBalance<Balance>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);
        api.get_total_value_locked(&at)
            .map_err(|e| runtime_error_into_rpc_err(e))
    }

    fn get_active_stake(&self, round: RoundIndex) -> Result<RpcBalance<Balance>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);
        api.get_active_stake(&at, round)
            .map_err(|e| runtime_error_into_rpc_err(e))
    }

    // fn get_executor_config(&self, who: AccountId) -> Result<Option<ExecutorInfo>> {
    //     let api = self.client.runtime_api();
    //     let at = BlockId::hash(self.client.info().best_hash);
    //     api.get_executor_config(&at, who)
    //         .map_err(|e| runtime_error_into_rpc_err(e))
    // }

    // fn get_executor_snapshot(
    //     &self,
    //     round: RoundIndex,
    //     who: AccountId,
    // ) -> Result<Option<ExecutorSnapshot<AccountId, RpcBalance<Balance>>>> {
    //     let api = self.client.runtime_api();
    //     let at = BlockId::hash(self.client.info().best_hash);
    //     api.get_executor_snapshot(&at, round, who)
    //         .map_err(|e| runtime_error_into_rpc_err(e))
    // }

    // fn get_candidate_info(
    //     &self,
    //     who: AccountId,
    // ) -> Result<Option<CandidateMetadataFormat<RpcBalance<Balance>>>> {
    //     let api = self.client.runtime_api();
    //     let at = BlockId::hash(self.client.info().best_hash);
    //     api.get_candidate_info(&at, who)
    //         .map_err(|e| runtime_error_into_rpc_err(e))
    // }

    // fn get_staker_info(
    //     &self,
    //     who: AccountId,
    // ) -> Result<Option<StakerMetadataFormat<AccountId, RpcBalance<Balance>>>> {
    //     let api = self.client.runtime_api();
    //     let at = BlockId::hash(self.client.info().best_hash);
    //     api.get_staker_info(&at, who)
    //         .map_err(|e| runtime_error_into_rpc_err(e))
    // }

    // fn list_candidates(&self) -> Result<OrderedSet<Bond<AccountId, RpcBalance<Balance>>>> {
    //     let api = self.client.runtime_api();
    //     let at = BlockId::hash(self.client.info().best_hash);
    //     api.list_candidates(&at)
    //         .map_err(|e| runtime_error_into_rpc_err(e))
    // }

    // fn list_active_set(&self) -> Result<Vec<AccountId>> {
    //     let api = self.client.runtime_api();
    //     let at = BlockId::hash(self.client.info().best_hash);
    //     api.list_active_set(&at)
    //         .map_err(|e| runtime_error_into_rpc_err(e))
    // }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{:?}", err).into()),
    }
}
