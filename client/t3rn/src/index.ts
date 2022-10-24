import { ApiPromise, WsProvider } from "@polkadot/api"

import types from './config/types.json';
import rpc from './config/rpc.json';
import {XDNS} from "./xdns";
import * as converter from "./converters";
import {Converter} from "./converters";

export class T3rn {

	rpcUrl: string;
	client: ApiPromise;
	xdns: XDNS;
	converter: Converter

	constructor(rpcUrl: string) {
		this.rpcUrl = rpcUrl;

	}

	// Initializes ApiPromise instance
	async init(): Promise<ApiPromise> {
		this.client = await ApiPromise.create({
			provider: new WsProvider(this.rpcUrl),
			types: types as any,
			rpc: rpc as any
		})

		this.xdns = new XDNS(this.client)
		await this.xdns.setup()

		this.converter = new Converter(this.xdns);

		return this.client
	}


}

const t3rn = new T3rn('ws://localhost:9944')

t3rn.init()