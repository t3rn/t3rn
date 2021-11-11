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

describe('Execution Delivery | Extrinsics', function() {
  const provider = new WsProvider('wss://dev.net.t3rn.io');
  describe('sudo register_gateway', function() {
    this.timeout(30000);
    it.only('should successfully register the Rococo gateway', async () => {
      const rococoUrl = 'wss://rococo-rpc.polkadot.io';
      const types = Object.values(definitions).reduce((res, { types }): object => ({ ...res, ...types }), {});

      const t3rnApi = await ApiPromise.create({
        provider,
        types,
      });

      const rococoApi = await ApiPromise.create({ provider: new WsProvider(rococoUrl) });

      // Constuct the keyring after the API (crypto has an async init)
      const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
      const alice = keyring.addFromUri('//Alice');

      // create the genesis config from the Rococo client
      const genesisConfig = await createGatewayGenesisConfig(rococoApi);
      const firstHeader = await rococoApi.rpc.chain.getHeader(rococoApi.genesisHash);
      const gatewayABIConfig = createGatewayABIConfig(
        t3rnApi.registry,
        32, 32, 32, 12, 'Sr25519', 'Keccak256',
      );

      console.log(genesisConfig.modules_encoded.toHuman());
      console.log(genesisConfig.modules_encoded.toHex());
      console.log(JSON.stringify(genesisConfig.modules_encoded));
      console.log(genesisConfig.modules_encoded.toU8a());
      const params: [Bytes, ChainId, GatewayABIConfig, GatewayVendor, GatewayType, GatewayGenesisConfig, Bytes, Option<Vec<AccountId>>, Vec<AllowedSideEffect>] = [
        t3rnApi.createType('Bytes', rococoUrl), // url
        t3rnApi.createType('ChainId', 'roco'), // gateway_id
        gatewayABIConfig, // GatewayABI
        createType(t3rnApi.registry, 'GatewayVendor', 'Substrate'), // GatewayVendor
        createType(t3rnApi.registry, 'GatewayType', 'External'), // GatewayType
        genesisConfig, // GatewayGenesisConfig
        t3rnApi.createType('Bytes', [firstHeader.toU8a()]), // first header
        t3rnApi.createType('Option<Vec<AccountId>>', []), // authorities
        t3rnApi.createType('Vec<AllowedSideEffect>', ['transfer']), // allowed side effects
      ];
      // console.log(gatewayABIConfig.toHex())
      // console.log(JSON.stringify(params))
        // console.log(genesisConfig.toHex())
      const registerGateway = t3rnApi.tx.execDelivery.registerGateway(...params);
      // console.log(registerGateway.data.toString());
      // Send the actual sudo transaction


      // 0x1503707773733a2f2f726f636f636f2d7270632e706f6c6b61646f742e696f726f636f200020000101200020000c000000010100010018726f636f636f487061726974792d726f636f636f2d76322e3000000000ab2300000000000008df6acb689907609b0300000037e397fc7c91f5e401000000000000000480c196f81260cf1686172b47a79cf002120735d7cb0eb1474e8adce56618456fff04000000

      // const txResult = await t3rnApi.tx.sudo.sudo(registerGateway).signAndSend(alice);
      const txResult = await registerGateway.signAndSend(alice);
      console.log(JSON.stringify(txResult));

      // console.log(registerGateway.toHex());
      // .signAndSend(alice, (result) => {
      //   console.log('Register Gateway called with hash', JSON.stringify(result));
      //   console.log(result.toHuman());
      // });
      // const hash = await registerGateway.signAndSend(alice);
      // console.log('Register Gateway called with hash', unsub.toHex());
    });
  });
});