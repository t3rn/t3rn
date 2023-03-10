import { ApiPromise, WsProvider } from "@polkadot/api"
import { Keyring } from '@polkadot/keyring'
import { SubmittableExtrinsic } from "@polkadot/api/types"
import { KeyringPair } from "@polkadot/keyring/types"
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { Sdk, Tx } from '../../client/packages/sdk';

const { exec } = require('child_process');

/**
 * The extrinsic type.
 */
export type Extrinsic = SubmittableExtrinsic<"promise">

/**
 * Simple event typed exposed to developers.
 */
export type SimpleEvent = {
  name: string
  data: { type: string; value: null | any }[]
}

/**
 * Signs and sends an extrinsic using @polkadot/api but handles dispatch
 * and pallet errors so that you can just await transaction inclusion.
 *
 * @param api- The Substrate node client.
 * @param signer - The extrinsic origin.
 * @param tx - The extrinsic to submit.
 * @returns The number of the block this extrinsic got included in and corresponding events.
 */
export async function signAndSendSafe(
  api: ApiPromise,
  signer: KeyringPair,
  tx: Extrinsic
): Promise<{ block: bigint; events: SimpleEvent[] }> {
  let nonce = await api.rpc.system.accountNextIndex(signer.address)
  return new Promise((resolve, reject) =>
    tx.signAndSend(
      signer,
      { nonce },
      async ({ dispatchError, status, events }) => {
        if (dispatchError?.isModule) {
          let err = api.registry.findMetaError(dispatchError.asModule)
          reject(Error(`${err.section}::${err.name}: ${err.docs.join(" ")}`))
        } else if (dispatchError) {
          reject(Error(String(dispatchError)))
        } else if (status.isInBlock) {
          let blockNumber = await api.rpc.chain
            .getBlock(status.asInBlock)
            .then(r => BigInt(String(r.block.header.number)))
          resolve({ block: blockNumber, events: events.map(formatEvent) })
        }
      }
    )
  )
}

/**
 * Maps Substrate events to a human readable format.
 *
 * @param events Raw events obtained from a Polkadot.js callback.
 * @returns Human readable event wrapper
 */
function formatEvent({
  // @ts-ignore
  event: { typeDef, section, method, data },
}): SimpleEvent {
  return {
    name: `${section}::${method}`,
    data: data.map((data: any, index: number) => {
      let type = typeDef[index].type
      let value
      let str = String(data)

      if (str) {
        if (type === "AccountId32") {
          value = data.toHuman()
        } else if (type === "u128") {
          value = BigInt(str)
        } else if (type === "Bytes") {
          value = Uint8Array.from(data.toJSON())
        } else if (type === "bool") {
          value = str === "true"
        } else if (type === "(u128,u128)") {
          value = data.toJSON().map(BigInt)
        } else {
          value = str
        }
      } else {
        value = null
      }

      return { type, value }
    }),
  }
}


// Paul's
const execute = async (command: string, waitInSec: number) => {
  console.log("Executing: ", command)
  return new Promise((resolve, reject) => {
    exec(
      `ts-node index.ts ${command}`,
      (error: any, stdout: string, stderr: string) => {
        console.log(stdout)
        if (error) {
          reject(error);
        } else {
          resolve(stdout);
        }
      });
  })
    .then(() => {
      return wait(waitInSec)
    })
}

const wait = (waitInSecs: number) => {
  console.log(`Waiting ${waitInSecs} seconds!`)
  return new Promise(resolve => setTimeout(resolve, waitInSecs * 1000));
};




async function main() {

  // already PRed by Noah, but rebased here and still asking, so here it is
  await cryptoWaitReady();

  // await execute("register roco", 10)

  // await submitNoBidShouldKill()
  await submitShouldSetToReady()
  // await submitSfxConfirmationMovesToFinished()
  // await notSubmitAfterBidMovesToRevert()
}



// TEST - 1
async function submitNoBidShouldKill() {

  const function_name = "[1 - submitNoBidShouldKill] "

  const keyring = new Keyring({ type: 'sr25519' });
  const signer = process.env.CIRCUIT_KEY === undefined
    ? keyring.addFromUri("//Alice")
    : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk("ws://127.0.0.1:9944", signer)
  const api = await sdk.init();
  const txs = new Tx(api, signer)

  const tx = api.tx.circuit.onExtrinsicTrigger(
    [
      {
        target: "roco",
        maxReward: 4000000000000,
        insurance: 100000000000,
        encodedAction: "tran",
        encodedArgs: [
          "0x6c51de0380769219d483abd2686e0b3f4feb4163a18fe6ae6d622f8014f21b72",
          "0x78f0f126e03af9a51a3fc1498a6249776fac77839b27170daa9ae04ca1b17b7a",
          "0x00e40b54020000000000000000000000",
          "0x00e8764817000000000000000000000000409452a30300000000000000000000"
        ],
        signature: "0x1234"
      }
    ],
    false,
  )
  console.log(function_name, "Created xtx with onExtrinsicTrigger")
  console.log(function_name, "Signing and sending...")

  await txs.signAndSendSafe(tx).catch(console.error).then(console.log)

  console.log(function_name, "Finished test")

  // NOAH: Wait and, if nobody bids, it will be killed
}




// TEST - 2
async function submitShouldSetToReady() {

  const function_name = "[2 - submitShouldSetToReady] "

  const keyring = new Keyring({ type: 'sr25519' });
  const signer = process.env.CIRCUIT_KEY === undefined
    ? keyring.addFromUri("//Alice")
    : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk("ws://127.0.0.1:9944", signer)
  const api = await sdk.init();
  const txs = new Tx(api, signer)

  const tx = api.tx.circuit.onExtrinsicTrigger(
    [
      {
        target: "roco",
        maxReward: 4000000000000,
        insurance: 100000000000,
        encodedAction: "tran",
        encodedArgs: [
          "0x6c51de0380769219d483abd2686e0b3f4feb4163a18fe6ae6d622f8014f21b72",
          "0x78f0f126e03af9a51a3fc1498a6249776fac77839b27170daa9ae04ca1b17b7a",
          "0x00e40b54020000000000000000000000",
          "0x00e8764817000000000000000000000000409452a30300000000000000000000"
        ],
        signature: "0x1234"
      }
    ],
    false,
  )
  console.log(function_name, "Created xtx with onExtrinsicTrigger")

  await txs.signAndSendSafe(tx).catch(r => console.error(r)).then(r => console.log(r))



  // How to check if it's in ready state?
}




// TEST - 3 
async function submitSfxConfirmationMovesToFinished() {

  const function_name = "[3 - submitSfxConfirmationMovesToFinished] "

  const keyring = new Keyring({ type: 'sr25519' });
  const signer = process.env.CIRCUIT_KEY === undefined
    ? keyring.addFromUri("//Alice")
    : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk("ws://127.0.0.1:9944", signer)
  const api = await sdk.init();
  const txs = new Tx(api, signer)

  const tx = api.tx.circuit.onExtrinsicTrigger(
    [
      {
        target: "roco",
        type: "tran",
        to: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
        amount: "4", // in ROC
        insurance: "1", // in TRN
        reward: "4", // in TRN
      },
    ],
    false,
  )
  console.log(function_name, "Created xtx with onExtrinsicTrigger")

  await txs.signAndSendSafe(tx).catch(console.error).then(console.log)
  console.log(function_name, "Signed and sent xtx")

  api.tx.circuit.bidSfx(
    api.createType("Hash", tx.hash),
    api.createType("u128", 1000),
  )
  console.log(function_name, "Bid '1000' on the SFX")

  // Is it in ready state?
}





// TESTS - 4
async function notSubmitAfterBidMovesToRevert() {

  const function_name = "[4 - notSubmitAfterBidMovesToRevert] "

  const keyring = new Keyring({ type: 'sr25519' });
  const signer = process.env.CIRCUIT_KEY === undefined
    ? keyring.addFromUri("//Alice")
    : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk("ws://127.0.0.1:9944", signer)
  const api = await sdk.init();

  const xtx = api.tx.circuit.onExtrinsicTrigger(
    [
      {
        target: "roco",
        type: "tran",
        to: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
        amount: "4000000000000", // in ROC
        insurance: "100000000000", // in TRN
        reward: "4", // in TRN
      },
    ],
    false,
  )
  console.log(function_name, "Created xtx with onExtrinsicTrigger")

  await xtx.signAndSend(signer)
  console.log(function_name, "Signed and sent xtx")

  api.tx.circuit.bidSfx(
    api.createType("Hash", xtx.hash),
    api.createType("u128", 1000),
  )
  console.log(function_name, "Bid '1000' on the SFX")

  // Is it in ready state?
}





main().catch(console.error).finally(() => process.exit());
