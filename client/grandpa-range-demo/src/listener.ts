import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api'
import {
  JustificationNotification,
  Header,
  BlockHash,
} from '@polkadot/types/interfaces'
import createDebug from 'debug'
import 'dotenv/config'

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
          await this.emitRange()
        }
      }
    )
  }

  async handleGrandpaSet() {
    const currentSetId: number = Number(
      (await this.kusama.query.grandpa.currentSetId()).toJSON()
    )

    if (this.grandpaSetId !== 0 && currentSetId !== this.grandpaSetId) {
      Listener.debug('grandpa set change', this.grandpaSetId, currentSetId)
      await this.emitRange()
    }

    this.grandpaSetId = currentSetId
  }

  async handleHeader(header: Header) {
    while (this.last !== 0 && header.number.toNumber() !== this.last + 1) {
      const missingHeader: Header = await this.kusama.rpc.chain.getHeader(
        await this.kusama.rpc.chain.getBlockHash(this.last + 1)
      )
      this.headers.push(missingHeader)
      this.last = missingHeader.number.toNumber()
    }

    this.headers.push(header)
    this.last = header.number.toNumber()
    Listener.debug(`#${this.last}`)
  }

  async emitRange() {
    const range: Header[] = this.headers.slice(
      this.offset,
      this.offset + this.rangeSize
    )
    this.offset += this.rangeSize

    const unsubJustifications =
      await this.kusama.rpc.grandpa.subscribeJustifications(
        async (justification: JustificationNotification) => {
          Listener.debug('got a grandpa justification...', justification)
          this.emit('range', range, justification, this.gatewayId)
          unsubJustifications()
        }
      )
  }

  kill() {
    Listener.debug('kill')
    this.unsubNewHeads()
  }
}
