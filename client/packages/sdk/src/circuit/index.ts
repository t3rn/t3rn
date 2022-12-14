import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";
import { AmountConverter } from "../converters/amounts";
import { Tx } from "./tx";

const DECIMALS = 12;
const VALUE_TYPE_SIZE = 16;

/**
 * The class for Circuit
 *
 */

export class Circuit {
  api: ApiPromise;
  // TODO get correct type
  signer: any;
  tx: Tx;

  /**
   * @param api - The ApiPromise instance
   * @param signer - The signer to use for signing transactions
   */

  constructor(api: ApiPromise, signer: any) {
    this.api = api;
    this.signer = signer;
    this.tx = new Tx(this.api, this.signer);
  }

  /**
   * Converts a float to a BN with the correct decimal precision
   * @param value - The value to convert
   * @returns The converted value
   */

  floatToBn(value: number): BN {
    return floatToBn(value)
  }

  /**
   * Converts a BN to a float with the correct decimal precision
   * @param value - The value to convert
   * @returns The converted value
   */

  toFloat(value: BN | number): number {
    return toFloat(value)
  }
}

/**
 * Converts a float to a BN with the correct decimal precision
 * @param value - The value to convert
 * @returns The converted value
 */

export const toFloat = (value: BN | number): number => {
  return new AmountConverter({
    value,
    decimals: DECIMALS,
    valueTypeSize: VALUE_TYPE_SIZE
  }
  ).toFloat()
}

/**
 * Converts a BN to a float with the correct decimal precision
 * @param value - The value to convert
 * @returns The converted value
 */

export const floatToBn = (value: number): BN => {
  return new AmountConverter({
    decimals: DECIMALS,
    valueTypeSize: VALUE_TYPE_SIZE
  }
  ).floatToBn(value)
}
