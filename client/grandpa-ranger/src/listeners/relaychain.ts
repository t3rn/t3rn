import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { HeaderExtended } from '@polkadot/api-derive/types'
import { grandpaDecode, decodeHeaderNumber } from './../util'
import { u8aConcat, u8aToU8a, u8aToHex } from '@polkadot/util';
import { xxhashAsU8a } from '@polkadot/util-crypto';
import HeaderListener from "./headers"
import createDebug from 'debug'
const BN = require("bn.js");
import 'dotenv/config'

export default class Relaychain extends EventEmitter {
    static debug = createDebug('listener')

    api: ApiPromise;
    headers: HeaderListener;
    gatewayId: string;
    //mapping for justifications, might be useful to store when dealing with authSetChange
    justifications: {[block: number]: any} = {};
    // latest finalized justification available
    latestJustification: number = 0
    // tracks the tip of a range to time the execution.
    rangeReadyAt: number = 0;
    unsubJustifications: () => void

    
    async setup(url: string, gatewayId: string) {
        this.gatewayId = gatewayId;
        this.api = await ApiPromise.create({
            provider: new WsProvider(url),
        })

        this.headers = await new HeaderListener();
        await this.headers.setup(url, true);

        this.headers.on("RangeComplete", (block: number) => {
            console.log("Received RangeComplete:", block)
            this.rangeReadyAt = block;
        })

        this.headers.start()

    }

    async start() {
        console.log("starting Relaychain-ranger")

        this.unsubJustifications = await this.api.rpc.grandpa.subscribeJustifications(
            async justification => {

                // TODO: Issue #3 Detect AuthoritySetChanges
                // the justification should contain the authoritySetId, so we need to update the decoder and detect the changes here. 
                // If an update is detected we either need to submit this, or the previous justification. (in the plance with no internet :((  )
                // We can figure the block out here, and then query the justification we need
                const { blockNumber } = await grandpaDecode(justification)
                this.justifications[blockNumber] = justification;
                this.latestJustification = blockNumber;

                console.log("Caught Justification:", blockNumber);

                if(this.latestJustification > 0 && this.latestJustification <= this.rangeReadyAt && blockNumber >= this.rangeReadyAt) {
                    console.log("Found Justification for Anchor!!")
                    const anchorIndex = this.headers.getHeaderIndex(blockNumber)
                    const anchorHeader = this.headers.headers[anchorIndex];
                    this.emit("SubmitFinalityProof", {
                        gatewayId: this.gatewayId,
                        justification,
                        anchorHeader,
                        anchorIndex
                    })
                }
            }
        )
    }

    async submitHeaderRange(anchorIndex: number) {
        let range = this.headers.headers
            .slice(0, anchorIndex + 1)
            .reverse();
        const anchorHeader = range.shift();

        // we need to pass the anchorIndex around, so we can delete these header if everthing was successful
        this.emit("SubmitHeaderRange", {
            gatewayId: this.gatewayId,
            range,
            anchorHeader,
            anchorIndex
        })
    }

    async finalize(anchorIndex: number) {
        this.headers.finalize(anchorIndex);
    }

    // This needs refactoring, as Paras and Heads are constants
    async getStorageProof(blockHash: any, parachainId: number) {
        const key = this.generateArgumentKey('Paras', 'Heads', parachainId);
        const proof = await this.api.rpc.state.getReadProof([key], blockHash);
        const encodedHeader: any = await this.api.rpc.state.getStorage(key, blockHash);
        const headerNumber = decodeHeaderNumber(encodedHeader.toJSON()) // this is the parachain header we verify. We later use it to generate to correct range.
        return [proof, headerNumber]
    }

    encodeParachainId(id: number) {
        // this is the correct storageKey parameter encoding for u32
        return "0x" + new BN(id).toBuffer("le", 4).toString("hex")
    }

    generateArgumentKey(module: string, variableName: string, parachainid: number) {
        // lets prepare the storage key for system events.
        let module_hash = xxhashAsU8a(module, 128);
        let storage_value_hash = xxhashAsU8a(variableName, 128);

        let encodedParachainId = this.encodeParachainId(parachainid)
        let argumenteKey = u8aConcat(xxhashAsU8a(encodedParachainId, 64), u8aToU8a(encodedParachainId))

        // Special syntax to concatenate Uint8Array
        let final_key = new Uint8Array([...module_hash, ...storage_value_hash, ...argumenteKey]);

        return u8aToHex(final_key);
    }
}
