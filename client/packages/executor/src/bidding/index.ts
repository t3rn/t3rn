import { SideEffect } from "../executionManager/sideEffect"
/**
 * The bidding engine is used for determining the bidding amount for a given 
 * side effect. It expects SFXs that have already been evaluated by the 
 * strategy engine and are deemed profitable.
 *
 * @group Bidding
 */
export class BiddingEngine {
    //** How close you are to the max profit */ 
    bidPercentile: number = 0.75
    //** At the beginning, it has never been outbid */ 
    timesBeenOutbid: number = 0
    // timesBeenOutbid: Map<SideEffect, number> = new Map<SideEffect, number>()
    //** Number of bids on each side effect */ 
    numberOfBidsOnSfx: Map<string, number>
    // numberOfBidsOnSfx: Map<SideEffect, number> = new Map<SideEffect, number>()
    //** Logger */ 
    logger: any
    //** Being aggressive when bidding means to oubtbid everyone to get the SFX */ 
    bidAggressive: boolean = false
    //** Being meek when bidding means to obtain the max profit */ 
    bidMeek: boolean = false
    //** When there's no competition yet, get the execution by getting the smallest profit */ 
    overrideNoCompetition: boolean = false
    //** If outbid, executor wants to make the same last bid */
    equalMinBid: boolean = false
    //** How close to be when been outbid, but still wants to be place a bid close to last one */ 
    closerPercentageBid: number = 0.1

    /**
     * Computes the bidding amount for a given SFX for a certain scenario.
     *
     * @param sfx The SFX object
     * @returns The bidding amount in USD
     */
    computeBid(sfx: SideEffect): number {
        const scenario = this.checkScenario(sfx)
        let bid: number = 0
        switch (scenario) {
            case Scenario.noBidAndNoCompetition:
                bid = this.computeNoBidAndNoCompetition(sfx)
            case Scenario.noBidButCompetition:
                bid = this.computeNoBidButCompetition(sfx)
            case Scenario.bidAndStillTopBidder:
                bid = this.computeBidAndStillTopBidder(sfx)
            case Scenario.beenOutbid:
                bid = this.computeBeenOutbid(sfx)
            default:
                this.logger.error("Cannot find the scenario for the bidding engine")
        }
        return bid
    }

    /**
     * When there are no other bids, the executor maximizes the profit or gets the execution.
     * 
     * @param sfx The side effect to bid on
     * @returns The bidding amount in USD
     */
    computeNoBidAndNoCompetition(sfx: SideEffect): number {
        const maxProfit = sfx.maxProfitUsd.getValue()
        const minProfit = sfx.minProfitUsd
        const txOutputCost = sfx.txOutputCostUsd
        let bidUsd: number
        if (this.overrideNoCompetition) {
            bidUsd = txOutputCost + minProfit
        } else {
            bidUsd = txOutputCost + minProfit + maxProfit
        }
        return bidUsd
    }

    /**
     * When there are other bids, the executor can choose to:
     *      - Outbid everyone
     *      - Get % (bidPercentile) close to the top bid
     *      - Get the max profit
     * The last two options suppose that executors can exit the
     * bidding process, so someone bidding less would still win it.
     * 
     * @param sfx The side effect to bid on
     * @returns The bidding amount in USD
     */
    computeNoBidButCompetition(sfx: SideEffect): number {
        const maxProfit = sfx.maxProfitUsd.getValue()
        const minProfit = sfx.minProfitUsd
        const txOutputCost = sfx.txOutputCostUsd
        let bidUsd: number
        if (this.bidAggressive) {
            bidUsd = txOutputCost + minProfit
        } else if (this.bidMeek) {
            bidUsd = txOutputCost + minProfit + maxProfit
        } else {
            bidUsd = txOutputCost + minProfit + (maxProfit - minProfit) * this.bidPercentile
        }
        return bidUsd
    }

    /**
     * When an executor has been undercut, it can choose to:
     *      - Undercut again
     *      - Keep the previous bid
     *      - Exit the bidding process
     * 
     * @param sfx 
     * @returns The bidding amount in USD
     */
    computeBeenOutbid(sfx: SideEffect): number {
        const lastBid = sfx.lastBid
        const minProfit = sfx.minProfitUsd
        const txOutputCost = sfx.txOutputCostUsd
        const minProfitBid = txOutputCost + minProfit
        const maxProfit = sfx.maxProfitUsd.getValue()
        const maxProfitBid = txOutputCost + minProfit + maxProfit

        if (sfx.lastBid === minProfitBid) {
            if (this.equalMinBid) {
                return minProfitBid
            } else {
                // TODO: here something better can be returned
                return maxProfitBid
            }
        } else {
            // Otherwise, if not wanting to make the minProftBid, it's possible
            // to get closer that minProfitBid by a certain percentage, but winning more.
            //
            // Note: this supposes that executors can drop out
            // of the bidding stage and others with larger profit bids 
            // can win the execution
            const closerBid = lastBid * (1 + this.closerPercentageBid)
            return closerBid
        }
    }

    /**
     * If the executor is still the top bidder, nothing has to be done.
     * 
     * @param _sfx 
     * @returns The bidding amount in USD
     */
    computeBidAndStillTopBidder(_sfx: SideEffect): number {
        // TODO: what to return to not modify anything else?
        return 0
    }

    /**
     * Check the scenario the executor+sfx are in
     * to select which behavior to apply.
     * 
     * @param sfx The SFX object
     * @returns The situation to choose the action 
     */
    checkScenario(sfx: SideEffect): Scenario {
        const executorPlacedBid: boolean = false
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
                return Scenario.noBidButCompetition
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
        if (this.numberOfBidsOnSfx[sfx.id] != undefined) {
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
        return sfx.changedBidLeader
    }
}


/**
 * Possible situations that define different actions.
 * 
 * There are different scenarios in which the bidding engine 
 * or the SFX can be. Right now, we define 4:
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
    noBidButCompetition,
    bidAndStillTopBidder,
    beenOutbid,
}
