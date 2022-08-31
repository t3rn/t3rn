import "@t3rn/types"
import {AccountId32, H256} from '@polkadot/types/interfaces';
import {SideEffect, SideEffectStatus} from "./sideEffect";
import {T3rnTypesSideEffect} from '@polkadot/types/lookup';
import {EventEmitter} from "events";

export enum ExecutionStatus {
    Open,
    ReadyToExecute,
    Complete,
    Reverted
}

export class Execution extends EventEmitter {
    status: ExecutionStatus = ExecutionStatus.Open;
    xtxId: H256;
    owner: AccountId32;
    sideEffects: {[key:string]: SideEffect} = {};

    steps: string[][] = [];
    currentStep: number;

    constructor(eventData: any) {
        super();
        this.owner = eventData[0]
        this.xtxId = eventData[1]
        this.initializeSideEffects(eventData[2], eventData[3])
        this.currentStep = 0;
    }

    // creates the new SideEffect instances, maps them locally and generates the steps as done in circuit.
    initializeSideEffects(sideEffects: T3rnTypesSideEffect[], ids: H256[]) {
        let insured: string[] = [];
        for(let i = 0; i < sideEffects.length; i++) {
            const sideEffect = new SideEffect(sideEffects[i], ids[i].toHex(), this.xtxId.toHex())
            this.sideEffects[sideEffect.id] = sideEffect
            if(sideEffect.hasInsurance) { // group insured steps into one step
                insured.push(ids[i].toHex());
            } else {
                this.steps.push([ids[i].toHex()]) // uninsured get their own step
            }
        }
        // prepend insured steps if we have any
        if(insured.length > 0) {
            this.steps = [insured, ...this.steps]
        }

        Object.keys(this.sideEffects).forEach(sfxId => {
            for(let i = 0; i < this.steps.length; i++) {
                if(this.steps[i].includes(sfxId)) {
                    this.sideEffects[sfxId].setStep(i)
                }
            }
        })

        console.log(this.sideEffects)
    }

    // update the status and set the step counter to the appropriate value.
    confirmSideEffect(sfxId: string) {
        this.sideEffects[sfxId].updateStatus(SideEffectStatus.ConfirmedOnTarget)

        // check how many transactions in the ExecutionStep are still open
        const readyToExecute = this.steps[this.currentStep].filter((sfxId: string) => {
            return this.sideEffects[sfxId].status < 5 // not executed SideEffects
        }).length;
        console.log("Current Step:", this.currentStep)

        console.log("SFX in current step:", this.steps[this.currentStep].length)
        console.log("Unexecuted in step:", readyToExecute);

        // If all steps are complete and there is a next step, move into it
        if (readyToExecute === 0 && this.steps[this.currentStep + 1] !== undefined) {
            this.currentStep += 1
            let readyToConfirm: SideEffect[] = [];

            // Check if we have SideEffects in the next step, that are ready to be confirmed
            this.steps[this.currentStep].forEach((sfxId: string) => {
                if(this.sideEffects[sfxId].status === SideEffectStatus.ConfirmedOnTarget) {
                    readyToConfirm.push(this.sideEffects[sfxId])
                }
            })

            // If we have found executed sfx, pass to executionManager
            if(readyToConfirm.length > 0) {
                this.emit("ExecutedSideEffectInCurrentStep", readyToConfirm)
            }
        }
    }

    readyToExecute() {
        this.status = ExecutionStatus.ReadyToExecute;
    }

    complete() {
        this.status = ExecutionStatus.Complete;
    }

    // returns the side effects that open to execution.
    // for dirty: ReadyToExec && !hasInsurance
    // for insured: Open && hasInsurance
    getOpenSideEffects(): SideEffect[] {
        return Object.values(this.sideEffects).filter(entry => {
            return (
                entry.status === SideEffectStatus.ReadyForExec && !entry.hasInsurance
                ||
                entry.status === SideEffectStatus.Open && entry.hasInsurance
            )
        })
    }
}