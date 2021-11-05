import {RegistryTypes} from "@polkadot/types/types";

export const types: RegistryTypes = {
  XdnsRecordId: "Hash",
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
