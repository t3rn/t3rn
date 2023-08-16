import ora from "ora"
import {
  Action,
  Target,
  estimateBidAmount,
} from "@t3rn/sdk/price-estimation"
import { colorLogMsg } from "@/utils/log.ts"
import { Args, WithRequiredProperty } from "@/types.ts"
import { getEstimationArgs } from "./args.ts"

export const spinner = ora()

type CommandArgs = Args<
  "target" | "action" | "profitMargin" | "args" | "sfx" | "signer" | "export"
>

export const handleEstimateBidAmount = async (args: CommandArgs) => {
  spinner.text = "Estimating..."
  spinner.start()
  const estimateArgs = await getEstimationArgs(
    args as WithRequiredProperty<CommandArgs, "target" | "action">,
  )
  try {
    const profitMargin = parseFloat(args?.profitMargin ?? "0")
    const estimate = await estimateBidAmount(
      {
        action: args.action as Action,
        target: args.target as Target,
        args: estimateArgs,
      },
      (fee) => fee * profitMargin,
    )
    console.log("\n")
    console.table(estimate)
    spinner.stop()
    process.exit(0)
  } catch (e) {
    spinner.fail(colorLogMsg("ERROR", e))
    process.exit(1)
  }
}
