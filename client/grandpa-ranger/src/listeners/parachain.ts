import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api'
import HeaderListener from "./headers"
import createDebug from 'debug'
import 'dotenv/config'

export default class Parachain extends EventEmitter {
    static debug = createDebug('listener')

    api: ApiPromise;
    headers: HeaderListener;
    gatewayId: string;
    parachainId: number;
    argumentKey: string;
   
    async setup(url: string, gatewayId: string, parachainId: number) {
        this.gatewayId = gatewayId;
        this.api = await ApiPromise.create({
            provider: new WsProvider(url),
        })
        
        this.headers = await new HeaderListener();
        await this.headers.setup(url, false);

        this.parachainId = parachainId;

        this.headers.start()

    }

    async finalize(anchorIndex: number) {
        this.headers.finalize(anchorIndex);
    }

    async submitHeaderRange(anchorHash: string) {
        console.log("anchorHash:", anchorHash)
        const anchorIndex = await this.findAnchorIndex(anchorHash)
        console.log("AnchorIndex:", anchorIndex);
        let range = this.headers.headers
            .slice(0, anchorIndex + 1)
            .reverse();

        console.log(range)
        const anchorHeader = range.shift()
        console.log("Parachain Anchor:", anchorHeader)

        // we need to pass the anchorIndex, so we can delete these header if everthing was successful
        this.emit("SubmitHeaderRange", {
            gatewayId: this.gatewayId,
            range,
            anchorHeader,
            anchorIndex
        })
    }

    async findAnchorIndex(anchorHash: string) {
        return this.headers.headers.findIndex(
            h => h.hash.toHuman() === anchorHash
        )
    }
}
