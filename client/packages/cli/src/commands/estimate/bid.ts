import ora from "ora"
import {
  Action,
  Target,
  estimateBidAmount,
  estimateGasFee,
} from "@t3rn/sdk/price-estimation"
import { colorLogMsg } from "@/utils/log.ts"
import { Args, WithRequiredProperty } from "@/types.ts"
import { getEstimationArgs } from "./args.ts"

export const spinner = ora()

type CommandArgs = Args<
  "target" | "action" | "profitMargin" | "args" | "sfx" | "export"
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
  } catch (e) {
    spinner.fail(colorLogMsg("ERROR", e))
  }
  spinner.stop()
}
