import { readSfxFile, submitSfx, submitSfxRaw } from "@/commands/submit/sfx.ts"
import { validate } from "../fns.ts"
import { ExtrinsicSchema, Extrinsic } from "@/schemas/extrinsic.ts"
// import * as fs from "fs"
import "@t3rn/types"
import { EventEmitter } from "events"
import { ApiPromise, WsProvider } from "@polkadot/api"

export enum ErrorMode {
    NoBidders = "NoBidders",
    ConfirmationTimeout = "ConfirmationTimeout",
    InvalidProof = "InvalidProof",
    InvalidExecutionValidProof = "InvalidExecutionValidProof",
    None = "None",
}

type ErrorRegistry = Record<ErrorMode, Extrinsic>

/**
 * Batch-create all the SFX with all the possible different errors.
 * 
 */
export const batchErrorCreation = async () => {
    // spin up event listener
    const provider = new WsProvider('ws://ws.t0rn.io')
    const api = await ApiPromise.create({ provider })
    const listener = new ErrorListener(api)
    listener.prependListener('event', () => console.log('Event listener is starting...'))
    listener.start()

    // listen on "event"s that can be emited
    listener.on('event', (eventData) => {
        // validateEvent(eventData)
        console.log("🆕 new event data received: ", eventData)
    })

    const submitableRegistry = new Map<ErrorMode, Extrinsic>()
    const submitedRegistry = new Map<ErrorMode, Extrinsic>()

    for (const errorMode in ErrorMode) {
        // const file_name = `./sfx.json`
        const file_name = `./transfer.json`
        const extrinsic = await processSfx(file_name, ErrorMode[errorMode])

        // registry what could be submitted
        submitableRegistry.set(ErrorMode[errorMode], extrinsic)

        // submit the extrinsic to the circuit
        try {
            const events = submitSfxRaw(extrinsic, true)
            console.log("Received events after submission: ", events)

            // TODO: check if, in the events returned, there's some info from the submited extrinsic so I can later check if the 

            // register the error with the extrinsic if successfull
            submitedRegistry.set(ErrorMode[errorMode], extrinsic)
        } catch (e) {
            // otherwise, stop everything
            console.log("❌ Error received when submiting extrinsic: ", e)
            process.exit(1)
        }
    }

    // Save the SFX as a json file if everything goes well
    // saveToJson(extrinsic, errorMode)
}

// const validateEvent = (eventData: ListenerEventData) => {
//     console.log("Received event with type: ", eventData.type)
//     console.log("Received event with data.vendor: ", eventData.data.vendor)
//     console.log("Received event with data.height: ", eventData.data.height)

//     switch (eventData.type) {
//         case ListenerEvents.DroppedAtBidding: { }
//         case ListenerEvents.RevertTimedOut: { }
//     }
// }


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

    return extrinsic
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
 * Check if `value` exists or not, and exit the process accordingly.
 * 
 * @param value anything that can be checked if exists or not
 */
const maybeExit = (value: Extrinsic | string) => {
    value ? process.exit(0) : process.exit(1)
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
    // too simple that you'd want to kill me
    // extrinsic.sideEffects.signature = errorMode
    extrinsic.sideEffects[0].signature = errorMode
    console.log("✅ Succesfully injected the error in the SFX!")
}


// /**
//  * Save the extrinsic as a json file.
//  * The default location is `./sfx_with_error_modes`.
//  * 
//  * @param sfx The extrinsic to be saved
//  * @param folder The place to store the extrinsic
//  */
// const saveToJson = (sfx: Extrinsic, errorMode: ErrorMode, folder = "./sfx_with_error_modes") => {
//     if (!fs.existsSync(folder)) {
//         fs.mkdirSync(folder)
//     }
//     fs.writeFileSync(`${folder}/sfx_${errorMode}.json`, JSON.stringify(sfx))
//     console.log("✅ Succesfully saved the SFX as a json file!")
// }



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
class ErrorListener extends EventEmitter {
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
const emitEvent = (listener: ErrorListener, event: ListenerEvents, notification: any, error = ErrorMode.None) => {
    console.log("🎶 Emiting event: ", event)

    listener.emit("event", <ListenerEventData>{
        type: event,
        data: notification.event.data,
        error: error,
    })

    // saveToJson(extrinsic)
    // return event
}