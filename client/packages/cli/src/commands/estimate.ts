import ora from "ora"
import {
  EthActions,
  EthSpeedModes,
  calculateGasFee,
} from "@t3rn/sdk/price-estimation"
import { ExtrinsicSchema, SpeedMode, SpeedModes } from "@/schemas/extrinsic.ts"
import { getConfig } from "@/utils/config.ts"
import { validate } from "@/utils/fns.ts"
import { colorLogMsg, log } from "@/utils/log.ts"
import { readSfxFile } from "@/utils/sfx.ts"
import { Args } from "@/types.ts"
import {
  SideEffect,
  SideEffectAction,
  SideEffectActions,
} from "@/schemas/sfx.ts"

export const spinner = ora()

export const handleEstimateFees = async (args: Args<"sfx" | "export">) => {
  const config = getConfig()
  if (!config) {
    process.exit(1)
  }

  if (!args.sfx) {
    log("ERROR", "No sfx file provided, use --sfx <file-path>")
    process.exit(1)
  }

  const unvalidatedExtrinsic = readSfxFile(args.sfx)

  if (!unvalidatedExtrinsic) {
    process.exit(1)
  }

  const extrinsic = validate(ExtrinsicSchema, unvalidatedExtrinsic, {
    configFileName: args.sfx,
  })

  if (!extrinsic) {
    process.exit(1)
  }

  spinner.text = "Estimating fees..."
  spinner.start()

  await Promise.allSettled(
    extrinsic.sideEffects.map((sfx, i) =>
      estimateFees(`#${i}`, sfx, extrinsic.speed_mode)
    )
  )

  spinner.stop()
}

const estimateFees = async (
  tag: string,
  sideEffect: SideEffect,
  speedMode: SpeedMode
) => {
  const withArgs = (fn: (...args: unknown[]) => unknown) =>
    fn(tag, sideEffect, speedMode)

  switch (sideEffect.target) {
    case "roco":
      return await withArgs(estimateSubstrateFees)
    case "eth":
    default:
      return await withArgs(estimateEthFees)
  }
}

const estimateEthFees = async (
  tag: string,
  sideEffect: SideEffect,
  speedMode: SpeedMode
) => {
  const mapSfxSpeedModeToEthSpeedMode = (speedMode: SpeedMode) => {
    switch (speedMode) {
      case SpeedModes.Fast:
        return EthSpeedModes.Fast
      case SpeedModes.Rational:
      case SpeedModes.Finalized:
        return EthSpeedModes.Standard
    }
  }
  const mapSfxActionToEthAction = (action: SideEffectAction) => {
    switch (action) {
      case SideEffectActions.Transfer:
      default:
        return EthActions.Transfer
    }
  }

  spinner.info(colorLogMsg("INFO", `Estimating fees for ${tag}...`))

  try {
    const gasFeesInEth = await calculateGasFee(
      mapSfxActionToEthAction(sideEffect.action),
      mapSfxSpeedModeToEthSpeedMode(speedMode)
    )
    spinner.succeed(
      colorLogMsg("SUCCESS", `Estimated fees for ${tag}: ${gasFeesInEth} ETH`)
    )
  } catch (e) {
    spinner.fail(
      colorLogMsg("ERROR", `Failed to estimate fees for ${tag}: ${e}`)
    )
  }
}

const estimateSubstrateFees = async (tag: string, sideEffect: SideEffect) => {
  spinner.warn(
    colorLogMsg(
      "WARN",
      `Skip fees estimation for ${tag}. REASON: price estimation for ${sideEffect.target} target is not yet supported!`
    )
  )
}
