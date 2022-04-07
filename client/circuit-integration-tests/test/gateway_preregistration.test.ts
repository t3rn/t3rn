import '@t3rn/types'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api'
import { assert } from 'chai'
import { T3rnPrimitivesXdnsXdnsRecord } from '@polkadot/types/lookup'

describe('Default Multi Finality Verifier', function () {
  this.timeout(60000)

  let circuit: ApiPromise

  before(async () => {
    circuit = await ApiPromise.create({
      provider: new WsProvider(process.env.CIRCUIT_WS_URL),
    })
  })

  after(async () => await circuit.disconnect())

  describe('gateway preregistration', () => {
    it('should have preregistered Polkadot and Kusama gateways', async () => {
      /*
      // FIXME: assert xdns record is stored
      const xdnsRecord = await circuit.query.xdns
        .xdnsRecords(Uint8Array.from([ 112, 100, 111, 116 ])) // pdot
        //                           [ 107, 115, 109, 97 ]    // ksma
        .then(encoded => encoded.toHuman())

      console.log('$$$$$$$ xdnsRecord', xdnsRecord)
      */

      // assert gtwy instantiated in mfv storage
      const instantiatedGateways =
        await circuit.query.multiFinalityVerifierDefault
          .instantiatedGatewaysMap()
          .then(encoded => encoded.toHuman())

      assert.deepEqual(instantiatedGateways, ['pdot', 'ksma'])
    })

    // TODO
    it('should have preregistered Rococo parachain gateways', async () => {})
  })
})
