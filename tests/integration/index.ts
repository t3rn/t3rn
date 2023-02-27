import { ApiPromise, WsProvider } from "@polkadot/api"
import { Keyring } from '@polkadot/keyring'
import { SubmittableExtrinsic } from "@polkadot/api/types"
import { KeyringPair } from "@polkadot/keyring/types"
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { Sdk } from '../../client/packages/sdk';

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


async function main() {

  await cryptoWaitReady();

  // const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  // const api = await ApiPromise.create({ provider: wsProvider });
  // const keyring = new Keyring({ type: 'sr25519' });
  // const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
  // const transfer = api.tx.balances.transfer(BOB, 12345);
  // const event = await signAndSendSafe(api, alice, transfer);
  // event['events'].forEach((e: any) => {
  //   console.log('[TRANSFER] event ->', e);
  // })


  //
  //
  // TESTS TESTS TESTS
  // TESTS TESTS TESTS
  // TESTS TESTS TESTS
  //
  //
  // await submitNoBidShouldKill()

  await submitShouldSetToReady()
}

main().catch(console.error).finally(() => process.exit());

// TEST
async function submitNoBidShouldKill() {
  const keyring = new Keyring({ type: 'sr25519' });
  const signer = process.env.CIRCUIT_KEY === undefined
    ? keyring.addFromUri("//Alice")
    : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk("ws://127.0.0.1:9944", signer)
  const api = await sdk.init();

  const encodedSfxId = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000001")
  const encodedAmount = api.createType("u128", 0)

  const xtx = api.tx.circuit.onExtrinsicTrigger(
    encodedSfxId,
    encodedAmount
  )

  console.log("[1 - submitNoBidShouldKill] Created xtx with onExtrinsicTrigger")

  await xtx.signAndSend(signer)

  console.log("[1 - submitNoBidShouldKill] Signed and sent xtx")
  //
  // Received event `circuit.XTransactionXtxDroppedAtBidding` with H256 value: `0xebe4e9da59c8e9a464d7f1760183f29339842900032194edde113d7277d85e4f`
  //
  // I guess it's the correct way to go, since the amount is 0 and it's not supposed to work then.
  //
  // From before:
  // This doesn't work.
  // const event = api.tx.circuit.bidSfx(xtx, 0)
  //
  // Fails with the following error:
  // Error: createType(Call):: Call: failed decoding circuit.bidSfx:: Struct: failed on args: {"sfx_id":"H256","bid_amount":"u128"}:: Struct: failed on sfx_id: H256:: Expected input with 32 bytes (256 bits), found 1 bytes
  //
  // This works. Why?
  // const event = api.tx.circuit.bidSfx(
  //   encodedSfxId,
  //   encodedAmount
  // )
  // await event.signAndSend(signer);
  // console.log(event);
}

// TEST
async function submitShouldSetToReady() {
  const keyring = new Keyring({ type: 'sr25519' });
  const signer = process.env.CIRCUIT_KEY === undefined
    ? keyring.addFromUri("//Alice")
    : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk("ws://127.0.0.1:9944", signer)
  const api = await sdk.init();

  const encodedSfxId = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000009")
  const encodedAmount = api.createType("u128", 9000000)

  const xtx = api.tx.circuit.onExtrinsicTrigger(
    encodedSfxId,
    encodedAmount
  )

  console.log("[2 - submitShouldSetToReady] Created xtx with onExtrinsicTrigger")

  await xtx.signAndSend(signer)

  console.log("[2 - submitShouldSetToReady] Signed and sent xtx")

  console.log('[2 - submitShouldSetToReady] XTX toHuman() -> ', xtx.toHuman())

  const confirmation = api.tx.circuit.confirmSideEffect(xtx)

  console.log('[2 - submitShouldSetToReady] Created confirmation tx -> ', confirmation.toHuman())


}





































// TEST -> Submit a sfx confirmation- moves to finished 
async function submitSfxConfirmationMovesToFinished() {
  const keyring = new Keyring({ type: 'sr25519' });
  const signer = process.env.CIRCUIT_KEY === undefined
    ? keyring.addFromUri("//Alice")
    : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk("ws://127.0.0.1:9944", signer)
  const api = await sdk.init();

  const encodedSfxId = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000010")
  const encodedAmount = api.createType("u128", 4000000)

  const xtx = api.tx.circuit.onExtrinsicTrigger(
    encodedSfxId,
    encodedAmount
  )

  console.log("[3 - submitSfxConfirmationMovesToFinished] Created xtx with onExtrinsicTrigger")

  await xtx.signAndSend(signer)

  console.log("[3 - submitSfxConfirmationMovesToFinished] Signed and sent xtx")


}

// TESTS -> Not submitting after bid moves to revert
async function notSubmitAfterBidMovesToRevert() {
  const keyring = new Keyring({ type: 'sr25519' });
  const signer = process.env.CIRCUIT_KEY === undefined
    ? keyring.addFromUri("//Alice")
    : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk("ws://127.0.0.1:9944", signer)
  const api = await sdk.init();

  const encodedSfxId = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000011")
  const encodedAmount = api.createType("u128", 4000000)

  const xtx = api.tx.circuit.onExtrinsicTrigger(
    encodedSfxId,
    encodedAmount
  )

  console.log("[4 - notSubmitAfterBidMovesToRevert] Created xtx with onExtrinsicTrigger")

  await xtx.signAndSend(signer)

  console.log("[4 - notSubmitAfterBidMovesToRevert] Signed and sent xtx")


}









// async function old() {
//   const wsProvider = new WsProvider('ws://127.0.0.1:9944');
//   const api = await ApiPromise.create({ provider: wsProvider });
//   const keyring = new Keyring({ type: 'sr25519' });

//   const alice = keyring.addFromUri('//Alice');
//   const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

//   // Create a extrinsic, transferring 12345 units to Bob
//   const transfer = api.tx.balances.transfer(BOB, 12345);

//   // const hash = await transfer.signAndSend(alice);
//   // console.log('Transfer sent with hash', hash.toHex());
//   const event = await signAndSendSafe(api, alice, transfer);
//   event['events'].forEach((e) => {
//     // @ts-ignore
//     console.log('event ->', e);
//   })

//   const encodedSfxId = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000011")
//   const encodedAmount = api.createType("u128", 4000000)
//   const xtx = api.tx.circuit.onExtrinsicTrigger(
//     encodedSfxId,
//     encodedAmount
//   )
//   const hashExtrinsic = await xtx.signAndSend(alice)

//   // Bid ?
//   const bid = api.tx.circuit.bid(encodedSfxId, encodedAmount)
//   const hashBid = await bid.signAndSend(alice)

//   console.log('bid hash -> ', hashBid.toHex())

// }