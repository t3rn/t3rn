import { ApiPromise, WsProvider } from "@polkadot/api"
import { BlockHash, Header } from '@polkadot/types/interfaces'
import { createTestPairs } from "@polkadot/keyring/testingPairs"
import createDebug from "debug"
import types from "./types.json"
import { EventEmitter } from "events"

const keyring = createTestPairs({ type: "sr25519" })

export default class Relayer extends EventEmitter {
  static debug = createDebug("relayer")
  api: ApiPromise

  async setup(url: string) {
    this.api = await ApiPromise.create({
      provider: new WsProvider(url),
      types: types as any,
    })
    Relayer.debug("Relayer Setup complete")
  }

  submitFinalityProof(
    gatewayId: string,
    justification: any,
    anchorHeader: Header,
    anchorIndex: number
  ) {
    const submitFinalityProof =
      this.api.tx.multiFinalityVerifierDefault.submitFinalityProof(
        anchorHeader,
        justification,
        gatewayId
      )

    // as this is event-driven now, we dont need the promises anymore
    submitFinalityProof.signAndSend(keyring.alice, async result => {
      // Issue #2 occures here
      if (result.isError) {
        // this doesn't work for all error in the circuit, but for some
        Relayer.debug("FinalityProofSubmitted failed")
      } else if (result.isInBlock) {
        this.emit("FinalityProofSubmitted", {
          gatewayId,
          anchorHash: anchorHeader.hash,
          anchorIndex,
        })
      }
    })
  }

  // here we pass the anchorHash, instead of anchorIndex. We can use the hash to find the index of the anchor header for the parachain.
  async submitParachainHeader(
    gatewayId: string,
    blockHash: BlockHash,
    proof: any,
    anchorNumber: number
  ) {
    Relayer.debug("Submitting Parachain Header")
    let submitParachainHeader =
      this.api.tx.circuitPortal.submitParachainHeader(
        blockHash,
        gatewayId,
        proof
      )

    // in substrate the nonce updates only once a transaction is confirmed. With this we can get the current nonce, inclusing transactions in mempool
    const nextNonce = await this.api.rpc.system.accountNextIndex(
      keyring.alice.address
    )
    submitParachainHeader.signAndSend(
      keyring.alice,
      {
        nonce: nextNonce,
      },
      result => {
        // Issue #2 occures here
        if (result.isError) {
          // this doesn't work for all error in the circuit, but for some
          Relayer.debug("ParachainHeaderSubmitted failed")
        } else if (result.isInBlock) {
          this.emit("ParachainHeaderSubmitted", {
            gatewayId,
            anchorNumber,
            anchorHash: blockHash.toJSON(),
          })
        }
      }
    )
  }

  async submitHeaderRange(
    gatewayId: string,
    range: Header[],
    anchorHeader: Header,
    anchorIndex: number
  ) {
    const submitHeaderRange =
      this.api.tx.multiFinalityVerifierDefault.submitHeaderRange(
        gatewayId,
        range,
        anchorHeader.hash
      )

    // potentially more the one tx per block, so we dont rely on default nonce
    const nextNonce = await this.api.rpc.system.accountNextIndex(
      keyring.alice.address
    )
    submitHeaderRange.signAndSend(
      keyring.alice,
      {
        nonce: nextNonce,
      },
      result => {
        // Issue #2 occures here
        if (result.isError) {
          // this doesn't work for all error in the circuit, but for some
          Relayer.debug("Header Range submissission failed")
        } else if (result.status.isFinalized) {
          this.emit("SubmittedHeaderRange", {
            gatewayId,
            anchorIndex,
          })
        }
      }
    )
  }
}
