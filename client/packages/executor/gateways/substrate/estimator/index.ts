import CostEstimator from "./cost";
import SubstrateRelayer from "../relayer";
import {SideEffect} from "../../../executionManager/sideEffect";
import {PriceEngine} from "../../../pricing";
import {GatewayDataService} from "../../../utils/gatewayDataService";

import {SfxType, SfxStatus} from "@t3rn/sdk/dist/src/side-effects/types";
import {BehaviorSubject} from "rxjs";

export default class Estimator {

	cost: CostEstimator;
	relayer: SubstrateRelayer;
	priceEngine: PriceEngine;
	gatewayService: GatewayDataService;

	constructor(relayer: SubstrateRelayer) {
		this.relayer = relayer;
		this.cost = new CostEstimator(relayer);
	}

	async getNativeTxCostSubject(sideEffect: SideEffect): Promise<BehaviorSubject<number>> {
		const sfxTx = this.relayer.buildTx(sideEffect)
		return this.cost.getTxCostSubject(sideEffect.id, sfxTx);
	}

	stopTrackingTxCost(sfxId: string) {
		this.cost.stopTracking(sfxId)
	}

	// async initTxCostTracking(sideEffect: SideEffect): Promise<Subject<number>> {
	// 	const sfxTx = this.relayer.buildTx(sideEffect)
	// 	const txCostSubject = await this.cost.watchTransactionCost(sideEffect.id, sfxTx);
	// 	return txCostSubject
	// }

	// getAssetCostUsd(sideEffect: SideEffect) {
	// 	let assetCostUsd;
	// 	switch(sideEffect.action){
	// 		case SfxType.Transfer:
	// 			const assetId = this.gatewayService.gateways[this.gatewayService.idMapper[sideEffect.target]].ticker
	// 			const humanTransactionCost = this.gatewayService.valueToHuman(sideEffect.getTxOutput(), sideEffect.target) // as float
	// 			assetCostUsd = this.priceEngine.getQuote(assetId, humanTransactionCost)
	// 	}
	// 	return assetCostUsd
	// }
}