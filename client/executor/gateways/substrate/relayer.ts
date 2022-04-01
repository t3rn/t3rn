import { EventEmitter } from 'events'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { SideEffect } from '../../utils/sideEffectInterfaces';

export default class SubstrateRelayer extends EventEmitter {
    
    api: ApiPromise;
    id: string;
    rpc: string;
    signer: any

    async setup(rpc: string) {
        this.rpc = rpc;
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc)
        })

        const keyring = new Keyring({ type: 'sr25519' });

        this.signer =
            process.env.SIGNER_KEY === undefined
                ? keyring.addFromUri('//Alice')
                : keyring.addFromMnemonic(process.env.SIGNER_KEY);
    }

    async handleTx(sideEffect: SideEffect) {
        console.log(sideEffect)

        // const unsub = await api.tx.balances.transfer(parameters.to, parameters.amount).signAndSend(signer, (result) => {
        //     if (result.status.isFinalized) {
        //         console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
        //         // print_events(result.events);

        //         const extrinsicEvent = result.events.filter((item) => {
        //             return item.event.method === 'ExtrinsicSuccess' || item.event.method === 'ExtrinsicFailed';
        //         });

        //         // filter transfer event
        //         const transferEvent = result.events.filter((item) => {
        //             return item.event.method === 'Transfer';
        //         });
        //         // assert(transferEvent.length == 1, 'Multiple transfer events');

        //         unsub();

        //         // resolve({
        //         //     blockHash: result.status.asFinalized as Hash,
        //         //     status: extrinsicEvent[0].event.method === 'ExtrinsicSuccess' ? true : false,
        //         //     events: transferEvent,
        //         // });
        //     }
        // });
    // });
       
    }

}