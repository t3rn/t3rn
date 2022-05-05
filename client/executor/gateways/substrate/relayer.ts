import { EventEmitter } from "events"
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"
import { SideEffect, TransactionType, EventMapper } from "../../utils/types"
import { getEventProofs, queryNonce } from "../../utils"
import createDebug from "debug"

export default class SubstrateRelayer extends EventEmitter {
  static debug = createDebug("substrate-relayer")

  api: ApiPromise
  id: string
  rpc: string
  signer: any
  name: string
  nonce: bigint

  async setup(rpc: string, name: string) {
    this.rpc = rpc
    this.api = await ApiPromise.create({
      provider: new WsProvider(rpc),
    })

    const keyring = new Keyring({ type: "sr25519" })

    this.signer =
      process.env.SIGNER_KEY === undefined
        ? keyring.addFromUri("//Alice")
        : keyring.addFromMnemonic(process.env.SIGNER_KEY)

    this.name = name

    this.nonce = await queryNonce(this.api, this.signer.address)
  }

  async executeTx(sideEffect: SideEffect) {
    const nonce = this.nonce++
    SubstrateRelayer.debug("executeTx nonce", nonce)

    switch (sideEffect.transactionType) {
      case TransactionType.Transfer: {
        const data = sideEffect.getTransactionArguments()
        await this.api.tx.balances
          .transfer(data[0], data[1])
          .signAndSend(this.signer, { nonce }, async result => {
            if (result.status.isFinalized) {
              this.handleTx(sideEffect, result)
            }
          })
      }
      case TransactionType.Swap:
      default:
        SubstrateRelayer.debug(`invalid tx type: ${sideEffect.transactionType}`)
    }
  }

  async handleTx(sideEffect: SideEffect, result) {
    if (result.status.isFinalized) {
      const blockHeader = result.status.asFinalized
      const blockNumber = await this.getBlockNumber(blockHeader)
      const event = this.getEvent(sideEffect.transactionType, result.events)

      // should always be last event
      const success =
        result.events[result.events.length - 1].event.method ===
        "ExtrinsicSuccess"
      const inclusionProof = await getEventProofs(this.api, blockHeader)

      sideEffect.execute(
        event,
        blockNumber,
        this.signer.address,
        inclusionProof,
        blockHeader,
        success
      )

      SubstrateRelayer.debug(`SideEffect Executed: ${success}, ${blockHeader}`)

      this.emit("SideEffectExecuted", sideEffect.getId())
    }
  }

  async getBlockNumber(hash: any) {
    return (await this.api.rpc.chain.getHeader(hash)).number.toNumber()
  }

  getEvent(transactionType: TransactionType, events: any[]) {
    const event = events.find(item => {
      return item.event.method === EventMapper[transactionType]
    })

    if (event) return event.event.toHex()

    SubstrateRelayer.debug("cannot find transaction's event")
  }
}
