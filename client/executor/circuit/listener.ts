// import "@t3rn/types"
import { EventEmitter } from "events"
import { ApiPromise, WsProvider } from "@polkadot/api"
import { SideEffect } from "../utils/types"
import { TextDecoder } from "util"
import createDebug from "debug"

export default class CircuitListener extends EventEmitter {
  static debug = createDebug("circuit-listener")

  api: ApiPromise

  async setup(rpc: string) {
    this.api = await ApiPromise.create({
      provider: new WsProvider(rpc),
    })
  }

  async start() {
    this.api.query.system.events(notifications => {
      notifications.forEach(notification => {
        if (notification.event.method === "XTransactionReadyForExec") {
          const { event } = notification
          const types = event.typeDef
          let xtxId
          for (let index = 0; index < event.data.length; index++) {
            switch (types[index].type) {
              case "H256":
                xtxId = event.data[index]
                break
            }
          }

          this.emit("XTransactionReadyForExec", xtxId)
        } else if (notification.event.method === "NewSideEffectsAvailable") {
          const { event } = notification
          const types = event.typeDef
          let allSideEffects: SideEffect[] = []
          let sideEffect = new SideEffect()
          for (let index = 0; index < event.data.length; index++) {
            switch (types[index].type) {
              case "AccountId32":
                sideEffect.setRequester(event.data[index])
                break
              case "H256":
                sideEffect.setXtxId(event.data[index])
                break
              case "Vec<T3rnTypesSideEffect>":
                ;(event.data[index] as any).forEach(element => {
                  let newSideEffect = new SideEffect()
                  newSideEffect.setSideEffect(element)
                  newSideEffect.setXtxId(sideEffect.xtxId)
                  newSideEffect.setRequester(sideEffect.requester)
                  allSideEffects.push(newSideEffect)
                })
                break
              case "Vec<H256>":
                ;(event.data[index] as any).forEach((element, cnt) => {
                  allSideEffects[cnt].setId(element)
                })
                break
            }
          }

          CircuitListener.debug(
            "saved up all_side_effects before emitting NewSideEffects",
            ...allSideEffects.map(sideEffect => sideEffect.getId())
          )

          this.emit("NewSideEffects", allSideEffects)
        } else if (notification.event.method === "NewHeaderRangeAvailable") {
          const data = {
            gatewayId: new TextDecoder().decode(
              notification.event.data[0].toU8a()
            ),
            height: notification.event.data[1],
            range: notification.event.data[2],
          }

          this.emit("NewHeaderRangeAvailable", data)
        }
      })
    })
  }
}
