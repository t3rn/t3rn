import "@t3rn/types"
import { EventEmitter } from "events"
import { TextDecoder } from "util"
import { ApiPromise } from "@polkadot/api"
import { Execution } from "../executionManager/execution"
import createDebug from "debug"

export enum ListenerEvents {
    NewSideEffectsAvailable,
    SFXNewBidReceived,
    XTransactionReadyForExec,
    HeaderSubmitted,
    SideEffectConfirmed,
    XtxCompleted,
    DroppedAtBidding,
    RevertTimedOut,
}

export type ListenerEventData = {
    type: ListenerEvents
    data: Execution | any
}

export class CircuitListener extends EventEmitter {
    static debug = createDebug("circuit-listener")

    client: ApiPromise

    constructor(client: ApiPromise) {
        super()
        this.client = client
    }

    async start() {
        this.client.query.system.events((notifications) => {
            for (let i = 0; i < notifications.length; i++) {
                if (
                    notifications[i].event.method === "NewSideEffectsAvailable"
                ) {
                    // receives new side effects
                    this.emit("Event", <ListenerEventData>{
                        type: ListenerEvents.NewSideEffectsAvailable,
                        data: notifications[i].event.data,
                    })
                } else if (
                    notifications[i].event.method === "SFXNewBidReceived"
                ) {
                    this.emit("Event", <ListenerEventData>{
                        type: ListenerEvents.SFXNewBidReceived,
                        data: notifications[i].event.data,
                    })
                } else if (
                    notifications[i].event.method === "XTransactionReadyForExec"
                ) {
                    this.emit("Event", <ListenerEventData>{
                        type: ListenerEvents.XTransactionReadyForExec,
                        data: notifications[i].event.data,
                    })
                } else if (
                    notifications[i].event.method === "HeaderSubmitted"
                ) {
                    const data = {
                        gatewayId: new TextDecoder().decode(
                            notifications[i].event.data[0].toU8a()
                        ),
                        height: parseInt(
                            notifications[i].event.data[1].toString(),
                            16
                        ),
                    }
                    this.emit("Event", <ListenerEventData>{
                        type: ListenerEvents.HeaderSubmitted,
                        data,
                    })
                } else if (
                    notifications[i].event.method === "SideEffectConfirmed"
                ) {
                    this.emit("Event", <ListenerEventData>{
                        type: ListenerEvents.SideEffectConfirmed,
                        data: notifications[i].event.data,
                    })
                } else if (
                    notifications[i].event.method ===
                    "XTransactionXtxFinishedExecAllSteps"
                ) {
                    this.emit("Event", <ListenerEventData>{
                        type: ListenerEvents.XtxCompleted,
                        data: notifications[i].event.data,
                    })
                } else if (
                    notifications[i].event.method ===
                    "XTransactionXtxDroppedAtBidding"
                ) {
                    this.emit("Event", <ListenerEventData>{
                        type: ListenerEvents.DroppedAtBidding,
                        data: notifications[i].event.data,
                    })
                } else if (
                    notifications[i].event.method ===
                    "XTransactionXtxRevertedAfterTimeOut"
                ) {
                    this.emit("Event", <ListenerEventData>{
                        type: ListenerEvents.RevertTimedOut,
                        data: notifications[i].event.data,
                    })
                }
            }
        })
    }
}
