import ora from "ora"
import { ApiPromise, WsProvider } from "@t3rn/sdk"
import { getConfig } from "@/utils/config.ts"
import { colorLogMsg } from "@/utils/log.ts"
import {
  ErrorListener,
  ErrorMode,
  ListenerEvents,
  processSfx,
} from "@/utils/dgf.ts"
import { Extrinsic } from "@/schemas/extrinsic.ts"
import { SfxSendType, submitSfx } from "@/utils/sfx.ts"
import { Args } from "@/types.ts"

const spinner = ora()
const submitedRegistry = new Map<string, Extrinsic>()

export const handleDgfCmd = async (
  args: Args<"sfx" | "timeout" | "export">
) => {
  const config = getConfig()

  if (!config) {
    process.exit(1)
  }

  spinner.start(
    colorLogMsg("INFO", "Starting the unhappy path generation process...")
  )

  try {
    const provider = new WsProvider(getConfig().circuit.ws)
    const api = await ApiPromise.create({ provider })
    const listener = new ErrorListener(api)

    await batchErrorCreation(args.sfx, Boolean(args.export))
    spinner.stopAndPersist({
      symbol: "🎉",
      text: colorLogMsg("SUCCESS", "Data generated for unhappy paths!"),
    })

    const timeout = parseInt(args.timeout) ?? 30
    const start = Date.now()

    listener.on("event", (eventData) => {
      spinner.info(
        colorLogMsg(
          "INFO",
          `Received ${ListenerEvents[eventData.type]} event for processing!`
        )
      )

      processEvent(eventData)

      const secondsLeftTillTimeout = timeout - (Date.now() - start) / 1000
      spinner.start(
        colorLogMsg(
          "INFO",
          `Waiting for events from the chain, ${secondsLeftTillTimeout.toFixed(
            2
          )}s till timeout...`
        )
      )
    })
    listener.start()

    spinner.start(
      `🕑 Waiting for events from the chain, ${timeout}s till timeout...`
    )

    await new Promise((resolve) => setTimeout(resolve, timeout * 1000))

    spinner.stopAndPersist({
      symbol: "🏁",
      text: colorLogMsg("INFO", "Timeout waiting for events from the chain!"),
    })
    process.exit(0)
  } catch (e) {
    spinner.fail(colorLogMsg("ERROR", e.message))
    process.exit(1)
  }
}

const batchErrorCreation = async (filePath: string, exportMode = false) => {
  spinner.stopAndPersist({
    symbol: "🚧",
    text: colorLogMsg(
      "INFO",
      "Generate and submit test SFXs that will result in various errors:"
    ),
  })

  for (const errorMode in ErrorMode) {
    if (errorMode === ErrorMode[ErrorMode.None]) {
      continue
    }

    const extrinsic = processSfx(filePath, ErrorMode[errorMode])

    spinner.stopAndPersist({
      symbol: " •",
      text: colorLogMsg(
        "SUCCESS",
        `🔨 Generated SFX that will result in ${errorMode} error`
      ),
    })
    spinner.start(colorLogMsg("INFO", "📦 Submitting this SFX..."))

    const response = await submitSfx(extrinsic, exportMode, SfxSendType.Raw)
    const hash = response[3].event.data[1]

    submitedRegistry.set(hash.toHuman(), extrinsic)
    spinner.stopAndPersist({
      symbol: " •",
      text: colorLogMsg("SUCCESS", "✨ The SFX was successfully submitted!"),
    })
  }
}

/**
 * Cleans the info received only for the events that matter to us right now.
 *
 * @param event emitted by the listener
 */
const processEvent = (eventData: {
  type: ListenerEvents
  data: object
  error: ErrorMode
}) => {
  if (ErrorMode[eventData.error] !== ErrorMode[ErrorMode.None]) {
    spinner.stopAndPersist({
      symbol: "🛬",
      text:
        "Received an event that is interesting to us:" +
        "\n   Type: " +
        ListenerEvents[eventData.type] +
        "\n   Extrinsic Hash: " +
        JSON.stringify(eventData.data) +
        "\n   Error mode: " +
        JSON.stringify(ErrorMode[eventData.error]),
    })

    validateExtrinsic(eventData)
  } else {
    spinner.warn(colorLogMsg("WARN", "Ignore this event, not interesting!"))
  }
}

/**
 * Validate event with info in the submitedRegistry.
 * Get the event hash and look for the matching one in the registry.
 * If both errors match, the event is valid and can be saved.
 *
 * @param event emitted by the listener
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
      symbol: "✅",
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
        `Event emited and created DO NOT match on the error mode.\n\tError mode received: \t${ErrorMode[eventData.error]
        }\n\tError mode sent: \t${reg.sideEffects[0].signature}`
      ),
    })
  }
}
