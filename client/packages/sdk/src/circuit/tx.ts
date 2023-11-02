import { ApiPromise } from '@polkadot/api'
import { ExtrinsicExport } from '../export'
import { SubmittableExtrinsic } from '@polkadot/api/promise/types'
import { SignerOptions } from '@polkadot/api/types/submittable'
import { EventRecord } from '@polkadot/types/interfaces'

/**
 * A class for batching and sending transaction to circuit. The main functionality here is signAndSendSafe, which takes care of nonce incrementation and error decoding. This is supposed to act as a default way of dealing with extrinsics.
 */

export class Tx {
  api: ApiPromise
  signer: any
  exportMode: boolean

  /**
   * @param api - The ApiPromise instance
   * @param signer - The signer to use for signing Transactions
   * @param exportMode
   */

  constructor(api: ApiPromise, signer: any, exportMode: boolean) {
    this.api = api
    this.signer = signer
    this.exportMode = exportMode
  }

  /**
   *
   *
   * @param {SubmittableExtrinsic} tx
   * @param {Partial<SignerOptions>} options
   * @returns {*}  {Promise<string>}
   * @memberof Tx
   */
  async signAndSend(
    tx: SubmittableExtrinsic,
    options: Partial<SignerOptions>,
  ): Promise<any> {
    let exportObj: ExtrinsicExport

    if (this.exportMode) {
      exportObj = new ExtrinsicExport(tx, this.signer.address)
    }
    return new Promise((resolve, reject) =>
      tx.signAndSend(
        this.signer,
        options,
        async ({ dispatchError, status, events }) => {
          events.forEach(({ event }) => {
            exportObj?.addEvent(event)
          })

          if (dispatchError?.isModule) {
            const err = this.api.registry.findMetaError(dispatchError.asModule)

            exportObj?.addErr(dispatchError).toFile()
            reject(Error(`${err.section}::${err.name}`))
          } else if (dispatchError) {
            exportObj?.addErr(dispatchError).toFile()

            reject(Error(dispatchError.toString()))
          } else if (status.isInBlock) {
            resolve({ status, events, dispatchError })
          }
        },
      ),
    ).then((result: any) => {
      return result
    })
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

  async signAndSendSafe(tx: SubmittableExtrinsic): Promise<string> {
    const nonce = await this.api.rpc.system.accountNextIndex(
      this.signer.address,
    )
    let exportObj: ExtrinsicExport

    if (this.exportMode) {
      exportObj = new ExtrinsicExport(tx, this.signer.address)
    }

    return new Promise((resolve, reject) =>
      tx.signAndSend(
        this.signer,
        { nonce },
        async ({ dispatchError, status, events }) => {
          events.forEach(({ event }) => {
            exportObj?.addEvent(event)
          })

          if (dispatchError?.isModule) {
            const err = this.api.registry.findMetaError(dispatchError.asModule)

            exportObj?.addErr(dispatchError).toFile()
            reject(Error(`${err.section}::${err.name}: ${err.docs.join(' ')}`))
          } else if (dispatchError) {
            exportObj?.addErr(dispatchError).toFile()
            reject(Error(dispatchError.toString()))
          } else if (events.length > 0) {
            // check if we have an error event in a custom module
            events.forEach((eventEntry: EventRecord) => {
              const eventEntryParsed = JSON.parse(JSON.stringify(eventEntry))
              if (
                eventEntryParsed &&
                eventEntryParsed.event &&
                eventEntryParsed.event.data &&
                Array.isArray(eventEntryParsed.event.data) &&
                eventEntryParsed.event.data.length > 0 &&
                eventEntryParsed.event.data[0].err
              ) {
                const pallet =
                  eventEntryParsed.event.data[0].err.module.index ||
                  'Un-parsed pallet index'
                const error =
                  eventEntryParsed.event.data[0].err.module.error ||
                  'Un-parsed error index'
                const moduleErrorMessage = `Pallet of index = ${pallet} returned an error of index = ${error}`
                exportObj?.addErr(moduleErrorMessage).toFile()
                reject(Error(moduleErrorMessage))
              }
            })
          } else if (
            status.isDropped ||
            status.isInvalid ||
            status.isUsurped ||
            status.isRetracted
          ) {
            reject(Error(status.type))
          } else if (status.isInBlock) {
            resolve(status['inBlock'] || status.asInBlock.toString())
          }
        },
      ),
    ).then((blockHash: any) =>
      this.api.rpc.chain.getBlock(blockHash).then((r) => {
        const number = r.block.header.number

        exportObj?.addSubmissionHeight(number.toNumber()).toFile()
        return number.toString()
      }),
    )
  }

  async signAndSendRaw(tx: SubmittableExtrinsic): Promise<any> {
    const nonce = await this.api.rpc.system.accountNextIndex(
      this.signer.address,
    )
    let exportObj: ExtrinsicExport

    if (this.exportMode) {
      exportObj = new ExtrinsicExport(tx, this.signer.address)
    }

    return new Promise((resolve, reject) =>
      tx.signAndSend(
        this.signer,
        { nonce },
        async ({ dispatchError, status, events }) => {
          events.forEach(({ event }) => {
            exportObj?.addEvent(event)
          })
          if (status.isInBlock) {
            resolve(events)
          }
        },
      ),
    ).then((events: any) => {
      return events
    })
  }

  /**
   * Wraps a transaction object into sudo
   * @param tx - The transaction to sudo
   */

  createSudo(tx: any) {
    return this.api.tx.sudo.sudo(tx)
  }

  /**
   * Batches transactions into a single batch object.
   * @param txs - The transactions to batch
   */

  createBatch(txs: any[]) {
    return this.api.tx.utility.batch(txs)
  }
}
