import { EventEmitter } from "stream";
import { SideEffectStateManager } from "./types"

type Queue = {
    gateways: {
        blockHeight: number,
        // used for SideEffects that are waiting for execution
        open: string[]
        // caught circuit event but not executed yet
        executing: string[],
        // to confirm we need to wait for a specific block height to be reached
        confirming: {
            [block: number]: string[],
        }
    }
}


export class ExecutionManager extends EventEmitter {

    // we map the current state in the queue
    queue: Queue = <Queue>{};
    // and use this mapping for storing the actual side effect instances
    sideEffects: {
        [id: string]: SideEffectStateManager
    } = {};

    // adds gateways on startup
    addGateway(id: string) {
        this.queue[id] = {
            blockHeight: 0,
            open: [],
            executing: [],
            confirming: {

            },
        }
    }

    // add SideEffect to queue (either executing immediatly or adding as inactive)
    // idea is that inactives could be picked up later if the risk profile changes
    addSideEffect(sideEffect: SideEffectStateManager) {
        this.sideEffects[sideEffect.getId()] = sideEffect
        // plug off-chain riks assessment here
        const acceptRisk = true;
        if(acceptRisk) {
            this.queue[sideEffect.getTarget()].executing.push(sideEffect.getId());
            // Trigger execution
            this.emit('ExecuteSideEffect', sideEffect)
        } else {
            this.queue[sideEffect.getTarget()].open.push(sideEffect.getId());
        }

        console.log("Added SideEffect: ", this.queue);
    }

    // once SideEffect has been executed on target we add it to the confirming pool
    // the transactions resides here until the circuit has received targets header range
    sideEffectExecuted(id: string) {
        const gatewayId = this.sideEffects[id].getTarget();
        const inclusionBlock = this.sideEffects[id].getTargetBlock()
        // this.queue[gatewayId].executing.remove(id)
        this.removeExecuting(id, gatewayId);

        //create array if inclusionBlock is not available
        if (!this.queue[gatewayId].confirming[inclusionBlock]) {
            this.queue[gatewayId].confirming[inclusionBlock] = []
        }

        this.queue[gatewayId].confirming[inclusionBlock].push(id);
        console.log("Executed:", this.queue);
    }

    // Once confirmed, delete from queue
    finalize(id: string) {
        const gatewayId = this.sideEffects[id].getTarget();
        const inclusionBlock = this.sideEffects[id].getTargetBlock()
        this.removeConfirming(id, gatewayId, inclusionBlock)
        delete this.sideEffects[id];
        console.log("Finalized:", this.queue);
    }

    // New header range was submitted on circuit
    // update local params and check which SideEffects can be confirmed
    updateGatewayHeight(gatewayId: string, blockHeight: number) {
        this.queue[gatewayId].blockHeight = blockHeight;
        this.executeQueue(gatewayId, blockHeight)
    }

    // checks which unconfirmed SideEffects can be executed on circuit
    executeQueue(gatewayId: string, blockHeight: number) { 
        // filter and flatten SideEffects ready to be confirmed.
        let ready = Object.keys(this.queue[gatewayId].confirming).filter(block => {
            //in node object keys are always strings
            parseInt(block) <= blockHeight
        }).flat()

        console.log("ready:", ready);

        const sideEffectsToConfirm = ready.map(sideEffectId => {
            return this.sideEffects[sideEffectId]
        })

        this.emit("ConfirmSideEffects", sideEffectsToConfirm);
    }

    private removeExecuting(id: string, gatewayId: string) {
        const index = this.queue[gatewayId].executing.indexOf(id);
        this.queue[gatewayId].executing.splice(index, 1);
    }

    private removeConfirming(id: string, gatewayId: string, blockHeight: number) {
        const index = this.queue[gatewayId].confirming[blockHeight].indexOf(id);
        // high chance their is only one SideEffect in this block, so this is faster.
        if (this.queue[gatewayId].confirming[blockHeight].length === 1) {
            delete this.queue[gatewayId].confirming[blockHeight];
        } else {
            this.queue[gatewayId].confirming[blockHeight].splice(index, 1);
        }
    }
}