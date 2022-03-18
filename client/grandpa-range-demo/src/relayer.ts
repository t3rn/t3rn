import { ApiPromise, WsProvider } from '@polkadot/api'
import { createTestPairs } from '@polkadot/keyring/testingPairs'
import {
  JustificationNotification,
  Header,
  BridgedHeader,
  GrandpaJustification,
} from '@polkadot/types/interfaces'
import registerKusamaGateway from './register'
import createDebug from 'debug'
import 'dotenv/config'

const keyring = createTestPairs({ type: 'sr25519' })

export default class Relayer {
  static debug = createDebug('relayer')

  circuit: ApiPromise
  circuitEndpoint: string = process.env.CIRCUIT_WS_URL as string
  gatewayId: Buffer = Buffer.from(process.env.GATEWAY_ID as string, 'utf8')

  async init() {
    this.circuit = await ApiPromise.create({
      provider: new WsProvider(this.circuitEndpoint),
      types: {
        /*3*/
      },
    })

    await registerKusamaGateway(this.circuit)

    Relayer.debug(`gateway ${this.gatewayId.toString()} registered`)

    return new Promise(async (resolve, reject) => {
      await this.circuit.tx.multiFinalityVerifierSubstrateLike
        .setOperational(true, this.gatewayId)
        .signAndSend(keyring.alice, result => {
          if (result.isError) {
            reject(Error(result.status.toString()))
          } else if (result.isInBlock) {
            Relayer.debug(`gateway ${this.gatewayId.toString()} operational`)
            resolve(undefined)
          }
        })
    })
  }

  async submit(
    range: Header[],
    justification: JustificationNotification,
    gatewayId: Buffer
  ) {
    Relayer.debug('submitting finality proof...')

    const anchor: Header = range[range.length - 1]

    const hash = await this.circuit.tx.multiFinalityVerifierSubstrateLike
      .submitFinalityProof(anchor, justification, gatewayId)
      .signAndSend(keyring.alice)

    Relayer.debug(
      'submitted finality proof, xt hash 0x' + Buffer.from(hash).toString('hex')
    )
  }
}
