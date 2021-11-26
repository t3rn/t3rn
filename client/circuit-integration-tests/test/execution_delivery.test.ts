import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { rpc, types } from '@t3rn/types';
import { createGatewayABIConfig, createGatewayGenesisConfig, randomGatewayId } from '../src/utils/utils';
import '@t3rn/types/dist/augment-api';
import '@t3rn/types/dist/augment-types';
import '@t3rn/types/dist/augment-api-rpc';
import '@t3rn/types/dist/augment-api-query';
import { Vec } from '@polkadot/types';
import { AllowedSideEffect, XdnsRecord } from '@t3rn/types/dist';
import { expect } from 'chai';
import { BN } from '@polkadot/util';

const timeoutIn = (seconds: number) =>
  new Promise<void>((resolve, reject) => setTimeout(() => reject(new Error('timeout')), seconds));

describe('Execution Delivery | Extrinsics', function () {
  this.timeout(30000);
  const circuitProvider = new WsProvider('ws://127.0.0.1:9944');
  let circuitApi: ApiPromise;

  before(async () => {
    circuitApi = await ApiPromise.create({
      provider: circuitProvider,
      types,
      rpc,
    });
  });

  describe('sudo register_gateway', () => {
    it('should successfully register a substrate gateway (Rococo)', async () => {
      const rococoUrl = 'wss://rococo-rpc.polkadot.io';
      const rococoProvider = new WsProvider(rococoUrl);
      const rococoApi = await ApiPromise.create({ provider: rococoProvider });
      const [rococoCurrentHeader, rococoMetadata, rococoRuntimeVersion, rococoGenesisHash] = await Promise.all([
        await rococoApi.rpc.chain.getHeader(),
        await rococoApi.runtimeMetadata,
        await rococoApi.runtimeVersion,
        await rococoApi.genesisHash,
      ]);
      const rococoAtGenesis = await rococoApi.at(rococoGenesisHash);
      const rococoInitialAuthorityList = await rococoAtGenesis.query.session.validators();
      await rococoApi.disconnect();

      // Constuct the keyring after the API (crypto has an async init)
      const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
      const alice = keyring.addFromUri('//Alice');
      const gatewayId = randomGatewayId();

      // Create the extrinsic call
      const registerGateway = circuitApi.tx.execDelivery.registerGateway(
        rococoUrl,
        gatewayId,
        createGatewayABIConfig(circuitApi, 32, 32, 32, 12, 'Sr25519', 'Blake2'), // GatewayABI
        circuitApi.createType('GatewayVendor', 'Substrate'), // GatewayVendor
        circuitApi.createType('GatewayType', { ProgrammableExternal: 1 }), // GatewayType
        createGatewayGenesisConfig(rococoMetadata, rococoRuntimeVersion, rococoGenesisHash, circuitApi), // GatewayGenesisConfig
        circuitApi.createType('Bytes', rococoCurrentHeader.toHex()), // first header
        circuitApi.createType('Option<Vec<AccountId>>', rococoInitialAuthorityList), // authorities
        <Vec<AllowedSideEffect>>circuitApi.createType('Vec<AllowedSideEffect>', ['transfer', 'get_storage']) // allowed side effects
      );

      // Wrap in sudo, submit the extrinsic and make wait until finalized
      const result = new Promise<void>((resolve) =>
        circuitApi.tx.sudo.sudo(registerGateway).signAndSend(alice, (result) => {
          if (result.status.isFinalized) {
            expect(result.dispatchError).to.be.undefined;
            expect(result.internalError).to.be.undefined;
            expect(result.dispatchInfo).to.be.ok;
            expect(result.dispatchInfo?.weight).to.be.ok;
            expect(result.dispatchInfo?.class.isNormal).to.be.true;
            expect(result.dispatchInfo?.paysFee.isYes).to.be.true;
            expect(result.events).to.be.an('array');
            // expect(result.events).to.contain.;
            resolve();
          }
        })
      );

      // The extrinsic should be finalized before the timeout
      await Promise.race([result, timeoutIn(30000)]);

      // assert the record was added to the xdns storage
      const xdns = await circuitApi.rpc.xdns.fetchRecords();
      expect(xdns.xdns_records.map((x: XdnsRecord) => x.gateway_id.toHuman())).to.include.deep.members([gatewayId]);

      // assert the gateway exists in multi-finality-verifier storage
      // await circuitApi.query.bridgePolkadotLikeMultiFinalityVerifier.initialHashMap(gatewayId)
    });

    it('should not register a gateway if it already exists (Polkadot)', async () => {
      const polkadotUrl = 'wss://rpc.polkadot.io';
      const provider = new WsProvider(polkadotUrl);
      const polkadotApi = await ApiPromise.create({ provider });
      const [polkadotMetadata, polkadotRuntimeVersion, polkadotGenesisHash] = await Promise.all([
        await polkadotApi.runtimeMetadata,
        await polkadotApi.runtimeVersion,
        await polkadotApi.genesisHash,
      ]);

      // Constuct the keyring after the API (crypto has an async init)
      const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
      const alice = keyring.addFromUri('//Alice');

      // Create the extrinsic call
      const registerGateway = circuitApi.tx.execDelivery.registerGateway(
        polkadotUrl,
        'pdot',
        createGatewayABIConfig(circuitApi, 32, 32, 32, 12, 'Sr25519', 'Blake2'), // GatewayABI
        circuitApi.createType('GatewayVendor', 'Substrate'), // GatewayVendor
        circuitApi.createType('GatewayType', { ProgrammableExternal: 1 }), // GatewayType
        createGatewayGenesisConfig(polkadotMetadata, polkadotRuntimeVersion, polkadotGenesisHash, circuitApi), // GatewayGenesisConfig
        circuitApi.createType('Bytes', []), // first header
        circuitApi.createType('Option<Vec<AccountId>>', []), // authorities
        <Vec<AllowedSideEffect>>circuitApi.createType('Vec<AllowedSideEffect>', ['transfer', 'get_storage']) // allowed side effects
      );

      // Wrap in sudo, submit the extrinsic and make wait until finalized
      const result = new Promise<void>((resolve) =>
        circuitApi.tx.sudo.sudo(registerGateway).signAndSend(alice, (result) => {
          if (result.status.isFinalized) {
            expect(result.dispatchError).to.be.undefined;
            expect(result.internalError).to.be.undefined;
            expect(result.dispatchInfo).to.be.ok;
            expect(result.dispatchInfo?.weight).to.be.ok;
            expect(result.dispatchInfo?.class.isNormal).to.be.true;
            expect(result.dispatchInfo?.paysFee.isYes).to.be.true;
            expect(result.events).to.be.an('array');
            expect(result.events.map((e) => e.event.method)).to.not.contain.members(['XdnsRecordStored']);
            resolve();
          }
        })
      );

      // The extrinsic should be finalized before the timeout
      await Promise.race([result, timeoutIn(60000)]);

      // assert the record was added to the xdns storage
      const xdns = await circuitApi.rpc.xdns.fetchRecords();
      expect(xdns.xdns_records.map((x: XdnsRecord) => x.gateway_id.toHuman())).to.not.include.deep.members(['pdot']);
      await polkadotApi.disconnect();
    });

    after(async () => await circuitApi.disconnect());
  });
});
