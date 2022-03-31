import { EventEmitter } from 'events'
import { promisify } from 'util'
import { exec as _exec } from 'child_process'
import { tmpdir } from 'os'
import { join } from 'path'
import { writeFile } from 'fs/promises'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { JustificationNotification, Header } from '@polkadot/types/interfaces'
import { sleep } from './util'
import createDebug from 'debug'
import 'dotenv/config'

const exec = promisify(_exec)

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

        if (this.headers.length - this.offset >= this.rangeSize) {
          await this.concludeRange()
        }
      }
    )
  }

  async handleGrandpaSet() {
    const currentSetId: number = Number(
      await this.kusama.query.grandpa.currentSetId().then(id => id.toJSON())
    )

    if (this.grandpaSetId !== 0 && currentSetId !== this.grandpaSetId) {
      Listener.debug('grandpa set change', this.grandpaSetId, currentSetId)
      await this.concludeRange()
    }

    this.grandpaSetId = currentSetId
  }

  async handleHeader(header: Header) {
    let attempts: number = 10

    while (this.last !== 0 && header.number.toNumber() > this.last + 1) {
      if (attempts-- <= 0) {
        throw Error(`cannot fetch block#${this.last + 1}`)
      }

      let missingHeader: Header | void

      try {
        missingHeader = await this.kusama.rpc.chain.getHeader(
          await this.kusama.rpc.chain.getBlockHash(this.last + 1)
        )
      } catch (_) {
        await sleep(6000)
      } finally {
        if (
          missingHeader &&
          missingHeader.number.toNumber() === this.last + 1
        ) {
          this.headers.push(missingHeader)
          this.last = missingHeader.number.toNumber()
          Listener.debug(`#${this.last}`)
        }
      }
    }

    this.headers.push(header)
    this.last = header.number.toNumber()
    Listener.debug(`#${this.last}`)
  }

  async concludeRange() {
    Listener.debug('concluding range...')
    const unsubJustifications =
      await this.kusama.rpc.grandpa.subscribeJustifications(
        async (justification: JustificationNotification) => {
          unsubJustifications()

          const tmpJustificationFile: string = join(
            tmpdir(),
            justification.toString().slice(0, 10)
          )

          await writeFile(tmpJustificationFile, justification.toString())

          const justificationBlockNumber: number = await exec(
            './justification-decoder/target/release/justification-decoder ' +
              tmpJustificationFile
          ).then(cmd => parseInt(cmd.stdout))

          Listener.debug('jus blk num', justificationBlockNumber)

          const justifiedHeaderIndex: number = this.headers.findIndex(
            h => h.number.toNumber() === justificationBlockNumber
          )

          const reversedRange: Header[] = this.headers
            .slice(this.offset, justifiedHeaderIndex + 1)
            .reverse()

          this.offset = justifiedHeaderIndex + 1

          const anchor: Header = reversedRange.shift() as Header

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
