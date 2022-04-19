import '@t3rn/types'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api'
import { T3rnPrimitivesXdnsXdnsRecord } from '@polkadot/types/lookup'
import { assert } from 'chai'

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
    it('should have preregistered relaychain gateways', async () => {
      const expectedGatewayIds = ['pdot', 'ksma', 'roco']

      // TODO: assert xdns records are stored

      const instantiatedGateways =
        await circuit.query.multiFinalityVerifierDefault
          .instantiatedGatewaysMap()
          .then(encoded => encoded.toHuman())

      assert.deepEqual(instantiatedGateways, expectedGatewayIds)
    })
  })
})
