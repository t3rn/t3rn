import "@t3rn/types"
import {AccountId32, H256} from '@polkadot/types/interfaces';
import {SideEffect} from "./sideEffect";
// @ts-ignore
import {T3rnTypesSideEffect,} from '@polkadot/types/lookup';
import {EventEmitter} from "events";

import {SecurityLevel, SfxStatus, XtxStatus} from "@t3rn/sdk/dist/src/side-effects/types";
import {Sdk} from "@t3rn/sdk";

export class Execution extends EventEmitter {
    status: XtxStatus = XtxStatus.PendingBidding;
    xtxId: H256;
    owner: AccountId32;
    sideEffects: {[key:string]: SideEffect} = {};

    steps: string[][] = [[], []];
    currentStep: number;

    constructor(eventData: any, sdk: Sdk) {
        super();
        this.owner = eventData[0]
        this.xtxId = eventData[1]
        this.initializeSideEffects(eventData[2], eventData[3], sdk)
        this.currentStep = 0;
    }

    // creates the new SideEffect instances, maps them locally and generates the steps as done in circuit.
    initializeSideEffects(sideEffects: T3rnTypesSideEffect[], ids: H256[], sdk: Sdk) {
        for(let i = 0; i < sideEffects.length; i++) {
            const sideEffect = new SideEffect(sideEffects[i], ids[i].toHex(), this.xtxId.toHex(), sdk)
            this.sideEffects[sideEffect.id] = sideEffect
            if(sideEffect.securityLevel === SecurityLevel.Escrow) { // group escrow steps into one step
                this.steps[0].push(ids[i].toHex());
            } else {
                this.steps[1].push(ids[i].toHex()) // optimistic get their own step
            }
        }

        // remove escrow steps, if there are none
        if(this.steps[0].length === 0) {
            this.steps = [this.steps[1]]
        }

        // set the step index for each sfx
        Object.keys(this.sideEffects).forEach(sfxId => {
            for(let i = 0; i < this.steps.length; i++) {
                if(this.steps[i].includes(sfxId)) {
                    this.sideEffects[sfxId].setStep(i)
                }
            }
        })
    }

    // generateExecutionSteps(): string[][] {
    //
    // }

    // update the status and set the step counter to the appropriate value.
    sideEffectConfirmed(sfxId: string) {
        this.sideEffects[sfxId].updateStatus(SfxStatus.Confirmed)

        // check how many transactions in the ExecutionStep are still open
        const readyToExecute = this.steps[this.currentStep].filter((sfxId: string) => {
            return this.sideEffects[sfxId].status === SfxStatus.PendingExecution
        }).length;

        // If all steps are complete and there is a next step, move into it
        if (readyToExecute === 0 && this.steps[this.currentStep + 1] !== undefined) {
            this.currentStep += 1
            let readyToConfirm: SideEffect[] = [];
            let readyToExecute: SideEffect[] = [];

            // Check if we have SideEffects in the next step, that are ready to be confirmed
            this.steps[this.currentStep].forEach((sfxId: string) => {
                if(this.sideEffects[sfxId].status === SfxStatus.ExecutedOnTarget) {
                    readyToConfirm.push(this.sideEffects[sfxId])
                } else if (this.sideEffects[sfxId].status === SfxStatus.PendingExecution) {
                    readyToExecute.push(this.sideEffects[sfxId])
                }
            })

            // If we have found executed sfx, pass to executionManager. This might be obsolete
            if(readyToConfirm.length > 0) {
                this.emit("ExecutedSideEffectInCurrentStep", readyToConfirm)
            }

            // if we have found waiting sfx, execute
            if(readyToExecute.length > 0) {
                this.emit("ExecuteSideEffectInCurrentStep", readyToExecute)
            }
        }
    }

    readyToExecute() {
        this.status = XtxStatus.Ready;
    }

    complete() {
        this.status = XtxStatus.FinishedAllSteps;
    }

    // returns the sfxs that ready to execute
    getReadyToExecute(): SideEffect[] {
        return Object.values(this.sideEffects).filter(entry => {
            return entry.status === SfxStatus.PendingExecution && entry.iAmExecuting && entry.step === this.currentStep
        })
    }
}