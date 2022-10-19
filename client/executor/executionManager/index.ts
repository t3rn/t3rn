import {EventEmitter} from "events"
import createDebug from "debug"
import {Execution} from "./execution";
import {SideEffect} from "./sideEffect";
import Estimator from "../gateways/substrate/estimator";
import SubstrateRelayer from "../gateways/substrate/relayer";
import {GatewayDataService} from "../utils/gatewayDataService";
import {PriceEngine} from "../pricing";
import {StrategyEngine} from "../strategy";

// A type used for storing the different SideEffects throughout their respective life-cycle.
// Please note that waitingForInsurance and readyToExecute are only used to track the progress. The actual logic is handeled in the execution
type Queue = {
    gateways: {
        blockHeight: number
        // sfx that are waiting the insurance to be bonded
        waitingForInsurance: string[]
        // sfx that can be executed on target, ignoring the step rules.
        readyToExecute: string[]
        // Executed sfx and their respective execution block.
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
    // a lookup mapping, to find a sfx xtxId
    sfxExecutionLookup: {
        [sfxId: string]: string
    } = {};

	targetEstimator: {
        [id: string]: Estimator
    } = {};

    gatewayService: GatewayDataService;
    priceEngine: PriceEngine;
    strategyEngine: StrategyEngine;

    constructor(gatewayService: GatewayDataService, priceEngine: PriceEngine) {
        super();
        this.gatewayService = gatewayService;
        this.priceEngine = priceEngine;
        this.strategyEngine = new StrategyEngine();
    }

    // adds gateways on startup
    addGateway(id: string, relayer: SubstrateRelayer) {
        this.queue[id] = {
            blockHeight: 0,
            waitingForInsurance: [],
            readyToExecute: [],
            readyToConfirm: {},
        }

		this.targetEstimator[id] = new Estimator(relayer, this.priceEngine, this.gatewayService);

    }

    // add a new execution to the execution manager and initialize event listeners
    async createExecution(execution: Execution) {
        this.executions[execution.xtxId.toHex()] = execution;
        let sfxId = Object.keys(execution.sideEffects)
        // Object.keys(execution.sideEffects).forEach(async (sfxId: string) => {
        for(let i = 0; i < sfxId.length; i++) {
            // add sfxId to execution lookup mapping
            this.sfxExecutionLookup[sfxId[i]] = execution.xtxId.toHex()
			let [txCost, assetCost] = await this.targetEstimator[execution.sideEffects[sfxId[i]].target].estimateProfit(execution.sideEffects[sfxId[i]])
            execution.sideEffects[sfxId[i]].updateRiskRewardParameters(txCost, assetCost)
            const shouldExecute = this.strategyEngine.evaluateSideEffect(execution.sideEffects[sfxId[i]])
            console.log("Want to Execute:", shouldExecute)
        }

        // listens for step confirmation signal. This is called after a step is confirmed, and SideEffects in the next step are already executed.
        // In the current configuration this is not used, as the executions of a next step are only started once the former one is complete (next listener)
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

        // adds transaction waiting for insurance bond into correct queue
        needInsurance.forEach(sfx => {
            this.queue[sfx.target].waitingForInsurance.push(sfx.id)
        })

        // emit insured txs we want to bond
        this.emit("BondInsurance", needInsurance)

        ExecutionManager.debug("New Execution Received:", this.queue)
    }

    // the entire execution has terminated
    executionComplete(xtxId: string) {
        this.executions[xtxId].complete()
        ExecutionManager.debug("✨✨✨ Execution Complete ✨✨✨", xtxId)
        console.log(this.queue)
    }

    // update sfx once insurance has been bonded
    insuranceBonded(sfxId: string, iAmExecuting: boolean) {
        if (!this.sfxExecutionLookup[sfxId]) return
        const sfx = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId];

        // remove from old queue
        this.removeFromQueue("waitingForInsurance", sfxId, sfx.target)
        // add to new one
        this.queue[sfx.target].readyToExecute.push(sfxId)

        this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId].insuranceBonded(iAmExecuting)
        ExecutionManager.debug(`Insurance Bonded: ${this.toHuman(sfxId)} - ${sfx.target} 💎`)
    }

    // the execution is ready to be executed. This happends once all insured sfx have received their insurance
    xtxReady(xtxId: string) {
        this.executions[xtxId].readyToExecute()

        // this could potentially be useless, as all sfxs should be readyToExecute at this stage. Other executor might have snatched some sfx, so leaving this here
        const canExecute = this.executions[xtxId].getReadyToExecute()

        canExecute.forEach(sideEffect=> {
            this.emit("ExecuteSideEffect", sideEffect)
        })
        ExecutionManager.debug("Execution Ready:", xtxId)
        console.log(this.queue)
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

        ExecutionManager.debug(`Executed: ${this.toHuman(sfxId)} - ${sfx.target} - #${sfx.targetInclusionHeight.toNumber()} 🏁`)
    }

    // Once confirmed on target, delete from queue
    sideEffectConfirmed(sfxId: string) {
        if (!this.sfxExecutionLookup[sfxId]) return
        const sfx = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId]
        //update status in execution and sfx
        this.executions[this.sfxExecutionLookup[sfxId]].sideEffectConfirmed(sfxId)
        //remove from local queue
        this.removeConfirming(sfxId, sfx.target, sfx.targetInclusionHeight.toNumber())
        ExecutionManager.debug(`Confirmed: ${this.toHuman(sfxId)} - ${sfx.target} ✅`)
    }

    // New header range was submitted on circuit
    // update local params and check which SideEffects can be confirmed
    updateGatewayHeight(gatewayId: string, blockHeight: any) {
        blockHeight = parseInt(blockHeight)
        if (this.queue[gatewayId]) {
            this.queue[gatewayId].blockHeight = blockHeight
            this.executeConfirmationQueue(gatewayId)
        }
    }

    // checks which executed SideEffects can be confirmed on circuit
    executeConfirmationQueue(gatewayId: string) {
        // contains the sfxIds of SideEffects that could be confirmed based on the current blockHeight of the light client
        let readyByHeight: string[] = [];
        Object.keys(this.queue[gatewayId].readyToConfirm).forEach(block => {
            //in node object keys are always strings
            if(parseInt(block) <= this.queue[gatewayId].blockHeight) {
                readyByHeight = readyByHeight.concat(this.queue[gatewayId].readyToConfirm[block])
            }
        })
        const readyByStep: SideEffect[] = [];

        // filter the SideEffects that can be confirmed in the current step
        readyByHeight.forEach(sfxId => {
            const sfx: SideEffect = this.executions[this.sfxExecutionLookup[sfxId]].sideEffects[sfxId];
            if(sfx.step === this.executions[sfx.xtxId].currentStep) {
                readyByStep.push(sfx)
            }
        })

        ExecutionManager.debug(`${gatewayId} - ${this.queue[gatewayId].blockHeight}: ReadyToConfirm: ${readyByStep.length}`)
        this.emit("ConfirmSideEffects", readyByStep)
    }

    // Checks if there are any sfx that can be confirmed, after another sfx was conmfirmed. This function is not used in the current configuration, as we wait with executing a nexts steps sfxs until step is complete
    executeConfirmationStepQueue(sideEffects: SideEffect[]) {
        let readyToConfirm = sideEffects.filter((sfx: SideEffect) => {
            return parseInt(this.queue[sfx.target].blockHeight)>= sfx.targetInclusionHeight.toNumber()
        })

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

    private toHuman(id: string) {
        return id.substring(0, 8)
    }
}
