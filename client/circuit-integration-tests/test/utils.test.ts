import { createGatewayGenesisConfig } from '../src/utils/utils';
import { expect } from 'chai';
import { ApiPromise, WsProvider } from '@polkadot/api';

import { types } from '@t3rn/types';

describe('utils', function() {
  describe('createGatewayGenesisConfig', function() {
    this.timeout(30000);
    it('should successfully create a genesis config for polkadot url', async () => {
      const targetApi = await ApiPromise.create({ provider: new WsProvider('wss://rpc.polkadot.io') });
      const circuitApi = await ApiPromise.create({ provider: new WsProvider('ws://127.0.0.1:9944'), types });
      const targetMetadata = await targetApi.runtimeMetadata;
      const targetRuntimeVersion = await targetApi.runtimeVersion;
      const targetGenesisHash = await targetApi.genesisHash;
      const actual = createGatewayGenesisConfig(targetMetadata, targetRuntimeVersion, targetGenesisHash, circuitApi);
      expect(actual.genesis_hash.toHex()).to.eql('0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3');
      await targetApi.disconnect();
      await circuitApi.disconnect();
    });
  });
});
