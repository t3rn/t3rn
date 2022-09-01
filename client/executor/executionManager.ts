import {EventEmitter} from "events"
import createDebug from "debug"
import {Execution} from "./utils/execution";
import {ProfitEstimation} from "./profitEstimation";
import {SideEffect, SideEffectStatus} from "./utils/sideEffect";

type Queue = {
    gateways: {
        blockHeight: number
        // used for SideEffects that are waiting for execution
        waitingForInsurance: string[]
        // caught circuit event but not executed yet
        readyToExecute: string[]
        // to confirm we need to wait for a specific block height to be reached
        readyToConfirm: {
            [block: number]: string[]
        }
    }
}

export class ExecutionManager extends EventEmitter {
    static debug = createDebug("execution-manager")

    // we map the current state in the queue
    queue: Queue = <Queue>{}
    // maps xtxId to Execution instance
    executions: {
        [id: string]: Execution
    } = {}
    // maps sfxId to xtxId
    sfxExecutionLookup: {
        [sfxId: string]: string
    } = {};

    profitEstimation: ProfitEstimation = new ProfitEstimation();

    // adds gateways on startup
    addGateway(id: string) {
        this.queue[id] = {
            blockHeight: 0,
            waitingForInsurance: [],
            readyToExecute: [],
            readyToConfirm: {},
        }
    }

    addExecution(execution: Execution) {
        this.executions[execution.xtxId.toHex()] = execution;

        Object.keys(execution.sideEffects).forEach((sfxId: string) => {
            // add sfxId to execution lookup mapping
            this.sfxExecutionLookup[sfxId] = execution.xtxId.toHex()
        })

        // confirmation trigger that is called if the previous step is complete and we have executed sfx
        execution.on("ConfirmSideEffectInCurrentStep", (sideEffects: SideEffect[]) => {
            this.executeConfirmationStepQueue(sideEffects)
        })

          // confirmation trigger that is called if the previous step is complete and we have executed sfx
        execution.on("ExecuteSideEffectInCurrentStep", (sideEffects: SideEffect[]) => {
            sideEffects.forEach(sideEffect=> {
                this.emit("ExecuteSideEffect", sideEffect)
            })
        })


        // add dirty transactions to queue
        Object.values(execution.sideEffects).filter((sfx: SideEffect) => !sfx.hasInsurance).forEach(sfx => {
            this.queue[sfx.target].readyToExecute.push(sfx.id)
        })


        const needInsurance = Object.values(execution.sideEffects).filter((sfx: SideEffect) => sfx.hasInsurance)

        // add waiting transactions to queue
        needInsurance.forEach(sfx => {
            this.queue[sfx.target].waitingForInsurance.push(sfx.id)
        })

        console.log("Needs insurance:", needInsurance.length)
        // emit insured txs we want to bond
        this.emit("BondInsurance", needInsurance)

        console.log(this.queue)
    }

    completeExecution(xtxId: string) {
        this.executions[xtxId].complete()
    }

    insuranceBonded(sfxId: string, iAmExecuting: boolean) {
        if (!this.sfxExecutionLookup[sfxId]) return
        const sfx = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId];

        // remove from old queue
        this.removeFromQueue("waitingForInsurance", sfxId, sfx.target)
        // add to new one
        this.queue[sfx.target].readyToExecute.push(sfxId)

        this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId].insuranceBonded(iAmExecuting)
    }

    xtxReady(xtxId: string) {
        console.log("Execution manager!")
        this.executions[xtxId].readyToExecute()

        // for now we will execute all sideEffects that are available
        const canExecute = this.executions[xtxId].getReadyToExecute()

        console.log("canExecute:", canExecute)

        canExecute.forEach(sideEffect=> {
            // this.queue[sideEffect.target].readyToExecute.push(sideEffect.id)
            this.emit("ExecuteSideEffect", sideEffect)
        })
        console.log("Queue:", this.queue)
    }

    // once SideEffect has been executed on target we add it to the confirming pool
    // the transactions resides here until the circuit has received targets header range
    sideEffectExecuted(sfxId: string) {
        if (!this.sfxExecutionLookup[sfxId]) return
        const sfx = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId];
        this.removeFromQueue("readyToExecute", sfxId, sfx.target)

        //create array if inclusionBlock is not available
        if (!this.queue[sfx.target].readyToConfirm[sfx.targetInclusionHeight.toNumber()]) {
            this.queue[sfx.target].readyToConfirm[sfx.targetInclusionHeight.toNumber()] = []
        }

        this.queue[sfx.target].readyToConfirm[sfx.targetInclusionHeight.toNumber()].push(sfxId)

        ExecutionManager.debug("Executed:", this.queue)
    }

    // Once confirmed, delete from queue
    sideEffectConfirmed(sfxId: string) {
        if (!this.sfxExecutionLookup[sfxId]) return
        const sfx = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId]
        //update status in execution and sfx
        this.executions[this.sfxExecutionLookup[sfxId]].sideEffectConfirmed(sfxId)
        //remove from local queue
        this.removeConfirming(sfxId, sfx.target, sfx.targetInclusionHeight.toNumber())
    }

    // New header range was submitted on circuit
    // update local params and check which SideEffects can be confirmed
    updateGatewayHeight(gatewayId: string, blockHeight: any) {
        blockHeight = parseInt(blockHeight)
        if (this.queue[gatewayId]) {
            this.queue[gatewayId].blockHeight = blockHeight
            this.executeQueue(gatewayId)
        }
    }

    // checks which executed SideEffects can be confirmed on circuit
    executeQueue(gatewayId: string) {
        // contains the sfxIds of SideEffects that could be confirmed based on the current blockHeight of the light client
        let readyByHeight: string[] = [];
        Object.keys(this.queue[gatewayId].readyToConfirm).forEach(block => {
            //in node object keys are always strings
            if(parseInt(block) <= this.queue[gatewayId].blockHeight) {
                readyByHeight = readyByHeight.concat(this.queue[gatewayId].readyToConfirm[block])
            }
        })

        console.log("Ready By Height:", readyByHeight)

        const readyByStep: SideEffect[] = [];

        // filter the SideEffects that can be confirmed in the current step
        // ToDo a Sfx confirmation should trigger this function again, as the step could have updated
        readyByHeight.forEach(sfxId => {
            console.log("Current Step:", this.executions[this.sfxExecutionLookup[sfxId]].currentStep)
            const sfx: SideEffect = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId];
            console.log("SFX step:", sfx.step)
            if(sfx.step === this.executions[sfx.xtxId].currentStep) {
                readyByStep.push(sfx)
            }
        })

        console.log("ready by step:", readyByStep.map(entry => entry.id))

        // ExecutionManager.debug("Ready to Submit", readyByStep)

        this.emit("ConfirmSideEffects", readyByStep)
    }

    executeConfirmationStepQueue(sideEffects: SideEffect[]) {
        let readyToConfirm = sideEffects.filter((sfx: SideEffect) => {
            return parseInt(this.queue[sfx.target].blockHeight)>= sfx.targetInclusionHeight.toNumber()
        })

         console.log("ready:", readyToConfirm.map(entry => entry.id))
        if(readyToConfirm.length > 0) {
            this.emit("ConfirmSideEffects", readyToConfirm)
        }
    }

    // removes sfx from executing queue
    private removeFromQueue(queue: string, id: string, gatewayId: string) {
        const index = this.queue[gatewayId][queue].indexOf(id)
        this.queue[gatewayId][queue].splice(index, 1)
    }

    // removes the SideEffects from the confirmation queue
    private removeConfirming(id: string, gatewayId: string, blockHeight: number) {
        // high chance there is only one SideEffect in this block, so this is faster.
        if (this.queue[gatewayId].readyToConfirm[blockHeight].length === 1) {
            delete this.queue[gatewayId].readyToConfirm[blockHeight]
        } else {
            const index = this.queue[gatewayId].readyToConfirm[blockHeight].indexOf(id)
            this.queue[gatewayId].readyToConfirm[blockHeight].splice(index, 1)
        }
    }
}
