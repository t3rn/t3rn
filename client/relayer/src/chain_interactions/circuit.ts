import { Bytes } from '@polkadot/types';
import { ApiPromise, Keyring } from '@polkadot/api';
import type {AccountId } from '@polkadot/types/interfaces/runtime';
import '@t3rn/types/dist/augment-api';
import '@t3rn/types/dist/augment-types';
import '@t3rn/types/dist/augment-api-rpc';
import { TransactionResult } from '../utils/types';
import type { Hash } from '@polkadot/types/interfaces/runtime';
import { XtxId } from 'types/src/interfaces/execution_delivery/types';
import { ConfirmedSideEffect, SideEffect } from 'types/src/interfaces/primitives/types';
import { print_events } from '../utils/event_print';

export async function send_tx_confirm_side_effect(
    api: ApiPromise, 
    requester: AccountId,
    xtx_id: XtxId,
    sideEffect: SideEffect, 
    inclusion_proofs: Bytes,
    encoded_effect: Bytes): Promise<TransactionResult> {
    return new Promise(async resolve => {
        // ToDo : Replace with real signer
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');

        let confirmed_side_effect: ConfirmedSideEffect = api.createType("ConfirmedSideEffect",
            {
                err: api.createType('Option<Bytes>', []),
                output: api.createType('Option<Bytes>', []),
                encoded_effect: api.createType('Bytes', encoded_effect),
                inclusion_proof: api.createType('Option<Bytes>', inclusion_proofs),
                executioner: api.createType('AccountId', requester),
                received_at: api.createType('BlockNumber', 1),
                cost: api.createType('Option<BalanceOf>', 2)
            });

        let tx = api.tx.execDelivery.confirmSideEffectBlind(
            xtx_id,
            sideEffect,
            confirmed_side_effect,
            []);
        let unsub = await tx.signAndSend(alice, (result => {
            if (result.status.isFinalized) {
                console.log(`Transaction ConfirmedSideEffect finalized at blockHash ${result.status.asFinalized}`);
                print_events(result.events);
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