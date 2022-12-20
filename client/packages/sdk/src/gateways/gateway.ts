// @ts-ignore
import {
  // @ts-ignore
  T3rnPrimitivesXdnsXdnsRecord,
  // @ts-ignore
  T3rnTypesSideEffect,
} from "@polkadot/types/lookup";
import * as BN from "bn.js";
import { AmountConverter, optionalInsurance } from "../converters/amounts";
import * as address from "../converters/address";
import { toU8aId } from "../converters/utils";
import { createSfx } from "../side-effects";
import { ExecutionLayerType } from "./types";

/**
 * A Gateway type enum
 */

export enum GatewayType {
  Substrate,
  Evm,
}

/**
 * A class that is used interact with a gateway
 */

export class Gateway {
  id: string;
  rpc: string;
  vendor: string;
  executionLayerType: ExecutionLayerType;
  gatewayType: any;
  ticker: string;
  decimals: number;
  addressFormat: number;
  valueTypeSize: number;
  allowedSideEffects: string[];
  createSfx: {} = {};

  /**
   * Create a Gateway instance
   * @param xdnsEntry - The xdns entry of the gateway
   */

  constructor(xdnsEntry: T3rnPrimitivesXdnsXdnsRecord) {
    console.log("sys_props", xdnsEntry.toHuman().gateway_sys_props);
    this.id = xdnsEntry.toHuman().gateway_id;
    this.rpc = xdnsEntry.url.toHuman();
    // @ts-ignore
    this.vendor = xdnsEntry.toHuman().gateway_vendor.toString();
    this.executionLayerType = this.getType(
      xdnsEntry.toHuman().gateway_vendor.toString()
    ) as unknown as ExecutionLayerType;
    // @ts-ignore
    this.ticker = xdnsEntry.toHuman().gateway_sys_props.token_symbol;
    // @ts-ignore
    this.decimals = parseInt(
      xdnsEntry.toHuman().gateway_sys_props.token_decimals
    );
    // @ts-ignore
    this.addressFormat = parseInt(
      xdnsEntry.toHuman().gateway_sys_props.ss58_format
    );
    // @ts-ignore
    this.valueTypeSize = parseInt(
      xdnsEntry.toHuman().gateway_abi.value_type_size
    );
    this.allowedSideEffects = xdnsEntry.toHuman().allowed_side_effects;
    this.gatewayType = xdnsEntry.toHuman().gateway_type;
    this.setSfxBindings();
  }

  /**
   * Create a side effect
   * @param args - The arguments to create the side effect
   * @param args.from - The address of the sender
   * @param args.to - The address of the receiver
   * @param args.value - The value to transfer
   * @param args.maxReward - The maximum reward for the side effect
   * @param args.insurance - The insurance for the side effect
   * @param args.nonce - The nonce of the side effect
   * @param args.signature - The signature of the side effect
   * @param args.enforceExecutioner - The address of the executioner
   */

  createTransferSfx = (args: {
    from: string;
    to: string;
    value: number | BN | string;
    maxReward: number | BN | string;
    insurance: number | BN | string;
    nonce: number;
    signature?: string;
    enforceExecutioner?: string;
  }): T3rnTypesSideEffect => {
    const encodedArgs: string[] = this.encodeTransferArgs(
      args.from,
      args.to,
      args.value,
      args.insurance,
      args.maxReward
    );

    const maxReward = new AmountConverter({ value: args.maxReward }).toBn();
    const insurance = new AmountConverter({ value: args.insurance }).toBn();

    return createSfx({
      target: toU8aId(this.id),
      nonce: args.nonce,
      maxReward,
      insurance,
      encodedArgs,
      encodedAction: "tran",
      signature: args.signature,
      enforceExecutioner: args.enforceExecutioner,
    });
  };

  /**
   * Encode transfer arguments
   * @param from - The address of the sender
   * @param to - The address of the receiver
   * @param value - The value to transfer
   * @param insurance - The insurance for the side effect
   * @param reward - The reward for the side effect
   */

  encodeTransferArgs(
    from: string,
    to: string,
    value: number | BN | string,
    insurance: number | BN | string,
    reward: number | BN | string
  ): string[] {
    if (!this.allowedSideEffects.includes("tran"))
      throw new Error(`Transfer Sfx not supported for ${this.id}`);
    // ensure we pass the correct address encoding (e.g. pub key for substrate)
    from = this.validateAddress(from);
    to = this.validateAddress(to);

    // convert value to LittleEndian
    const encodedAmount = new AmountConverter({
      value,
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize,
    }).toLeHex();

    // generate optionalInsurance
    const encodedOptionalInsurance = optionalInsurance(insurance, reward);
    return [from, to, encodedAmount, encodedOptionalInsurance];
  }

  /**
   * Convert an address into t3rn compatible form. For example, we want to ensure we pass the public key for polkadot addresses
   * @param address - The address to convert
   */

  validateAddress(addr: string) {
    switch (this.executionLayerType) {
      case ExecutionLayerType.Substrate:
        return address.substrate.addrToPub(addr);
        break;
      default:
        return addr;
    }
  }

  /**
   * Convert a float value into the correct integer, accounting for decimals
   * @param value - The value to convert
   */

  floatToBn(value: number): BN {
    return new AmountConverter({
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize,
    }).floatToBn(value);
  }

  /**
   * Parse LE encoded value to correct integer, accounting for decimals
   * @param value - The value to convert
   */

  parseLe(value: string): BN {
    return new AmountConverter({
      value,
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize
    }
    ).toBn()
  }

  /**
     * Parse integer to float, accounting for decimals
     * @param value - The value to convert
     */

  toFloat(value: BN | number): number {
    return new AmountConverter({
      value,
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize
    }
    ).toFloat()
  }

  /**
   * Get gateway type from vendor
   * @param vendor - The vendor of the gateway
   */

  getType(vendor: string) {
    if (["Rococo", "Kusama", "Polkadot"].includes(vendor)) {
      return GatewayType.Substrate;
    } else if (vendor === "Ethereum") {
      return GatewayType.Evm;
    }
  }

  /**
   * Set the side effect bindings
   */

  setSfxBindings() {
    for (let i = 0; i < this.allowedSideEffects.length; i++) {
      switch (this.allowedSideEffects[i]) {
        case "tran":
          this.createSfx["tran"] = this.createTransferSfx;
      }
    }
  }
}

export { ExecutionLayerType };
