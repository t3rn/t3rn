import { SideEffect } from "../executionManager/sideEffect"
/**
 * The bidding engine is used for determining the bidding amount for a given 
 * side effect. It expects SFXs that have already been evaluated by the 
 * strategy engine and are deemed profitable.
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
    // Number of bids on each SFX
    numberOfBidsOnSfx: Map<string, number>

    /**
     * Computes the bidding amount for a given SFX for a certain scenario.
     *
     * @param sfx The SFX object
     * @returns The bidding amount in USD
     */
    computeBid(sfx: SideEffect): number {
        const scenario = this.checkScenario(sfx)

        let bid = 0
        switch (scenario) {
            case Scenario.noBidAndNoCompetition:
                bid = this.computeNoBidAndNoCompetition(sfx)

            case Scenario.noBidYetButCompetition:
                bid = this.computeNoBidYetButCompetition(sfx)

            case Scenario.bidAndStillTopBidder:
                bid = this.computeBidAndStillTopBidder(sfx)

            case Scenario.beenOutbid:
                bid = this.computeBeenOutbid(sfx)
        }

        return bid
    }

    /**
     * When there are no other bids, the executor wants to maximize the profit
     * 
     * @param sfx The side effect to bid on
     * @returns the amount to bid in USD
     */
    computeNoBidAndNoCompetition(sfx: SideEffect): number {
        const maxProfit = sfx.maxProfitUsd.getValue()
        const minProfit = sfx.minProfitUsd
        const txOutputCost = sfx.txOutputCostUsd
        const bidUsd = txOutputCost + minProfit + maxProfit

        return bidUsd
    }

    /**
     * When there are no other bids, the executor would like to approach the sfx
     * bidding to get the SFX.
     * 
     * @param sfx The side effect to bid on
     * @returns the amount to bid in USD
     */
    computeNoBidYetButCompetition(sfx: SideEffect): number {
        const maxProfit = sfx.maxProfitUsd.getValue()
        const minProfit = sfx.minProfitUsd
        const txOutputCost = sfx.txOutputCostUsd
        const bidUsd = txOutputCost + minProfit + maxProfit

        return bidUsd
    }

    /**
     * 
     * @param sfx 
     * @returns the amount to bid in USD
     */
    computeBidAndStillTopBidder(sfx: SideEffect): number {
        return 0
    }

    /**
     * 
     * @param sfx 
     * @returns the amount to bid in USD
     */
    computeBeenOutbid(sfx: SideEffect): number {
        return 0
    }


    /**
     * Check the scenario the executor+sfx are in
     * to select which behavior to apply.
     * 
     * @param sfx The SFX object
     * @returns The Scenario 
     */
    checkScenario(sfx: SideEffect): Scenario {
        // TODO
        const executorPlacedBid: boolean = true
        // TODO: check that this is correct
        const executorIsTopBidder = sfx.isBidder
        const otherBidsOnSFX: boolean = this.numberOfBidsOnSfx[sfx.id] > 0 ? true : false

        if (executorPlacedBid) {
            if (executorIsTopBidder) {
                return Scenario.bidAndStillTopBidder
            } else {
                return Scenario.beenOutbid
            }
        } else {
            if (otherBidsOnSFX) {
                return Scenario.noBidYetButCompetition
            } else {
                return Scenario.noBidAndNoCompetition
            }
        }
    }

    /**
     * Keep a record of how many executor have bid into each SFX.
     * 
     * @param sfx The side effect bidding on
     */
    addBidToSfx(sfx: SideEffect) {
        if (this.numberOfBidsOnSfx.get(sfx.id) != undefined) {
            this.numberOfBidsOnSfx[sfx.id] = (this.numberOfBidsOnSfx[sfx.id] + 1) || 1
        } else {
            throw new Error("Incorrect SFX id")
        }
    }

    /**
     * Check if the executor has been outbid in the SFX.
     * 
     * @param sfx The side effect in question
     * @returns true if the top bidder changed
     */
    checkOutbid(sfx: SideEffect): boolean {
        return true
    }
}


/**
 * There are different scenarios in which the bidding engine 
 * or the SFX can be.
 * Right now, we define several possible scenarios (currently, 4):
 *      - There are no bids on the SFX (noBidAndNoCompetition),
 *      - There are bids from other executors on the SFX 
 *          (noBidYetButCompetition),
 *      - You placed a bid on the SFX and are the current 
 *        top bidder (bidAndStillTopBidder).
 *      - You placed a bid (that was the top one), but other
 *        executor outbid it (beenOutbid).
 */
enum Scenario {
    noBidAndNoCompetition,
    noBidYetButCompetition,
    bidAndStillTopBidder,
    beenOutbid,
}
