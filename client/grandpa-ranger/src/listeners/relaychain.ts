import { EventEmitter } from "events"
import { ApiPromise, WsProvider } from "@polkadot/api"
import {
decodeJustificationTarget,
containsAuthoritySetUpdate,
decodeHeaderNumber,
fetchMissingHeaders,
} from "./../util"
import { u8aConcat, u8aToU8a, u8aToHex } from "@polkadot/util"
import { xxhashAsU8a } from "@polkadot/util-crypto"
import HeaderListener from "./headers"
import createDebug from "debug"
import BN from "bn.js"
import { Header } from "@polkadot/types/interfaces"

export default class RelaychainListener extends EventEmitter {
static debug = createDebug("relaychain-listener")

api: ApiPromise
headerListener: HeaderListener
gatewayId: string
// mapping for justifications, might be useful to store when dealing
// with authSetChange
justifications: { [block: number]: any } = {}
// latest finalized justification available
latestJustification: number = 0
// tracks the tip of a range to time the execution.
rangeReadyAt: number = 0
unsubJustifications: () => void

async setup(url: string, gatewayId: string) {
    this.gatewayId = gatewayId
    this.api = await ApiPromise.create({
        provider: new WsProvider(url),
    })

    this.headerListener = await new HeaderListener()
    await this.headerListener.setup(url, true)

    this.headerListener.on("RangeComplete", (block: number) => {
        RelaychainListener.debug("Received RangeComplete:", block)
        this.rangeReadyAt = block
    })

    await this.headerListener.start()
}

async start() {
    RelaychainListener.debug("starting Relaychain-ranger")

    this.unsubJustifications =
        await this.api.rpc.grandpa.subscribeJustifications(
            async justification => {
                const blockNumber = await decodeJustificationTarget(justification)

                // early exit if already known to avoid nonce issues when submitting finality proof
                if (blockNumber === this.latestJustification) {
                    RelaychainListener.debug(
                        "ignoring known justification",
                        blockNumber
                    )
                    return
                }

                this.justifications[blockNumber] = justification
                this.latestJustification = blockNumber

                RelaychainListener.debug("Caught Justification:", blockNumber)
                if (
                    this.latestJustification > 0 &&
                    await containsAuthoritySetUpdate(this.api, blockNumber)
                ) {
                    RelaychainListener.debug("Authority Set Detected! Submitting Finality Proof")
                    this.triggerSubmitFinalityProof(blockNumber, justification);

                } else if (

                    this.latestJustification > 0 &&
                    this.latestJustification <= this.rangeReadyAt &&
                    blockNumber >= this.rangeReadyAt
                ) {
                    RelaychainListener.debug("Found Justification for Anchor!!")
                    this.triggerSubmitFinalityProof(blockNumber, justification)
                }
            }
        )
}

async triggerSubmitFinalityProof(blockNumber: number, justification: any) {
    this.headerListener.headers = await fetchMissingHeaders(
        this.api,
        this.headerListener.headers,
        blockNumber
    )

    const anchorIndex = this.headerListener.getHeaderIndex(blockNumber)
    if (anchorIndex !== -1) {
        const anchorHeader = this.headerListener.headers[anchorIndex]
        this.emit("SubmitFinalityProof", {
            gatewayId: this.gatewayId,
            justification,
            anchorHeader,
            anchorIndex,
        })
    } else {
        RelaychainListener.debug(
            `cannot find ${this.gatewayId} anchor for ${blockNumber} in stored headers`
        )
    }
}

submitHeaderRangeParams(anchorIndex: number): {
    gatewayId: string
    range: Header[]
    anchorHeader: Header
    anchorIndex: number
} {
    let range = this.headerListener.headers.slice(0, anchorIndex + 1).reverse()
    const anchorHeader = range.shift()

    return {
        gatewayId: this.gatewayId,
        range,
        anchorHeader,
        anchorIndex,
    }
}

async finalize(anchorNumber: number) {
    this.headerListener.finalize(anchorNumber)
}

// This needs refactoring, as Paras and Heads are constants
async getStorageProof(blockHash: any, parachainId: number) {
    const key = this.generateArgumentKey("Paras", "Heads", parachainId)
    const proof = await this.api.rpc.state.getReadProof([key], blockHash)
    const encodedHeader: any = await this.api.rpc.state.getStorage(
        key,
        blockHash
    )
    // this is the parachain header we verify. We later use it to generate to correct range.
    const headerNumber = decodeHeaderNumber(encodedHeader.toJSON())
    return [proof, headerNumber]
}

encodeParachainId(id: number) {
    // this is the correct storageKey parameter encoding for u32
    return "0x" + new BN(id).toBuffer("le", 4).toString("hex")
}

generateArgumentKey(
    module: string,
    variableName: string,
    parachainid: number
) {
    // lets prepare the storage key for system events.
    let module_hash = xxhashAsU8a(module, 128)
    let storage_value_hash = xxhashAsU8a(variableName, 128)

    let encodedParachainId = this.encodeParachainId(parachainid)
    let argumenteKey = u8aConcat(
        xxhashAsU8a(encodedParachainId, 64),
        u8aToU8a(encodedParachainId)
    )

    // Special syntax to concatenate Uint8Array
    let final_key = new Uint8Array([
        ...module_hash,
        ...storage_value_hash,
        ...argumenteKey,
    ])

    return u8aToHex(final_key)
}
}
