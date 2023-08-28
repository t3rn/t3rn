import { SubmittableExtrinsic } from "@polkadot/api/promise/types"
import {
  EstimateSchema,
  EstimateSubmittableExtrinsicSchema,
} from "@/schemas/estimate.ts"
import { Extrinsic } from "@/schemas/extrinsic.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { validate } from "@/utils/fns.ts"
import { buildSfx, readSfxFile } from "@/utils/sfx.ts"
import { EstimateSubmittableExtrinsicParams } from "@t3rn/sdk/dist/price-estimation/substrate.js"
import {
  EstimateEthActionParams,
  EstimateParams,
  SpeedMode,
} from "@t3rn/sdk/price-estimation"
import { Keyring } from "@t3rn/sdk"
import { Args } from "@/types.ts"
import { log } from "@/utils/log.ts"

export const getEstimationArgs = async <
  T extends {
    target: string
    action: string
    args?: string
    sfx?: string
    signer?: string
    export?: string
  },
>(
  commandArgs: T,
): Promise<EstimateParams> => {
  const exportMode = Boolean(commandArgs.export)
  const rawEstimationArgs = buildRawEstimationArgs(commandArgs)

  if (!rawEstimationArgs) {
    log("ERROR", "No estimation arguments provided")
    process.exit(1)
  }

  const opts = validate(
    EstimateSchema,
    {
      target: commandArgs.target,
      action: commandArgs.action,
      args: rawEstimationArgs,
    },
    {
      configFileName: "gas estimation arguments",
    },
  )

  if (!opts) process.exit(1)

  try {
    const { sideEffect, signer } = EstimateSubmittableExtrinsicSchema.parse(
      JSON.parse(rawEstimationArgs),
    )
    const tx = (await buildExtrinsic(
      sideEffect,
      exportMode,
    )) as Awaited<SubmittableExtrinsic>
    return { tx, account: signer } as EstimateSubmittableExtrinsicParams
  } catch (e) {
    //
  }

  try {
    const parsedData = JSON.parse(opts.args)
    return parsedData as EstimateEthActionParams
  } catch {
    return opts.args as SpeedMode
  }
}

const buildRawEstimationArgs = <
  T extends { args?: string; sfx?: string; signer?: string },
>(
  args: T,
): string => {
  if (args.sfx) {
    const maybeSideEffect: Record<string, unknown> = readSfxFile(args.sfx)

    let signer = args?.signer
    if (!args?.signer) {
      const keyring = new Keyring({ type: "sr25519" })
      const alice = keyring.addFromUri("//Alice")
      signer = alice.address
    }

    return JSON.stringify({ sideEffect: maybeSideEffect, signer })
  }

  return args.args
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

export const validateEstimationArgs =
  (callback: (args: Args<never>) => void) =>
    (args: Args<"args" | "sfx" | "signer">) => {
      if (args.sfx && args.args) {
        log("ERROR", "Cannot specify both --sfx and --args")
        process.exit(1)
      }

      if (args.args && args.signer) {
        log("ERROR", "The --signer option can only be used with --sfx")
        process.exit(1)
      }

      callback(args)
    }
