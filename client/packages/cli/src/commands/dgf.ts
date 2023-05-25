import { getConfig } from "@/utils/config.ts"
import { colorLogMsg } from "@/utils/log.ts"
import ora from "ora"
import { ErrorListener, ErrorMode, processSfx } from "@/utils/dgf/creation.ts"
import { ApiPromise, WsProvider } from "@t3rn/sdk"
import { Extrinsic } from "@/schemas/extrinsic.ts"
import { submitSfxRaw } from "@/commands/submit/sfx.ts"

const spinner = ora()

export const handleDgfCmd = async (
    // sfxId: string,
    // options: Args<"export">
) => {
    const config = getConfig()
    if (!config) {
        process.exit(1)
    }

    // const { circuit, sdk } = await createCircuitContext(Boolean(options.export))

    spinner.start(colorLogMsg("INFO", 'Generating data for unhappy paths...'))

    await batchErrorCreation()

    spinner.text = "Data generated for unhappy paths!"
}




const batchErrorCreation = async () => {
    // spin up event listener
    const provider = new WsProvider('ws://127.0.0.1:9944')  // ws.t0rn.io
    const api = await ApiPromise.create({ provider })
    const listener = new ErrorListener(api)
    listener.prependListener('event', () => console.log('Event listener is starting...'))
    listener.start()

    spinner.text = "Event listener started!"
    spinner.start()

    // listen on "event"s that can be emited
    listener.on('event', (eventData) => {
        // validateEvent(eventData)
        console.log("🆕 new event data received: ", eventData)
    })

    const submitableRegistry = new Map<ErrorMode, Extrinsic>()
    const submitedRegistry = new Map<ErrorMode, Extrinsic>()

    spinner.text = "Starting to process SFX..."

    for (const errorMode in ErrorMode) {
        // const file_name = `./sfx.json`
        const file_name = `./transfer.json`
        const extrinsic = await processSfx(file_name, ErrorMode[errorMode])

        spinner.text = `SFX processed for error mode ${errorMode}!`

        // registry what could be submitted
        submitableRegistry.set(ErrorMode[errorMode], extrinsic)

        // submit the extrinsic to the circuit
        try {
            const events = submitSfxRaw(extrinsic, true)
            console.log("Received events after submission: ", events)

            spinner.stopAndPersist({
                symbol: "🚀",
                text: colorLogMsg(
                    "SUCCESS",
                    `SFX submitted, event received ${events}`
                ),
            })

            // TODO: check if, in the events returned, there's some info from the submited extrinsic so I can later check if the 

            // register the error with the extrinsic if successfull
            submitedRegistry.set(ErrorMode[errorMode], extrinsic)
        } catch (e) {
            // otherwise, stop everything
            spinner.fail(`❌ Extrinsic submission failed: ${e}`)
            process.exit(1)
        }
    }

    // Save the SFX as a json file if everything goes well
    // saveToJson(extrinsic, errorMode)
}