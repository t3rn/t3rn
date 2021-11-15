import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import * as definitions from '@t3rn/types';
import { createGatewayABIConfig, createGatewayGenesisConfig } from '../src/utils/utils';
import '@t3rn/types/dist/augment-api';
import '@t3rn/types/dist/augment-types';
import '@t3rn/types/dist/augment-api-rpc';
import { Vec } from '@polkadot/types';
import {
  AllowedSideEffect,
} from '@t3rn/types/dist';
import { expect } from 'chai';

describe('Execution Delivery | Extrinsics', function() {
  this.timeout(30000);
  const rococoUrl = 'wss://rococo-rpc.polkadot.io';
  const circuitProvider = new WsProvider('ws://localhost:9944');
  const rococoProvider = new WsProvider('wss://rococo-rpc.polkadot.io');
  const types = Object.values(definitions).reduce((res, { types }): object => ({ ...res, ...types }), {});
  let circuitApi: ApiPromise;

  beforeEach(async () => {
    circuitApi = await ApiPromise.create({
      provider: circuitProvider,
      types,
      rpc: {
        xdns: {
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
      },
    });
  });

  describe('sudo register_gateway', () => {
    it.only('should successfully register the Rococo gateway', async () => {
      const rococoApi = await ApiPromise.create({ provider: rococoProvider });
      const firstHeader = await rococoApi.rpc.chain.getHeader(rococoApi.genesisHash);

      // Constuct the keyring after the API (crypto has an async init)
      const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
      const alice = keyring.addFromUri('//Alice');
      const rococoMetadata = await rococoApi.runtimeMetadata;
      const rococoRuntimeVersion = await rococoApi.runtimeVersion;
      const rococoGenesisHash = await rococoApi.genesisHash;

      const registerGateway = circuitApi.tx.execDelivery.registerGateway(
        rococoUrl, // url
        'roco', // gateway_id
        createGatewayABIConfig(circuitApi, 32, 32, 32, 12, 'Sr25519', 'Blake2'), // GatewayABI
        circuitApi.createType('GatewayVendor', 'Substrate'), // GatewayVendor
        circuitApi.createType( 'GatewayType', { ProgrammableExternal: 1 }), // GatewayType
        createGatewayGenesisConfig(rococoMetadata, rococoRuntimeVersion, rococoGenesisHash, circuitApi), // GatewayGenesisConfig
        circuitApi.createType('Bytes', firstHeader.toU8a()), // first header
        circuitApi.createType('Option<Vec<AccountId>>', []), // authorities
        <Vec<AllowedSideEffect>>circuitApi.createType('Vec<AllowedSideEffect>', ['transfer', 'get_storage']), // allowed side effects
      );

      // submit the extrinsic and make wait until finalized
      const unsub = await circuitApi.tx.sudo.sudo(registerGateway).signAndSend(alice,(result => {
        if (result.status.isFinalized) {
          console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      }));

      // assert the new xdns record was added successfully
      const xdns = await circuitApi.rpc.xdns.fetchRecords();
      console.log(xdns.toJSON());
      // expect(xdns.xdns_records).to.have.length(3)

      // assert the bridge was instantiated properly
    });
    it('should not register a gateway if it already exists (Polkadot)', async () => {

    });
  });
  afterEach(async () => await circuitApi.disconnect());
});
