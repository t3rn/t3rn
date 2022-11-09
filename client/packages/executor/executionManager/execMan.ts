import createDebug from "debug"
import {Execution} from "./execution";
import {SideEffect} from "./sideEffect";
import Estimator from "../gateways/substrate/estimator";
import SubstrateRelayer from "../gateways/substrate/relayer";
import {PriceEngine} from "../pricing";
import {BehaviorSubject} from "rxjs";
import {StrategyEngine} from "../strategy";
import {Sdk} from "@t3rn/sdk";


// A type used for storing the different SideEffects throughout their respective life-cycle.
// Please note that waitingForInsurance and readyToExecute are only used to track the progress. The actual logic is handeled in the execution
type Queue = {
    gateways: {
        blockHeight: number
		isBidding: string[],
		isExecuting: string[],
        // Executed sfx and their respective execution block.
        isConfirming: {
            [block: number]: string[]
        },
		complete: string[]
    }
}


export class ExecutionManager {
	static debug = createDebug("execution-manager")

	// we map the current state in the queue
	queue: Queue = <Queue>{}
    // maps xtxId to Execution instance
    xtx: {
        [id: string]: Execution
    } = {}
    // a lookup mapping, to find a sfx xtxId
    sfxToXtx: {
        [sfxId: string]: string
    } = {};

	targetEstimator: {
        [id: string]: Estimator
    } = {};

	priceEngine: PriceEngine;
	strategyEngine: StrategyEngine;

	constructor(priceEngine: PriceEngine) {
		this.priceEngine = priceEngine;
		this.strategyEngine = new StrategyEngine();
	}


	// adds gateways on startup
    addGateway(id: string, estimator: Estimator) {
        this.queue[id] = {
            blockHeight: 0,
            waitingForInsurance: [],
            readyToExecute: [],
            readyToConfirm: {},
        }

		this.targetEstimator[id] = estimator;
    }

	async addXtx(xtxData: any, sdk: Sdk) {
		const xtx = new Execution(xtxData, sdk, this.strategyEngine)
		try {
			this.strategyEngine.evaluateXtx(xtx)

		} catch(e) {
			console.log("Xtx eval failed!", e.toString())
			return
		}

		this.xtx[xtx.id] = xtx
		for (const [sfxId, sfx] of xtx.sideEffects.entries()) {
			this.sfxToXtx[sfxId] = xtx.id
			await this.addRiskRewardParameters(sfx)
		}
	}

	async addRiskRewardParameters(sfx: SideEffect) {
		const txCostSubject = await this.targetEstimator[sfx.target].getNativeTxCostSubject(sfx);
		const nativeAssetPriceSubject = this.priceEngine.getAssetPrice(sfx.gateway.ticker);

		const txOutput = sfx.getTxOutputs()
		const txOutputPriceSubject = this.priceEngine.getAssetPrice(txOutput.asset);
		const rewardAssetPriceSubject = this.priceEngine.getAssetPrice("TRN");

		sfx.setRiskRewardParameters(txCostSubject, nativeAssetPriceSubject, txOutputPriceSubject, rewardAssetPriceSubject)
	}



}