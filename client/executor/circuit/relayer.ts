import { EventEmitter } from 'events'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { SideEffect } from '../utils/sideEffectInterfaces';
import { getEventProofs } from "./utils/helpers"
// import { generateConfirmedSideEffect } from 'executor/utils/types';
export default class CircuitRelayer extends EventEmitter {

    api: ApiPromise;
    id: string;
    rpc: string;
    signer: any;

    async setup(rpc: string) {
        this.rpc = rpc;
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc),
        })

        const keyring = new Keyring({ type: 'sr25519' });

        this.signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri('//Alice')
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY);
    }

    async confirmSideEffect(sideEffect: SideEffect, transactionObject: any) {

        // let confirmedSideEffect = generateConfirmedSideEffect()


        // const inclusionProof = this.api.createType('Option<Bytes>', transactionObject.inclusionProofs);
        let confirmedSideEffect  = this.api.createType('ConfirmedSideEffect', {
            err: this.api.createType('Option<Bytes>', []),
            output: this.api.createType('Option<Bytes>', []),
            encoded_effect: this.api.createType('Bytes', transactionObject.event),
            inclusionProof: this.api.createType('Option<Bytes>', []),
            executioner: this.api.createType('AccountId', sideEffect.sender),
            received_at: this.api.createType('BlockNumber', 1),
            cost: this.api.createType('Option<BalanceOf>', ''),
        });

        let cse = {
            err: null,
            output: null,
            encoded_effect: transactionObject.event,
            inclusion_proof: null,
            executioner: transactionObject.executioner,
            blocknumber: 1,
            cost: null
        }

        let circuitSideEffect = {
            target: sideEffect.target.toHuman(),
            prize: sideEffect.prize.toHuman(),
            ordered_at: sideEffect.orderedAt.toHuman(),
            encoded_action: sideEffect.encodedAction.toHuman(),
            encoded_args: [sideEffect.decodedArgs.from, sideEffect.decodedArgs.to, sideEffect.decodedArgs.amount],
            signature: sideEffect.signature.toHuman(),
            enforce_executioner: sideEffect.enforceExecutioner.toHuman(),
        }

        // console.log(sideEffect.xtxId.toHuman())
    

        let tx = this.api.tx.circuit.confirmSideEffect(sideEffect.xtxId, circuitSideEffect, cse, null, null)
            // , circuitSideEffect);
            // , confirmedSideEffect, this.api.createType('Option<Vec<Bytes>>', []), this.api.createType('Option<Bytes>', []));
        let unsub = await tx.signAndSend(this.signer, (result) => {
            if (result.status.isFinalized) {
                console.log(`Transaction ConfirmedSideEffect finalized at blockHash ${result.status.asFinalized}`);
                // print_events(result.events);
                const extrinsicEvent = result.events.filter((item) => {
                    return item.event.method === 'ExtrinsicSuccess' || item.event.method === 'ExtrinsicFailed';
                });

                // console.log(extrinsicEvent.toHuman())

                // console.log({
                //     blockHash: result.status.asFinalized,
                //     status: extrinsicEvent[0].event.method === 'ExtrinsicSuccess' ? true : false,
                //     events: result.events,
                // })
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