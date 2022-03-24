import { ApiPromise, WsProvider } from '@polkadot/api'
import { createTestPairs } from '@polkadot/keyring/testingPairs'
import {
  JustificationNotification,
  Header,
  BridgedHeader,
  GrandpaJustification,
} from '@polkadot/types/interfaces'
import registerKusamaGateway from './register'
import { formatEvents } from './util'
import createDebug from 'debug'
import 'dotenv/config'
import types from './types.json'

const keyring = createTestPairs({ type: 'sr25519' })

export default class Relayer {
  static debug = createDebug('relayer')

  circuit: ApiPromise
  circuitEndpoint: string = process.env.CIRCUIT_WS_URL as string
  gatewayId: Buffer = Buffer.from(process.env.GATEWAY_ID as string, 'utf8')

  async init() {
    this.circuit = await ApiPromise.create({
      provider: new WsProvider(this.circuitEndpoint),
      types: types as any,
    })

    await registerKusamaGateway(this.circuit)

    Relayer.debug(`gateway ${this.gatewayId.toString()} registered`)

    const setOperational =
      this.circuit.tx.multiFinalityVerifierPolkadotLike.setOperational(
        true,
        this.gatewayId
      )

    return new Promise(async (resolve, reject) => {
      await this.circuit.tx.sudo
        .sudo(setOperational)
        .signAndSend(keyring.alice, result => {
          if (result.isError) {
            reject('submitting setOperational failed')
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
    Relayer.debug('submitting header range and finality proof...')

    const reversedRange: Header[] = range.reverse()
    const anchor: Header = reversedRange[0]

    const submitHeaderRange =
      this.circuit.tx.multiFinalityVerifierPolkadotLike.submitHeaderRange(
        gatewayId,
        reversedRange,
        anchor.hash
      )

    await new Promise(async (resolve, reject) => {
      await submitHeaderRange.signAndSend(keyring.alice, result => {
        if (result.isError) {
          reject(Error('submitting header range failed'))
        } else if (result.isInBlock) {
          Relayer.debug(
            'submit_header_range events',
            ...formatEvents(result.events)
          )
          resolve(undefined)
        }
      })
    })

    const submitFinalityProof =
      this.circuit.tx.multiFinalityVerifierPolkadotLike.submitFinalityProof(
        anchor,
        justification,
        gatewayId
      )

    await new Promise(async (resolve, reject) => {
      await submitFinalityProof.signAndSend(keyring.alice, result => {
        if (result.isError) {
          reject(Error('submitting finality proof failed'))
        } else if (result.isInBlock) {
          Relayer.debug(
            'submit_finality_proof events',
            ...formatEvents(result.events)
          )
          resolve(undefined)
        }
      })
    })
  }
}
