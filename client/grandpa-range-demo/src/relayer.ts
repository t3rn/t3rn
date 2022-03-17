import { ApiPromise, WsProvider } from '@polkadot/api'
import { createTestPairs } from '@polkadot/keyring/testingPairs'
import {
  JustificationNotification,
  Header,
  BridgedHeader,
  GrandpaJustification,
} from '@polkadot/types/interfaces'
import createDebug from 'debug'
import 'dotenv/config'

const keyring = createTestPairs({ type: 'sr25519' })

export default class Relayer {
  static debug = createDebug('relayer')

  endpoint: string = process.env.CIRCUIT_WS_URL as string
  circuitPromise: Promise<ApiPromise> = ApiPromise.create({
    provider: new WsProvider(this.endpoint),
    types: {
      /*3*/
    },
  })

  async submit(
    range: Header[],
    justification: JustificationNotification,
    gatewayId: Buffer
  ) {
    Relayer.debug('submitting finality proof...')

    const circuit: ApiPromise = await this.circuitPromise

    const anchor: Header = range[range.length - 1]

    await circuit.tx.multiFinalityVerifierSubstrateLike
      .submitFinalityProof(anchor, justification, gatewayId)
      .signAndSend(keyring.alice, result => {
        if (result.isError) {
          Relayer.debug(`error: ${result.status.toString()}`)
        } else if (result.isInBlock) {
          Relayer.debug('in block')
        }
      })
  }
}
