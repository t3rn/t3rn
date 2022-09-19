import "@t3rn/types"
import { T3rnTypesSideEffect } from '@polkadot/types/lookup';
import { TextDecoder } from "util"
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
    WaitingForInsurance,
    ReadyForExec,
    ExecutedOnTarget,
    SideEffectConfirmed,
}

export class SideEffect {
    step: number;
    status: SideEffectStatus;
    action: TransactionType;
    target: string;
    hasInsurance: boolean;
    iAmExecuting: boolean;

    // SideEffect data
    id: string;
    xtxId: string;
    arguments: string[];
    raw: T3rnTypesSideEffect;

    // TargetConfirmation
    inclusionData: any; // contains encoded payload, inclusionProof, and blockHash
    targetInclusionHeight: any;
    executor: string;

    constructor(sideEffect: T3rnTypesSideEffect, id: string, xtxId: string) {
        if(this.knownTransactionInterface(sideEffect.encodedAction)) {
            this.raw = sideEffect;
            this.id = id;
            this.xtxId = xtxId
            this.arguments = sideEffect.encodedArgs.map(entry => entry.toString());
            this.hasInsurance = this.checkForInsurance(this.arguments.length, this.action)
            this.target =  new TextDecoder().decode(sideEffect.target.toU8a())
        } else {
            console.log("SideEffect interface unknown!!")
        }
    }

    // sets the step of the sideEffect in its execution
    setStep(step: number) {
        this.step = step
    }

    // ToDo remove once merged https://github.com/t3rn/t3rn/issues/432
    checkForInsurance(argsLength: number, action: TransactionType): boolean {
        switch(action) {
            case TransactionType.Transfer: {
                if(argsLength === 4) {
                    this.status = SideEffectStatus.WaitingForInsurance;
                    return true;
                } else {
                    // if the sfx is dirty, its ready on creation.
                    this.status = SideEffectStatus.ReadyForExec;
                    this.iAmExecuting = true; // Dirty sfx can always be executed without bond
                    return false
                }
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

    // return an array of arguments to execute on target.
    execute(): any[] | void {
        switch(this.action) {
            case TransactionType.Transfer: {
                return this.getTransferArguments()
            }
            case TransactionType.Swap: {
                return []
            }
        }
    }

    // updates status
    insuranceBonded(iAmExecuting: boolean) {
        this.status = SideEffectStatus.ReadyForExec;
        this.iAmExecuting = iAmExecuting;
    }

    // sfx was successfully executed on target and has the inclusion proof data
    executedOnTarget(inclusionData: any, executor: any, targetInclusionHeight: any) {
        this.inclusionData = inclusionData;
        this.executor = executor;
        this.targetInclusionHeight = targetInclusionHeight;
        this.status = SideEffectStatus.ExecutedOnTarget;
    }

    // ensure we can deal with the sfx action and set TransactionType
    private knownTransactionInterface(encodedAction: any): boolean {
        switch(encodedAction.toHuman()) {
            case "tran": {
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
    private getTransferArguments(): string[] {
        return [
            // ToDo query prefix from xdns
            this.arguments[1],
            new BN(this.arguments[2].split("0x")[1], 16,"le").toString(),
        ]
    }
}