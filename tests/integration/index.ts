import { ApiPromise, WsProvider } from "@polkadot/api"
import { Keyring } from '@polkadot/keyring'
import { SubmittableExtrinsic } from "@polkadot/api/types"
import { KeyringPair } from "@polkadot/keyring/types"

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
  // New provider instance for local zombienet
  const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider: wsProvider });
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');
  const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

  // Create a extrinsic, transferring 12345 units to Bob
  // const transfer = api.tx.balances.transfer(BOB, 12345);
  // const hash = await transfer.signAndSend(alice);
  // console.log('Transfer sent with hash', hash.toHex());  
  // const event = await signAndSendSafe(api, alice, transfer);
  // event['events'].forEach((e: any) => {
  //   console.log('[TRANSFER] event ->', e);
  // })

  // // Create a bid on an SFX and send it
  // const encodedSfxId = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000000")
  // const encodedAmount = api.createType("u128", 1)
  // const bidSfx = api.tx.circuit.bidSfx(encodedSfxId, encodedAmount)
  // const event = await bidSfx.signAndSend(alice)
  // console.log('[BIDSFX] event -> ', event)

  //
  //
  // TESTS TESTS TESTS
  // TESTS TESTS TESTS
  // TESTS TESTS TESTS
  //
  //
  // Test
  // Submit an sfx - no bid should kill
  const encodedSfxId1 = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000001")
  const encodedAmount1 = api.createType("u128", 0)
  const bidSfx1 = api.tx.circuit.bidSfx(encodedSfxId1, encodedAmount1)

  console.log('[BIDSFX] BEFORE bidSfx itself -> ', bidSfx1)
  console.log('[BIDSFX] BEFORE bidSfx unwrap -> ', bidSfx1.unwrap())


  const event1 = await bidSfx1.signAndSend(alice)

  console.log('[BIDSFX] Hash of the signAndSend -> ', event1.toHuman())

  console.log('[BIDSFX] bidSfx itself -> ', bidSfx1)
  console.log('[BIDSFX] bidSfx unwrap -> ', bidSfx1.unwrap())


  // Test
  // Submit a sfx - bid should set to ready
  // const encodedSfxId2 = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000002")
  // const encodedAmount2 = api.createType("u128", 1)
  // const bidSfx2 = api.tx.circuit.bidSfx(encodedSfxId2, encodedAmount2)
  // const event2 = await bidSfx2.signAndSend(alice)
  // console.log('[BIDSFX] event -> ', event2)

  // Test
  // Submit a sfx confirmation - moves to finished
  // const encodedSfxId2 = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000002")
  // const encodedAmount2 = api.createType("u128", 1)
  // const bidSfx2 = api.tx.circuit.bidSfx(encodedSfxId2, encodedAmount2)
  // const event2 = await bidSfx2.signAndSend(alice)
  // console.log('[BIDSFX] event -> ', event2)

  // Test
  // Not submitting after bid moves to revert
  // const encodedSfxId2 = api.createType("Hash", "0x0000000000000000000000000000000000000000000000000000000000000002")
  // const encodedAmount2 = api.createType("u128", 1)
  // const bidSfx2 = api.tx.circuit.bidSfx(encodedSfxId2, encodedAmount2)
  // const event2 = await bidSfx2.signAndSend(alice)
  // console.log('[BIDSFX] event -> ', event2)

}

main().catch(console.error).finally(() => process.exit());