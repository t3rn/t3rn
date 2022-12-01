import { EventEmitter } from "events"
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"
import { EventMapper, SideEffect } from "../../executionManager/sideEffect"
import { getEventProofs } from "../../utils"
import createDebug from "debug"
import { SubmittableExtrinsic } from "@polkadot/api/promise/types"
import { SfxType } from "@t3rn/sdk/dist/src/side-effects/types"
import {InclusionData, RelayerEventData, RelayerEvents} from "../types"
import Estimator from "./estimator"
import {CostEstimator, Estimate} from "./estimator/cost"

/** Class responsible for submitting transactions to a target chain.
 * Three main tasks are handled by this class:
 * - Build correct TX objects for the target chain
 * - Sign and submit the TXs
 * - Generate inclusion proofs for the TXs
 *
 * @group Gateways
 * @category Substrate
 */
export class SubstrateRelayer extends EventEmitter {
    static debug = createDebug("substrate-relayer")

    /** Target chain client */
    client: ApiPromise
    /** Signer for target */
    signer: any
    /** Nonce of the signer, tracked locally to enable optimistic nonce increment */
    nonce: number
    /** Name of the target */
    name: string
    logger: any

    async setup(rpc: string, signer: string | undefined, name: string, logger: any) {
        this.client = await ApiPromise.create({
            provider: new WsProvider(rpc),
        })
        const keyring = new Keyring({ type: "sr25519" })

        this.signer = signer === undefined ? keyring.addFromUri("//Executor//default") : keyring.addFromMnemonic(signer)

        this.nonce = await this.fetchNonce(this.client, this.signer.address)
        this.name = name
        this.logger = logger
    }

    /** Builds the TX object for the different types of SFX
     *
     * @param sideEffect object
     * @returns SubmittableExtrinsic tx that can be submitted to the target
     */
    buildTx(sideEffect: SideEffect): SubmittableExtrinsic {
        switch (sideEffect.action) {
            case SfxType.Transfer: {
                const data = sideEffect.execute()
                return this.client.tx.balances.transfer(data[0], data[1])
            }
        }
    }

    /** Submit the transaction to the target chain.
     * This function increments the nonce locally to enable optimistic nonce increment.
     * This allows transaction to be submitted in parallel without waiting for the previous one to be included.
     * A successful submission will trigger inclusion proof generation.
     * @param sfx object to execute
     */
    async executeTx(sfx: SideEffect) {
        this.logger.info(`Execution started SFX: ${sfx.humanId} - ${sfx.target} with nonce: ${this.nonce} 🔮`)
        const tx: SubmittableExtrinsic = this.buildTx(sfx)
        let nonce = this.nonce
        this.nonce += 1 // we optimistically increment the nonce before we go async. If the tx fails, we will decrement it which might be a bad idea
        return new Promise<void>((resolve, reject) =>
            tx.signAndSend(this.signer, { nonce }, async ({ dispatchError, status, events }) => {
                if (dispatchError?.isModule) {
                    // something went wrong and we can decode the error
                    let err = this.client.registry.findMetaError(dispatchError.asModule)
                    this.logger.info(`Execution failed SFX: ${sfx.humanId}`)
                    this.emit("Event", <RelayerEventData>{
                        type: RelayerEvents.SfxExecutionError,
                        data: `${err.section}::${err.name}: ${err.docs.join(" ")}`,
                        sfxId: sfx.id,
                    })
                    // we attempt to restore the correct nonce
                    this.nonce = await this.fetchNonce(this.client, this.signer.address)
                    reject(Error(`${err.section}::${err.name}: ${err.docs.join(" ")}`))
                } else if (dispatchError) {
                    // something went wrong and we can't decode the error
                    this.emit("Event", <RelayerEventData>{
                        type: RelayerEvents.SfxExecutionError,
                        data: dispatchError.toString(),
                        sfxId: sfx.id,
                    })
                    // we attempt to restore the correct nonce
                    this.nonce = await this.fetchNonce(this.client, this.signer.address)
                    reject(Error(dispatchError.toString()))
                } else if (status.isFinalized) {
                    const blockNumber = await this.generateInclusionProof(sfx, status.asFinalized, events)
                    this.logger.info(`Execution complete SFX:  ${sfx.humanId} - #${blockNumber} 🏁`)
                    this.emit("Event", <RelayerEventData>{
                        type: RelayerEvents.SfxExecutedOnTarget,
                        sfxId: sfx.id,
                        target: this.name,
                        data: "",
                        blockNumber,
                    })
                    resolve()
                }
            })
        )
    }

    /** Generates the inclusion proof of an executed transaction/SFX and adds it to the SFX object
     *
     * @param sfx object
     * @param blockHash hash of the block in which the transaction was included
     * @param events events emitted by the transaction
     * @returns block number in which the transaction was included
     */
    async generateInclusionProof(sfx: SideEffect, blockHash: any, events: any[]): Promise<number> {
        const blockNumber = await this.getBlockNumber(blockHash)
        const event = this.getEvent(sfx.action, events)

        const inclusionProof = await getEventProofs(this.client, blockHash)
        const inclusionData: InclusionData = {
            encoded_payload: event.toHex(),
            proof: {
                // @ts-ignore
                trieNodes: inclusionProof.toJSON().proof,
            },
            block_hash: blockHash,
        }

        // Add the inclusion proof to the SFX object
        sfx.executedOnTarget(inclusionData, this.signer.addressRaw, blockNumber.toNumber())

        return blockNumber.toNumber()
    }

    /** Fetch block number from block hash
     *
     * @param hash of block
     * @returns block number
     */
    async getBlockNumber(hash: any) {
        return (await this.client.rpc.chain.getHeader(hash)).number
    }

    /** Filter events for the event emitted by the transaction
     *
     * @param transactionType of the transaction
     * @param events array of events in that block
     * @returns event emitted by the transaction
     */
    getEvent(transactionType: SfxType, events: any[]) {
        const event = events.find((item) => {
            return item.event.method === EventMapper[transactionType]
        })

        if (event) {
            return event.event
        } else {
            //Todo: deal with this somehow
            console.log("Event not found")
        }
    }

    /** Fetch nonce of the signer
     *
     * @param api client
     * @param address of the signer
     * @returns nonce of the signer
     */
    async fetchNonce(api: ApiPromise, address: string): Promise<number> {
        return parseInt((await api.rpc.system.accountNextIndex(address)).toHuman())
    }
}

export{Estimator, CostEstimator, Estimate, InclusionData}
