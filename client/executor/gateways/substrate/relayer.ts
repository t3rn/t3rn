import { EventEmitter } from "events"
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"
import { SideEffect, TransactionType, EventMapper } from "../../utils/sideEffect"
import { getEventProofs, fetchNonce } from "../../utils"
import createDebug from "debug"

export default class SubstrateRelayer extends EventEmitter {
    static debug = createDebug("substrate-relayer")

    api: ApiPromise
    id: string
    rpc: string
    signer: any
    name: string

    async setup(rpc: string, name: string) {
        this.rpc = rpc
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc),
        })

        const keyring = new Keyring({ type: "sr25519" })

        this.signer =
            process.env.SIGNER_ROCOCO === undefined
              ? keyring.addFromUri("//Executor//default")
              : keyring.addFromMnemonic(process.env.SIGNER_ROCOCO)

          this.name = name
    }

    async executeTx(sideEffect: SideEffect) {
        const nonce = await fetchNonce(this.api, this.signer.address)
        SubstrateRelayer.debug("executeTx nonce", nonce.toString())

        switch (sideEffect.action) {
            case TransactionType.Transfer: {
                const data = sideEffect.execute()
                await this.api.tx.balances
                    .transfer(data[0], data[1])
                    .signAndSend(this.signer, { nonce }, async result => {
                        if (result.status.isFinalized) {
                            this.handleTx(sideEffect, result)
                        }
                    })
                break
            }
            case TransactionType.Swap:
            default:
              SubstrateRelayer.debug(`invalid tx type: ${sideEffect.action}`)
        }
    }

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

            sideEffect.executionConfirmed(
                inclusionData,
                this.signer.addressRaw,
                blockNumber
            )

            SubstrateRelayer.debug(`SideEffect Executed: ${success}, ${blockHash}`)
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
}
