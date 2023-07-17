import ora from "ora"
import { estimateMaxReward, Action, Target } from "@t3rn/sdk/price-estimation"
import { MaxRewardEstimateSchema } from "@/schemas/estimate.ts"
import { validate } from "@/utils/fns.ts"
import { colorLogMsg } from "@/utils/log.ts"
import { Args } from "@/types.ts"

export const spinner = ora()

export const handleEstimateMaxReward = async (
  _args: Args<
    | "action"
    | "baseAsset"
    | "target"
    | "targetAmount"
    | "targetAsset"
    | "overSpend"
  >,
) => {
  const args = validate(
    MaxRewardEstimateSchema,
    {
      ..._args,
      targetAmount: parseFloat(_args?.targetAmount),
      overSpend: parseFloat(_args?.overSpend),
    },
    {
      configFileName: "estimation arguments",
    },
  )

  if (!args) {
    process.exit()
  }

  spinner.text = "Estimating..."
  spinner.start()

  try {
    const estimate = await estimateMaxReward({
      action: args.action as Action,
      asset: args.baseAsset,
      target: args.target as Target,
      targetAmount: args.targetAmount,
      targetAsset: args.targetAsset,
      overSpendPercent: args.overSpend,
    })
    console.log("\n")
    console.table(estimate)
  } catch (e) {
    spinner.fail(colorLogMsg("ERROR", e))
  }

  spinner.stop()
}
