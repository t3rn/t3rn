import config from "../../config/config"
import { SideEffect } from "../executionManager/sideEffect"
import { Execution } from "../executionManager/execution"

/** Type used for describing XTX strategies
 * When an XTX is created, the XTX strategy will be evaluated.
 * If the XTX fails the evaluation, the XTX will be rejected, preventing any bids to be submitted
 */
type XtxStrategy = {
    minInsuranceAmountUsd: number
    minInsuranceShare: number // minInsuranceAmountUsd / maxProfit
}

/** Type used for describing SFX strategies.
 * These are used to determine the profitability of a given SFX, deciding if a bid should be submitted.
 */
type SfxStrategy = {
    minProfitUsd: number
    minYield: number
    maxTxFeesUsd: number
    maxTxFeeShare: number // txCost / maxProfit
    maxAssetCost: number // maximum value spend
}


export class StrategyEngine {
    sfxStrategies: {
        [target: string]: SfxStrategy
    } = {}

    xtxStrategies: {
        [target: string]: XtxStrategy
    } = {}

    constructor() {
        this.loadStrategies()
    }

    /** Loads the strategies from the config file. */
    loadStrategies() {
        const strategyTargets = Object.keys(config.strategies)
        for (let i = 0; i < strategyTargets.length; i++) {
            this.sfxStrategies[strategyTargets[i]] = {
                minProfitUsd: config.strategies[strategyTargets[i]].sfx.minProfitUsd,
                minYield: config.strategies[strategyTargets[i]].sfx.minYield,
                maxTxFeesUsd: config.strategies[strategyTargets[i]].sfx.maxTxFeesUsd,
                maxTxFeeShare: config.strategies[strategyTargets[i]].sfx.maxTxFeeShare,
                maxAssetCost: config.strategies[strategyTargets[i]].sfx.maxAssetCost,

            }
            this.xtxStrategies[strategyTargets[i]] = {
                minInsuranceAmountUsd: config.strategies[strategyTargets[i]].xtx.minInsuranceAmountUsd,
                minInsuranceShare: config.strategies[strategyTargets[i]].xtx.minInsuranceShare,
            }
        }
    }

    /** Evaluates the XTX strategy for a given XTX.
     * If the XTX fails the evaluation, the XTX will be rejected, preventing any bids to be submitted
     * @param xtx object of XTX to be evaluated
     */
    evaluateXtx(xtx: Execution): void | Error {
        for (let [_id, sfx] of xtx.sideEffects) {
            const strategy = this.xtxStrategies[sfx.target]
            try {
                this.minInsuranceAmountRejected(sfx, strategy)
                this.minInsuranceShareRejected(sfx, strategy)
            } catch (e) {
                break
                throw e
            }
        }
    }

    /** Evaluates the SFX strategy for a given SFX.
     *
     * @param sfx object of SFX to be evaluated
     */
    evaluateSfx(sfx: SideEffect): void | Error {
        const strategy = this.sfxStrategies[sfx.target]

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
        const strategy = this.sfxStrategies[sfx.target]
        return strategy.minProfitUsd
    }

    minProfitRejected(sideEffect: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.minProfitUsd) {
            if (sideEffect.maxProfitUsd.getValue() <= strategy.minProfitUsd) {
                throw new Error("Min Profit condition not met!")
            }
        }
    }

    minYieldRejected(sideEffect: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.minYield) {
            if (this.computeYield(sideEffect) <= strategy.minYield) {
                throw new Error("Min Yield condition not met!")
            }
        }
    }

    maxTxFeesRejected(sideEffect: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.maxTxFeesUsd) {
            if (sideEffect.txCostUsd >= strategy.maxTxFeesUsd) {
                throw new Error("Max Tx Fees condition not met!")
            }
        }
    }

    maxTxFeeShareRejected(sideEffect: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.maxTxFeeShare) {
            if (this.computeFeeShare(sideEffect) >= strategy.maxTxFeeShare) {
                throw new Error("Max Tx Fee Share condition not met!")
            }
        }
    }

    maxAssetCostRejected(sideEffect: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.maxAssetCost) {
            if (sideEffect.txOutputCostUsd >= strategy.maxAssetCost) {
                throw new Error("Max Asset Cost condition not met!")
            }
        }
    }

    minInsuranceAmountRejected(sideEffect: SideEffect, strategy: XtxStrategy): void | Error {
        if (strategy.minInsuranceAmountUsd) {
            if (sideEffect.insurance < strategy.minInsuranceAmountUsd) {
                throw new Error("Min Insurance Amount  condition not met!")
            }
        }
    }

    minInsuranceShareRejected(sideEffect: SideEffect, strategy: XtxStrategy): void | Error {
        if (strategy.minInsuranceShare) {
            // reward and insurance are in the same asset, so no USD conversion is needed
            if (sideEffect.insurance / sideEffect.reward.getValue() < strategy.minInsuranceAmountUsd) {
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
