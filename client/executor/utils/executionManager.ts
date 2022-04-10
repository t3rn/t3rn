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
            blockNumber: string[],
        }
    }
}


export class ExecutionManager {

    // we map the current state in the queue
    queue: Queue;
    // and use this mapping for storing the actual side effect instances
    sideEffects: object;

    constructor() {
        this.queue = <Queue>{};
    }

    // adds gateways on startup
    addGateway(id: string) {
        this.queue[id] = {
            blockHeight: 0,
            open: [],
            executing: [],
            confirming: {},
        }
    }

    // add SideEffect to queue (either executing immediatly or adding as inactive)
    // idea is that inactives could be picked up later if the risk profile changes
    addSideEffect(sideEffect: SideEffectStateManager) {
        this.sideEffects[sideEffect.getId()] = sideEffect

        // plug off-chain riks assessment here
        const acceptRisk = true;
        if(acceptRisk) {
            this.queue[sideEffect.getTarget()].executing.append(sideEffect.getId());
            // this.super.executeSideEffect(sideEffect)
        } else {
            this.queue[sideEffect.getTarget()].open.append(sideEffect.getId());
        }
    }

    // once SideEffect has been executed on target we add it to the confirming pool
    // the transactions resides here until the circuit has received targets header range
    executedSideEffect(id: string) {
        const gatewayId = this.sideEffects[id].getTarget();
        const inclusionBlock = this.sideEffects[id].getTargetBlock()
        this.queue[gatewayId].executing.remove(id)
        this.queue[gatewayId].confirming[inclusionBlock].append(id);
    }

    finalize(id: string) {
        const gatewayId = this.sideEffects[id].getTarget();
        const inclusionBlock = this.sideEffects[id].getTargetBlock()
        this.queue[gatewayId].confirming[inclusionBlock].remove(id);
        delete this.sideEffects[id];
    }

    // New header range was submitted on circuit
    // update local params and check which SideEffects can be confirmed
    updateGatewayHeight(gatewayId: string, blockHeight: number) {
        this.queue[gatewayId].blockHeight = blockHeight;

        // this.executeQueue(gatewayId, blockHeight)
    }

    // checks which unconfirmed SideEffects can be executed on circuit
    private executeQueue(gatewayId: string, blockHeight: number) {
        let ready = Object.keys(this.queue[gatewayId].confirming).filter(block => {
            // flatten array of SideEffectIds
            block <= blockHeight
        })

        for(let i = 0; i < ready.length; i++) {
            // this.super.
        }
    }
}