import { ApiPromise } from "@polkadot/api";

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
 * The class for Transactions
 */

export class Tx {
  api: ApiPromise;
  signer: any;

  /**
   * @param api - The ApiPromise instance
   * @param signer - The signer to use for signing Transactions
   */

  constructor(api: ApiPromise, signer: any) {
    this.api = api;
    this.signer = signer;
  }

  /**
   * Safe send that queries the correct nonce and then submits the transaction.
   * This should not be used when submitting transactions in fast succession as the nonce wont have time to update.
   * In that case use the optimistic send or batch the transaction.
   * Returns the block height the transaction was included in.
   *
   * @param tx - The transaction to send
   *
   * @returns The block height the transaction was included in
   */

  async signAndSendSafe(tx): Promise<string> {
    let nonce = await this.api.rpc.system.accountNextIndex(this.signer.address);

    return new Promise((resolve, reject) =>
      tx.signAndSend(
        this.signer,
        { nonce },
        async ({ dispatchError, status }) => {
          if (dispatchError?.isModule) {
            let err = this.api.registry.findMetaError(dispatchError.asModule);
            reject(Error(`${err.section}::${err.name}: ${err.docs.join(" ")}`));
          } else if (dispatchError) {
            reject(Error(dispatchError.toString()));
          } else if (status.isInBlock) {
            resolve(status.asInBlock);
          }
        }
      )
    ).then((blockHash: any) =>
      this.api.rpc.chain
        .getBlock(blockHash)
        .then((r) => r.block.header.number.toString())
    );
  }

  /**
   * Create a sudo transaction
   * @param tx - The transaction to sudo
   */

  createSudo(tx: any) {
    return this.api.tx.sudo.sudo(tx);
  }

  /**
   * Batch transactions
   * @param txs - The transactions to batch
   */

  createBatch(txs: any[]) {
    return this.api.tx.utility.batch(txs);
  }
}
