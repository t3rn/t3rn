import { EventEmitter } from "events"
import { ApiPromise, WsProvider } from "@polkadot/api"
import { Header } from "@polkadot/types/interfaces"
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
    await this.headerListener.start()
  }

  async finalize(anchorNumber: number) {
    this.headerListener.finalize(anchorNumber)
  }
}
