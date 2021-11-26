export default {
  rpc: {
    fetchRecords: {
      description: 'Fetches all available XDNS Records on Circuit',
      params: [
        {
          name: 'at',
          type: 'Hash',
          isOptional: true,
        },
      ],
      type: 'FetchXdnsRecordsResponse',
    },
  },
  types: {
    XdnsRecordId: 'Hash',
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
    FetchXdnsRecordsResponse: {
      xdns_records: 'Vec<XdnsRecord<AccountId>>',
    },
  },
};
