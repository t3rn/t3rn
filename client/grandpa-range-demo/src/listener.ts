import { EventEmitter } from 'events'
import { exec as _exec } from 'child_process'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { JustificationNotification, Header } from '@polkadot/types/interfaces'
import { grandpaDecode } from './util'
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
  // last known grandpa set id
  grandpaSetId: number = 0

  unsubNewHeads: () => void

  async init() {
    this.kusama = await ApiPromise.create({
      provider: new WsProvider(this.kusamaEndpoint),
    })

    this.unsubNewHeads = await this.kusama.derive.chain.subscribeNewHeads(
      async header => {
        await this.handleGrandpaSet()

        await this.handleHeader(header)

        if (this.headers.length - this.offset >= this.rangeSize) {
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
    if (
      this.headers.length === 0 ||
      this.headers[this.headers.length - 1].number.toNumber() + 1 ===
        header.number.toNumber()
    ) {
      this.headers.push(header)
      Listener.debug(`#${header.number.toNumber()}`)
    }
  }

  async concludeRange() {
    Listener.debug('concluding range...')
    const unsubJustifications =
      await this.kusama.rpc.grandpa.subscribeJustifications(
        async justification => {
          unsubJustifications()

          const { blockNumber } = await grandpaDecode(justification)

          Listener.debug('decoded block number', blockNumber)

          const justifiedHeaderIndex = this.headers.findIndex(
            h => h.number.toNumber() === blockNumber
          )

          const reversedRange = this.headers
            .slice(this.offset, justifiedHeaderIndex + 1)
            .reverse()

          this.offset = justifiedHeaderIndex + 1

          const anchor = reversedRange.shift() as Header

          this.emit(
            'range',
            this.gatewayId,
            anchor,
            reversedRange,
            justification
          )
        }
      )
  }

  kill() {
    Listener.debug('kill')
    this.unsubNewHeads()
  }
}
