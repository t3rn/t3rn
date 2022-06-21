import { ApiPromise, WsProvider } from "@polkadot/api"
import { BlockHash, Header } from '@polkadot/types/interfaces'
import { createTestPairs } from "@polkadot/keyring/testingPairs"
import createDebug from "debug"
import types from "./types.json"
import { EventEmitter } from "events"
import { scaleEncodeHeader, scaleEncodeHash, scaleEncodeHeaderRange, computeHash } from "./encoder"

const keyring = createTestPairs({ type: "sr25519" })

export default class Relayer extends EventEmitter {
    static debug = createDebug("relayer")
    circuit: ApiPromise

    async setup(url: string) {
        this.circuit = await ApiPromise.create({
            provider: new WsProvider(url),
            types: types as any,
        })
        Relayer.debug("Relayer Setup complete")
    }

    async submitHeader(
        data: any
    ) {
        const encoded = await scaleEncodeHeader(data.header, this.circuit);
        const submitHeader = this.circuit.tx.bscfv.submitHeader(encoded);

        // as this is event-driven now, we dont need the promises anymore
        submitHeader.signAndSend(keyring.alice, async result => {
            // Issue #2 occures here
            if (result.isError) {
                // this doesn't work for all error in the circuit, but for some
                console.log("HeaderSubmitted failed")
            } else if (result.isInBlock) {
                this.emit("HeaderSubmitted", {
                    headerIndex: data.headerIndex
                })
            }
        })
    }

    async submitHeaderRange(
        range: any[],
        anchorHeader: any,
    ) {
        const encodedRange = await scaleEncodeHeaderRange(range, this.circuit)
        const anchorHash = await computeHash(anchorHeader, this.circuit);
        // console.log(encodedRange[0])
        console.log(anchorHash)
        const submitHeaderRange =
            this.circuit.tx.bscfv.submitHeaderRange(
                encodedRange,
                anchorHash
            )

        console.log("submitting header range")
        submitHeaderRange.signAndSend(
            keyring.alice,
           
            async result => {
                // Issue #2 occures here
                if (result.isError) {
                    // this doesn't work for all error in the circuit, but for some
                    Relayer.debug("Header Range submissission failed")
                } else if (result.status.isFinalized) {
                    this.emit("SubmittedHeaderRange", {
                        anchorHash
                    })
                }
            }
        )
    }
}
