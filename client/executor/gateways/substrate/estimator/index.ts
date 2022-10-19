import CostEstimator from "./cost";
import SubstrateRelayer from "../relayer";
import {SideEffect, SideEffectStatus, TransactionType} from "../../../executionManager/sideEffect";
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

	async getTxCostUsd(sideEffect: SideEffect) {
		const sfxTx = this.relayer.buildTx(sideEffect)
		const nativeTransactionCost = await this.cost.currentTransactionCost(sfxTx) // cost in native asset
		let txCostUsd;
		switch(sideEffect.action){
			case TransactionType.Transfer:
				const assetId = this.gatewayService.gateways[this.gatewayService.idMapper[sideEffect.target]].ticker
				const humanTransactionCost = this.gatewayService.valueToHuman(nativeTransactionCost, sideEffect.target) // as float
				txCostUsd = this.priceEngine.getQuote(assetId, humanTransactionCost)
		}
		return txCostUsd
	}

	getAssetCostUsd(sideEffect: SideEffect) {
		let assetCostUsd;
		switch(sideEffect.action){
			case TransactionType.Transfer:
				const assetId = this.gatewayService.gateways[this.gatewayService.idMapper[sideEffect.target]].ticker
				const humanTransactionCost = this.gatewayService.valueToHuman(sideEffect.getTxOutput(), sideEffect.target) // as float
				assetCostUsd = this.priceEngine.getQuote(assetId, humanTransactionCost)
		}
		return assetCostUsd
	}

	async estimateProfit(sideEffect: SideEffect) {
		let txCost = await this.getTxCostUsd(sideEffect)
		let assetCost = this.getAssetCostUsd(sideEffect)
		return [txCost, assetCost]
	}
}