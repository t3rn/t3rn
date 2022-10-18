import CostEstimator from "./cost";
import SubstrateRelayer from "../relayer";
import {SideEffect, TransactionType} from "../../../executionManager/sideEffect";
import {PriceEngine} from "../../../pricing";
import {GatewayDataService} from "../../../utils/gatewayDataService";

export default class Estimator {

	cost: CostEstimator;
	relayer: SubstrateRelayer;
	priceEngine: PriceEngine;
	gatewayService: GatewayDataService;

	constructor(relayer: SubstrateRelayer, priceEngine: PriceEngine, gatewayService: GatewayDataService) {
		this.relayer = relayer;
		this.cost = new CostEstimator(relayer)
		this.priceEngine = priceEngine;
		this.gatewayService = gatewayService
	}

	async estimateProfit(sideEffect: SideEffect) {
		const sfxTx = this.relayer.buildTx(sideEffect)
		const nativeTransactionCost = await this.cost.currentTransactionCost(sfxTx)
		let txCostUsd;
		switch(sideEffect.action){
			case TransactionType.Transfer:
				const assetId = this.gatewayService.gateways[this.gatewayService.idMapper[sideEffect.target]].ticker
				const humanTransactionCost = this.gatewayService.valueToHuman(nativeTransactionCost, sideEffect.target) // as float
				txCostUsd = this.priceEngine.getQuote(assetId, humanTransactionCost)
		}
		console.log("TX cost USD:", txCostUsd);
	}




}