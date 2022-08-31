import {EventEmitter} from "events"
import createDebug from "debug"
import {Execution} from "./utils/execution";
import {ProfitEstimation} from "./profitEstimation";
import {SideEffect, SideEffectStatus} from "./utils/sideEffect";

type Queue = {
    gateways: {
        blockHeight: number
        // used for SideEffects that are waiting for execution
        open: string[]
        // caught circuit event but not executed yet
        executing: string[]
        // to confirm we need to wait for a specific block height to be reached
        confirming: {
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
            open: [],
            executing: [],
            confirming: {},
        }
    }

    addExecution(execution: Execution) {
        this.executions[execution.xtxId.toHex()] = execution;

        Object.keys(execution.sideEffects).forEach((sfxId: string) => {
            // add sfxId to execution lookup mapping
            this.sfxExecutionLookup[sfxId] = execution.xtxId.toHex()
        })
    }

    completeExecution(xtxId: string) {
        this.executions[xtxId].complete()
    }

    xtxReady(xtxId: string) {
        this.executions[xtxId].readyToExecute()

        // for now we will execute all sideEffects that are available
        const canExecute = this.executions[xtxId].getOpenSideEffects()

        canExecute.forEach(sideEffect=> {
            this.queue[sideEffect.target].executing.push(sideEffect.id)
            this.emit("ExecuteSideEffect", sideEffect)
        })
        console.log("Queue:", this.queue)
    }

    sfxReady(sfxId: string) {
        this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId].ready()
    }

    // once SideEffect has been executed on target we add it to the confirming pool
    // the transactions resides here until the circuit has received targets header range
    sideEffectExecuted(sfxId: string) {
        if (!this.sfxExecutionLookup[sfxId]) return
        const sfx = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId];
        this.removeExecuting(sfxId, sfx.target)

        //create array if inclusionBlock is not available
        if (!this.queue[sfx.target].confirming[sfx.targetInclusionHeight.toNumber()]) {
            this.queue[sfx.target].confirming[sfx.targetInclusionHeight.toNumber()] = []
        }

        this.queue[sfx.target].confirming[sfx.targetInclusionHeight.toNumber()].push(sfxId)
        ExecutionManager.debug("Executed:", this.queue)
    }

    // Once confirmed, delete from queue
    sideEffectConfirmed(sfxId: string) {
        console.log("ExecutionManager - sideEffectConfirmed:", sfxId)
        if (!this.sfxExecutionLookup[sfxId]) return
        const sfx = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId]
        this.executions[this.sfxExecutionLookup[sfxId]].confirmSideEffect(sfxId)
        this.removeConfirming(sfxId, sfx.target, sfx.targetInclusionHeight.toNumber())
    }

    // New header range was submitted on circuit
    // update local params and check which SideEffects can be confirmed
    updateGatewayHeight(gatewayId: string, blockHeight: any) {
        blockHeight = parseInt(blockHeight)
        if (this.queue[gatewayId]) {
            this.queue[gatewayId].blockHeight = blockHeight
            this.executeQueue(gatewayId, blockHeight)
        }
    }

    // checks which executed SideEffects can be confirmed on circuit
    executeQueue(gatewayId: string, blockHeight: number) {
        // contains the sfxIds of SideEffects that could be confirmed based on the current blockHeight of the light client
        let readyByHeight: string[] = [];
        Object.keys(this.queue[gatewayId].confirming).forEach(block => {
            //in node object keys are always strings
            if(parseInt(block) <= blockHeight) {
                console.log(this.queue[gatewayId].confirming[block])
                readyByHeight = readyByHeight.concat(this.queue[gatewayId].confirming[block])
            }
        })

        console.log("Ready By Height:", readyByHeight)

        const readyByStep: SideEffect[] = [];

        // filter the SideEffects that can be confirmed in the current step
        // ToDo a Sfx confirmation should trigger this function again, as the step could have updated
        readyByHeight.forEach(sfxId => {
            const sfx: SideEffect = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId];
            if(sfx.step === this.executions[sfx.xtxId].currentStep) {
                readyByStep.push(sfx)
            }
        })

        // ExecutionManager.debug("Ready to Submit", readyByStep)

        this.emit("ConfirmSideEffects", readyByStep)
    }

    private removeExecuting(id: string, gatewayId: string) {
        const index = this.queue[gatewayId].executing.indexOf(id)
        this.queue[gatewayId].executing.splice(index, 1)
    }

    private removeConfirming(id: string, gatewayId: string, blockHeight: number) {
        const index = this.queue[gatewayId].confirming[blockHeight].indexOf(id)
        // high chance their is only one SideEffect in this block, so this is faster.
        if (this.queue[gatewayId].confirming[blockHeight].length === 1) {
            delete this.queue[gatewayId].confirming[blockHeight]
        } else {
            this.queue[gatewayId].confirming[blockHeight].splice(index, 1)
        }
    }
}
