import {SideEffect} from "../executionManager/sideEffect";



export class BiddingEngine {

	// for now, we create bids based on the bidding percentile. In the future, we might want to use a more sophisticated bidding strategy
	// we know the minProfitAmount and maxProfitAmount, so the bidding percentile is used to compute the next bidding amount
	bidPercentile: number = 0.75;


	computeBid(sfx: SideEffect): number {
		const maxProfit = sfx.maxProfitUsd.getValue();
		const minProfit = sfx.minProfitUsd;
		const txOutputCost = sfx.txOutputCostUsd;
		const bidUsd = txOutputCost + minProfit + (maxProfit - minProfit) * this.bidPercentile;

		return bidUsd;
	}

}