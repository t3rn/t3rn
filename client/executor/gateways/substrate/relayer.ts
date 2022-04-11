import { EventEmitter } from 'events'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { SideEffect, TransactionType, EventMapper } from "../../utils/types"
import { getEventProofs } from './utils/helper';
import chalk from 'chalk';

export default class SubstrateRelayer extends EventEmitter {

    debug: any;
    api: ApiPromise;
    id: string;
    rpc: string;
    signer: any;
    color: string;
    name: string;

    log(msg: string) {
        console.log(chalk[this.color](this.name + " - "), msg)
    }

    async setup(rpc: string, name: string, color: string) {
        this.rpc = rpc;
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc)
        })
        
        const keyring = new Keyring({ type: 'sr25519' });
        
        this.signer =
        process.env.SIGNER_KEY === undefined
        ? keyring.addFromUri('//Alice')
        : keyring.addFromMnemonic(process.env.SIGNER_KEY);
        
        this.color = color;
        this.name = name;
    }

    async executeTx(sideEffect: SideEffect) {
        switch (sideEffect.transactionType) {
            case TransactionType.Transfer: {
                const unsub = await this.api.tx.balances.transfer(...sideEffect.getTransactionArguments()).signAndSend(this.signer, async (result) => {
                    if (result.status.isFinalized) {
                        this.handleTx(sideEffect, result, unsub);
                    }
                })
            }
            case TransactionType.Swap: {
                return null;
            }
        }
    }

    async handleTx(sideEffect: SideEffect, result, unsub) {
        if (result.status.isFinalized) {
            const blockHeader = result.status.asFinalized;
            const blockNumber = await this.getBlockNumber(blockHeader);
            const event = this.getEvent(sideEffect.transactionType, result.events);

            // should always be last event
            const success = result.events[result.events.length - 1].event.method === "ExtrinsicSuccess";
            const inclusionProof = await getEventProofs(this.api, blockHeader);

            sideEffect.execute(
                event,
                blockNumber,
                this.signer.address,
                inclusionProof,
                blockHeader,
                success
            )

            this.log(`SideEffect Executed: ${success}, ${blockHeader}`)
            // console.log(`Transaction finalized at blockHash ${blockHeader}`);

            this.emit("SideEffectExecuted", sideEffect.getId())
            
            unsub();
        }
    }

    async getBlockNumber(hash: any) {
        return (await this.api.rpc.chain.getHeader(hash)).number.toNumber()
    }

    getEvent(transactionType: TransactionType, events: any[]) {
        const event = events.find((item) => {
            return item.event.method === EventMapper[transactionType];
        })

        if(event) return event.event.toHex()

        console.log("Transaction Successful but correct event not found!")
        console.log("This could indicate an empty transaction value")
    }

}