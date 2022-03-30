import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { createTestPairs } from '@polkadot/keyring/testingPairs'
import {
  BlockHash,
  EncodedFinalityProofs,
  Header,
} from '@polkadot/types/interfaces'
import { sleep } from './util'
import createDebug from 'debug'
import 'dotenv/config'

const keyring = createTestPairs({ type: 'sr25519' })

export default class Listener extends EventEmitter {
  static debug = createDebug('listener')

  kusama: ApiPromise
  kusamaEndpoint: string = process.env.KUSAMA_RPC as string
  rangeSize: number = Number(process.env.RANGE_SIZE)
  gatewayId: Buffer = Buffer.from(process.env.GATEWAY_ID as string, 'utf8')
  headers: Header[] = []
  // offset in this.headers for the current range batch
  offset: number = 0
  // block number of the last enqueued header
  last: number = 0
  // last known grandpa set id
  grandpaSetId: number = 0

  unsubNewHeads: () => void

  async init() {
    this.kusama = await ApiPromise.create({
      provider: new WsProvider(this.kusamaEndpoint),
    })

    this.unsubNewHeads = await this.kusama.derive.chain.subscribeNewHeads(
      async (header: Header) => {
        await this.handleGrandpaSet()

        await this.handleHeader(header)

        if (this.headers.length - this.offset === this.rangeSize) {
          await this.concludeRange()
        }
      }
    )
  }

  async handleGrandpaSet() {
    const currentSetId = Number(
      await this.kusama.query.grandpa.currentSetId().then(id => id.toJSON())
    )

    if (this.grandpaSetId !== 0 && currentSetId !== this.grandpaSetId) {
      Listener.debug('grandpa set change', this.grandpaSetId, currentSetId)
      await this.concludeRange()
    }

    this.grandpaSetId = currentSetId
  }

  async handleHeader(header: Header) {
    // FIXME: don't trap
    while (this.last !== 0 && header.number.toNumber() > this.last + 1) {
      let missingHeader: Header | void
      try {
        missingHeader = await this.kusama.rpc.chain.getHeader(
          await this.kusama.rpc.chain.getBlockHash(this.last + 1)
        )
      } catch (_) {
        await sleep(6000)
      } finally {
        if (missingHeader) {
          this.headers.push(missingHeader)
          this.last = missingHeader.number.toNumber()
        }
      }
    }

    this.headers.push(header)
    this.last = header.number.toNumber()
    Listener.debug(`#${this.last}`)
  }

  async concludeRange() {
    const reversedRange: Header[] = this.headers
      .slice(this.offset, this.offset + this.rangeSize)
      .reverse()

    this.offset += this.rangeSize

    const anchor: Header = reversedRange.shift() as Header

    Listener.debug(
      'concluding... anchor',
      anchor.number.toNumber(),
      'range',
      ...reversedRange.map(h => h.number.toNumber())
    )

    let anchorFinalized = false
    while (!anchorFinalized) {
      const head: BlockHash | void =
        await this.kusama.rpc.chain.getFinalizedHead()
      if (head && head.eq(anchor.hash)) {
        anchorFinalized = true
      }
    }

    const proofs: EncodedFinalityProofs = await this.kusama.rpc.grandpa
      .proveFinality(anchor.number.toNumber())
      .then(opt => opt.unwrap())
    // https://github.com/polkadot-js/api/issues/4583
    Listener.debug('$$$$$', proofs, proofs.toJSON())
    const justification: any = proofs // TODO

    this.emit('range', this.gatewayId, anchor, reversedRange, justification)
  }

  kill() {
    Listener.debug('kill')
    this.unsubNewHeads()
  }
}
