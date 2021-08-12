//! RPC interface for the contracts registry pallet.

#[rpc]
pub trait ContractsRegistryApi<BlockHash, AccountId> {
    /// Returns the contracts searchable by name or author
    #[rpc(name = "contracts_fetchContracts")]
    fn fetch_contracts(
        &self,
        author: Option<AccountId>,
        contract_id: Option<Bytes>,
        data: Option<Bytes>,
        at: Option<BlockHash>,
    ) -> Result<RpcFetchContractsResult>;
}

/// An RPC serializable result of contracts fetch.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum RpcFetchContractsResult {
    /// Successful execution
    Success {
        /// The return flags
        flags: u32,
        /// Output data
        data: Bytes,
        /// How much gas was consumed by the call.
        gas_consumed: u64,
    },
    /// Error execution
    Error(()),
}

impl<C, Block, AccountId> ContractsRegistryApi<
    <Block as BlockT>::Hash,
    C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ContractsRegistryRuntimeApi<>

>
