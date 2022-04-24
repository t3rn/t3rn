import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { HeaderExtended } from '@polkadot/api-derive/types'
import { grandpaDecode, decodeHeader } from './../util'
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { xxhashAsU8a } from '@polkadot/util-crypto';
import HeaderListener from "./headers"
import createDebug from 'debug'
const BN = require("bn.js");
import 'dotenv/config'

// StorageKey for Paras_Heads
const STORAGE_KEY = "cd710b30bd2eab0352ddcc26417aa1941b3c252fcb29d88eff4f3de5de4476c3";

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

                // TODO: Detect AuthoritySetChanges
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

    async getStorageProof(blockHash: any, parachainId: number) {
        const key = this.generateArgumentKey(parachainId);
        const proof = await this.api.rpc.state.getReadProof([key], blockHash);
        const encodedHeader = await this.api.rpc.state.getStorage(key, blockHash).toString();
        const headerHash = decodeHeader(encodedHeader)
        return [proof, headerHash]
    }

    generateArgumentKey(parachainId: number) {
        const encodedParachainId = "0x" + new BN(parachainId).toBuffer("le", 4).toString("hex");
        return STORAGE_KEY + u8aConcat(xxhashAsU8a(encodedParachainId, 64), u8aToU8a(encodedParachainId))
    }

    async finalize(anchorIndex: number) {
        this.headers.finalize(anchorIndex);
    }
}
