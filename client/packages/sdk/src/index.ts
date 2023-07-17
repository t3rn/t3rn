import "@polkadot/api-augment"; // DO NOT REMOVE THIS LINE
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { cryptoWaitReady } from "@polkadot/util-crypto";

import types from "./config/types.json";
import rpc from "./config/rpc.json";
import { Gateway, initGateways } from "./gateways";
import * as Utils from "./utils";
import * as Types from "./types";

// @ts-ignore
import { T3rnTypesSideEffect } from "@polkadot/types/lookup";
import * as Encodings from "./encodings";
import * as Converters from "./converters";
import { Circuit, Tx } from "./circuit";

/**
 * The main class for the SDK
 */

export class Sdk {
  provider: WsProvider;
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
  exportMode: boolean;

  /**
   * @param provider - RPC url or WsProvider instance of circuit
   * @param circuitSigner - The signer to use for signing transactions
   * @param exportMode
   */
  constructor(
    provider: string | WsProvider,
    circuitSigner: any,
    exportMode: boolean = false
  ) {
    this.signer = circuitSigner;
    if (typeof provider === "string") {
      this.provider = new WsProvider(provider);
    } else {
      this.provider = provider;
    }
    this.exportMode = exportMode;
  }

  /**
   * Initializes ApiPromise instance and loads available gateways via XDNS
   * @returns ApiPromise instance
   */
  async init(): Promise<ApiPromise> {
    await cryptoWaitReady();
    this.client = await ApiPromise.create({
      provider: this.provider,
      types: types as any,
      rpc: rpc as any,
    });
    this.gateways = await initGateways(this.client);
    this.circuit = new Circuit(this.client, this.signer, this.exportMode);

    return this.client;
  }
}

export {
  Encodings,
  Converters,
  Types,
  Gateway,
  Circuit,
  Tx,
  Utils,
  ApiPromise,
  WsProvider,
  Keyring,
  cryptoWaitReady,
};
