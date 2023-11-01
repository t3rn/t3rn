import ora from "ora"
import { Args } from "@/types.js"
import { validate } from "@/utils/fns.js"
import { colorLogMsg } from "@/utils/log.js"
import {
  ApiPromise,
  WsProvider,
  Keyring,
  cryptoWaitReady,
  Sdk,
} from "@t3rn/sdk"
import { FastWriterSchema } from "@/schemas/fastWriter.js"
import { createCircuitContext } from "@/utils/circuit.js"
import {
  build_tx_batch_single_order,
  Order,
  build_tx_vacuum_single_order,
  build_tx_vacuum_multi_order,
} from "@/commands/fastWriter/tx_builder.js"
import * as assert from "assert"

export const spinner = ora()

export const handleFastWriterCommand = async (
  _args: Args<
    | "signer"
    | "endpoint"
    | "dest"
    | "source"
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
    repeatInterval: _args?.repeatInterval
      ? parseInt(_args?.repeatInterval)
      : undefined,
  })

  if (!args) {
    process.exit()
  }

  spinner.text = "Warm-up checks for Fast Writer... \n"
  spinner.start()

  await cryptoWaitReady()
  const { circuit, sdk } = await createCircuitContext(false)

  const writeToVacuum = async () => {
    if (args.repeat > 0 && args.asUtilityBatch) {
      await write_batch_single_order(
        circuit,
        args.dest,
        args.targetAsset,
        args.targetAccount,
        args.targetAmount,
        args.rewardAsset,
        args.maxReward,
        args.insurance,
        args.speedMode,
        args.repeat,
      )
    } else if (args.repeat > 0 && args.asMultiSfx) {
      await write_multi_order(
        circuit,
        args.dest,
        args.targetAsset,
        args.targetAccount,
        args.targetAmount,
        args.rewardAsset,
        args.maxReward,
        args.insurance,
        args.speedMode,
        args.repeat,
      )
    } else {
      // Submit a single order
      await write_single_order(
        circuit,
        args.dest,
        args.targetAsset,
        args.targetAccount,
        args.targetAmount,
        args.rewardAsset,
        args.maxReward,
        args.insurance,
        args.speedMode,
      )
    }
  }

  if (args.repeatInterval) {
    const repeatInterval = args.repeatInterval * 1000

    if (args.source == "0x03030303") {
      spinner.text = `Writing to Vacuum every ${repeatInterval}ms... \n`
      spinner.start()
      setInterval(writeToVacuum, repeatInterval)
    } else if (args.source == "sepl") {
      console.log("sepl")
      // TODO: implement sepl
    } else {
      console.log("source not supported")
    }
    spinner.text = `Writing to Vacuum every ${repeatInterval}ms... \n`
    spinner.start()
    setInterval(writeToVacuum, repeatInterval)
  } else {
    if (args.source == "0x03030303") {
      await writeToVacuum()
    } else if (args.source == "sepl") {
      console.log("sepl")
    } else {
      console.log("source not supported")
    }
  }

  spinner.stopAndPersist({
    symbol: "🎉",
    text: colorLogMsg("SUCCESS", `Parsed arguments: ${JSON.stringify(args)}`),
  })
  spinner.stop()

  process.exit(0)
}

export const mock_test_multi_order = async (
  endpoint: string,
  signer_in: string,
  dest: string,
  asset: number,
  repeat: number,
) => {
  const keyring = new Keyring({ type: "sr25519" })
  const signer = keyring.addFromMnemonic(signer_in)
  const sdk = new Sdk(endpoint, signer, false)
  const circuit = await sdk.init()

  await write_multi_order(
    circuit,
    dest,
    asset,
    signer.address,
    100,
    asset,
    101,
    10,
    3,
    repeat,
  )
}

export const mock_test_batch_order = async (
  endpoint: string,
  signer_in: string,
  dest: string,
  asset: number,
  repeat: number,
) => {
  const keyring = new Keyring({ type: "sr25519" })
  const signer = keyring.addFromMnemonic(signer_in)
  const sdk = new Sdk(endpoint, signer, false)
  const circuit = await sdk.init()

  await write_batch_single_order(
    circuit,
    dest,
    asset,
    signer.address,
    100,
    asset,
    101,
    10,
    3,
    repeat,
  )
}

export const mock_test_single_order = async (
  endpoint: string,
  signer_in: string,
  dest: string,
  asset: number,
) => {
  const keyring = new Keyring({ type: "sr25519" })
  const signer = keyring.addFromMnemonic(signer_in)
  const sdk = new Sdk(endpoint, signer, false)
  const circuit = await sdk.init()

  await write_single_order(
    circuit,
    dest,
    asset,
    signer.address,
    100,
    asset,
    101,
    10,
    3,
  )
}

export const write_single_order = async (
  circuit: ApiPromise,
  dest: string,
  targetAsset: number,
  targetAccount: string,
  targetAmount: number,
  rewardAsset: number,
  maxReward: number,
  insurance: number,
  speedMode: string,
) => {
  return signAndSender(
    circuit,
    build_tx_vacuum_single_order(
      circuit,
      new Order(
        dest,
        targetAsset,
        targetAccount,
        targetAmount,
        maxReward,
        rewardAsset,
        insurance,
        0,
      ),
      speedMode,
    ),
  )
}

export const write_batch_single_order = async (
  circuit: ApiPromise,
  dest: string,
  targetAsset: number,
  targetAccount: string,
  targetAmount: number,
  rewardAsset: number,
  maxReward: number,
  insurance: number,
  speedMode: string,
  repeat: number,
) => {
  const batchOrders = []
  for (let i = 0; i < repeat; i++) {
    batchOrders.push(
      new Order(
        dest,
        targetAsset,
        targetAccount,
        targetAmount,
        maxReward,
        rewardAsset,
        insurance,
        i,
      ),
    )
  }
  return signAndSender(
    circuit,
    build_tx_batch_single_order(circuit, batchOrders, speedMode),
  )
}

export const write_multi_order = async (
  circuit: ApiPromise,
  dest: string,
  targetAsset: number,
  targetAccount: string,
  targetAmount: number,
  rewardAsset: number,
  maxReward: number,
  insurance: number,
  speedMode: string,
  repeat: number,
) => {
  const batchOrders = []
  for (let i = 0; i < repeat; i++) {
    batchOrders.push(
      new Order(
        dest,
        targetAsset,
        targetAccount,
        targetAmount,
        maxReward,
        rewardAsset,
        insurance,
        i,
      ),
    )
  }
  return signAndSender(
    circuit,
    build_tx_vacuum_multi_order(circuit, batchOrders, speedMode),
  )
}

export const signAndSender = async (circuit: ApiPromise, tx: any) => {
  const txSize = Math.floor(tx.encodedLength / 1024)
  console.debug(`writer::signAndSend tx size: ${txSize}kB`)

  const res = await circuit.tx.signAndSendSafe(tx)
  console.debug(`writer::signAndSend response ${res}`)
  assert.ok(res)

  // Sleep for 5 seconds for tx to settle
  await new Promise((resolve) => setTimeout(resolve, 5000))
  return res
}
