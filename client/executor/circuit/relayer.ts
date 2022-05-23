import { EventEmitter } from "events"
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"
import { SideEffect, queryNonce } from "../utils"
import createDebug from "debug"

export default class CircuitRelayer extends EventEmitter {
  static debug = createDebug("circuit-relayer")

  api: ApiPromise
  id: string
  rpc: string
  signer: any

  async setup(rpc: string) {
    this.rpc = rpc
    this.api = await ApiPromise.create({
      provider: new WsProvider(rpc),
    })

    const keyring = new Keyring({ type: "sr25519" })

    this.signer =
      process.env.CIRCUIT_KEY === undefined
        ? keyring.addFromUri("//Alice")
        : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  }

  async bondInsuranceDeposits(sideEffects: SideEffect[]) {
    const calls = sideEffects
      // four args mean the call requires an insurance deposit
      .filter(sideEffect => sideEffect.object.encodedArgs.length === 4)
      .map(sideEffect =>
        this.api.tx.circuit.bondInsuranceDeposit(
          sideEffect.xtxId,
          sideEffect.getId()
        )
      )

    if (calls.length) {
      const nonce = await queryNonce(this.api, this.signer.address)
      CircuitRelayer.debug("bondInsuranceDeposits nonce", nonce)
      await this.api.tx.utility
        .batchAll(calls)
        .signAndSend(this.signer, { nonce })
    }
  }

  async confirmSideEffects(sideEffects: SideEffect[]) {
    let nonce = await queryNonce(this.api, this.signer.address)
    await Promise.all(
      sideEffects.map(async sideEffect =>
        this.confirmSideEffect(sideEffect, nonce++)
      )
    )
  }

  async confirmSideEffect(sideEffect: SideEffect, nonce: bigint) {
    CircuitRelayer.debug("confirmSideEffect nonce", nonce)
    await this.api.tx.circuit
      .confirmSideEffect(
        sideEffect.xtxId,
        sideEffect.object,
        sideEffect.confirmedSideEffect,
        sideEffect.inclusionProof.toJSON().proof,
        sideEffect.execBlockHeader.toJSON()
      )
      .signAndSend(this.signer, { nonce }, result => {
        if (result.status.isFinalized) {
          const success =
            result.events[result.events.length - 1].event.method ===
            "ExtrinsicSuccess"
          CircuitRelayer.debug(
            `SideEffect confirmed: ${success}, ${result.status.asFinalized}`
          )
          sideEffect.confirm(success, result.status.asFinalized)
          this.emit("SideEffectConfirmed", sideEffect.getId())
        }
      })
  }
}
