import { EventEmitter } from "events"
import { ApiPromise, WsProvider } from "@polkadot/api"
import HeaderListener from "./headers"
import createDebug from "debug"

export default class ParachainListener extends EventEmitter {
  static debug = createDebug("parachain-listener")

  api: ApiPromise
  headerListener: HeaderListener
  gatewayId: string
  parachainId: number
  argumentKey: string

  async setup(url: string, gatewayId: string, parachainId: number) {
    this.gatewayId = gatewayId
    this.parachainId = parachainId

    this.api = await ApiPromise.create({
      provider: new WsProvider(url),
    })

    this.headerListener = await new HeaderListener()
    await this.headerListener.setup(url, false)
    this.headerListener.start()
  }

  async finalize(anchorIndex: number) {
    this.headerListener.finalize(anchorIndex)
  }

  async submitHeaderRange(anchorHash: string) {
    ParachainListener.debug("anchorHash:", anchorHash)
    const anchorIndex = await this.findAnchorIndex(anchorHash)
    ParachainListener.debug("AnchorIndex:", anchorIndex)
    let range = this.headerListener.headers.slice(0, anchorIndex + 1).reverse()

    ParachainListener.debug(range)
    const anchorHeader = range.shift()
    ParachainListener.debug("Parachain Anchor:", anchorHeader)

    // we need to pass the anchorIndex, so we can delete these header
    // if everthing was successful
    this.emit("SubmitHeaderRange", {
      gatewayId: this.gatewayId,
      range,
      anchorHeader,
      anchorIndex,
    })
  }

  async findAnchorIndex(anchorHash: string) {
    ParachainListener.debug(`looking for anchor ${anchorHash} in ${this.headerListener.headers}`)
    return this.headerListener.headers.findIndex(h => {
      ParachainListener.debug("header hash", h.hash.toHuman(), "anchor hash", anchorHash)
      return h.hash.toHuman() === anchorHash
    })
  }
}
