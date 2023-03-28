import { createType } from "@t3rn/types"
import { EventEmitter } from "events"
import { SideEffect } from "../executionManager/sideEffect"
import createDebug from "debug"
import { BN } from "@polkadot/util"
import { Sdk, ApiPromise } from "@t3rn/sdk"
import { SubmittableExtrinsic } from "@polkadot/api/promise/types"
import { T3rnTypesSfxConfirmedSideEffect } from "@polkadot/types/lookup"
import fs from "fs"

/**
 * Class responsible for submitting any type of transaction to the circuit. All communication with the circuit is done through the circuit relayer.
 *
 * @group t3rn Circuit
 */
export class CircuitRelayer extends EventEmitter {
  static debug = createDebug("circuit-relayer")

  api: ApiPromise
  sdk: Sdk
  id: string
  rpc: string

  constructor(sdk: Sdk) {
    super()
    // @ts-ignore
    this.api = sdk.client
    this.sdk = sdk
  }

  /**
   * Builds and submits a sfxBid to the circuit
   *
   * @param sfxId The bid is for
   * @param amount The bidding amount, as integer in the reward asset
   */
  async bidSfx(sfxId: string, amount: BN): Promise<string> {
    const encodedSfxId = createType("Hash", sfxId)
    const encodedAmount = createType("u128", amount)
    const tx = this.api.tx.circuit.bidSfx(encodedSfxId as never, encodedAmount as never)
    return this.sdk.circuit.tx.signAndSendSafe(tx)
  }

  /**
   * Builds and submits a SFX confirmation tx to the circuit. These confirmations are submitted as TX batch
   *
   * @param sfxs Array of SideEffect objects that should be confirmed
   * @returns The block height of the included tx
   */
  async confirmSideEffects(sfxs: SideEffect[]): Promise<string> {
    const txs: SubmittableExtrinsic[] = sfxs.map((sfx) => this.createConfirmTx(sfx))
    if (txs.length > 1) {
      // only batch if more than one tx
      const batch = await this.sdk.circuit.tx.createBatch(txs)
      return this.sdk.circuit.tx.signAndSendSafe(batch)
    } else {
      return this.sdk.circuit.tx.signAndSendSafe(txs[0])
    }
  }

    /**
     * Builds the actual confirm tx for a given SideEffect
     *
     * @param sfx The SideEffect to confirm
     */
    createConfirmTx(sfx: SideEffect): SubmittableExtrinsic {
        let inclusionData

        if (sfx.target === "roco") {
            inclusionData = this.api.createType("RelaychainInclusionProof", {
                encoded_payload: sfx.inclusionProof.encoded_payload,
                payload_proof: sfx.inclusionProof.payload_proof,
                block_hash: sfx.inclusionProof.block_hash,
            })
        } else {
            inclusionData = this.api.createType("ParachainInclusionProof", {
                encoded_payload: sfx.inclusionProof.encoded_payload,
                payload_proof: sfx.inclusionProof.payload_proof,
                header_proof: sfx.inclusionProof.header_proof,
                relay_block_hash: sfx.inclusionProof.block_hash,
            })
        }

        const confirmedSideEffect: T3rnTypesSfxConfirmedSideEffect = createType("T3rnTypesSfxConfirmedSideEffect", {
            err: null,
            output: null,
            inclusionData: inclusionData.toHex(),
            executioner: sfx.executor,
            receivedAt: 0,
            cost: null,
        })

    return this.api.tx.circuit.confirmSideEffect(sfx.id, confirmedSideEffect.toJSON())
  }
}

// in combination with transfer.ts
const indexes = [7, 8, 9, 10, 12, 13, 15, 16, 18, 21, 9999, 111111, 222222, 33333, 444444]
let counter = 0
export const exportData = (data: any, fileName: string, transactionType: string) => {
  let deepCopy
  // since its pass-by-reference
  if (Array.isArray(data)) {
    deepCopy = [...data]
  } else {
    deepCopy = { ...data }
  }
  const encoded = encodeExport(deepCopy, transactionType)
  fs.writeFile("exports/" + indexes[counter] + "-" + fileName, JSON.stringify(encoded, null, 4), (err) => {
    if (err) {
      console.log("Err", err)
    } else {
    }
  })

  counter += 1
}

// encodes data for exporting. We export in encoded and human format.
// Encoded: We use for seeding protal rust tests
// Human: Debugging those tests and viewing data
export const encodeExport = (data: any, transactionType: string) => {
  if (Array.isArray(data)) {
    return data.map((entry) => iterateEncode(entry, transactionType))
  } else {
    return iterateEncode(data, transactionType)
  }
}

const iterateEncode = (data: any, transactionType: string) => {
  const keys = Object.keys(data)
  const result = {}
  if (keys.includes("initialU8aLength")) {
    // this is a polkadot/apiPromise object
    return {
      data: data.toHuman(),
      transaction_type: transactionType,
      encoded_data: data.toHex().substring(2),
    }
  } else {
    for (let i = 0; i < keys.length; i++) {
      result["encoded_" + toSnakeCase(keys[i])] = data[keys[i]].toHex().substring(2)
      result[toSnakeCase(keys[i])] = data[keys[i]].toHuman()
    }
    result["transaction_type"] = transactionType
    result["submission_height"] = 0 // we ignore it here for now
    return result
  }
}

const toSnakeCase = (str) =>
  str &&
  str
    .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
    .map((x) => x.toLowerCase())
    .join("_")
