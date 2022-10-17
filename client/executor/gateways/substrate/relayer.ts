import {EventEmitter} from "events"
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api"
import {EventMapper, SideEffect, TransactionType} from "../../circuit/executions/sideEffect"
import {fetchNonce, getEventProofs} from "../../utils"
import createDebug from "debug"
import CostEstimator from "./costEstimator";
import {SubmittableExtrinsic} from "@polkadot/api/promise/types";

export default class SubstrateRelayer extends EventEmitter {
    static debug = createDebug("substrate-relayer")

    api: ApiPromise
    id: string
    rpc: string
    signer: any
    name: string
    nonce: number

    async setup(rpc: string, name: string, id: string) {
        this.rpc = rpc
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc),
        })

        const keyring = new Keyring({ type: "sr25519" })

        this.signer =
            process.env.SIGNER_ROCOCO === undefined
              ? keyring.addFromUri("//Executor//default")
              : keyring.addFromMnemonic(process.env.SIGNER_ROCOCO)

        let cost = new CostEstimator(id, this.api, this.signer)
        this.name = name
        this.nonce = await this.fetchNonce(this.api, this.signer.address)
    }

    // Builds tx object for the different side effects. This can be used for estimating fees or to submit tx
    buildTx(sideEffect: SideEffect): SubmittableExtrinsic {
        switch(sideEffect.action) {
            case TransactionType.Transfer: {
                const data = sideEffect.execute()
                return this.api.tx.balances
                    .transfer(data[0], data[1])
            }
        }
    }

    // Submit the sfx tx to the target
    async executeTx(sideEffect: SideEffect) {
        SubstrateRelayer.debug(`Executing sfx ${this.toHuman(sideEffect.id)} - ${sideEffect.target} with nonce: ${this.nonce} 🔮`)
        const tx: SubmittableExtrinsic = this.buildTx(sideEffect)
        tx.signAndSend(this.signer, {nonce: this.nonce}, async result => {
            if (result.status.isFinalized) {
                this.handleTx(sideEffect, result)
            }
        })
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

            const inclusionProof = await getEventProofs(this.api, blockHash)
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
        return (await this.api.rpc.chain.getHeader(hash)).number
    }

    getEvent(transactionType: TransactionType, events: any[]) {
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
