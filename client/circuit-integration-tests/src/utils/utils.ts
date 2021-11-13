import { u16, Vec } from '@polkadot/types';
import { Registry } from '@polkadot/types/types';
import { GatewayABIConfig, GatewayGenesisConfig } from '@t3rn/types/dist';
import { ApiPromise } from '@polkadot/api';
import * as definitions from '@t3rn/types';

export function createGatewayABIConfig(
  registry: Registry,
  hash_size: number,
  address_length: number,
  block_number_type_size: number,
  decimals: number,
  crypto: 'Sr25519' | 'Ed25519',
  hasher: 'Blake2' | 'Keccak256'
): GatewayABIConfig {
  function toCryptoAlgo(crypto: 'Sr25519' | 'Ed25519') {
    return crypto === 'Sr25519' ? 0 : crypto === 'Ed25519' ? 1 : new Error('Unknown crypto');
  }

  function toHasherAlgo(hasher: 'Blake2' | 'Keccak256') {
    return hasher === 'Blake2' ? 0 : hasher === 'Keccak256' ? 1 : new Error('Unknown hasher');
  }
  return registry.createType('GatewayABIConfig', [
    new u16(registry, block_number_type_size),
    new u16(registry, hash_size),
    toHasherAlgo(hasher),
    toCryptoAlgo(crypto),
    new u16(registry, address_length),
    new u16(registry, 32),
    new u16(registry, decimals),
    new Vec(registry, 'StructDecl', []),
  ]);
}

export async function createGatewayGenesisConfig(
  gatewayApi: ApiPromise,
  circuitApi: ApiPromise
): Promise<GatewayGenesisConfig> {
  gatewayApi.registerTypes({
    GatewayGenesisConfig: definitions.primitives.types.GatewayGenesisConfig,
  });
  // fetch runtime metadata
  const gatewayMetadata = await gatewayApi.runtimeMetadata;
  // fetch runtime version
  const runtimeVersion = await gatewayApi.runtimeVersion;
  // fetch genesis hash
  const genesisHash = await gatewayApi.genesisHash;

  return circuitApi.createType('GatewayGenesisConfig', [
    circuitApi.createType('Option<Bytes>', gatewayMetadata.asV14.pallets.toHex()),
    circuitApi.createType('Option<Bytes>', gatewayMetadata.asV14.extrinsic.signedExtensions.toHex()),
    runtimeVersion,
    gatewayMetadata.asV14.extrinsic.version,
    genesisHash,
  ]);
}
