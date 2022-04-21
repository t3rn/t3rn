import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { HeaderExtended } from '@polkadot/api-derive/types'
import { grandpaDecode } from './../util'
import createDebug from 'debug'
import 'dotenv/config'

export default class Listener extends EventEmitter {
    static debug = createDebug('listener')

    api: ApiPromise
    endpoint: string = process.env.RELAY_ENDPOINT as string
    rangeSize: number = Number(process.env.RANGE_SIZE)
    gatewayId: Buffer = Buffer.from(process.env.GATEWAY_ID as string, 'utf8')
    headers: HeaderExtended[] = []
    // offset in this.headers for the current range batch
    offset: number = 0
    // last known grandpa set id
    grandpaSetId: number = 0
    // last emitted anchor
    anchor: number = 0
    // bike shed mutex
    busy: boolean = false
    unsubNewHeads: () => void

    async init() {
        this.api = await ApiPromise.create({
            provider: new WsProvider(this.endpoint),
        })

        this.unsubNewHeads = await this.api.derive.chain.subscribeNewHeads(
            async header => {
                await this.handleGrandpaSet()

                await this.handleHeader(header)

                if (this.headers.length > 0 && !this.busy && this.headers.length % this.rangeSize == 0) {
                    await this.concludeRange()
                }
            }
        )
    }

    removeAddedHeaders(index: number) {
        this.headers = this.headers.splice(index);
    }

    async handleGrandpaSet() {
        const currentSetId = await this.api.query.grandpa
            .currentSetId()
            .then(id => Number(id.toJSON()))

        if (this.grandpaSetId !== 0 && currentSetId !== this.grandpaSetId) {
            Listener.debug('grandpa set change', this.grandpaSetId, currentSetId)
            await this.concludeRange()
        }

        this.grandpaSetId = currentSetId
    }

    async handleHeader(header: HeaderExtended) {
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
        this.busy = true
        Listener.debug('concluding range...')
        const unsubJustifications =
            await this.api.rpc.grandpa.subscribeJustifications(
                async justification => {
                    unsubJustifications()

                    const { blockNumber } = await grandpaDecode(justification)
                    
                    const anchorIndex = this.headers.findIndex(
                        h => h.number.toNumber() === blockNumber
                    )

                    const reversedRange = this.headers
                    .slice(0, anchorIndex + 1)
                    .reverse()

                    const anchor = reversedRange.shift() as HeaderExtended

                    Listener.debug(
                        'anchor',
                        anchor.number.toNumber(),
                        'reversed range',
                        reversedRange.map(h => h.number.toNumber()),
                        'range size',
                        reversedRange.length
                    )

                    this.emit(
                        'range',
                        this.gatewayId,
                        anchor,
                        reversedRange,
                        justification,
                        anchorIndex
                    )

                    this.busy = false
                }
            )
    }

    kill() {
        Listener.debug('kill')
        this.unsubNewHeads()
    }
}
