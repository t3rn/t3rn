import "@t3rn/types"
import { H256 } from '@polkadot/types/interfaces';
import { T3rnTypesSideEffect } from '@polkadot/types/lookup';
import { pub2Address } from "./converters/substrate";

const BN = require('bn.js')

// contains the different side_effect types
export enum TransactionType {
    Transfer,
    Swap,
}

// maps event names to TransactionType enum;
export const EventMapper = ["Transfer", "MultiTransfer"]

export enum SideEffectStatus {
    Invalid,
    Open,
    IAmExecuting,
    SomeoneIsExecuting,
    ConfirmedOnTarget,
    SideEffectConfirmed,
    AllStepsConfirmed
}

export class SideEffect {
    status: SideEffectStatus;
    action: TransactionType;
    target: string;
    hasInsurance: boolean;

    // SideEffect data
    id: H256;
    xtxId: H256;
    arguments: string[];
    raw: T3rnTypesSideEffect;

    // TargetConfirmation
    payload: any; // this could be the transfer event, storage entry, etc
    proof: any;
    blockHeight: number;
    blockHash: string;
    executor: string;

    constructor(sideEffect: T3rnTypesSideEffect, id: H256, xtxId: H256) {
        if(this.knownTransactionInterface(sideEffect.encodedAction)) {
            this.raw = sideEffect;
            this.id = id;
            this.xtxId = xtxId
            this.arguments = sideEffect.encodedArgs.map(entry => entry.toString());
            this.hasInsurance = this.checkForInsurance(this.arguments.length, this.action)
        } else {
            console.log("SideEffect interface unknown!!")
        }
    }

    // ToDo remove once merged https://github.com/t3rn/t3rn/issues/432
    checkForInsurance(argsLength: number, action: TransactionType): boolean {
        switch(action) {
            case TransactionType.Transfer: {
                return argsLength === 4;
                break;
            }
            case TransactionType.Swap: {
                return argsLength === 5;
                break;
            }
        }
    }

    updateStatus(status: SideEffectStatus) {
        this.status = status;
    }

    execute(): any[] | void {
        if(this.status === SideEffectStatus.Open) {
            switch(this.action) {
                case TransactionType.Transfer: {
                    return this.getTransferArguments()
                }
                case TransactionType.Swap: {
                    return []
                }
            }
        } else {
            console.log("SideEffect not open!")
            console.log(this.id.toHex())
        }
    }

    executionConfirmed(proof: any, payload: any, executor: string, blockHeight: number, blockHash: string) {
        this.proof = proof;
        this.payload = payload;
        this.executor = executor;
        this.blockHeight = blockHeight;
        this.blockHash = blockHash;
        this.status = SideEffectStatus.ConfirmedOnTarget;
    }

    private knownTransactionInterface(encodedAction: any): boolean {
        switch(encodedAction.toHuman()) {
            case "tran": {
                this.status = SideEffectStatus.Open;
                this.action = TransactionType.Transfer
                return true
                break;
            }
            default: {
                this.status = SideEffectStatus.Invalid;
                return false;
            }
        }
    }

    // returns the arguments
    private getTransferArguments() {
        return [
            // ToDo query prefix from xdns
            this.arguments[1],
            new BN(this.arguments[2], "le").toString(),
        ]
    }
}