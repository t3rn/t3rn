import ora from "ora"
import { Args, WithRequiredProperty } from "@/types.ts"
import { getEstimationArgs } from "./args.ts"
import { Action, Target, estimateMaxReward } from "@t3rn/sdk/price-estimation"
import { colorLogMsg } from "@/utils/log.ts"
import { validate } from "@/utils/fns.ts"
import { MaxRewardEstimateSchema } from "@/schemas/estimate.ts"

export const spinner = ora()

type CommandArgs = Args<
  | "action"
  | "baseAsset"
  | "target"
  | "targetAmount"
  | "targetAsset"
  | "overSpend"
  | "args"
  | "sfx"
  | "export"
>

export const handleEstimateMaxReward = async (args: CommandArgs) => {
  spinner.text = "Estimating..."
  spinner.start()
  try {
    const opts = validate(
      MaxRewardEstimateSchema,
      {
        action: args?.action,
        baseAsset: args?.baseAsset,
        target: args?.target,
        targetAmount: parseFloat(args?.targetAmount),
        targetAsset: args?.targetAsset,
        overSpend: parseFloat(args?.overSpend),
      },
      {
        configFileName: "estimation arguments",
      },
    )
    if (!opts) {
      process.exit()
    }

    const estimateArgs = await getEstimationArgs(
      args as WithRequiredProperty<CommandArgs, "target" | "action">,
    )
    const estimate = await estimateMaxReward({
      action: opts.action as Action,
      baseAsset: opts.baseAsset,
      target: opts.target as Target,
      targetAmount: opts.targetAmount,
      targetAsset: opts.targetAsset,
      overSpendPercent: opts.overSpend,
      args: estimateArgs,
    })
    console.log("\n")
    console.table(estimate)
  } catch (e) {
    spinner.fail(colorLogMsg("ERROR", e))
  }
  spinner.stop()
}
