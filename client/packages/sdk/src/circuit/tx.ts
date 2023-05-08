import { ApiPromise } from "@polkadot/api";
import { ExtrinsicExport } from "../export";
// @ts-ignore
import {
  // @ts-ignore
  T3rnPrimitivesXdnsXdnsRecord,
  // @ts-ignore
  T3rnTypesSideEffect,
  // @ts-ignore
  u128,
} from "@polkadot/types/lookup";

/**
 * A class for batching and sending transaction to circuit. The main functionality here is signAndSendSafe, which takes care of nonce incrementation and error decoding. This is supposed to act as a default way of dealing with extrinsics.
 */

export class Tx {
  api: ApiPromise;
  signer: any;
  exportMode: boolean;

  /**
   * @param api - The ApiPromise instance
   * @param signer - The signer to use for signing Transactions
   * @param exportMode
   */

  constructor(api: ApiPromise, signer: any, exportMode: boolean) {
    this.api = api;
    this.signer = signer;
    this.exportMode = exportMode;
  }

  /**
   * Recommended when looking to send multiple TXs in a single block.
   * signAndSafeSend queries the correct nonce and then submits the transaction.
   * This should not be used when submitting transactions in fast succession as the nonce won't have time to update.
   * In that case use the optimistic send or batch the transaction.
   * If an error occurs, it is decoded and returned in the promise.
   * Returns the block height the transaction was included in.
   *
   * @param tx - The transaction to send
   *
   * @returns The block height the transaction was included in
   */

  async signAndSendSafe(tx: any): Promise<string> {
    let nonce = await this.api.rpc.system.accountNextIndex(this.signer.address);

    let exportObj = null;
    if(this.exportMode) {
      exportObj = new ExtrinsicExport(tx, this.signer.address)
    }
    return new Promise((resolve, reject) =>
      tx.signAndSend(
        this.signer,
        { nonce },
        async ({ dispatchError, status, events }) => {
          events.forEach(({ event }) => {
            exportObj?.addEvent(event);
          });
          if (dispatchError?.isModule) {
            let err = this.api.registry.findMetaError(dispatchError.asModule);
            exportObj?.addErr(dispatchError);
            reject(Error(`${err.section}::${err.name}: ${err.docs.join(" ")}`));
          } else if (dispatchError) {
            exportObj?.addErr(dispatchError);
            reject(Error(dispatchError.toString()));
          } else if (status.isInBlock) {
            resolve(status.asInBlock);
          }
        }
      )
    ).then((blockHash: any) =>
      this.api.rpc.chain
        .getBlock(blockHash)
        .then((r) => {
          const number = r.block.header.number
          exportObj?.addSubmissionHeight(number.toNumber());
          return number.toString()
        })
    );
  }

  /**
   * Wraps a transaction object into sudo
   * @param tx - The transaction to sudo
   */

  createSudo(tx: any) {
    return this.api.tx.sudo.sudo(tx);
  }

  /**
   * Batches transactions into a single batch object.
   * @param txs - The transactions to batch
   */

  createBatch(txs: any[]) {
    return this.api.tx.utility.batch(txs);
  }
}
