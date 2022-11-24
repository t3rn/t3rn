import "@t3rn/types"
import {AccountId32, H256} from '@polkadot/types/interfaces';
import {SideEffect} from "./sideEffect";
// @ts-ignore
import {T3rnTypesSideEffect,} from '@polkadot/types/lookup';
import {EventEmitter} from "events";

import {SecurityLevel, SfxStatus, XtxStatus} from "@t3rn/sdk/dist/src/side-effects/types";
import {Sdk} from "@t3rn/sdk";
import {StrategyEngine} from "../strategy";
import {BiddingEngine} from "../bidding";

export class Execution extends EventEmitter {
    status: XtxStatus = XtxStatus.PendingBidding;
    xtxId: H256;
    owner: AccountId32;
    sideEffects: Map<string, SideEffect> = new Map<string, SideEffect>();
    id: string;
    humanId: string;

    steps: string[][] = [[], []];
    currentStep: number;

    circuitSignerAddress: string;

    constructor(eventData: any, sdk: Sdk, strategyEngine: StrategyEngine, biddingEngine: BiddingEngine, circuitSignerAddress: string) {
        super();
        this.owner = eventData[0]
        this.xtxId = eventData[1]
        this.id = this.xtxId.toHex()
        this.humanId = this.id.slice(0, 8);
        this.circuitSignerAddress = circuitSignerAddress;
        this.initializeSideEffects(eventData[2], eventData[3], sdk, strategyEngine, biddingEngine)
        this.currentStep = 0;
    }

    // creates the new SideEffect instances, maps them locally and generates the steps as done in circuit.
    initializeSideEffects(sideEffects: T3rnTypesSideEffect[], ids: H256[], sdk: Sdk, strategyEngine: StrategyEngine, biddingEngine: BiddingEngine) {
        for(let i = 0; i < sideEffects.length; i++) {
            const sideEffect = new SideEffect(sideEffects[i], ids[i].toHex(), this.xtxId.toHex(), sdk, strategyEngine, biddingEngine, this.circuitSignerAddress)
            this.sideEffects.set(sideEffect.id, sideEffect)

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
        for(let [sfxId,sfx] of this.sideEffects){
            for(let i = 0; i < this.steps.length; i++) {
                if(this.steps[i].includes(sfxId)) {
                    sfx.setStep(i)
                }
            }
        }
    }

    setReadyToExecute() {
        this.status = XtxStatus.Ready;

        //Updates each Sfx
        for(let [sfxId,sfx] of this.sideEffects){
            sfx.readyToExecute();
        }
        console.log(`Execution ${this.humanId} is ready to execute`)
    }

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

    completed() {
        this.status = XtxStatus.FinishedAllSteps;
        console.log(`Execution ${this.humanId} is completed`)
    }

    // returns the sfxs that ready to execute
    getReadyToExecute(): SideEffect[] {
        let result: SideEffect[] = [];
        for(let [_sfxId,sfx] of this.sideEffects){
            if(sfx.status === SfxStatus.PendingExecution && sfx.isBidder && sfx.step === this.currentStep) {
                result.push(sfx)
            }
        }
        return result
    }
}