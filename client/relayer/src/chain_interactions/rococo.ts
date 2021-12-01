import { TransferArguments } from './../utils/types';
import { ApiPromise, Keyring } from '@polkadot/api';
import { xxhashAsU8a } from '@polkadot/util-crypto';
import { u8aToHex } from '@polkadot/util';
import { TypeRegistry } from '@polkadot/types';
import type { Hash } from '@polkadot/types/interfaces/runtime';
import { TransactionResult } from '../utils/types';

export async function submit_transfer(api: ApiPromise, parameters: TransferArguments): Promise<TransactionResult> {
  return new Promise(async resolve => {
    console.log(`Submitting transfer`);
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    const unsub = await api.tx.balances
      .transfer(parameters.to, parameters.amount)
      .signAndSend(alice, (result) => {
        console.log(`Current status is ${result.status}`);

        if (result.status.isFinalized) {
          console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);

          const extrinsicEvent = result.events.filter((item) => {
            return (item.event.method === 'ExtrinsicSuccess' || item.event.method === 'ExtrinsicFailed');
          });

          unsub();

          // there can only be one event
          resolve({
            'blockHash': result.status.asFinalized as Hash,
            'status': (extrinsicEvent[0].event.method === 'ExtrinsicSuccess') ? true : false,
          });
        }

      });
  });
}

function generateSystemEventKey() {
  // lets prepare the storage key for system events.
  let module_hash = xxhashAsU8a('System', 128);
  let storage_value_hash = xxhashAsU8a('Events', 128);

  // Special syntax to concatenate Uint8Array
  let final_key = new Uint8Array([
    ...module_hash,
    ...storage_value_hash,
  ]);

  return u8aToHex(final_key);

}

export async function getEventProofs(api: ApiPromise, blockHash: any) {
  let key = generateSystemEventKey();
  console.log(key);
  let proofs = await api.rpc.state.getReadProof([key], blockHash);
  console.log(`getProofs : success : ${blockHash}`);
  return proofs;
}