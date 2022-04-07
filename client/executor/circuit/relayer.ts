import { EventEmitter } from 'events'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { SideEffectStateManager } from '../utils/types';
import chalk from 'chalk';

export default class CircuitRelayer extends EventEmitter {

    api: ApiPromise;
    id: string;
    rpc: string;
    signer: any;
    color: string;

    log(msg: string) {
        console.log(chalk[this.color]("Circuit - "), msg)
    }

    async setup(rpc: string, color: string) {
        this.rpc = rpc;
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc),
        })

        this.color = color;
        
        const keyring = new Keyring({ type: 'sr25519' });

        this.signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri('//Alice')
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY);
    }

    async confirmSideEffect(sideEffectStateManager: SideEffectStateManager) {

        let tx = this.api.tx.circuit.confirmSideEffect(
            sideEffectStateManager.xtxId, 
            sideEffectStateManager.sideEffect, 
            sideEffectStateManager.confirmedSideEffect, 
            null, 
            null
        )

        let unsub = await tx.signAndSend(this.signer, (result) => {
            if (result.status.isFinalized) {
                const success = result.events[result.events.length - 1].event.method === "ExtrinsicSuccess";
                this.log(`SideEffect confirmed: ${success}, ${result.status.asFinalized}`)

                sideEffectStateManager.confirm(success, result.status.asFinalized)
               
                unsub();
            }
        });

    }

}