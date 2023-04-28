import "@t3rn/types"
import { createType } from "@t3rn/types"
import ora from "ora"
import { Sdk } from "@t3rn/sdk"
import { existsSync, readFileSync } from "fs"
import { validate } from "@/utils/fns.ts"
import { colorLogMsg, log } from "@/utils/log.ts"
import { ExtrinsicSchema, Extrinsic } from "@/schemas/extrinsic.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { getConfig } from "@/utils/config.ts"
import { T3rnTypesSfxSideEffect } from "@polkadot/types/lookup"

export const spinner = ora()

export const handleSubmitExtrinsicCmd = (extrinsicFile: string) => {
  const unvalidatedExtrinsic = readExtrinsicFile(extrinsicFile)

  if (!unvalidatedExtrinsic) {
    process.exit(1)
  }

  const extrinsic = validate(ExtrinsicSchema, unvalidatedExtrinsic, {
    configFileName: extrinsicFile,
  })

  if (!extrinsic) {
    process.exit(1)
  }

  submitExtrinsic(extrinsic)
}

export const readExtrinsicFile = (filePath: string) => {
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

export const submitExtrinsic = async (extrinsic: Extrinsic) => {
  const config = getConfig()

  spinner.text = "Submitting extrinsic..."
  spinner.start()

  if (!config) {
    process.exit(1)
  }

  const { circuit, sdk } = await createCircuitContext()
  const transactionArgs = buildExtrinsic(
    circuit,
    extrinsic.sideEffects,
    extrinsic.sequential,
    sdk
  )

  try {
    const transaction = circuit.tx.circuit.onExtrinsicTrigger(
      transactionArgs.sideEffects,
      transactionArgs.sequential
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

export const buildExtrinsic = (
  circuitApi: Awaited<ReturnType<typeof createCircuitContext>>["circuit"],
  sideEffects: Extrinsic["sideEffects"],
  sequential: boolean,
  sdk: Sdk
) => {
  return {
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
          nonce: 0,
        })
        return obj
      })
      // @ts-expect-error - TS doesn't know that we are creating a type here
    ).toJSON(),
    sequential: circuitApi.createType("bool", sequential),
  }
}
