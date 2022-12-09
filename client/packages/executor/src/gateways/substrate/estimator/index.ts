import {CostEstimator} from "./cost";
import {SubstrateRelayer} from "../relayer";
import {SideEffect} from "../../../executionManager/sideEffect";
import {BehaviorSubject} from "rxjs";

/** Class used for estimating the TX cost on the target chain
 * It uses the relayer to estimate the cost of the TX which is part of the risk/reward computation
 *
 *
 * @group Gateways
 * @category Substrate
 */
export default class Estimator {

	cost: CostEstimator;
	relayer: SubstrateRelayer;

	constructor(relayer: SubstrateRelayer) {
		this.relayer = relayer;
		this.cost = new CostEstimator(relayer);
	}

	/** Estimates the cost of the side effect on the target chain.
	 * We first build the rtx object we would use to execute the TX on target.
	 * The cost estimator class then uses the relayer to estimate the cost of the TX.
	 * We then convert the cost to USD using the price engine.
	 *
	 * @param sideEffect object of SFX we want to estimate the cost for
	 * @returns the cost of the SFX in USD as a subject
	 */
	async getNativeTxCostSubject(sideEffect: SideEffect): Promise<BehaviorSubject<number>> {
		const sfxTx = this.relayer.buildTx(sideEffect)
		return this.cost.getTxCostSubject(sideEffect.id, sfxTx);
	}
}