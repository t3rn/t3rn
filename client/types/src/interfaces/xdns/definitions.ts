export default {
  rpc: {},
  types: {
    XdnsRecordId: {},
    XdnsRecord: {
      url: 'Vec<u8>',
      gateway_abi: 'GatewayABIConfig',
      gateway_genesis: 'GatewayGenesisConfig',
      gateway_vendor: 'GatewayVendor',
      gateway_type: 'GatewayType',
      gateway_id: 'ChainId',
      registrant: 'Option<AccountId>',
      last_finalized: 'Option<u64>',
    },
  }
}
