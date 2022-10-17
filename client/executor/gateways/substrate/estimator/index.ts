import CostEstimator from "./cost";
import SubstrateRelayer from "../relayer";
import {SideEffect} from "../../../executionManager/sideEffect";

export default class Estimator {


	cost: CostEstimator;
	relayer: SubstrateRelayer;

	constructor(relayer: SubstrateRelayer) {
		this.relayer = relayer;
		this.cost = new CostEstimator(relayer)
	}

	estimateProfit(sideEffect: SideEffect) {
		const sfxTx = this.relayer.buildTx(sideEffect)
		const cost = this.cost.currentSideEffectCost(sfxTx)
	}


}