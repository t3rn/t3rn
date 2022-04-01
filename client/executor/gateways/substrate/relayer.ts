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
        const unsub = await this.api.tx.balances.transfer(sideEffect.encodedArgs.to, sideEffect.encodedArgs.amount).signAndSend(this.signer, (result) => {
            if (result.status.isFinalized) {
                console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);

                // should always be last event
                const success = result.events[result.events.length - 1].event.method === "ExtrinsicSuccess";
                console.log("Transaction Successful:", success)

                // find transfer event
                const transferEvent = result.events.find((item) => {
                    return item.event.method === 'Transfer';
                });

                if(success) {
                    this.emit("txFinalized", {
                        blockHash: result.status.asFinalized,
                        events: transferEvent,
                        xtxId: sideEffect.xtxId
                    })
                }

                unsub();
            }
        });
       
    }

}