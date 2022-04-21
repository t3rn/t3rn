import { ApiPromise, WsProvider } from '@polkadot/api'
import { createTestPairs } from '@polkadot/keyring/testingPairs'
import { Header } from '@polkadot/types/interfaces'
import { formatEvents } from './util'
import createDebug from 'debug'
import 'dotenv/config'
import types from './types.json'
import { EventEmitter } from 'stream'

const keyring = createTestPairs({ type: 'sr25519' })

export default class Relayer extends EventEmitter {
  static debug = createDebug('relayer')

  circuit: ApiPromise
  circuitEndpoint: string = process.env.CIRCUIT_WS_URL as string
  gatewayId: Buffer = Buffer.from(process.env.GATEWAY_ID as string, 'utf8')

  async init() {
    this.circuit = await ApiPromise.create({
      provider: new WsProvider(this.circuitEndpoint),
      types: types as any,
    })
  }

  async submit(
    gatewayId: Buffer,
    anchor: Header,
    reversedRange: Header[],
    justification: any,
    offset: number
  ) {
    Relayer.debug('submitting finality proof and header range...')
    Relayer.debug(
      `submit_finality_proof(\n\t${anchor},\n\t${justification
        .toString()
        .slice(0, 10)}...,\n\t${gatewayId}\n)`
    )

    const submitFinalityProof =
      this.circuit.tx.multiFinalityVerifierDefault.submitFinalityProof(
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
          return resolve(undefined)
        }
      })
    })

    Relayer.debug(
      `submit_header_range(\n\t${gatewayId},\n\t${reversedRange},\n\t${anchor.hash}\n)`
    )

    const submitHeaderRange =
      this.circuit.tx.multiFinalityVerifierDefault.submitHeaderRange(
        gatewayId,
        reversedRange,
        anchor.hash
      )

    await new Promise(async (resolve, reject) => {
      await submitHeaderRange.signAndSend(keyring.alice, result => {
        if (result.isError) {
          reject(Error('submitting header range failed'))
        } else if (result.status.isFinalized) {
          Relayer.debug(
            'submit_header_range events',
            ...formatEvents(result.events)
          )
          this.emit("RangeSubmitted", offset);
          resolve(undefined)
        }
      })
    })
  }
}
