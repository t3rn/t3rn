import { log } from "@/utils/log.ts"
import "@t3rn/types"
import ora from "ora"
import { colorLogMsg } from "@/utils/log.ts"
import { createCircuitContext } from "@/utils/circuit.ts"

export const spinner = ora()

export const handleResetGatewayCommand = async (
  arg: string,
  options: { [key: string]: any },
) => {
  log("INFO", `Resetting ${arg} gateway...`)

  if (!arg) {
    log("ERROR", "No gateway provided!")
    process.exit(1)
  }

  if (arg != "pdot" && arg != "kusm" && arg != "roco") {
    log("ERROR", "Gateway must be pdot, kusm or roco!")
    process.exit(1)
  }

  const { circuit, sdk, endpoint, signer } = await createCircuitContext()

  if (!["ws://localhost:9944", "ws://0.0.0.0:9944", "ws://127.0.0.1:9944"].includes(endpoint) && !options.force) {
    log(
      "ERROR",
      `Circuit endpoint is not localhost:9944. We don't want to reset live gateway! Aborting.`,
    )
    process.exit(1)
  }

  // TODO: check if gateway is already reset

  spinner.start()
  try {
    if (arg == "pdot") {
      await sdk.circuit.tx.signAndSendSafe(
        sdk.circuit.tx.createSudo(circuit.tx.polkadotBridge.reset()),
      )
    } else if (arg == "kusm") {
      await sdk.circuit.tx.signAndSendSafe(
        sdk.circuit.tx.createSudo(circuit.tx.kusamaBridge.reset()),
      )
    } else if (arg == "roco") {
      await sdk.circuit.tx.signAndSendSafe(
        sdk.circuit.tx.createSudo(circuit.tx.rococoBridge.reset()),
      )
    }

    spinner.succeed(colorLogMsg("SUCCESS", `Gateway has been reset`))
    spinner.stopAndPersist({
      symbol: "🎉",
      text: colorLogMsg("SUCCESS", `Gateway has been reset`),
    })
    spinner.stop()

    process.exit(0)
  } catch (error) {
    spinner.fail(colorLogMsg("ERROR", `Gateway reset failed! ${error}`))
    process.exit(1)
  }
}
