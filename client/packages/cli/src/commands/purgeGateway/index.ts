import { Args } from "@/types.ts"
import { log } from "@/utils/log.ts"
import "@t3rn/types"
import ora from "ora"
import { colorLogMsg } from "@/utils/log.ts"
import { createCircuitContext } from "@/utils/circuit.ts"

export const spinner = ora()

export const handlePurgeGatewayCommand = async (
  args: Args<"gateway" | "export">,
  options: { [key: string]: any },
) => {
  log("INFO", `Purging ${args} gateway...`)

  if (!args) {
    log("ERROR", "No vendor provided!")
    process.exit(1)
  }

  const { circuit, sdk, endpoint, signer } = await createCircuitContext()

  if (endpoint != "ws://localhost:9944" && !options.force) {
    log(
      "ERROR",
      `Circuit endpoint is not localhost:9944. We don't want to purge live gateway! Aborting.`,
    )
    process.exit(1)
  }

  spinner.start()
  try {
    await sdk.circuit.tx.signAndSendSafe(
      sdk.circuit.tx.createSudo(
        circuit.tx.xdns.purgeGatewayRecord(signer.address, args),
      ),
    )

    spinner.succeed(colorLogMsg("SUCCESS", `Gateway purged`))
    spinner.stopAndPersist({
      symbol: "🎉",
      text: colorLogMsg("SUCCESS", `Gateway purged`),
    })
    spinner.stop()

    process.exit(0)
  } catch (error) {
    spinner.fail(colorLogMsg("ERROR", `Gateway purge failed! ${error}`))
    process.exit(1)
  }
}
