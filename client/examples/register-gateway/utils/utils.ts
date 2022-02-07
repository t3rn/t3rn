import { Bytes, Metadata } from '@polkadot/types';
import { ApiPromise } from '@polkadot/api';
import { Hash, RuntimeVersion } from '@polkadot/types/interfaces';
import { GatewaySysProps } from 'client/types/dist';

export function createGatewayABIConfig(
  api: ApiPromise,
  hash_size: number,
  address_length: number,
  block_number_type_size: number,
  decimals: number,
  crypto: 'Ed25519' | 'Sr25519' | 'Ecdsa',
  hasher: 'Blake2' | 'Keccak256'
): any {
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
): any {
  return circuitApi.createType('GatewayGenesisConfig', [
    circuitApi.createType('Option<Bytes>', metadata.asV14.pallets.toHex()),
    runtimeVersion,
    metadata.asV14.extrinsic.version,
    genesisHash,
  ]);
}

export function createGatewaySysProps(
  api: ApiPromise,
  ss58Format: number,
  tokenSymbol: string,
  tokenDecimals: number
): GatewaySysProps {
  return api.createType('GatewaySysProps', [
    api.createType('u16', ss58Format),
    api.createType('Bytes', new Bytes(api.registry, tokenSymbol)),
    api.createType('u8', tokenDecimals),
  ]);
}

export function randomGatewayId() {
  return String.fromCharCode(...[0, 0, 0, 0].map(() => Math.floor(97 + Math.random() * 26)));
}
