import { EventEmitter } from "events"
import { ApiPromise, WsProvider } from "@polkadot/api"
import createDebug from "debug"
import config from "../../config.json"

export default class HeaderListener extends EventEmitter {
  static debug = createDebug("header-listener")

  api: ApiPromise
  gatewayId: string
  headers: any[] = []
  unsubNewHeads: () => void
  relaychain: boolean
  grandpaSetId: number = 0
  rangeSize: number = parseInt((config as any).rangeSize)

  async setup(url: string, relaychain: boolean) {
    this.api = await new ApiPromise({
      provider: new WsProvider(url),
    })

    await this.api.isReady
    this.relaychain = relaychain
  }

  async start() {
    this.unsubNewHeads = await this.api.derive.chain.subscribeNewHeads(
      async header => {
        this.handleHeader(header)
      }
    )
  }

  async handleHeader(header: any) {
    if (
      this.headers.length === 0 ||
      this.headers[this.headers.length - 1].number.toNumber() + 1 ===
        header.number.toNumber()
    ) {
      this.headers.push(header)
    }

    if (!this.relaychain) {
      HeaderListener.debug(
        "Para-Header",
        header.number.toNumber(),
        "-",
        header.hash.toJSON()
      )
    }

    // this is relaychain specific. Parachains follow the submissions of the
    // relaychain and get notified differently
    if (
      this.relaychain &&
      this.headers.length > 0 &&
      this.headers.length % this.rangeSize === 0
    ) {
      HeaderListener.debug("Range complete at:", header.number.toNumber())
      this.emit("RangeComplete", header.number.toNumber())
    }
  }

  // returns the index of a certain header on this instance
  getHeaderIndex(blockHeight: number) {
    return this.headers.findIndex(
      (h: any) => h.number.toNumber() === blockHeight
    )
  }

  // called after a range submission is completed
  // might need to add a mutex lock here, because if the we run into thw
  // situation that 2 submissions run in parallel this messes stuff up
  finalize(index: number) {
    this.headers = this.headers.splice(index)
  }
}
