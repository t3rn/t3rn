import { ApiPromise, WsProvider } from '@polkadot/api';
import { RpcPromiseResult } from '@polkadot/api/types';
// import {RpcPromiseResult} from "@polkadot/api/types";

export class SubstrateListener {

    rangeSize: number;
    wsEndpoint: string;
    headers: any[] = [];
    headerListener: any;
    anchorJustification: any;
    apiPromise: ApiPromise;


    constructor(wsEndpoint: string, rangeSize: number) {
        this.wsEndpoint = wsEndpoint;
        this.rangeSize = rangeSize;
    }

    async initListener() {
        this.apiPromise = await ApiPromise.create({ provider: new WsProvider(this.wsEndpoint)});

        this.headerListener = await this.apiPromise.rpc.chain.subscribeNewHeads(async (header) => {
            console.log(`Received Header: #${header.number}`);
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
            this.anchorJustification = justification;
            this.conclude()
            listener();
        })
    }

    async conclude() {
        this.headerListener() // terminate header listener
        console.log("Headers found:", this.headers.length);
        console.log(this.anchorJustification)
    }

}