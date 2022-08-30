import "@t3rn/types"
import { EventEmitter } from "events"
import { TextDecoder } from "util"
import{ ApiPromise, WsProvider }from '@polkadot/api';
import { H256 } from '@polkadot/types/interfaces';
import { Execution } from "../utils/execution"
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
                    let xtxId = notification.event.data[0].toHex();
                    this.emit("XTransactionReadyForExec", xtxId)
                } else if (notification.event.method === "SideEffectInsuranceReceived") {
                    let sfxId = notification.event.data[0].toHex();
                    this.emit("SideEffectInsuranceReceived", sfxId)
                } else if (notification.event.method === "SideEffectConfirmed") {
                    let sfxId = notification.event.data[0].toHex();
                    this.emit("SideEffectConfirmed", sfxId)
                } else if (notification.event.method === "NewSideEffectsAvailable") {
                    const execution = new Execution(notification.event.data)
                    this.emit("NewExecution", execution)
                } else if (notification.event.method === "HeaderSubmitted") {
                    const data = {
                        gatewayId: new TextDecoder().decode(
                            notification.event.data[0].toU8a()
                        ),
                        height: parseInt(notification.event.data[1].toString(), 16)
                    }

                    this.emit("NewHeaderRangeAvailable", data)
                }
            })
        })
    }
}
