import "@polkadot/api-augment"; // DO NOT REMOVE THIS LINE
import { ApiPromise, WsProvider } from "@polkadot/api";

import types from "./config/types.json";
import rpc from "./config/rpc.json";
import { Gateway, initGateways } from "./gateways";

import * as Types from "./types";

// @ts-ignore
import { T3rnTypesSideEffect } from "@polkadot/types/lookup";
import * as encodings from "./encodings";
import * as converters from "./converters";
import { Circuit } from "./circuit";

/**
 * The main class for the SDK
 */

export class Sdk {
  rpcUrl: string;
  client: ApiPromise;
  gateways: {
    [id: string]: Gateway;
  };
  circuit: Circuit;
  signer: any;

  /**
   * @param rpcUrl - The RPC URL of the node to connect to
   * @param circuitSigner - The signer to use for signing transactions
   */

  constructor(rpcUrl: string, circuitSigner: any) {
    this.rpcUrl = rpcUrl;
    this.signer = circuitSigner;
  }

  /**
   * Initializes ApiPromise instance and loads available gateways via XDNS
   * @returns ApiPromise instance
   */

  async init(): Promise<ApiPromise> {
    this.client = await ApiPromise.create({
      provider: new WsProvider(this.rpcUrl),
      types: types as any,
      rpc: rpc as any,
    });
    this.gateways = await initGateways(this.client);
    this.circuit = new Circuit(this.client, this.signer);

    return this.client;
  }
}

export { encodings, converters, Types };

// (async () => {
//
// 	const t3rn = new Sdk('ws://localhost:9944')
// 	await t3rn.init()
//
// 	const sfx: T3rnTypesSideEffect = t3rn.gateways.roco.createTransferSfx({
// 		from: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
// 		to: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
// 		value: t3rn.gateways.roco.floatToBn(0.0001),
// 		maxReward: "1000000000",
// 		insurance: "100000",
// 		nonce: 1
// 	});
//
// 	console.log(sfx)
//
// 	console.log(t3rn.client.createType("T3rnTypesSideEffect", sfx))
// })()
