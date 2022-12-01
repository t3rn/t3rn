import config from "../../config/config"
import { SideEffect } from "../executionManager/SideEffect"
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
    /** Minimum profit in USD that a SFX should have to be considered profitable.*/
    minProfitUsd: number
    /** A percentage value for the minimum yield that a SFX should have
     * Yield is defined by (minProfit / totalCost)
     * */
    minYield: number
    /** The max tx costs in USD for an execution.
     * This can be useful to prevent executions during network congestion.
     */
    maxTxFeesUsd: number
    /** The max share of txCost to profit.
     * This is defined by txCost / maxProfit
     */
    maxTxFeeShare: number // txCost / maxProfit
    /** The maximum cost in USD to spend on a single SFX.
     * This only includes the cost of the assets that are being sent.
     * This puts a cap on the value of assets to be sent.
     * */
    maxAssetCost: number // maximum value spend
}

/** The strategy engine is used to decide if a SFX should be executed or not.
 * The decision is seperated into two parts:
 *
 * XTX Strategy:
 * Before the strategy engine evaluates individual SFXs, it evaluates the XTX as a whole.
 * To complete a XTX, all SFXs must be executed and confirmed.
 * For this reason the XTX strategy can be used to evaluate a set of constraints for each SFX in the XTX.
 * If any of the SFXs in the XTX fail the XTX strategy, the XTX will be rejected and not further tracked.
 *
 * For example, this can be useful for ensuring every SFX as a certain amount of insurance.
 * XTXs containing SFXs with low insurance risk being reverted, as the fine for not completing the SFX is potentially too low.
 * Executors can use the parameters provided in the XtXStrategy to enforce XTX-wide constraints.
 *
 * SFX Strategy:
 * If an XTX passes the XTX strategy, the strategy engine will evaluate each SFX individually.
 * To goal here is to determine if a SFX passes all constraints set by the SFX strategy.
 * If so, the executor will then submit a bid for the SFX.
 * The bid amount is _not_ determined by the strategy engine, but by the bidding engine.
 *
 */
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

    /** Returns minProfitUsd constraint from the SFX strategy for a given target.
     *
     * @param sfx object of SFX to be evaluated
     * @returns minProfitUsd constraint from the SFX strategy for a given target
     */
    getMinProfitUsd(sfx: SideEffect): number {
        const strategy = this.sfxStrategies[sfx.target]
        return strategy.minProfitUsd
    }

    /** Evaluates the minProfitUsd constraint from the SFX strategy for a given SFX.
     *
     * @param sfx side effect to be evaluated
     * @param strategy strategy for the specific target
     * @returns error if the minProfitUsd constraint is not met
     */
    minProfitRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.minProfitUsd) {
            if (sfx.maxProfitUsd.getValue() <= strategy.minProfitUsd) {
                throw new Error("Min Profit condition not met!")
            }
        }
    }

    /** Evaluates the minYield constraint from the SFX strategy for a given SFX.
     *
     * @param sfx side effect to be evaluated
     * @param strategy strategy for the specific target
     * @returns error if the minYield constraint is not met
     */
    minYieldRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.minYield) {
            if (this.computeShare(sfx, "yield") <= strategy.minYield) {
                throw new Error("Min Yield condition not met!")
            }
        }
    }

    /** Evaluates the maxTxFeesUsd constraint from the SFX strategy for a given SFX.
     *
     * @param sfx side effect to be evaluated
     * @param strategy strategy for the specific target
     * @returns error if the maxTxFeesUsd constraint is not met
     */
    maxTxFeesRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.maxTxFeesUsd) {
            if (sfx.txCostUsd >= strategy.maxTxFeesUsd) {
                throw new Error("Max Tx Fees condition not met!")
            }
        }
    }

    /** Evaluates the maxTxFeeShare constraint from the SFX strategy for a given SFX.
     *
     * @param sfx side effect to be evaluated
     * @param strategy strategy for the specific target
     * @returns error if the maxTxFeeShare constraint is not met
     */
    maxTxFeeShareRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.maxTxFeeShare) {
            if (this.computeShare(sfx, "fee") >= strategy.maxTxFeeShare) {
                throw new Error("Max Tx Fee Share condition not met!")
            }
        }
    }

    /** Evaluates the maxAssetCost constraint from the SFX strategy for a given SFX.
     *
     * @param sfx side effect to be evaluated
     * @param strategy strategy for the specific target
     * @returns error if the maxAssetCost constraint is not met
     */
    maxAssetCostRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
        if (strategy.maxAssetCost) {
            if (sfx.txOutputCostUsd >= strategy.maxAssetCost) {
                throw new Error("Max Asset Cost condition not met!")
            }
        }
    }

    /** Evaluates the minInsuranceAmountUsd constraint from the SFX strategy for a given SFX.
     *
     * @param sfx side effect to be evaluated
     * @param strategy strategy for the specific target
     * @returns error if the minInsuranceAmountUsd constraint is not met
     */
    minInsuranceAmountRejected(sfx: SideEffect, strategy: XtxStrategy): void | Error {
        if (strategy.minInsuranceAmountUsd) {
            if (sfx.insurance < strategy.minInsuranceAmountUsd) {
                throw new Error("Min Insurance Amount  condition not met!")
            }
        }
    }

    /** Evaluates the minInsuranceAmountUsd constraint from the SFX strategy for a given SFX.
     *
     * @param sfx side effect to be evaluated
     * @param strategy strategy for the specific target
     * @returns error if the minInsuranceAmountUsd constraint is not met
     */
    minInsuranceShareRejected(sfx: SideEffect, strategy: XtxStrategy): void | Error {
        if (strategy.minInsuranceShare) {
            // reward and insurance are in the same asset, so no USD conversion is needed
            if (this.computeShare(sfx, "insurance") > strategy.minInsuranceShare) {
                throw new Error("Min Insurance Share condition not met!")
            }
        }
    }

    /** Computes different types of shares for a given SFX.
     *
     * @param sfx object of SFX compute
     * @param type - fee, insurance or yield
     * @returns share of the given type
     */
    computeShare(sfx: SideEffect, type: string): number {
        if (type === "fee") {
            return sfx.txCostUsd / sfx.maxProfitUsd.getValue()
        } else if (type === "insurance") {
            return sfx.insurance / sfx.reward.getValue()
        } else if (type === "yield") {
            return sfx.maxProfitUsd.getValue() / sfx.txOutputCostUsd
        }

        return 0
    }
}
