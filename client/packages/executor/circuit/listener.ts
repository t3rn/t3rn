import "@t3rn/types"
import { EventEmitter } from "events"
import { TextDecoder } from "util"
import{ ApiPromise, WsProvider }from '@polkadot/api';
import { H256 } from '@polkadot/types/interfaces';
import { Execution } from "../executionManager/execution"
import createDebug from "debug"

export enum Events {
    NewSideEffectsAvailable,
    SFXNewBidReceived
}

export type EventData = {
    type: Events,
    data: Execution | any
}

export class CircuitListener extends EventEmitter {
    static debug = createDebug("circuit-listener")

    client: ApiPromise

    constructor(client: ApiPromise) {
        super()
        this.client = client;
    }

    async start() {
        this.client.query.system.events(notifications => {
            for(let i = 0; i < notifications.length; i++) {
                if (notifications[i].event.method === "NewSideEffectsAvailable") { // receives new side effects
                    this.emit(
                        "Event",
                        <EventData>{
                            type: Events.NewSideEffectsAvailable,
                            data: notifications[i].event.data
                        }
                    )
                } else if(notifications[i].event.method === "SFXNewBidReceived") {
                    this.emit(
                        "Event",
                        <EventData>{
                            type: Events.SFXNewBidReceived,
                            data: notifications[i].event.data
                        }
                    )
                }
            }
            // return notifications.forEach(notification => {
            //
            //     // else if (notification.event.method === "XTransactionReadyForExec") {
            //     //     let xtxId = notification.event.data[0].toHex();
            //     //     this.emit("XTransactionReadyForExec", xtxId)
            //     // } else if (notification.event.method === "SideEffectInsuranceReceived") {
            //     //     let sfxId = notification.event.data[0].toHex();
            //     //     let executor = notification.event.data[1];
            //     //     this.emit("SideEffectInsuranceReceived", sfxId, executor)
            //     // } else if (notification.event.method === "SideEffectConfirmed") {
            //     //     let sfxId = notification.event.data[0].toHex();
            //     //     this.emit("SideEffectConfirmed", sfxId)
            //     // } else if (notification.event.method === "XTransactionXtxFinishedExecAllSteps") {
            //     //     const xtxId = notification.event.data[0].toHex();
            //     //     this.emit("ExecutionComplete", xtxId)
            //     // } else if (notification.event.method === "HeaderSubmitted") {
            //     //     const data = {
            //     //         gatewayId: new TextDecoder().decode(
            //     //             notification.event.data[0].toU8a()
            //     //         ),
            //     //         height: parseInt(notification.event.data[1].toString(), 16)
            //     //     }
            //     //
            //     //     this.emit("NewHeaderRangeAvailable", data)
            //     // }
            // })
        })
    }
}
