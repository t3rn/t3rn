import { EventEmitter } from "events"
import { ApiPromise, WsProvider } from "@polkadot/api"
import HeaderListener from "./headers"
import createDebug from "debug"

export default class ParachainListener extends EventEmitter {
  static debug = createDebug("parachain-listener")

  api: ApiPromise
  headers: HeaderListener
  gatewayId: string
  parachainId: number
  argumentKey: string

  async setup(url: string, gatewayId: string, parachainId: number) {
    this.gatewayId = gatewayId
    this.parachainId = parachainId

    this.api = await ApiPromise.create({
      provider: new WsProvider(url),
    })

    this.headers = await new HeaderListener()
    await this.headers.setup(url, false)
    this.headers.start()
  }

  async finalize(anchorIndex: number) {
    this.headers.finalize(anchorIndex)
  }

  async submitHeaderRange(anchorHash: string) {
    ParachainListener.debug("anchorHash:", anchorHash)
    const anchorIndex = await this.findAnchorIndex(anchorHash)
    ParachainListener.debug("AnchorIndex:", anchorIndex)
    let range = this.headers.headers.slice(0, anchorIndex + 1).reverse()

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
    return this.headers.headers.findIndex(h => h.hash.toHuman() === anchorHash)
  }
}
