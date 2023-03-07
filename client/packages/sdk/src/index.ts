import "@t3rn/types"; // DO NOT REMOVE THIS LINE
import { ApiPromise, WsProvider } from "@polkadot/api";
import { cryptoWaitReady } from '@polkadot/util-crypto';

import types from "./config/types.json";
import rpc from "./config/rpc.json";
import { Gateway, initGateways } from "./gateways";

import * as Types from "./types";
import * as Encodings from "./encodings";
import * as Converters from "./converters";
import { Circuit, Tx } from "./circuit";

/**
 * The main class for the SDK
 */

export class Sdk {
  /*RPC url of the circuit */
  rpcUrl: string;
  /* ApiPromise instance of the circuit */
  client: ApiPromise;
  /* Mapping for looking up Gateway instances via ID */
  gateways: {
    [id: string]: Gateway;
  };
  /*Circuit Instance */
  circuit: Circuit;
  /* Circuit signer */
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
    await cryptoWaitReady()
    this.client = await ApiPromise.create({
      provider: new WsProvider(this.rpcUrl),
      rpc: rpc as any, // ToDo: figure out why the rpc augmentation is not working
      types: types as any, // ToDo: remove once rpc augmentation is working
    });
    this.gateways = await initGateways(this.client);
    this.circuit = new Circuit(this.client, this.signer);

    return this.client;
  }
}

export { Encodings, Converters, Types, Gateway, Circuit, Tx };
