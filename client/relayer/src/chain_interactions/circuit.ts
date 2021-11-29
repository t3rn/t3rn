import { Bytes, Vec } from '@polkadot/types';
import { ApiPromise, Keyring } from '@polkadot/api';
import '@t3rn/types/dist/augment-api';
import '@t3rn/types/dist/augment-types';
import '@t3rn/types/dist/augment-api-rpc';
import { TransactionResult } from '../utils/types';
import type { Hash } from '@polkadot/types/interfaces/runtime';

export async function send_tx_confirm_side_effect(api: ApiPromise, proofs: Vec<Bytes>): Promise<TransactionResult> {
    return new Promise(async resolve => {
        // ToDo : Replace with real signer
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');

        let tx = api.tx.execDelivery.confirmSideEffectBlind(
            api.createType("XtxId", "xtxId"),
            api.createType("ConfirmedSideEffect",
                {
                    err: api.createType('Option<Bytes>', []),
                    output: api.createType('Option<Bytes>', []),
                    encoded_effect: api.createType('Bytes', []),
                    inclusion_proof: api.createType('Option<Bytes>', []),
                    executioner: api.createType('AccountId', "account_id"),
                    received_at: api.createType('BlockNumber', 1),
                    cost: api.createType('Option<BalanceOf>', 2)
                }),
            proofs);
        let unsub = await tx.signAndSend(alice, (result => {
            if (result.status.isFinalized) {
                console.log(`Transaction ConfirmedSideEffect finalized at blockHash ${result.status.asFinalized}`);

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
        }))
    });
}