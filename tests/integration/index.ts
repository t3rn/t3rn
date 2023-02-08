import { ApiPromise, WsProvider } from "@polkadot/api"
// import { BlockHash } from "@polkadot/types/interfaces"
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


// Noah's

const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

async function main() {
  // New provider instance for local zombienet
  const wsProvider = new WsProvider('ws://127.0.0.1:9900');

  // Instantiate the API
  const api = await ApiPromise.create({ provider: wsProvider });

  // Construct the keyring after the API (crypto has an async init)
  const keyring = new Keyring({ type: 'sr25519' });

  // Add Alice to our keyring with a hard-derivation path (empty phrase, so uses dev)
  const alice = keyring.addFromUri('//Alice');

  // Create a extrinsic, transferring 12345 units to Bob
  const transfer = api.tx.balances.transfer(BOB, 12345);

  // Sign and send the transaction using our account
  const hash = await transfer.signAndSend(alice);
  console.log('Transfer sent with hash', hash.toHex());

  // From @chiefbiiko <3
  const event = await signAndSendSafe(api, alice, transfer);
  console.log('event ->',
    // event['events']
    // JSON.stringify(event)
    // JSON.stringify(event['events'])
    JSON.parse(
      JSON.stringify(
        event['events'],
        (key, value) => typeof value === 'bigint' ? value.toString() : value
      )
    )
  );

}

main().catch(console.error).finally(() => process.exit());