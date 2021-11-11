import { createGatewayGenesisConfig } from '../src/utils/utils';
import {expect} from 'chai'
import { ApiPromise, WsProvider } from '@polkadot/api';

describe('utils', function () {
  describe('createGatewayGenesisConfig', function () {
    this.timeout(30000);
    it('should successfully create a genesis config for polkadot url', async () => {
        const api = new ApiPromise({provider: new WsProvider('wss://rpc.polkadot.io')})
        const actual = await createGatewayGenesisConfig(api);
        expect(actual.genesis_hash.toHex()).to.eql('0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3')
    })
  });
});