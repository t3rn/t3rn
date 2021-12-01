import { EventRecord } from '@polkadot/types/interfaces/system';
import { ReadProof } from '@polkadot/types/interfaces/state';
import { Bytes, Vec } from '@polkadot/types';
import { ApiPromise, Keyring } from '@polkadot/api';
import type {AccountId } from '@polkadot/types/interfaces/runtime';
import '@t3rn/types/dist/augment-api';
import '@t3rn/types/dist/augment-types';
import '@t3rn/types/dist/augment-api-rpc';
import { TransactionResult } from '../utils/types';
import type { Hash } from '@polkadot/types/interfaces/runtime';
import { XtxId } from 'types/src/interfaces/execution_delivery/types';
import { ConfirmedSideEffect, SideEffect } from 'types/src/interfaces/primitives/types';

function print_events(events : EventRecord[])
{
    events.forEach((record: { event: any; phase: any; }) => {
        // Extract the phase, event and the event types
        const { event, phase } = record;
        const types = event.typeDef;

        console.log(`\t${event.section}:${event.method}`);
        event.data.forEach((data: { toString: () => any; }, index: string | number) => {
          console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
        });
      });
}

export async function send_tx_confirm_side_effect(
    api: ApiPromise, 
    requester: AccountId,
    xtx_id: XtxId,
    sideEffect: SideEffect, 
    proofs: ReadProof): Promise<TransactionResult> {
    return new Promise(async resolve => {
        // ToDo : Replace with real signer
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');

        let confirmed_side_effect: ConfirmedSideEffect = api.createType("ConfirmedSideEffect",
            {
                err: api.createType('Option<Bytes>', []),
                output: api.createType('Option<Bytes>', []),
                encoded_effect: api.createType('Bytes', []),
                inclusion_proof: api.createType('Option<Bytes>', []),
                executioner: api.createType('AccountId', requester),
                received_at: api.createType('BlockNumber', 1),
                cost: api.createType('Option<BalanceOf>', 2)
            });

        let tx = api.tx.execDelivery.confirmSideEffectBlind(
            xtx_id,
            sideEffect,
            confirmed_side_effect,
            proofs.proof);
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