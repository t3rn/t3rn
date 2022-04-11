import '@t3rn/types';
import { AccountId32, H256,  } from '@polkadot/types/interfaces/runtime';
import { T3rnPrimitivesSideEffect, } from '@polkadot/types/lookup';
import { TextDecoder } from 'util';
const crypto = require("crypto");
const BN = require("bn.js");


export enum TransactionType {
    Transfer,
    Swap,
}

// maps event names to TransactionType enum;
export const EventMapper = ["Transfer", "MultiTransfer"]

export class SideEffectStateManager {
    requester: AccountId32;
    executor: AccountId32;
    xtxId: H256;
    sideEffect: T3rnPrimitivesSideEffect;
    confirmedSideEffect: object;

    inclusionProof: any;
    execBlockHeader: any;

    transactionType: TransactionType;
    executed: boolean;
    confirmed: boolean;
    confirmBlockHeader: any;

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

    execute(encodedEffect: any, blockNumber: number, executioner: any, inclusionProof: any, blockHeader: any, executed: boolean) {
        this.confirmedSideEffect = {
            err: null,
            output: null,
            encodedEffect: encodedEffect,
            inclusionProof: null,
            executioner: executioner,
            receivedAt: blockNumber,
            cost: null,
        };

        this.executed = executed;
        this.executor = executioner;
        this.inclusionProof = inclusionProof;
        this.execBlockHeader = blockHeader;
    }

    confirm(confirmed: boolean, blockHeader: any) {
        this.confirmed = confirmed;
        this.confirmBlockHeader = blockHeader;
    }

    getId() {
        return crypto.createHash('sha256').update(JSON.stringify(this.sideEffect)).digest('hex');
    }

    /// returns xtxId as string
    getXtxId() {
        return this.xtxId.toString()
    }

    /// returns target as string
    getTarget() {
        return new TextDecoder().decode(this.sideEffect.target.toU8a());
    }

    getTargetBlock() {
        if(!this.executed) return null;
        // @ts-ignore
        return this.confirmedSideEffect.receivedAt;
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
                break;
            }
        }

    }
}