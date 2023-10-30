import ora from "ora"
import { Args } from "@/types.js"
import { validate } from "@/utils/fns.js"
import { colorLogMsg } from "@/utils/log.js"
import {ApiPromise, WsProvider, Keyring, cryptoWaitReady} from "@t3rn/sdk"
import { FastWriterSchema } from "@/schemas/fastWriter.js"
import {createCircuitContext} from "@/utils/circuit.js";

export const spinner = ora()

export const handleFastWriterCommand = async (
  _args: Args<
    | "signer"
    | "endpoint"
    | "dest"
    | "recipient"
    | "targetAsset"
    | "targetAmount"
    | "rewardAsset"
    | "maxReward"
    | "insurance"
    | "type"
    | "speedMode"
    | "asUtilityBatch"
    | "asSequentialTx"
    | "asMultiSfx"
    | "repeat"
    | "repeatInterval"
  >,
) => {
  const args = validate(FastWriterSchema, {
    ..._args,
    targetAmount: parseFloat(_args?.targetAmount),
    maxReward: parseFloat(_args?.maxReward),
    insurance: parseFloat(_args?.insurance),
    rewardAsset: parseInt(_args?.rewardAsset),
    targetAsset: parseInt(_args?.targetAsset),
    repeat: _args?.repeat ? parseInt(_args?.repeat) : undefined,
    repeatInterval: _args?.repeatInterval ? parseInt(_args?.repeatInterval) : undefined,
  })

  if (!args) {
    process.exit()
  }

  spinner.text = "Warm-up checks for Fast Writer... \n"
  spinner.start()

  await cryptoWaitReady()
  const { circuit, sdk } = await createCircuitContext(false)



  spinner.stopAndPersist({
    symbol: "🎉",
    text: colorLogMsg("SUCCESS", `Parsed arguments: ${JSON.stringify(args)}`),
  })
  spinner.stop()

  process.exit(0)

}
