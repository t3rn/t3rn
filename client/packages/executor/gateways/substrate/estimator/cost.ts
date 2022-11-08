import {SubmittableExtrinsic} from "@polkadot/api/promise/types";
import SubstrateRelayer from "../relayer";
import { BehaviorSubject } from 'rxjs';

type Estimate = {
	tx: SubmittableExtrinsic,
	costSubject: BehaviorSubject<number>
}

export default class CostEstimator {
	relayer: SubstrateRelayer;
	trackingMap: Map<string, Estimate> = new Map<string, Estimate>();

	constructor(relayer: SubstrateRelayer) {
		this.relayer = relayer
		this.update()
	}

	/// returns the transaction cost of a specific side effect in native asset
	async currentTransactionCost(tx: SubmittableExtrinsic): Promise<number> {
		const paymentInfo = await tx.paymentInfo(this.relayer.signer);
		return paymentInfo.partialFee.toJSON()
	}

	// adds a sfx to tracking list and returns an observable that emits the transaction cost in native asset
	async getTxCostSubject(sfxId: string, tx: SubmittableExtrinsic): Promise<BehaviorSubject<number>> {
		const txCost = await this.currentTransactionCost(tx); // get cost of tx
		const costSubject = new BehaviorSubject<number>(txCost); // create a new subject
		this.trackingMap.set(sfxId, {tx, costSubject}) // add to tracking map
		return costSubject
	}

	// fetch new prices for all tracked transactions
	async update() {
		for (const [_sfxId, estimate] of this.trackingMap.entries()) {
			estimate.costSubject.next(await this.currentTransactionCost(estimate.tx))
		}
		// rerun this every 10s
		setTimeout(this.update.bind(this), 10000);
	}

	async stopTracking(sfxId: string) {
		this.trackingMap.delete(sfxId)
	}
}

