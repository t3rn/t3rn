import { EventEmitter } from 'events'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { SideEffect } from '../utils/sideEffectInterfaces';
import { getEventProofs } from "./utils/helpers"
import types from "../types.json";

export default class CircuitRelayer extends EventEmitter {

    api: ApiPromise;
    id: string;
    rpc: string;
    signer: any;

    async setup(rpc: string) {
        this.rpc = rpc;
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc),
            types: types as any,
        })

        const keyring = new Keyring({ type: 'sr25519' });

        this.signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri('//Alice')
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY);
    }

    async confirmSideEffect(sideEffect: SideEffect, transactionObject: any) {


        const inclusionProof = this.api.createType('Option<Bytes>', transactionObject.inclusionProofs);
        let confirmedSideEffect  = this.api.createType('ConfirmedSideEffect', {
            err: this.api.createType('Option<Bytes>', []),
            output: this.api.createType('Option<Bytes>', []),
            encoded_effect: this.api.createType('Bytes', transactionObject.event.toHex()),
            inclusionProof: this.api.createType('Option<Bytes>', []),
            executioner: this.api.createType('AccountId', sideEffect.sender),
            received_at: this.api.createType('BlockNumber', 1),
            cost: this.api.createType('Option<BalanceOf>', ''),
        });

        let circuitSideEffect = this.api.createType('SideEffect', (
            this.api.createType('TargetId', sideEffect.target),
            this.api.createType('BalanceOf', sideEffect.prize),
            this.api.createType('BlockNumber', sideEffect.orderedAt),
            this.api.createType('Bytes', sideEffect.encodedAction),
            this.api.createType('Vec<Bytes>', sideEffect.encodedArgs),
            this.api.createType('Bytes', sideEffect.signature),
            this.api.createType('Option<AccountId>', sideEffect.enforceExecutioner)
        ));

       

        let tx = this.api.tx.circuit.confirmSideEffect(sideEffect.xtxId, circuitSideEffect);
            // , confirmedSideEffect, this.api.createType('Option<Vec<Bytes>>', []), this.api.createType('Option<Bytes>', []));
        let unsub = await tx.signAndSend(this.signer, (result) => {
            if (result.status.isFinalized) {
                console.log(`Transaction ConfirmedSideEffect finalized at blockHash ${result.status.asFinalized}`);
                // print_events(result.events);
                const extrinsicEvent = result.events.filter((item) => {
                    return item.event.method === 'ExtrinsicSuccess' || item.event.method === 'ExtrinsicFailed';
                });

                console.log(extrinsicEvent)

                console.log({
                    blockHash: result.status.asFinalized,
                    status: extrinsicEvent[0].event.method === 'ExtrinsicSuccess' ? true : false,
                    events: result.events,
                })
                unsub();

                // // there can only be one event
                // resolve({
                //     blockHash: result.status.asFinalized as Hash,
                //     status: extrinsicEvent[0].event.method === 'ExtrinsicSuccess' ? true : false,
                //     events: result.events,
                // });
            }
        });

    }

}