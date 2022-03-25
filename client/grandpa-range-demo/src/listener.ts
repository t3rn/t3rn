import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { JustificationNotification, Header } from '@polkadot/types/interfaces'
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
  // tmp cache holding early bird headers
  // i.e. if last #1, curr #3, then it would cache #3 until we got #2
  cache: Map<number, Header> = new Map<number, Header>()
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
    if (this.last !== 0 && header.number.toNumber() > this.last + 1) {
      // out of sequence early bird header
      this.cache.set(header.number.toNumber(), header)
      Listener.debug(`header#${header.number.toNumber()} out of sequence`)
    } else if (this.last === 0 || header.number.toNumber() === this.last + 1) {
      // in sequence
      this.headers.push(header)
      this.last = header.number.toNumber()
      Listener.debug(`header#${header.number.toNumber()} in sequence`)
      // if it is now a cached header's turn push it
      const cached: Header | undefined = this.cache.get(this.last + 1)
      if (cached) {
        this.headers.push(cached)
        this.cache.delete(this.last + 1)
        this.last = cached.number.toNumber()
        Listener.debug(`pushed cached header#${cached.number.toNumber()}`)
      }
    } else {
      // old header we already got
    }
  }

  async emitRange() {
    Listener.debug('emitting range...')

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
