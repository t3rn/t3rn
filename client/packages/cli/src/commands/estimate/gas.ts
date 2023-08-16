import ora from "ora"
import { Action, Target, estimateGasFee } from "@t3rn/sdk/price-estimation"
import { colorLogMsg } from "@/utils/log.ts"
import { Args, WithRequiredProperty } from "@/types.ts"
import { getEstimationArgs } from "./args.ts"

export const spinner = ora()

type CommandArgs = Args<"target" | "action" | "args" | "sfx" | "signer" | "export">

export const handleEstimateGasFee = async (args: CommandArgs) => {
  spinner.text = "Estimating..."
  spinner.start()
  const estimateArgs = await getEstimationArgs(
    args as WithRequiredProperty<CommandArgs, "target" | "action">,
  )
  try {
    const estimate = await estimateGasFee({
      action: args.action as Action,
      target: args.target as Target,
      args: estimateArgs,
    })
    console.log("\n")
    console.table(estimate)
    spinner.stop()
    process.exit(0)
  } catch (e) {
    spinner.fail(colorLogMsg("ERROR", e))
    process.exit(1)
  }
}
