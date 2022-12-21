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

    // At the beginning, it has never been outbid
    timesBeenOutbid: number = 0
    // 

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

    /**
     * Check the scenario the executor+sfx are in
     * to select which behavior to apply.
     * 
     * @param sfx The SFX object being outbid
     * @returns 
     */
    checkScenario(sfx: SideEffect): Scenario {
        // 
        const topBid = sfx.isBidder

        // We need a way to check if a bid has been placed
        const placedBid: boolean = true
        // We need a way to check if there are other bids
        const otherBids: boolean = true

        if (placedBid) {
            if (topBid) {
                return Scenario.bidAndStillTopBidder
            } else {
                return Scenario.beenOutbid
            }
        } else {
            if (otherBids) {
                return Scenario.noBidYetButCompetition
            } else {
                return Scenario.noBidAndNoCompetition
            }
        }
    }
}


/**
 * There are different scenarios in which the bidding engine 
 * or the SFX can find themselves in.
 * 
 * Right now, we define several possible scenarios (currently, 4):
 *      - There are no bids on the SFX,
 *      - There are bids from other executors on the SFX,
 *      - You placed a bid on the SFX and are the current 
 *        top bidder.
 *      - You placed a bid (that was the top one), but other
 *        executor outbid it.
 */
enum Scenario {
    noBidAndNoCompetition,
    noBidYetButCompetition,
    bidAndStillTopBidder,
    beenOutbid,
}
