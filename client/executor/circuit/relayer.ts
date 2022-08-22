import { EventEmitter } from "events"
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"
import { SideEffect, fetchNonce } from "../utils"
import createDebug from "debug"
import types from "../types.json"
const fs = require("fs");

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
      types: types as any
    })

    const keyring = new Keyring({ type: "sr25519" })

    this.signer =
      process.env.CIRCUIT_KEY === undefined
        ? keyring.addFromUri("//Executor//default")
        : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)

      console.log(this.signer.address)
  }

  async bondInsuranceDeposits(sideEffects: SideEffect[]) {
    const calls = sideEffects
      // four args mean the call requires an insurance deposit
      .filter(sideEffect => sideEffect.object.encodedArgs.length === 4)
      .map(sideEffect => {
        const xtxId = this.api.createType("Hash", sideEffect.xtxId);
        const id = this.api.createType("Hash", sideEffect.getId());
        exportData([{xtxId, id}], "post-bond-roco.json", "bond");
        return this.api.tx.circuit.bondInsuranceDeposit(
          sideEffect.xtxId,
          sideEffect.getId()
        )
      }
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
      // @ts-ignore
      sideEffect.confirmedSideEffect.inclusion_data = this.api.createType("InclusionData", sideEffect.confirmedSideEffect.inclusion_data).toHex();
      // @ts-ignore
      sideEffect.confirmedSideEffect.received_at = sideEffect.confirmedSideEffect.receivedAt;
      // @ts-ignore
      const confirmed: any = this.api.createType("ConfirmedSideEffect", sideEffect.confirmedSideEffect)
      const sideEffectObj: any = this.api.createType("SideEffect", sideEffect.object);
      const xtxId: any = this.api.createType("XtxId", sideEffect.xtxId);
      console.log(sideEffect.confirmedSideEffect)
      exportData([{xtxId, sideEffect: sideEffectObj, confirmed}], "confirm-transfer-roco.json", "confirm")

      await new Promise((resolve, reject) => {
        this.api.tx.circuit
          .confirmSideEffect(
            sideEffect.xtxId,
            sideEffect.object,
            sideEffect.confirmedSideEffect,
            null,
            null
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
let counter = 1;
export const exportData = (data: any, fileName: string, transactionType: string) => {
    let deepCopy;
    // since its pass-by-reference
    if(Array.isArray(data)) {
        deepCopy = [...data];
    } else {
        deepCopy = {...data};
    }
    let encoded = encodeExport(deepCopy, transactionType);
    fs.writeFile("exports/" + counter + '-' + fileName, JSON.stringify(encoded, null, 4), (err) => {
        if(err) {
            console.log(err);
        } else {
            counter += 1;
            console.log("JSON saved to " + fileName);
        }
    });
}

// encodes data for exporting. We export in encoded and human format.
// Encoded: We use for seeding protal rust tests
// Human: Debugging those tests and viewing data
export const encodeExport = (data: any, transactionType: string) => {
    if(Array.isArray(data)) {
        return data.map(entry => iterateEncode(entry, transactionType))
    } else {
        return iterateEncode(data, transactionType)
    }
}

const iterateEncode = (data: any, transactionType: string) => {
    let keys = Object.keys(data);
    let result = {};
    if(keys.includes("initialU8aLength")) { // this is a polkadot/apiPromise object
        return {
            data: data.toHuman(),
            transaction_type: transactionType,
            encoded_data: data.toHex().substring(2)
        }
    } else {
        for(let i = 0; i < keys.length; i++) {
            result['encoded_' + toSnakeCase(keys[i])] = data[keys[i]].toHex().substring(2)
            result[toSnakeCase(keys[i])] = data[keys[i]].toHuman()
        }
        result['transaction_type'] = transactionType;
        result['submission_height'] = 0; // we ignore it here for now
        return result;
    }
}

const toSnakeCase = str =>
  str &&
  str
    .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
    .map(x => x.toLowerCase())
    .join('_');