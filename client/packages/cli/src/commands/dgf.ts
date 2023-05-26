import { getConfig } from "@/utils/config.ts"
import { colorLogMsg } from "@/utils/log.ts"
import ora from "ora"
import {
  ErrorListener,
  ErrorMode,
  ListenerEvents,
  processSfx,
} from "@/utils/dgf/creation.ts"
import { ApiPromise, WsProvider } from "@t3rn/sdk"
import { Extrinsic } from "@/schemas/extrinsic.ts"
import { SfxSendType, submitSfx } from "@/commands/submit/sfx.ts"

const spinner = ora()

// FIXME: kind of global because I don't know to do it locally better
const submitedRegistry = new Map<string, Extrinsic>()

export const handleDgfCmd = async () => {
  const config = getConfig()
  if (!config) {
    process.exit(1)
  }

  spinner.start(
    colorLogMsg("INFO", "Starting the unhappy path generation process")
  )

  await batchErrorCreation()

  spinner.succeed("Data generated for unhappy paths!")

  spinner.info("Now waiting for events from the chain...")
}

const batchErrorCreation = async () => {
  // spin up event listener
  const provider = new WsProvider("ws://127.0.0.1:9944") // ws.t0rn.io
  const api = await ApiPromise.create({ provider })
  const listener = new ErrorListener(api)

  listener.on("event", (eventData) => {
    processEvent(eventData)
  })
  spinner.info("Event listener is starting...")
  listener.start()
  spinner.succeed("Event listener started!")

  spinner.info("Starting to process SFXs...")

  for (const errorMode in ErrorMode) {
    if (errorMode === ErrorMode[ErrorMode.None]) {
      continue
    }

    const file_name = "./transfer.json"

    spinner.info(`Processing error mode: ${errorMode}`)
    const extrinsic = processSfx(file_name, ErrorMode[errorMode])
    spinner.succeed(`Processed error mode ${errorMode}!`)

    spinner.info("Submitting processed SFX...")
    const response = await submitSfx(extrinsic, true, SfxSendType.Raw)
    const hash = response[3].event.data[1]
    submitedRegistry.set(hash.toHuman(), extrinsic)
  }

  // console.log("Registry of submitted SFX:")
  // submitedRegistry.forEach((value, key) => {
  //     console.log(value.sideEffects[0].signature, " -- ", key)
  // })
}

/**
 * Cleans the info received only for the events that matter to us right now.
 *
 * @param eventData event emitted by the listener
 */
const processEvent = (eventData: {
  type: ListenerEvents
  data: object
  error: ErrorMode
}) => {
  if (ErrorMode[eventData.error] !== ErrorMode[ErrorMode.None]) {
    spinner.info(`New event data received with:`)
    spinner.info(`\tType: ${ListenerEvents[eventData.type]}`)
    spinner.info(`\tHash of the extrinsic: ${JSON.stringify(eventData.data)}`)
    spinner.info(`\tError mode: ${JSON.stringify(ErrorMode[eventData.error])}`)

    validateExtrinsic(eventData)
  } else {
    spinner.info(`Not interesing event...`)
  }
}

/**
 * Validate event with info in the submitedRegistry.
 * If the event hash is in the registry, compare with the one in the event.
 * that the errorMode is the same as the on in the event.
 * If it matches, the event is valid.
 */
const validateExtrinsic = (eventData: {
  type: ListenerEvents
  data: object
  error: ErrorMode
}) => {
  // The hash of the extrinsic emited
  const hash = eventData.data[0].toHuman()
  // Look for the extrinsic generated with that hash
  const reg = submitedRegistry.get(hash)

  // Check if the error modes match
  if (reg.sideEffects[0].signature === ErrorMode[eventData.error]) {
    spinner.stopAndPersist({
      symbol: "🚩",
      text: colorLogMsg(
        "SUCCESS",
        `Event emited and created have matching error modes: ${ErrorMode[eventData.error]
        }`
      ),
    })
  } else {
    spinner.stopAndPersist({
      symbol: "🚩",
      text: colorLogMsg(
        "ERROR",
        `Event emited and created DO NOT match on the error modes: ${ErrorMode[eventData.error]
        }`
      ),
    })
  }
}
