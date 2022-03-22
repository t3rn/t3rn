export default {
  types: {
    RawAliveContractInfo: {
      trie_id: "TrieId",
      storage_size: "u32",
      pair_count: "u32",
      code_hash: "CodeHash",
      rent_allowance: "Balance",
      rent_paid: "Balance",
      deduct_block: "BlockNumber",
      last_write: "Option<BlockNumber>",
      _reserved: "Option<()>",
    },
  },
};
