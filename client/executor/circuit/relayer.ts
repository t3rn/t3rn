import { EventEmitter } from "events"
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"
import { SideEffect, fetchNonce } from "../utils"
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
      const nonce = await fetchNonce(this.api, this.signer.address)
      CircuitRelayer.debug("bondInsuranceDeposits nonce", nonce.toString())
      await this.api.tx.utility
        .batchAll(calls)
        .signAndSend(this.signer, { nonce })
    }
  }

  async confirmSideEffects(sideEffects: SideEffect[]) {
    // confirmations must be submitted sequentially
    for (const sideEffect of sideEffects) {
      const nonce = await fetchNonce(this.api, this.signer.address)

      await new Promise((resolve, reject) => {
        this.api.tx.circuit
          .confirmSideEffect(
            sideEffect.xtxId,
            sideEffect.object,
            sideEffect.confirmedSideEffect,
            sideEffect.inclusionProof.toJSON().proof,
            sideEffect.execBlockHeader.toJSON()
          )
          .signAndSend(this.signer, { nonce }, result => {
            if (result.status.isFinalized) {
              CircuitRelayer.debug(
                "### confirmSideEffects result",
                JSON.stringify(result, null, 2)
              )

              const success =
                result.events[result.events.length - 1].event.method ===
                "ExtrinsicSuccess"

              sideEffects.forEach(sideEffect => {
                sideEffect.confirm(success, result.status.asFinalized)
                this.emit("SideEffectConfirmed", sideEffect.getId())
              })

              CircuitRelayer.debug(
                `sfx confirmed: ${success}, ${result.status.asFinalized}`
              )

              if (success) resolve(undefined)
              else
                reject(
                  Error(`sfx confirmation failed for ${sideEffect.getId()}`)
                )
            }
          })
      })
    }
  }
}
