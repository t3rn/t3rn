import "@t3rn/types"
import { createType } from "@t3rn/types"
import ora from "ora"
import { existsSync, readFileSync } from "fs"
import { Sdk } from "@t3rn/sdk"
import { T3rnTypesSfxSideEffect } from "@polkadot/types/lookup"
import { validate } from "@/utils/fns.ts"
import { colorLogMsg, log } from "@/utils/log.ts"
import { ExtrinsicSchema, Extrinsic, SpeedMode } from "@/schemas/extrinsic.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { getConfig } from "@/utils/config.ts"
import { Circuit } from "@/types.ts"

export const spinner = ora()

export const handleSubmitSfxCmd = (sfxFile: string) => {
  const unvalidatedExtrinsic = readSfxFile(sfxFile)

  if (!unvalidatedExtrinsic) {
    process.exit(1)
  }

  const extrinsic = validate(ExtrinsicSchema, unvalidatedExtrinsic, {
    configFileName: sfxFile,
  })

  if (!extrinsic) {
    process.exit(1)
  }

  submitSfx(extrinsic)
}

export const readSfxFile = (filePath: string) => {
  if (!existsSync(filePath)) {
    log("ERROR", `File ${filePath} does not exist`)
    return
  }

  const file = readFileSync(filePath, "utf8")
  try {
    return JSON.parse(file)
  } catch (e) {
    log("ERROR", `Unable to parse ${filePath} as JSON`)
  }
}

export const submitSfx = async (extrinsic: Extrinsic) => {
  const config = getConfig()

  spinner.text = "Submitting extrinsic..."
  spinner.start()

  if (!config) {
    process.exit(1)
  }

  const { circuit, sdk } = await createCircuitContext()
  const transactionArgs = buildSfx(
    circuit,
    extrinsic.sideEffects,
    extrinsic.speed_mode,
    sdk
  )

  try {
    const transaction = circuit.tx.circuit.onExtrinsicTrigger(
      transactionArgs.sideEffects,
      transactionArgs.speed_mode
    )
    const submissionHeight = await sdk.circuit.tx.signAndSendSafe(transaction)
    spinner.stopAndPersist({
      symbol: "🚀",
      text: colorLogMsg(
        "SUCCESS",
        `Extrinsic submitted at block #${submissionHeight}`
      ),
    })
    process.exit(0)
  } catch (e) {
    spinner.fail(`Extrinsic submission failed: ${e}`)
    process.exit(1)
  }
}

export const buildSfx = (
  circuitApi: Circuit,
  sideEffects: Extrinsic["sideEffects"],
  speedMode: SpeedMode,
  sdk: Sdk
) => {
  return {
    // @ts-ignore - A weird error that I don't understand
    sideEffects: createType(
      "Vec<T3rnTypesSfxSideEffect>",
      sideEffects.map((data) => {
        const obj: T3rnTypesSfxSideEffect = sdk.gateways[data.target].createSfx[
          data.type
        ]({
          from: data.from,
          to: data.to,
          value: sdk.gateways[data.target].floatToBn(parseFloat(data.amount)),
          maxReward: sdk.circuit.floatToBn(parseFloat(data.reward)),
          insurance: sdk.circuit.floatToBn(parseFloat(data.insurance)),
        })
        return obj
      })
      // @ts-ignore - TS doesn't know that we are creating a type here
    ).toJSON(),
    speed_mode: circuitApi.createType("T3rnPrimitivesSpeedMode", speedMode),
  }
}
