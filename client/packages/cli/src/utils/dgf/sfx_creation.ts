import { readSfxFile, submitSfx, submitSfxRaw } from "@/commands/submit/sfx.ts"
import { validate } from "../fns.js"
import { ExtrinsicSchema, Extrinsic } from "@/schemas/extrinsic.ts"
import * as fs from "fs"
import "@t3rn/types"
import { EventEmitter } from "events"
import { ApiPromise, WsProvider } from "@polkadot/api"

export enum ErrorMode {
    NoBidders = "NoBidders",
    ConfirmationTimeout = "ConfirmationTimeout",
    InvalidProof = "InvalidProof",
    InvalidExecutionValidProof = "InvalidExecutionValidProof",
}


/**
 * Process the get, injection and save of the SFX.
 * 
 * @param args arguments passed to the CLI
 * @param sfxFile file containing the SFX
 */
const processSfx = async (sfxFile: string, errorMode: ErrorMode) => {
    // Get the extrinsic from the file
    const extrinsic = getExtrinsic(sfxFile)

    // Attach the error on the SFX
    injectErrorMode(extrinsic, errorMode)

    // send the extrinsic to the circuit
    submitSfxRaw(extrinsic, true) // if nothing is returned so, how do I check it works?

    // Check w/ event listener
    const provider = new WsProvider('ws://ws.t0rn.io')
    const api = await ApiPromise.create({ provider })
    const listener = new ErrorListener(api, errorMode)
    listener.start()

    // TODO: Save the SFX as a json file if everything goes well
    saveToJson(extrinsic, errorMode)
    // TODO: check that the file that's loaded is the bidding one? i dunno
}

/**
 * Get the SFX and validate it.
 * 
 * @param sfxFile file to read the extrinsic from
 * @returns the validated extrinsic
 */
const getExtrinsic = (sfxFile: string) => {
    // Read from file the extrinsic
    const unvalidatedExtrinsic = readSfxFile(sfxFile)
    maybeExit(unvalidatedExtrinsic)

    // Validate it
    const extrinsic: Extrinsic = validate(ExtrinsicSchema, unvalidatedExtrinsic, {
        configFileName: sfxFile,
    })
    maybeExit(extrinsic)

    return extrinsic
}


/**
 * Modify the SFX buy injecting the error mode in the signature field.
 * 
 * What is accepted is a transaction args object, which contains the
 * side effect and the speed mode.
 * 
 * @param sfx 
 * @param errorMode 
 */
const injectErrorMode = (extrinsic: Extrinsic, errorMode: ErrorMode) => {
    // too simple that you'd want to kill me
    extrinsic.sideEffects.signature = errorMode
    console.log("✅ Succesfully injected the error in the SFX!")
}


/**
 * Check if `value` exists or not, and exit the process accordingly.
 * 
 * @param value anything that can be checked if exists or not
 */
const maybeExit = (value: any) => {
    value ? process.exit(0) : process.exit(1)
}

/**
 * Save the extrinsic as a json file.
 * The default location is `./sfx_with_error_modes`.
 * 
 * @param sfx The extrinsic to be saved
 * @param folder The place to store the extrinsic
 */
const saveToJson = (sfx: Extrinsic, errorMode: ErrorMode, folder = "./sfx_with_error_modes") => {
    if (!fs.existsSync(folder)) {
        fs.mkdirSync(folder)
    }
    fs.writeFileSync(`${folder}/sfx_${errorMode}.json`, JSON.stringify(sfx))
    console.log("✅ Succesfully saved the SFX as a json file!")
}


/**
 * Batch-create all the SFX with all the possible different errors.
 * 
 */
export const batchErrorModes = () => {
    for (const errorMode in ErrorMode) {
        const file_name = `./sfx.json`
        processSfx(file_name, ErrorMode[errorMode])
    }
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
export type ListenerEventData = {
    type: ListenerEvents;
    data: {
        vendor: string,
        height: number
    };
};

/**
 * (another custom) Event listener for the relayer
 * 
 * @group t3rn Circuit
 */
class ErrorListener extends EventEmitter {
    client: ApiPromise
    error: ErrorMode

    constructor(client: ApiPromise, error: ErrorMode) {
        super()
        this.client = client
        this.error = error
    }

    async start() {
        this.client.query.system.events((notifications) => {
            notifications.forEach((notification) => {
                switch (notification.event.method) {
                    case "NewSideEffectsAvailable": {
                        const e = emitEvent(this, ListenerEvents.NewSideEffectsAvailable, notification)
                        return e
                    }
                    case "SFXNewBidReceived": {
                        const e = emitEvent(this, ListenerEvents.SFXNewBidReceived, notification)
                        return e
                    }
                    case "XTransactionReadyForExec": {
                        const e = emitEvent(this, ListenerEvents.XTransactionReadyForExec, notification)
                        return e
                    }
                    case "HeadersAdded": {
                        console.log(notification.toHuman())
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
                        return ListenerEvents.HeaderSubmitted
                    }
                    case "SideEffectConfirmed": {
                        const e = emitEvent(this, ListenerEvents.SideEffectConfirmed, notification)
                        return e
                    }
                    case "XTransactionXtxFinishedExecAllSteps": {
                        const e = emitEvent(this, ListenerEvents.XtxCompleted, notification)
                        return e
                    }
                    case "XTransactionXtxDroppedAtBidding": {
                        const e = emitEvent(this, ListenerEvents.DroppedAtBidding, notification)
                        return e

                    }
                    case "XTransactionXtxRevertedAfterTimeOut": {
                        const e = emitEvent(this, ListenerEvents.RevertTimedOut, notification)
                        return e
                    }
                    default: {
                        console.log("Did not recognise the event. Skipping")
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
const emitEvent = (listener: ErrorListener, event: ListenerEvents, notification: any) => {
    console.log("Emiting event: ", event)
    listener.emit("Event", <ListenerEventData>{
        type: event,
        data: notification.event.data,
    })
    return event
}