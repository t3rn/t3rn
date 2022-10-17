import { EventEmitter } from "events"
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"
import { fetchNonce } from "../utils/"
import { SideEffect } from "../executionManager/sideEffect"
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

    }

    async bondInsuranceDeposits(sideEffects: SideEffect[]) {
        for(const sideEffect of sideEffects) {
            const nonce = await fetchNonce(this.api, this.signer.address)
            const xtxId = this.api.createType("Hash", sideEffect.xtxId);
            const id = this.api.createType("Hash", sideEffect.id);

            await this.api.tx.circuit.bondInsuranceDeposit(xtxId, id)
                .signAndSend(this.signer, {nonce}, async res => {
                    if(res.status.isFinalized) {
                        exportData([{xtxId, id}], `bond-insurance-${sideEffect.id.substring(2, 10)}.json`, "confirm")

                    } else {
                        // console.log(res.status.toHuman())
                        // add some error handling here
                    }
                })
        }
    }

    async confirmSideEffects(sideEffects: SideEffect[]) {
        for (const sideEffect of sideEffects) {
            const nonce = await fetchNonce(this.api, this.signer.address)

            const inclusionData = this.api.createType("InclusionData", sideEffect.inclusionData);
            const receivedAt = this.api.createType("BlockNumber", 0); // ToDo figure out what to do here
            const confirmedSideEffect = this.api.createType("ConfirmedSideEffect", {
                err: null,
                output: null,
                inclusion_data: inclusionData.toHex(),
                executioner: sideEffect.executor,
                receivedAt: receivedAt,
                cost: null,
            })

            const xtxId: any = this.api.createType("XtxId", sideEffect.xtxId);
            exportData([{xtxId, sideEffect: sideEffect.raw, confirmedSideEffect}], `confirm-transfer-${sideEffect.id.substring(2, 10)}.json`, "confirm")

            await new Promise((resolve, reject) => {
                this.api.tx.circuit
                    .confirmSideEffect(
                        xtxId.toHuman(),
                        sideEffect.raw.toHuman(),
                        { // for some reason im not able to pass confirmedSideEffect here
                            err: null,
                            output: null,
                            inclusionData: inclusionData.toHex(),
                            executioner: sideEffect.executor,
                            receivedAt: receivedAt,
                            cost: null,
                        },
                        null,
                        null
                    )
                    .signAndSend(this.signer, { nonce }, result => {
                        if (result.status.isFinalized) {
                            const success =
                                result.events[result.events.length - 1].event.method ===
                                "ExtrinsicSuccess"

                            if (success) resolve(undefined)
                        else
                            reject(
                              Error(`sfx confirmation failed for ${sideEffect.id}`)
                            )
                        }
                    })
              })
        }
    }
}

// in combination with transfer.ts
let indexes = [7, 8, 9, 10, 12, 13, 15, 16, 18, 21, 9999, 111111, 222222, 33333, 444444];
let counter = 0
export const exportData = (data: any, fileName: string, transactionType: string) => {
    let deepCopy;
    // since its pass-by-reference
    if(Array.isArray(data)) {
        deepCopy = [...data];
    } else {
        deepCopy = {...data};
    }
    let encoded = encodeExport(deepCopy, transactionType);
    fs.writeFile("exports/" + indexes[counter] + '-' + fileName, JSON.stringify(encoded, null, 4), (err) => {
        if(err) {
            console.log("Err", err);
        } else {
        }
    });

    counter += 1;
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