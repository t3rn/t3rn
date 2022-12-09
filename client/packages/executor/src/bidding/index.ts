import { SideEffect } from "../executionManager/sideEffect"
/**
 * The bidding engine is used for determining the bidding amount for a given side effect. It expects SFXs that have already been evaluated
 * by the strategy engine and are deemed profitable.
 *
 * @group Bidding
 */
export class BiddingEngine {
    /**
     * For now, we create bids based on the bidding percentile. In the future, we might want to use a more sophisticated bidding strategy we
     * know the minProfitAmount and maxProfitAmount, so the bidding percentile is used to compute the next bidding amount
     */
    bidPercentile: number = 0.75

    /**
     * Computes the bidding amount for a given SFX. Currently, this is implemented in the simplest way possible, by using the
     * minProfitAmount and maxProfitAmount. In the future, we might want to use a more sophisticated bidding strategy.
     *
     * @param sfx The SFX object
     * @returns The bidding amount in USD
     */
    computeBid(sfx: SideEffect): number {
        const maxProfit = sfx.maxProfitUsd.getValue()
        const minProfit = sfx.minProfitUsd
        const txOutputCost = sfx.txOutputCostUsd
        const bidUsd = txOutputCost + minProfit + (maxProfit - minProfit) * this.bidPercentile

        return bidUsd
    }
}
