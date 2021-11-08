import {CryptoAlgo, GatewayABIConfig, HasherAlgo} from '@t3rn/types'
import {u16} from "@polkadot/types";
import {Registry} from "@polkadot/types/types";

export function createGatewayABIConfig(registry: Registry, hash_size: number, address_length: number, block_number_type_size: number, decimals: number, crypto: CryptoAlgo, hasher: HasherAlgo): GatewayABIConfig {
  return <GatewayABIConfig>{
    address_length: new u16(registry, address_length),
    block_number_type_size: new u16(registry, block_number_type_size),
    crypto: <CryptoAlgo><unknown>'sr25519',
    decimals: new u16(registry, decimals),
    hash_size: new u16(registry, hash_size),
    hasher: <HasherAlgo><unknown>'keccak2',
    structs: undefined,
    value_type_size: undefined
  };

  // {
  //   address_length: new BN(address_length) as u16,
  //   decimals: new BN(decimals) as u16,
  //   crypto,
  //   hasher,
  //   hash_size: new BN(hash_size) as u16,
  //   block_number_type_size: new BN(block_number_type_size) as u16,
  //
  // }
}