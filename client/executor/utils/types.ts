import '@t3rn/types';
import { AccountId32, H256,  } from '@polkadot/types/interfaces/runtime';
import { T3rnPrimitivesSideEffect, T3rnPrimitivesSideEffectConfirmedSideEffect } from '@polkadot/types/lookup';
import { TextDecoder } from 'util';
import { match } from 'assert';
const BN = require("bn.js");

export enum TransactionType {
    Transfer,
    Swap,
}

export class SideEffectStateManager {
    requester: AccountId32;
    executor: AccountId32;
    xtxId: H256;
    sideEffect: T3rnPrimitivesSideEffect;
    confirmedSideEffect: T3rnPrimitivesSideEffectConfirmedSideEffect;

    inclusionProof: any;
    blockHeader: any;

    transactionType: TransactionType;
    execd: boolean;
    confirmed: boolean;

    setRequester(requester: AccountId32) {
        this.requester = requester;
    }

    setXtxId(xtxId: H256) {
        this.xtxId = xtxId;
    }

    setSideEffect(sideEffect: T3rnPrimitivesSideEffect) {
        this.sideEffect = sideEffect;
        this.setTransactionType()
    }

    getTransactionArguments() {
        switch(this.transactionType) {
            case TransactionType.Transfer: {
                return this.getTransferArguments()
            }
            case TransactionType.Swap: {
                return []
            }
            
        }

    }

    executed(encodedEffect: any, blockNumber: number, executioner: any, inclusionProof: any, blockHeader: any, execd: boolean) {
        this.confirmedSideEffect = <any>{
            err: null,
            output: null,
            encodedEffect: encodedEffect,
            inclusionProof: null,
            executioner: executioner,
            receivedAt: blockNumber,
            cost: null,
        };

        this.execd = execd;
        this.executor = executioner;
        this.inclusionProof = inclusionProof;
        this.blockHeader = blockHeader;
    }

    /// returns xtxId as string
    getId() {
        return this.xtxId.toString()
    }

    /// returns target as string
    getTarget() {
        return new TextDecoder().decode(this.sideEffect.target.toU8a());
    }

    private getTransferArguments() {
        return [this.sideEffect.encodedArgs[1], new BN(this.sideEffect.encodedArgs[2], "le").toString()]
    }

    private setTransactionType() {
        switch (this.sideEffect.encodedAction.toHuman()) {
            case "tran": {
                this.transactionType = TransactionType.Transfer;
                break;
            }
            case "swap": {
                this.transactionType = TransactionType.Swap;
                console.log("set swap")
                break;
            }
        }

    }
}