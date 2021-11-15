import { Metadata } from '@polkadot/types';
import { GatewayABIConfig, GatewayGenesisConfig } from '@t3rn/types/dist';
import { ApiPromise } from '@polkadot/api';
import { Hash, RuntimeVersion } from '@polkadot/types/interfaces';

export function createGatewayABIConfig(
  api: ApiPromise,
  hash_size: number,
  address_length: number,
  block_number_type_size: number,
  decimals: number,
  crypto: 'Ed25519' | 'Sr25519' | 'Ecdsa',
  hasher: 'Blake2' | 'Keccak256'
): GatewayABIConfig {
  return api.createType('GatewayABIConfig', [
    api.createType('u16', block_number_type_size),
    api.createType('u16', hash_size),
    api.createType('HasherAlgo', hasher),
    api.createType('CryptoAlgo', crypto),
    api.createType('u16', address_length),
    api.createType('u16', 32),
    api.createType('u16', decimals),
    api.createType('Vec<StructDecl>', []),
  ]);
}

export function createGatewayGenesisConfig(
  metadata: Metadata,
  runtimeVersion: RuntimeVersion,
  genesisHash: Hash,
  circuitApi: ApiPromise
): GatewayGenesisConfig {
  return circuitApi.createType('GatewayGenesisConfig', [
    circuitApi.createType('Option<Bytes>', metadata.asV14.pallets.toHex()),
    runtimeVersion,
    metadata.asV14.extrinsic.version,
    genesisHash,
  ]);
}
