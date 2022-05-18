import { EventEmitter } from "events"
import { SideEffect } from "./utils/types"
import createDebug from "debug"

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
  // and use this mapping for storing the actual side effect instances
  sideEffects: {
    [id: string]: SideEffect
  } = {}

  // adds gateways on startup
  addGateway(id: string) {
    this.queue[id] = {
      blockHeight: 0,
      open: [],
      executing: [],
      confirming: {},
    }
  }

  addSideEffects(sideEffects: SideEffect[]) {
    // plug off-chain riks assessment here
    const acceptRisk = true
    if (acceptRisk) {
      sideEffects.forEach(sideEffect => {
        this.sideEffects[sideEffect.getId()] = sideEffect
        this.queue[sideEffect.getTarget()].executing.push(sideEffect.getId())
      })
      // Trigger execution
      this.emit("ExecuteSideEffects", sideEffects)
    } else {
      sideEffects.forEach(sideEffect => {
        this.sideEffects[sideEffect.getId()] = sideEffect
        this.queue[sideEffect.getTarget()].open.push(sideEffect.getId())
      })
    }

    ExecutionManager.debug("Added SideEffects: ", this.queue)
  }

  // once SideEffect has been executed on target we add it to the confirming pool
  // the transactions resides here until the circuit has received targets header range
  sideEffectExecuted(id: string) {
    const gatewayId = this.sideEffects[id].getTarget()
    const inclusionBlock = this.sideEffects[id].getTargetBlock()
    // this.queue[gatewayId].executing.remove(id)
    this.removeExecuting(id, gatewayId)

    //create array if inclusionBlock is not available
    if (!this.queue[gatewayId].confirming[inclusionBlock]) {
      this.queue[gatewayId].confirming[inclusionBlock] = []
    }

    this.queue[gatewayId].confirming[inclusionBlock].push(id)
    ExecutionManager.debug("Executed:", this.queue)
  }

  // Once confirmed, delete from queue
  finalize(id: string) {
    const gatewayId = this.sideEffects[id].getTarget()
    const inclusionBlock = this.sideEffects[id].getTargetBlock()
    this.removeConfirming(id, gatewayId, inclusionBlock)
    delete this.sideEffects[id]
    ExecutionManager.debug("Finalized:", this.queue)
  }

  // New header range was submitted on circuit
  // update local params and check which SideEffects can be confirmed
  updateGatewayHeight(gatewayId: string, blockHeight: any) {
    blockHeight = parseInt(blockHeight)

    this.queue[gatewayId].blockHeight = blockHeight
    this.executeQueue(gatewayId, blockHeight)
  }

  // checks which unconfirmed SideEffects can be executed on circuit
  // ToDo: This crime needs to be refactored
  executeQueue(gatewayId: string, blockHeight: number) {
    // filter and flatten SideEffects ready to be confirmed.
    let ready = Object.keys(this.queue[gatewayId].confirming).filter(block => {
      //in node object keys are always strings
      return parseInt(block) <= blockHeight
    })

    if (ready.length > 0) {
      let res: any[] = []
      for (let i = 0; i < ready.length; i++) {
        let sideEff = this.queue[gatewayId].confirming[ready[i]]
        res.push(sideEff)
      }

      let sideEffectsToConfirm = res.flat().map(sideEffectId => {
        return this.sideEffects[sideEffectId]
      })

      ExecutionManager.debug("Ready to Submit", ready)

      this.emit("ConfirmSideEffects", sideEffectsToConfirm)
    }
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
