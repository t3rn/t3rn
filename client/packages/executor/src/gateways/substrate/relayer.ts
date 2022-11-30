import {EventEmitter} from "events"
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api"
import {EventMapper, SideEffect} from "../../executionManager/sideEffect"
import {getEventProofs} from "../../utils"
import createDebug from "debug"
import {SubmittableExtrinsic} from "@polkadot/api/promise/types";
import {SfxType} from "@t3rn/sdk/dist/src/side-effects/types";
import {RelayerEventData, RelayerEvents} from "../types";

export default class SubstrateRelayer extends EventEmitter {
    static debug = createDebug("substrate-relayer")

    client: ApiPromise
    signer: any
    nonce: number
    name: string

    async setup(rpc: string, signer: string | undefined, name: string) {
        this.client = await ApiPromise.create({
			provider: new WsProvider(rpc),
		})
        const keyring = new Keyring({ type: "sr25519" })

        this.signer =
            signer === undefined
              ? keyring.addFromUri("//Executor//default")
              : keyring.addFromMnemonic(signer)

        this.nonce = await this.fetchNonce(this.client, this.signer.address)
        this.name = name
    }

    // Builds tx object for the different side effects. This can be used for estimating fees or to submit tx
    // @ts-ignore
    buildTx(sideEffect: SideEffect): SubmittableExtrinsic {
        switch(sideEffect.action) {
            case SfxType.Transfer: {
                const data = sideEffect.execute()
                return this.client.tx.balances
                    .transfer(data[0], data[1])
            }
        }
    }

    // Submit the sfx tx to the target
    async executeTx(sfx: SideEffect) {
        SubstrateRelayer.debug(`Executing sfx ${this.toHuman(sfx.id)} - ${sfx.target} with nonce: ${this.nonce} 🔮`)
        const tx: SubmittableExtrinsic = this.buildTx(sfx)
        let nonce = this.nonce
        this.nonce += 1; // we optimistically increment the nonce before we go async. If the tx fails, we will decrement it which might be a bad idea
        return new Promise<void>((resolve, reject) =>
			tx.signAndSend(this.signer, { nonce }, async ({ dispatchError, status, events }) => {
				if (dispatchError?.isModule) {
					let err = this.client.registry.findMetaError(dispatchError.asModule)
                    this.nonce -= 1;
                    this.emit(
                        "Event",
                        <RelayerEventData>{
                            type: RelayerEvents.SfxExecutionError,
                            data: `${err.section}::${err.name}: ${err.docs.join(" ")}`,
                            sfxId: sfx.id
                        }
                    )
					reject(Error(`${err.section}::${err.name}: ${err.docs.join(" ")}`))
				} else if (dispatchError) {
                    this.nonce -= 1;
                    this.emit(
                        "Event",
                        <RelayerEventData>{
                            type: RelayerEvents.SfxExecutionError,
                            data: dispatchError.toString(),
                            sfxId: sfx.id
                        }
                    )
					reject(Error(dispatchError.toString()))
				} else if (status.isFinalized) {
                    const blockNumber = await this.generateInclusionProof(sfx, status.asFinalized, events)
                    this.emit(
                        "Event",
                        <RelayerEventData>{
                            type: RelayerEvents.SfxExecutedOnTarget,
                            sfxId: sfx.id,
                            target: this.name,
                            data: "",
                            blockNumber
                        }
                    )
                    resolve()
                }
			})
		)
    }

    // if sfx execution successful, generate inclusion proof and notify of successful execution
    async generateInclusionProof(sfx: SideEffect, blockHash: any, events: any[]): Promise<number> {
            const blockNumber = await this.getBlockNumber(blockHash)
            const event = this.getEvent(sfx.action, events)

            // should always be last event
            const success =
              events[events.length - 1].event.method ===
              "ExtrinsicSuccess"

            const inclusionProof = await getEventProofs(this.client, blockHash)
            const inclusionData = {
                encoded_payload: event.toHex(),
                proof: {
                    trieNodes: inclusionProof.toJSON().proof
                },
                block_hash: blockHash
            }

            sfx.executedOnTarget(
                inclusionData,
                this.signer.addressRaw,
                blockNumber
            )

            return blockNumber.toNumber()
    }

    async getBlockNumber(hash: any) {
        return (await this.client.rpc.chain.getHeader(hash)).number
    }

    getEvent(transactionType: SfxType, events: any[]) {
        const event = events.find(item => {
            return item.event.method === EventMapper[transactionType]
        })

        if (event) return event.event
        SubstrateRelayer.debug("cannot find transaction's event")
    }

    async fetchNonce(
        api: ApiPromise,
        address: string
    ): Promise<number> {
        return parseInt((await api.rpc.system.accountNextIndex(address)).toHuman())
    }

    private toHuman(id: string) {
        return id.substring(0, 8)
    }
}