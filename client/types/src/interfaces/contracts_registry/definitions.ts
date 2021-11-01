export default {
  types: {
    RegistryContractId: 'Hash',
    RegistryContract: {
      code_txt: 'Vec<u8>',
      bytes: 'Vec<u8>',
      author: 'AccountId',
      author_fees_per_single_use: 'Option<BalanceOf>',
      abi: 'Option<Vec<u8>>',
      action_descriptions: 'Vec<ContractActionDesc<Hash, ChainId, AccountId>>',
      info: 'Option<RawAliveContractInfo<Hash, BalanceOf, BlockNumber>>',
      meta: 'ContractMetadata',
    },
  }
}
