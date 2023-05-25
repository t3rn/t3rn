import { getConfig } from "@/utils/config.ts"
import { colorLogMsg } from "@/utils/log.ts"
import ora from "ora"
import { ErrorListener, ErrorMode, processSfx } from "@/utils/dgf/creation.ts"
import { ApiPromise, WsProvider } from "@t3rn/sdk"
import { Extrinsic } from "@/schemas/extrinsic.ts"
import { submitSfxRaw } from "@/commands/submit/sfx.ts"

const spinner = ora()

export const handleDgfCmd = async () => {
    const config = getConfig()
    if (!config) {
        process.exit(1)
    }

    spinner.start(colorLogMsg("INFO", 'Starting the unhappy path generation process'))

    await batchErrorCreation()

    spinner.succeed("Data generated for unhappy paths!")
}


const batchErrorCreation = async () => {
    // spin up event listener
    const provider = new WsProvider('ws://127.0.0.1:9944')  // ws.t0rn.io
    const api = await ApiPromise.create({ provider })
    const listener = new ErrorListener(api)
    listener.prependListener('event', () => spinner.info("Event listener is starting..."))
    listener.start()
    listener.on('event', (eventData) => {
        spinner.info(`🆕 new event data received: ${eventData}`)
    })
    spinner.succeed("Event listener started!")

    const submitedRegistry = new Map<ErrorMode, Extrinsic>()

    spinner.info("Starting to process SFXs...")

    for (const errorMode in ErrorMode) {
        const file_name = './transfer.json'
        spinner.info(`-- Processing error mode: ${errorMode}`)

        const extrinsic = processSfx(file_name, ErrorMode[errorMode])
        spinner.succeed(`---- Processed error mode ${errorMode}!`)

        spinner.info("-- Submitting processed SFX...")
        await submitSfxRaw(extrinsic, true)
        spinner.succeed(`---- SFX submitted!`)
        submitedRegistry.set(ErrorMode[errorMode], extrinsic)
    }
}