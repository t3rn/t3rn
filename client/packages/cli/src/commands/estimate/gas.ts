import ora from "ora"
import {
  EstimateParams,
  Action,
  Target,
  estimateGasFee,
} from "@t3rn/sdk/price-estimation"
import { GasFeeEstimateSchema } from "@/schemas/estimate.ts"
import { validate } from "@/utils/fns.ts"
import { colorLogMsg, log } from "@/utils/log.ts"
import { buildSfx, readSfxFile } from "@/utils/sfx.ts"
import { Extrinsic, ExtrinsicSchema } from "@/schemas/extrinsic.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { Args } from "@/types.ts"

export const spinner = ora()

export const handleEstimateGasFee = async (
  args: Args<"target" | "action" | "args" | "sfx" | "export">,
) => {
  const exportMode = Boolean(args.export)
  if (!args.sfx && !args.args) {
    log("ERROR", "Either --args or --sfx must be provided")
    process.exit(1)
  }

  const estimationArgs: string = (() => {
    if (args.sfx) {
      const sfxFile = JSON.stringify(readSfxFile(args.sfx))
      return sfxFile
    } else {
      return args.args
    }
  })()

  if (!estimationArgs) process.exit()
  const opts = validate(
    GasFeeEstimateSchema,
    { ...args, args: estimationArgs },
    {
      configFileName: "gas estimation arguments",
    },
  )

  if (!opts) process.exit()

  let isExtrinsicArgs: boolean
  try {
    isExtrinsicArgs = Boolean(
      args.sfx || ExtrinsicSchema.safeParse(JSON.parse(opts.args)).success,
    )
  } catch (e) {
    isExtrinsicArgs = false
  }

  spinner.text = "Estimating..."
  spinner.start()

  if (isExtrinsicArgs) {
    const buildExtrinsic = async (
      extrinsic: Extrinsic,
      exportMode: boolean,
    ) => {
      const { circuit, sdk } = await createCircuitContext(exportMode)
      const transactionArgs = buildSfx(
        circuit,
        extrinsic.sideEffects,
        extrinsic.speed_mode,
        sdk,
      )
      return circuit.tx.circuit.onExtrinsicTrigger(
        transactionArgs.sideEffects as Parameters<
          typeof circuit.tx.circuit.onExtrinsicTrigger
        >[0],
        transactionArgs.speed_mode,
      )
    }

    opts.args = await buildExtrinsic(JSON.parse(opts.args), exportMode)
  }

  try {
    const parsedData = JSON.parse(opts.args)
    opts.args = parsedData
  } catch {
    // do nothing
  }

  try {
    const estimate = await estimateGasFee({
      action: opts.action as Action,
      target: opts.target as Target,
      args: opts.args as EstimateParams,
    })
    console.log("\n")
    console.table(estimate)
  } catch (e) {
    spinner.fail(colorLogMsg("ERROR", e))
  }

  spinner.stop()
}
