import "@t3rn/types"
import { H256, AccountId32 } from '@polkadot/types/interfaces';
import { SideEffect } from "./sideEffect";
import { T3rnTypesSideEffect } from '@polkadot/types/lookup';

export class Execution {
    xtxId: H256;
    owner: AccountId32;
    sideEffects: SideEffect[];
    // maps sfx id to index in array
    sideEffectsLookup: {[key:string]: number} = {}

    steps: string[][];
    currentStep: number;

    constructor(eventData: any) {
        this.owner = eventData[0]
        this.xtxId = eventData[1]
        this.currentStep = 0;
    }

    // creates the new SideEffect instances, maps them locally and generates the steps as done in circuit.
    initializeSideEffects(sideEffects: T3rnTypesSideEffect[], ids: H256[]) {
        let insured: string[] = [];

        for(let i = 0; i < sideEffects.length; i++) {
            const sideEffect = new SideEffect(sideEffects[i], ids[i], this.xtxId)
            this.sideEffects.push(sideEffect)
            this.sideEffectsLookup[ids[i].toHex()] = i;
            if(sideEffect.hasInsurance) { // group insured steps into one step
                insured.push(ids[i].toHex());
            } else {
                this.steps.push([ids[i].toHex()]) // uninsured get their own step
            }
        }
        // prepend to array
        this.steps = [insured, ...this.steps]
    }

    executeSfx(id: string): any[] | void {
        return this.sideEffects[this.sideEffectsLookup[id]].execute();
    }
}