import { existsSync, readFileSync } from "fs"
import { Sdk } from "@t3rn/sdk"
import { createType } from "@t3rn/types"
import { T3rnTypesSfxSideEffect } from "@polkadot/types/lookup"
import { Extrinsic, SpeedMode } from "@/schemas/extrinsic.ts"
import {
  EncodedArgs,
  SideEffectAction,
  SideEffectActions,
  TransferEncodedArgs,
} from "@/schemas/sfx.ts"
import { Circuit } from "@/types.ts"
import { createCircuitContext } from "./circuit.ts"
import { getConfig } from "./config.ts"
import { log } from "./log.ts"

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
          data.action
        ]({
          ...mapEncodedArgs(data.action, data.encodedArgs as EncodedArgs),
          maxReward: sdk.circuit.floatToBn(parseFloat(data.maxReward)),
          insurance: sdk.circuit.floatToBn(parseFloat(data.insurance)),
          signature: data.signature,
          enforceExecutioner: data.enforceExecutor,
          rewardAssetId: data.rewardAssetId,
        })
        return obj
      })
      // @ts-ignore - TS doesn't know that we are creating a type here
    ).toJSON(),
    speed_mode: circuitApi.createType("T3rnPrimitivesSpeedMode", speedMode),
  }
}

export enum SfxSendType {
  Safe = "safe",
  Raw = "raw",
}

export const submitSfx = async (
  extrinsic: Extrinsic,
  exportMode: boolean,
  sendType = SfxSendType.Safe
) => {
  const config = getConfig()

  if (!config) {
    process.exit(1)
  }

  const { circuit, sdk } = await createCircuitContext(exportMode)
  const transactionArgs = buildSfx(
    circuit,
    extrinsic.sideEffects,
    extrinsic.speed_mode,
    sdk
  )
  const transaction = circuit.tx.circuit.onExtrinsicTrigger(
    transactionArgs.sideEffects as Parameters<
      typeof circuit.tx.circuit.onExtrinsicTrigger
    >[0],
    transactionArgs.speed_mode
  )
  const response = await sdk.circuit.tx[
    sendType === SfxSendType.Raw ? "signAndSendRaw" : "signAndSendSafe"
  ](transaction)

  return response
}

export const mapEncodedArgs = (
  action: SideEffectAction,
  encodedArgs: EncodedArgs
) => {
  switch (action) {
    case SideEffectActions.Transfer: {
      const args: TransferEncodedArgs = encodedArgs[0]
      return {
        from: args.from,
        to: args.to,
      }
    }
    default:
      return null
  }
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
