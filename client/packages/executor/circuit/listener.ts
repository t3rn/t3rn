import "@t3rn/types"
import { EventEmitter } from "events"
import { TextDecoder } from "util"
import{ ApiPromise, WsProvider }from '@polkadot/api';
import { H256 } from '@polkadot/types/interfaces';
import { Execution } from "../executionManager/execution"
import createDebug from "debug"

export default class CircuitListener extends EventEmitter {
    static debug = createDebug("circuit-listener")

    client: ApiPromise

    constructor(client: ApiPromise) {
        super();
        this.client = client;
    }

    async start() {
        this.client.query.system.events(notifications => {
            notifications.forEach(notification => {
                if (notification.event.method === "NewSideEffectsAvailable") { // receives new side effects
                    const execution = new Execution(notification.event.data)
                    this.emit("NewExecution", execution)
                } else if (notification.event.method === "XTransactionReadyForExec") {
                    let xtxId = notification.event.data[0].toHex();
                    this.emit("XTransactionReadyForExec", xtxId)
                } else if (notification.event.method === "SideEffectInsuranceReceived") {
                    let sfxId = notification.event.data[0].toHex();
                    let executor = notification.event.data[1];
                    this.emit("SideEffectInsuranceReceived", sfxId, executor)
                } else if (notification.event.method === "SideEffectConfirmed") {
                    let sfxId = notification.event.data[0].toHex();
                    this.emit("SideEffectConfirmed", sfxId)
                } else if (notification.event.method === "XTransactionXtxFinishedExecAllSteps") {
                    const xtxId = notification.event.data[0].toHex();
                    this.emit("ExecutionComplete", xtxId)
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
