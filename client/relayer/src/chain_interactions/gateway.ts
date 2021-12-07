import { GetStorageArguments, StorageResult, TransferArguments } from '../utils/types';
import { ApiPromise, Keyring } from '@polkadot/api';
import { xxhashAsU8a } from '@polkadot/util-crypto';
import { u8aToHex } from '@polkadot/util';
import type { Hash } from '@polkadot/types/interfaces/runtime';
import { TransactionResult } from '../utils/types';
import { print_events } from '../utils/event_print';

export async function submit_transfer(api: ApiPromise, parameters: TransferArguments): Promise<TransactionResult> {
  return new Promise(async (resolve) => {
    console.log(`Submitting transfer`);
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    const signer =
      process.env.SIGNER_KEY === undefined
        ? keyring.addFromUri('//Alice')
        : keyring.addFromMnemonic(process.env.SIGNER_KEY);

    const unsub = await api.tx.balances.transfer(parameters.to, parameters.amount).signAndSend(signer, (result) => {
      if (result.status.isFinalized) {
        console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
        print_events(result.events);

        const extrinsicEvent = result.events.filter((item) => {
          return item.event.method === 'ExtrinsicSuccess' || item.event.method === 'ExtrinsicFailed';
        });

        // filter transfer event
        const transferEvent = result.events.filter((item) => {
          return item.event.method === 'Transfer';
        });

        unsub();

        resolve({
          blockHash: result.status.asFinalized as Hash,
          status: extrinsicEvent[0].event.method === 'ExtrinsicSuccess' ? true : false,
          events: transferEvent,
        });
      }
    });
  });
}

export async function getStorage(api: ApiPromise, parameters: GetStorageArguments): Promise<StorageResult> {
  return new Promise(async (resolve) => {
    let res = await api.rpc.state.getStorage(parameters.key);
    resolve({
      // @ts-ignore
      // { value: '0x1c86d8cbffffffffffffffffffffffff', status: true }
      // We may have to change it later down the line.
      value: res.toHex(),
      status: res !== undefined ? true : false,
    });
  });
}

function generateSystemEventKey() {
  return generateKeyForStorageValue('System', 'Events');
}

function generateKeyForStorageValue(module: string, variableName: string) {
  // lets prepare the storage key for system events.
  let module_hash = xxhashAsU8a(module, 128);
  let storage_value_hash = xxhashAsU8a(variableName, 128);

  // Special syntax to concatenate Uint8Array
  let final_key = new Uint8Array([...module_hash, ...storage_value_hash]);

  return u8aToHex(final_key);
}

export async function getEventProofs(api: ApiPromise, blockHash: any) {
  let key = generateSystemEventKey();
  let proofs = await api.rpc.state.getReadProof([key], blockHash);
  console.log(`getProofs : success : ${blockHash}`);
  return proofs;
}
