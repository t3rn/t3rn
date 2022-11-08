import {EventEmitter} from "events"
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api"
import {EventMapper, SideEffect} from "../../executionManager/sideEffect"
import {fetchNonce, getEventProofs} from "../../utils"
import createDebug from "debug"
import Estimator from "./estimator";
import {SubmittableExtrinsic} from "@polkadot/api/promise/types";
import {SfxType} from "@t3rn/sdk/dist/src/side-effects/types";

export default class SubstrateRelayer extends EventEmitter {
    static debug = createDebug("substrate-relayer")

    client: ApiPromise
    signer: any
    nonce: number

    async setup(rpc: string, signer: string | undefined) {
        this.client = await ApiPromise.create({
			provider: new WsProvider(rpc),
		})
        const keyring = new Keyring({ type: "sr25519" })

        this.signer =
            signer === undefined
              ? keyring.addFromUri("//Executor//default")
              : keyring.addFromMnemonic(signer)

        this.nonce = await this.fetchNonce(this.client, this.signer.address)
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
    async executeTx(sideEffect: SideEffect) {
        SubstrateRelayer.debug(`Executing sfx ${this.toHuman(sideEffect.id)} - ${sideEffect.target} with nonce: ${this.nonce} 🔮`)
        const tx: SubmittableExtrinsic = this.buildTx(sideEffect)
        // tx.signAndSend(this.signer, {nonce: this.nonce}, async result => {
        //     if (result.status.isFinalized) {
        //         this.handleTx(sideEffect, result)
        //     }
        // })
        this.nonce += 1; // we optimistically increment the nonce. If a transaction fails, this will mess things up. The alternative is to do it sequentially, which is very slow.
    }

    // if sfx execution successful, generate inclusion proof and notify of successful execution
    async handleTx(sideEffect: SideEffect, result) {
        if (result.status.isFinalized) {
            const blockHash = result.status.asFinalized
            const blockNumber = await this.getBlockNumber(blockHash)
            const event = this.getEvent(sideEffect.action, result.events)

            // should always be last event
            const success =
              result.events[result.events.length - 1].event.method ===
              "ExtrinsicSuccess"

            const inclusionProof = await getEventProofs(this.client, blockHash)
            const inclusionData = {
                encoded_payload: event.toHex(),
                proof: {
                    trieNodes: inclusionProof.toJSON().proof
                },
                block_hash: blockHash
            }

            sideEffect.executedOnTarget(
                inclusionData,
                this.signer.addressRaw,
                blockNumber
            )

            this.emit("SideEffectExecuted", sideEffect.id)
        }
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
