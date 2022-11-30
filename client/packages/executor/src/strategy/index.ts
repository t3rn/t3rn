import config from "../../config/config"
import { SideEffect } from "../executionManager/sideEffect"
import { Execution } from "../executionManager/execution"

type Strategy = {
    minProfitUsd: number
    minYield: number
    maxTxFeesUsd: number
    maxTxFeeShare: number // txCost / maxProfit
    maxAssetCost: number // maximum value spend
    minInsuranceAmountUsd: number
    minInsuranceShare: number // minInsuranceAmountUsd / maxProfit
}

export class StrategyEngine {
    strategies: {
        [target: string]: Strategy
    } = {}

    constructor() {
        this.loadStrategies()
    }

    loadStrategies() {
        const strategyTargets = Object.keys(config.strategies)
        for (let i = 0; i < strategyTargets.length; i++) {
            this.strategies[strategyTargets[i]] = {
                minProfitUsd:
                    config.strategies[strategyTargets[i]].minProfitUsd,
                minYield: config.strategies[strategyTargets[i]].minYield,
                maxTxFeesUsd:
                    config.strategies[strategyTargets[i]].maxTxFeesUsd,
                maxTxFeeShare:
                    config.strategies[strategyTargets[i]].maxTxFeeShare,
                maxAssetCost:
                    config.strategies[strategyTargets[i]].maxAssetCost,
                minInsuranceAmountUsd:
                    config.strategies[strategyTargets[i]].minInsuranceAmountUsd,
                minInsuranceShare:
                    config.strategies[strategyTargets[i]].minInsuranceShare,
            }
        }
    }

    // evaluate if the conditions for a given xtx are met. If this is not the case, the xtx will be untracked
    evaluateXtx(xtx: Execution): void | Error {
        for (let [_id, sfx] of xtx.sideEffects) {
            const strategy = this.strategies[sfx.target]
            try {
                this.minInsuranceAmountRejected(sfx, strategy)
                this.minInsuranceShareRejected(sfx, strategy)
            } catch (e) {
                break
                throw e
            }
        }
    }

    // checks the constraints for any given side effect
    evaluateSfx(sfx: SideEffect): void | Error {
        const strategy = this.strategies[sfx.target]

        try {
            this.minProfitRejected(sfx, strategy)
            this.minYieldRejected(sfx, strategy)
            this.maxTxFeesRejected(sfx, strategy)
            this.maxTxFeeShareRejected(sfx, strategy)
            this.maxAssetCostRejected(sfx, strategy)
        } catch (e) {
            throw e
        }
    }

    getMinProfitUsd(sfx: SideEffect) {
        const strategy = this.strategies[sfx.target]
        return strategy.minProfitUsd
    }

    minProfitRejected(
        sideEffect: SideEffect,
        strategy: Strategy
    ): void | Error {
        if (strategy.minProfitUsd) {
            if (sideEffect.maxProfitUsd.getValue() <= strategy.minProfitUsd) {
                throw new Error("Min Profit condition not met!")
            }
        }
    }

    minYieldRejected(sideEffect: SideEffect, strategy: Strategy): void | Error {
        if (strategy.minYield) {
            if (this.computeYield(sideEffect) <= strategy.minYield) {
                throw new Error("Min Yield condition not met!")
            }
        }
    }

    maxTxFeesRejected(
        sideEffect: SideEffect,
        strategy: Strategy
    ): void | Error {
        if (strategy.maxTxFeesUsd) {
            if (sideEffect.txCostUsd >= strategy.maxTxFeesUsd) {
                throw new Error("Max Tx Fees condition not met!")
            }
        }
    }

    maxTxFeeShareRejected(
        sideEffect: SideEffect,
        strategy: Strategy
    ): void | Error {
        if (strategy.maxTxFeeShare) {
            if (this.computeFeeShare(sideEffect) >= strategy.maxTxFeeShare) {
                throw new Error("Max Tx Fee Share condition not met!")
            }
        }
    }

    maxAssetCostRejected(
        sideEffect: SideEffect,
        strategy: Strategy
    ): void | Error {
        if (strategy.maxAssetCost) {
            if (sideEffect.txOutputCostUsd >= strategy.maxAssetCost) {
                throw new Error("Max Asset Cost condition not met!")
            }
        }
    }

    minInsuranceAmountRejected(
        sideEffect: SideEffect,
        strategy: Strategy
    ): void | Error {
        if (strategy.minInsuranceAmountUsd) {
            if (sideEffect.insurance < strategy.minInsuranceAmountUsd) {
                throw new Error("Min Insurance Amount  condition not met!")
            }
        }
    }

    minInsuranceShareRejected(
        sideEffect: SideEffect,
        strategy: Strategy
    ): void | Error {
        if (strategy.minInsuranceShare) {
            // reward and insurance are in the same asset, so no USD conversion is needed
            if (
                sideEffect.insurance / sideEffect.reward.getValue() <
                strategy.minInsuranceAmountUsd
            ) {
                throw new Error("Min Insurance Share condition not met!")
            }
        }
    }

    computeYield(sideEffect: SideEffect) {
        return sideEffect.maxProfitUsd.getValue() / sideEffect.txOutputCostUsd
    }

    computeFeeShare(sideEffect: SideEffect) {
        return sideEffect.txCostUsd / sideEffect.maxProfitUsd.getValue()
    }
}
