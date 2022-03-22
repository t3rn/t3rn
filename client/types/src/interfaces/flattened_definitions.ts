import * as definitions from "./definitions";

// just flatten everything to be directly importable to @polkadot/api ApiPromise
export const types = Object.values(definitions).reduce(
  (res, { types }): object => ({ ...res, ...types }),
  {}
);

// temporary hack to export rpc interfaces just for pallet xdns
export const rpc = {
  xdns: { ...definitions.xdns.rpc },
};

export * from "./types";
