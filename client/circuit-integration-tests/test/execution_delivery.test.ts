import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import * as definitions from '@t3rn/types';
import { createGatewayABIConfig, createGatewayGenesisConfig } from '../src/utils/utils';
import '@t3rn/types/dist/augment-api';
import '@t3rn/types/dist/augment-types';
import { Bytes, createType, Option, Vec } from '@polkadot/types';
import {
  AllowedSideEffect,
  GatewayABIConfig,
  GatewayGenesisConfig,
  GatewayType,
  GatewayVendor,
} from '@t3rn/types/dist';
import { AccountId, ChainId } from '@polkadot/types/interfaces';
import * as console from 'console';

describe('Execution Delivery | Extrinsics', function () {
  const provider = new WsProvider('ws://localhost:9944');
  const apis: ApiPromise[] = [];
  describe('sudo register_gateway', function () {
    this.timeout(30000);
    it.only('should successfully register the Rococo gateway', async () => {
      const rococoUrl = 'wss://rococo-rpc.polkadot.io';
      const types = Object.values(definitions).reduce((res, { types }): object => ({ ...res, ...types }), {});

      const circuitApi = await ApiPromise.create({
        provider,
        types,
      });

      const rococoApi = await ApiPromise.create({ provider: new WsProvider(rococoUrl) });
      apis.push(circuitApi, rococoApi);
      // Constuct the keyring after the API (crypto has an async init)
      const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
      const alice = keyring.addFromUri('//Alice');

      // create the genesis config from the Rococo client
      const genesisConfig = await createGatewayGenesisConfig(rococoApi, circuitApi);
      const firstHeader = await rococoApi.rpc.chain.getHeader(rococoApi.genesisHash);
      const gatewayABIConfig = createGatewayABIConfig(circuitApi.registry, 32, 32, 32, 12, 'Sr25519', 'Keccak256');

      // genesisConfig.signed_extension.toU8a()
      // console.log(genesisConfig.modules_encoded.toHuman());
      console.log(genesisConfig.signed_extension.unwrap().toU8a());
      // console.log(JSON.stringify(genesisConfig.modules_encoded));
      console.log(genesisConfig.modules_encoded.unwrap().toU8a());
      const params: [
        Bytes,
        ChainId,
        GatewayABIConfig,
        GatewayVendor,
        GatewayType,
        GatewayGenesisConfig,
        Bytes,
        Option<Vec<AccountId>>,
        Vec<AllowedSideEffect>
      ] = [
        circuitApi.createType('Bytes', rococoUrl), // url
        circuitApi.createType('ChainId', 'roco'), // gateway_id
        gatewayABIConfig, // GatewayABI
        createType(circuitApi.registry, 'GatewayVendor', 'Substrate'), // GatewayVendor
        createType(circuitApi.registry, 'GatewayType', 'External'), // GatewayType
        genesisConfig, // GatewayGenesisConfig
        circuitApi.createType('Bytes', [firstHeader.toU8a()]), // first header
        circuitApi.createType('Option<Vec<AccountId>>', []), // authorities
        circuitApi.createType('Vec<AllowedSideEffect>', ['transfer']), // allowed side effects
      ];
      const registerGateway = circuitApi.tx.execDelivery.registerGateway(...params);

      // Send the actual sudo transaction
      const txResult = await circuitApi.tx.sudo.sudo(registerGateway).signAndSend(alice);
      // const txResult = await registerGateway.signAndSend(alice);
      // console.log(JSON.stringify(txResult));

      // console.log(registerGateway.toHex());
      // .signAndSend(alice, (result) => {
      //   console.log('Register Gateway called with hash', JSON.stringify(result));
      //   console.log(result.toHuman());
      // });
      // const hash = await registerGateway.signAndSend(alice);
      // console.log('Register Gateway called with hash', unsub.toHex());
    });
  });
  afterEach(function () {
    apis.map((i) => i.disconnect());
  });
});
