import { SideEffect } from "../executionManager/sideEffect"
import { config } from "../../config/config"

/**
 * The bidding engine is used for determining the bidding amount for a given 
 * side effect. It expects SFXs that have already been evaluated by the 
 * strategy engine and are deemed profitable.
 *
 * @group Bidding
 */
export class BiddingEngine {
    /** How close you are to the max profit */
    bidPercentile: number = config.bidding.bidPercentile
    /** At the beginning, it has never been outbid */
    timesBeenOutbid: Map<string, number>
    // timesBeenOutbid: Map<SideEffect, number> = new Map<SideEffect, number>()
    /** Number of bids on each side effect. KEYs: sfx id; VALUEs: number of bids */
    numberOfBidsOnSfx: Map<string, number>
    // numberOfBidsOnSfx: Map<SideEffect, number> = new Map<SideEffect, number>()
    /** Logger */
    logger: any
    /** Being aggressive when bidding means to oubtbid everyone to get the SFX */
    bidAggressive: boolean = config.bidding.bidAggressive
    /** Being meek when bidding means to obtain the max profit */
    bidMeek: boolean = config.bidding.bidMeek
    /** When there's no competition yet, get the execution by getting the smallest profit */
    overrideNoCompetition: boolean = config.bidding.overrideNoCompetition
    /** If outbid, executor makes the same last bid */
    equalMinProfitBid: boolean = config.bidding.equalMinBid
    /** How close to be when been outbid, but still wants to be place a bid close to last one */
    closerPercentageBid: number = config.bidding.closerPercentageBid
    /** Which executor is bidding on which side effect. KEYs: sfx id; VALUEs: executor ids array */
    whoBidsOnWhat: Map<string, string[]>

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
        const lastBid = sfx.lastBids.at(-1) || 0
        const minProfit = sfx.minProfitUsd
        const txOutputCost = sfx.txOutputCostUsd
        const minProfitBid = txOutputCost + minProfit
        const maxProfit = sfx.maxProfitUsd.getValue()
        const maxProfitBid = txOutputCost + minProfit + maxProfit

        if (lastBid === minProfitBid) {
            // If the last bid was the minProfitBid,
            // return the same value (in case others dropout)
            return minProfitBid
        } else {
            // Otherwise, get closer that minProfitBid from above
            const closerBid = lastBid * (1 + this.closerPercentageBid)
            if (closerBid >= maxProfitBid) {
                return maxProfitBid
            } else {
                return closerBid
            }
        }
    }

    /**
     * Check the scenario the executor+sfx are in
     * to select which behavior to apply.
     * 
     * @param sfx The SFX object
     * @returns The situation to choose the action 
     */
    checkScenario(sfx: SideEffect): Scenario {
        const executorIsTopBidder = sfx.isBidder
        const otherBidsOnSFX: boolean = sfx.lastBids.length > 0 ? true : false

        if (!executorIsTopBidder) {
            return Scenario.beenOutbid
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
        if (this.numberOfBidsOnSfx[sfx.id] !== undefined) {
            this.numberOfBidsOnSfx[sfx.id] = (this.numberOfBidsOnSfx[sfx.id] + 1) || 1
        } else {
            throw new Error("Incorrect SFX id")
        }
    }

    /**
     * Check if the executor has been outbid in the SFX, and if so, updates the map.
     * 
     * @param sfx The side effect in question
     * @returns true if the top bidder changed
     */
    checkOutbid(sfx: SideEffect): boolean {
        const changedBidLeader = sfx.changedBidLeader
        if (changedBidLeader) {
            this.timesBeenOutbid[sfx.id] = (this.timesBeenOutbid[sfx.id] + 1) || 1
        }
        return changedBidLeader
    }

    /**
     * Store who (bidder id) bids on what (sfx id), to 
     * keep a "database" to, maybe, implement exclusion rules
     * for bidding (e.g., I don't want to bid if this ID 
     * is in there; or bid to keep those people out).
     * 
     * @param sfxId The side effect ID
     * @param bidderId The executor ID
     */
    storeWhoBidOnWhat(sfxId: string, bidderId: string) {
        if (this.whoBidsOnWhat !== undefined) {
            let previousBidderIds = this.whoBidsOnWhat.get(sfxId)
            if (previousBidderIds !== undefined) {
                previousBidderIds.push(bidderId);
                this.whoBidsOnWhat.set(sfxId, previousBidderIds);
            } else {
                this.whoBidsOnWhat.set(sfxId, [bidderId]);
            }
        } else {
            this.whoBidsOnWhat = new Map([[sfxId, [bidderId]]]);
        }
    }

    /**
     * Clean the data structures after the bidding phase is over.
     * Called in `removeFromQueue` in executionManager
     * 
     * @param sfxId The side effect ID
     */
    cleanUp(sfxId: string) {
        this.numberOfBidsOnSfx.delete(sfxId)
        this.whoBidsOnWhat.delete(sfxId)
        this.timesBeenOutbid.delete(sfxId)
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
    noBidButCompetition,   // <- might not be useful
    beenOutbid,            // <- 
}
