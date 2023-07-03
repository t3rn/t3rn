import { SubmittableExtrinsic } from "@polkadot/api/promise/types";
import { SubstrateRelayer } from "../relayer";
import { BehaviorSubject } from "rxjs";

/** Type used for storing an TX cost estimate
 * @group Gateways
 * @category Substrate
 */
export type Estimate = {
  tx: SubmittableExtrinsic;
  costSubject: BehaviorSubject<number>;
};

/** Class used for estimating the TX cost on the target chain.
 * 	It makes use of substrates ts.estimateFee() method to estimate the cost of the TX.
 * 	It is also a producer, able to trigger the re-evaluation of a given SFX. Currently, this is triggered every 30s
 *
 * @group Gateways
 * @category Substrate
 */
export class CostEstimator {
  relayer: SubstrateRelayer;
  //ToDo we need to stop tracking when the SFX is completed
  /** Map containg the estimates for each SFX */
  trackingMap: Map<string, Estimate> = new Map<string, Estimate>();

  constructor(relayer: SubstrateRelayer) {
    this.relayer = relayer;
    this.update();
  }

  /** returns the transaction cost of a specific side effect in native asset
   *
   * @param tx of the SFX on target
   * @returns the cost of the SFX in native asset
   */
  async currentTransactionCost(tx: SubmittableExtrinsic): Promise<number> {
    const paymentInfo = await tx.paymentInfo(this.relayer.signer);
    return paymentInfo.partialFee.toJSON();
  }

  /** adds a sfx to tracking list and returns an observable that emits the transaction cost in native asset
   *
   * @param sfxId
   * @param tx of the SFX on target
   */
  async getTxCostSubject(
    sfxId: string,
    tx: SubmittableExtrinsic
  ): Promise<BehaviorSubject<number>> {
    const txCost = await this.currentTransactionCost(tx); // get cost of tx
    const costSubject = new BehaviorSubject<number>(txCost); // create a new subject
    this.trackingMap.set(sfxId, { tx, costSubject }); // add to tracking map
    return costSubject;
  }

  /** Automatically fetch new prices for all tracked transactions and publish
   *
   */
  async update() {
    for (const [, estimate] of this.trackingMap.entries()) {
      const txCost = await this.currentTransactionCost(estimate.tx);
      if (txCost !== estimate.costSubject.getValue()) {
        estimate.costSubject.next(txCost);
      }
    }

    setTimeout(this.update.bind(this), 30000);
  }

  async stopTracking(sfxId: string) {
    this.trackingMap.delete(sfxId);
  }
}
