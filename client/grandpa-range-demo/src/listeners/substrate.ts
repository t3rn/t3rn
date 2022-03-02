import { ApiPromise, WsProvider } from '@polkadot/api';
import { JustificationNotification } from '@polkadot/types/interfaces/';
import { Header } from "@polkadot/api/node_modules/@polkadot/types/interfaces/runtime/types"
require('dotenv').config();

export class SubstrateListener{
    wsProvider: WsProvider
    rangeSize: number;
    gatewayId: number;
    headers: Header[] = [];
    headerListener: any;
    anchorJustification: JustificationNotification;
    apiPromise: ApiPromise;

    constructor() {
        this.wsProvider = new WsProvider(process.env.TARGET_RPC);
        this.rangeSize = Number(process.env.RANGE_SIZE);
        this.gatewayId = Number(process.env.GATEWAY_ID) 
    }

    async initListener() {
        this.apiPromise = await ApiPromise.create({ provider: this.wsProvider});
        this.headerListener = await this.apiPromise.rpc.chain.subscribeNewHeads(async (header) => {
            this.headers.push(header)
            
            if (this.headers.length === this.rangeSize) {
                console.log("range size reached! continuing listen until matchig justification is found")
                this.fetchIncomingGrandpaJustification();
            }
        });
    }

    async fetchIncomingGrandpaJustification() {
        console.log("Started Grandpa Justification Listener...")
        let listener = await this.apiPromise.rpc.grandpa.subscribeJustifications((justification) => {
            console.log("Caught Justification!")
            console.log(justification)
            // this.anchorJustification = justification;

            this.conclude()
            listener();
        })
    }

    async conclude() {
        this.headerListener() // terminate header listener
        console.log("Headers found:", this.headers.length);

        // TS might be undefined error workarounf
        (<any>process).send({instruction: "results", gatewayId: this.gatewayId, anchor: this.anchorJustification, headers: this.headers})
    }

}

let instance = new SubstrateListener();

process.on("message", (msg:string) => {
    if(msg === "init") {
        instance.initListener()
    }
})

