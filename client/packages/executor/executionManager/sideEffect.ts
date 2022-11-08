import "@t3rn/types"
// @ts-ignore
import { T3rnTypesSideEffect } from '@polkadot/types/lookup';
import { TextDecoder } from "util"
const BN = require('bn.js')
import {SfxType, SfxStatus, SecurityLevel} from "@t3rn/sdk/dist/src/side-effects/types";
import {Sdk} from "@t3rn/sdk";

// maps event names to SfxType enum;
export const EventMapper = ["Transfer", "MultiTransfer"]


export class SideEffect {
    step: number;
    status: SfxStatus;
    action: SfxType;
    target: string;
    hasInsurance: boolean;

    securityLevel: SecurityLevel
    iAmExecuting: boolean;




    relayer: any

    // SideEffect data
    id: string;
    xtxId: string;
    arguments: string[];
    insurance: number;
    reward: number;
    raw: T3rnTypesSideEffect;

    // TargetConfirmation
    inclusionData: any; // contains encoded payload, inclusionProof, and blockHash
    targetInclusionHeight: any;
    executor: string;

    // Risk/Reward Analysis
    txCostUsd: number;
    maxProfitUsd: number;
    assetCostUsd: number;

    constructor(sideEffect: T3rnTypesSideEffect, id: string, xtxId: string, sdk: Sdk) {
        if(this.knownTransactionInterface(sideEffect.encodedAction)) {
            this.raw = sideEffect;
            this.id = id;
            this.xtxId = xtxId
            this.arguments = sideEffect.encodedArgs.map(entry => entry.toString());
            // this.hasInsurance = this.checkForInsurance(this.arguments.length, this.action)
            this.target =  new TextDecoder().decode(sideEffect.target.toU8a())
            this.securityLevel = this.evalSecurityLevel(sdk.gateways[this.target].gatewayType)
            console.log("SecurityLevel:", this.securityLevel)
            this.txCostUsd = 0;
            this.maxProfitUsd = 0;
            this.assetCostUsd = 0;
        } else {
            console.log("SideEffect interface unknown!!")
        }
    }

    evalSecurityLevel(gatewayType: any): SecurityLevel {
        if (gatewayType.ProgrammableExternal === '0' || gatewayType.OnCircuit === '0') {
            return SecurityLevel.Escrow
        } else {
            return SecurityLevel.Optimistic
        }
    }

    // sets the step of the sideEffect in its execution
    setStep(step: number) {
        this.step = step
    }

    updateRiskRewardParameters(txCostUsd: number, assetCostUsd: number) {
        this.txCostUsd = txCostUsd;
        this.assetCostUsd = assetCostUsd;
        this.maxProfitUsd = 10; // hardcode for now
    }
    //
    // // ToDo remove once merged https://github.com/t3rn/t3rn/issues/432
    // checkForInsurance(argsLength: number, action: SfxType): boolean {
    //     switch(action) {
    //         case SfxType.Transfer: {
    //             if(argsLength === 4) {
    //                 this.status = Sfx.WaitingForInsurance;
    //                 return true;
    //             } else {
    //                 // if the sfx is dirty, its ready on creation.
    //                 this.status = SfxStatus.ReadyForExec;
    //                 this.iAmExecuting = true; // Dirty sfx can always be executed without bond
    //                 return false
    //             }
    //             break;
    //         }
    //     }
    // }

    updateStatus(status: SfxStatus) {
        this.status = status;
    }

    // return an array of arguments to execute on target.
    execute(): any[] | void {
        switch(this.action) {
            case SfxType.Transfer: {
                return this.getTransferArguments()
            }
        }
    }

    getTxOutput() {
        switch(this.action) {
            case SfxType.Transfer: {
                return parseInt(this.getTransferArguments()[1])
            }
        }
    }

    // updates status
    insuranceBonded(iAmExecuting: boolean) {
        this.status = SfxStatus.PendingExecution;
        this.iAmExecuting = iAmExecuting;
    }

    // sfx was successfully executed on target and has the inclusion proof data
    executedOnTarget(inclusionData: any, executor: any, targetInclusionHeight: any) {
        this.inclusionData = inclusionData;
        this.executor = executor;
        this.targetInclusionHeight = targetInclusionHeight;
        this.status = SfxStatus.ExecutedOnTarget;
    }

    // ensure we can deal with the sfx action and set SfxType
    private knownTransactionInterface(encodedAction: any): boolean {
        switch(encodedAction.toHuman()) {
            case "tran": {
                this.action = SfxType.Transfer
                return true
                break;
            }
            default: {
                return false
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