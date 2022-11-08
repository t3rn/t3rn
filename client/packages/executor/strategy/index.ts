import {PriceEngine} from "../pricing";
import config from "../config/config"
import {SideEffect} from "../executionManager/sideEffect";



type Strategy = {
	minProfitUsd: number,
	minYield: number,
	maxTxFeesUsd: number,
	maxTxFeeShare: number, // txCost / maxProfit
	maxAssetCost: number, // maximum value spend
}

export class StrategyEngine {

	strategies: {
		[target: string]: Strategy
	} = {};

	constructor() {
		this.loadStrategies()
	}

	loadStrategies() {
		const strategyTargets = Object.keys(config.strategies);
		for(let i = 0; i < strategyTargets.length; i++) {
			this.strategies[strategyTargets[i]] = {
				minProfitUsd: config.strategies[strategyTargets[i]].minProfitUsd,
				minYield: config.strategies[strategyTargets[i]].minYield,
				maxTxFeesUsd: config.strategies[strategyTargets[i]].maxTxFeesUsd,
				maxTxFeeShare: config.strategies[strategyTargets[i]].maxTxFeeShare,
				maxAssetCost: config.strategies[strategyTargets[i]].maxAssetCost,
			}
		}
	}

	// checks the constraints for any given side effect
	evaluateSideEffect(sideEffect: SideEffect) {
		const strategy = this.strategies[sideEffect.target];

		if (this.minProfitRejected(sideEffect, strategy)) {
			console.log("minProfitRejected")
			return false
		}
		if (this.minYieldRejected(sideEffect, strategy)) {
			console.log("minYieldRejected")
			return false
		}
		if (this.maxTxFeesRejected(sideEffect, strategy)) {
			console.log("maxTxFeesRejected")
			return false
		}
		if (this.maxTxFeeShareRejected(sideEffect, strategy)) {
			console.log("maxTxFeeShareRejected")
			return false
		}
		if (this.maxAssetCostRejected(sideEffect, strategy)) {
			console.log("maxAssetCostRejected")
			return false
		}

		return true;
	}

	minProfitRejected(sideEffect: SideEffect, strategy: Strategy): boolean {
		if(strategy.minProfitUsd) {
			return sideEffect.maxProfitUsd <= strategy.minProfitUsd
		}
		return false
	}

	minYieldRejected(sideEffect: SideEffect, strategy: Strategy): boolean {
		if(strategy.minYield) {
			return this.computeYield(sideEffect) <= strategy.minYield
		}
		return false
	}

	maxTxFeesRejected(sideEffect: SideEffect, strategy: Strategy): boolean {
		if(strategy.maxTxFeesUsd) {
			return sideEffect.txCostUsd >= strategy.maxTxFeesUsd
		}
		return false
	}

	maxTxFeeShareRejected(sideEffect: SideEffect, strategy: Strategy): boolean {
		if(strategy.maxTxFeeShare) {
			return this.computeFeeShare(sideEffect) >= strategy.maxTxFeeShare
		}
		return false
	}

	maxAssetCostRejected(sideEffect: SideEffect, strategy: Strategy): boolean {
		if(strategy.maxAssetCost) {
			return sideEffect.assetCostUsd >= strategy.maxAssetCost
		}
		return false
	}
	
	computeYield(sideEffect: SideEffect) {
		return sideEffect.maxProfitUsd / sideEffect.assetCostUsd
	}

	computeFeeShare(sideEffect: SideEffect) {
		return sideEffect.txCostUsd / sideEffect.maxProfitUsd
	}
}