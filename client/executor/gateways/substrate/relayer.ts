import { EventEmitter } from 'events'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { T3rnPrimitivesSideEffect } from '@polkadot/types/lookup';
import { SideEffectStateManager, TransactionType } from "../../utils/types"
const BN = require("bn.js");
import { getEventProofs } from './utils/helper';
import { threadId } from 'worker_threads';

export default class SubstrateRelayer extends EventEmitter {

    debug: any;
    api: ApiPromise;
    id: string;
    rpc: string;
    signer: any

    async setup(rpc: string, name: string) {
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

    async executeTx(sideEffectStateManager: SideEffectStateManager) {
        switch (sideEffectStateManager.transactionType) {
            case TransactionType.Transfer: {
                const unsub = await this.api.tx.balances.transfer(...sideEffectStateManager.getTransactionArguments()).signAndSend(this.signer, async (result) => {
                    if (result.status.isFinalized) {
                        console.log("is finalized")
                        this.handleTx(sideEffectStateManager, result, unsub);
                    }
                })
            }
            case TransactionType.Swap: {
                return null;
            }
        }


        // this replaces a SCALE decode, which is hard to use here because the arguments are just bytes. 
        // const amount = new BN(sideEffect.encodedArgs[2], "le");
        // console.log("Amount:", amount.toString());
        // const unsub = await this.api.tx.balances.transfer(sideEffect.encodedArgs[1], amount.toString()).signAndSend(this.signer, async (result) => {
        //     if (result.status.isFinalized) {
        //         const blockHash = result.status.asFinalized;
        //         const blockNumber = await this.getBlockNumber(blockHash);

                
                
        //         const event = result.events.find((item) => {
        //             return item.event.method === 'Transfer';
        //         }).event.toHex();
                
        //         if(!event) {
        //             console.error("No Transfer Event found");
        //             unsub()
        //         }
                
        //         // should always be last event
        //         const success = result.events[result.events.length - 1].event.method === "ExtrinsicSuccess";
        //         const inclusionProof = await getEventProofs(this.api, blockHash);

        //         console.log("Transaction Successful:", success)
        //         console.log(`Transaction finalized at blockHash ${blockHash}`);

        //         let completionData = success ? <CompletionData>{
        //             success,
        //             blockHash,
        //             blockNumber,
        //             event,
        //             inclusionProof
        //         } : <CompletionData>{
        //             success
        //         }
                
        //         this.emit("txFinalized", completionData)

        //         unsub();
        //     }
        // });
       
    }

    async handleTx(sideEffectStateManager: SideEffectStateManager, result, unsub) {
        if (result.status.isFinalized) {
            const blockHeader = result.status.asFinalized;
            const blockNumber = await this.getBlockNumber(blockHeader);
            
            const event = result.events.find((item) => {
                return item.event.method === 'Transfer';
            }).event;
            
            if(!event) {
                console.error("No Transfer Event found");
            }
            
            // should always be last event
            const success = result.events[result.events.length - 1].event.method === "ExtrinsicSuccess";
            const inclusionProof = await getEventProofs(this.api, blockHeader);

            sideEffectStateManager.executed(
                event,
                blockNumber,
                this.signer.address,
                inclusionProof,
                blockHeader,
                success
            )

            console.log("Transaction Successful:", success)
            console.log(`Transaction finalized at blockHash ${blockHeader}`);

            this.emit("txFinalized", sideEffectStateManager.getId())
            
            unsub();
        }
    }

    async getBlockNumber(hash: any) {
        return (await this.api.rpc.chain.getHeader(hash)).number.toNumber()
    }
}