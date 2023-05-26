import { readSfxFile } from "@/commands/submit/sfx.ts"
import { validate } from "../fns.ts"
import { ExtrinsicSchema, Extrinsic } from "@/schemas/extrinsic.ts"
import "@t3rn/types"
import { EventEmitter } from "events"
import { ApiPromise } from "@polkadot/api"
import ora from "ora"

const spinner = ora()

export enum ErrorMode {
    NoBidders = "NoBidders",
    ConfirmationTimeout = "ConfirmationTimeout",
    InvalidProof = "InvalidProof",
    InvalidExecutionValidProof = "InvalidExecutionValidProof",
    None = "None",
}


/**
 * Process the get, injection and save of the SFX.
 * 
 * @param sfxFile file containing the SFX
 * @param errorMode the type of error to be injected
 */
export const processSfx = (sfxFile: string, errorMode: ErrorMode) => {
    const extrinsic = getExtrinsic(sfxFile)
    injectErrorMode(extrinsic, errorMode)
    return extrinsic
}


/**
 * Get the SFX and validate it.
 * 
 * @param sfxFile file to read the extrinsic from
 * @returns the validated extrinsic
 */
const getExtrinsic = (sfxFile: string) => {
    const unvalidatedExtrinsic = readSfxFile(sfxFile)
    const extrinsic: Extrinsic = validate(ExtrinsicSchema, unvalidatedExtrinsic, {
        configFileName: sfxFile,
    })
    return extrinsic
}


/**
 * Modify the SFX by injecting the error mode in the signature field.
 * 
 * What is accepted is a transaction args object, which contains the
 * side effect and the speed mode.
 * 
 * @param sfx 
 * @param errorMode 
 */
const injectErrorMode = (extrinsic: Extrinsic, errorMode: ErrorMode) => {
    extrinsic.sideEffects[0].signature = errorMode
}


/**
 * Enum for the different types of events emitted by the relayer
 *
 * @group t3rn Circuit
 */
export enum ListenerEvents {
    /** A new XTX was detected on Circuit */
    NewSideEffectsAvailable,
    /** A new SFX bid was detected */
    SFXNewBidReceived,
    /** An XTX is ready to be executed */
    XTransactionReadyForExec,
    /** New headers where detected for a specific gateway */
    HeaderSubmitted,
    /** A SFX was confirmed on circuit */
    SideEffectConfirmed,
    /** A XTX was finalized */
    XtxCompleted,
    /** A XTX was dropped at bidding */
    DroppedAtBidding,
    /** A XTX was reverted */
    RevertTimedOut,
    /** Event not recognized */
    NotRecognized,
}


/**
 * Type for transporting events
 *
 * @group t3rn Circuit
 */
type ListenerEventData = {
    type: ListenerEvents;
    data: {
        vendor: string,
        height: number
        error: ErrorMode,
    };
};


/**
 * (another custom) Event listener for the relayer
 * 
 * @group t3rn Circuit
 */
export class ErrorListener extends EventEmitter {
    client: ApiPromise

    constructor(client: ApiPromise) {
        super()
        this.client = client
    }

    async start() {
        this.client.query.system.events((notifications) => {
            notifications.forEach((notification) => {
                switch (notification.event.method) {
                    case "NewSideEffectsAvailable": {
                        emitEvent(this, ListenerEvents.NewSideEffectsAvailable, notification)
                        break
                    }
                    case "SFXNewBidReceived": {
                        emitEvent(this, ListenerEvents.SFXNewBidReceived, notification)
                        break
                    }
                    case "XTransactionReadyForExec": {
                        emitEvent(this, ListenerEvents.XTransactionReadyForExec, notification)
                        break
                    }
                    case "HeadersAdded": {
                        console.log("🎶 Emiting event: ", notification.toHuman())
                        let vendor
                        if (notification.event.section === "rococoBridge") {
                            vendor = "Rococo"
                        }
                        const data = {
                            vendor,
                            height: parseInt(notification.event.data[0].toString()),
                        }
                        this.emit("Event", <ListenerEventData>{
                            type: ListenerEvents.HeaderSubmitted,
                            data,
                        })
                        // return ListenerEvents.HeaderSubmitted
                        break
                    }
                    case "SideEffectConfirmed": {
                        emitEvent(this, ListenerEvents.SideEffectConfirmed, notification)
                        break
                    }
                    case "XTransactionXtxFinishedExecAllSteps": {
                        emitEvent(this, ListenerEvents.XtxCompleted, notification)
                        break
                    }
                    case "XTransactionXtxDroppedAtBidding": {
                        emitEvent(this, ListenerEvents.DroppedAtBidding, notification, ErrorMode.NoBidders)
                        // return ErrorMode.NoBidders
                        break
                    }
                    case "XTransactionXtxRevertedAfterTimeOut": {
                        emitEvent(this, ListenerEvents.RevertTimedOut, notification, ErrorMode.ConfirmationTimeout)
                        // return ErrorMode.ConfirmationTimeout
                        break
                    }
                    default: {
                        // console.log("Did not recognise the event. Skipping")
                        return ListenerEvents.NotRecognized
                    }
                }
            })
        })
    }
}


/**
 * Easy emit an event with a listener, log it and return the type of event.
 * 
 * @param listener Instance of the listener
 * @param event What has to be emited
 * @param notification The type of notification emited
 * @returns The emited event to return it
 */
const emitEvent = (
    listener: ErrorListener,
    event: ListenerEvents,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    notification: any,
    error = ErrorMode.None
) => {
    listener.emit("event", <ListenerEventData>{
        type: event,
        data: notification.event.data,
        error: error,
    })
}