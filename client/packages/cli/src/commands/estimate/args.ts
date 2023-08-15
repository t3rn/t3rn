import { EstimateSchema } from "@/schemas/estimate.ts"
import { Extrinsic, ExtrinsicSchema } from "@/schemas/extrinsic.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { validate } from "@/utils/fns.ts"
import { buildSfx, readSfxFile } from "@/utils/sfx.ts"
import { EstimateSubmittableExtrinsicParams } from "@t3rn/sdk/dist/price-estimation/substrate.js"
import {
  EstimateEthActionParams,
  EstimateParams,
  SpeedMode,
} from "@t3rn/sdk/price-estimation"

export const getEstimationArgs = async <
  T extends {
    target: string
    action: string
    args?: string
    sfx?: string
    export?: string
  },
>(
  args: T,
): Promise<EstimateParams> => {
  const exportMode = Boolean(args.export)

  const estimationArgs = readEstimationArgs(args)
  if (!estimationArgs) process.exit()

  const opts = validate(
    EstimateSchema,
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

  if (isExtrinsicArgs) {
    return (await buildExtrinsic(
      JSON.parse(opts.args),
      exportMode,
    )) as EstimateSubmittableExtrinsicParams
  }

  try {
    const parsedData = JSON.parse(opts.args)
    return parsedData as EstimateEthActionParams
  } catch {
    return opts.args as SpeedMode
  }
}

const readEstimationArgs = <T extends { args?: string; sfx?: string }>(
  args: T,
) => {
  if (args.sfx) {
    const sfxFile = JSON.stringify(readSfxFile(args.sfx))
    return sfxFile
  } else {
    return args.args
  }
}

const buildExtrinsic = async (extrinsic: Extrinsic, exportMode: boolean) => {
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
