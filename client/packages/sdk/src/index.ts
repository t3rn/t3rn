import { ApiPromise, WsProvider } from "@polkadot/api"

import types from './config/types.json';
import rpc from './config/rpc.json';
import {Gateway, initGateways} from "./gateways";
// @ts-ignore
import {T3rnTypesSideEffect} from "@polkadot/types/lookup";

export class Sdk {

	rpcUrl: string;
	client: ApiPromise;
	gateways: {
		[id: string]: Gateway
	}

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

		this.gateways = await initGateways(this.client)
		return this.client
	}
}


(async () => {

	const t3rn = new Sdk('ws://localhost:9944')
	await t3rn.init()

	const sfx: T3rnTypesSideEffect = t3rn.gateways.roco.createTransferSfx({
		from: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
		to: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		value: t3rn.gateways.roco.floatToBn(0.0001),
		maxReward: "1000000000",
		insurance: "100000",
		nonce: 1
	});

	console.log(sfx)

	console.log(t3rn.client.createType("T3rnTypesSideEffect", sfx))
})()
