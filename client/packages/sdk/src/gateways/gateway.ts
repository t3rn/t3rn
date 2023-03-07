import {
  T3rnPrimitivesXdnsXdnsRecord,
  T3rnTypesSfxSideEffect,
} from "@polkadot/types/lookup";
import * as BN from "bn.js";
import { AmountConverter, optionalInsurance } from "../converters/amounts";
import * as Address from "../converters/address";
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
 * A class used to make the connected gateways available, taking care of target specific conversions.
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
    this.id = xdnsEntry.gatewayId.toString();
    this.rpc = xdnsEntry.url.toHuman().toString();
    this.vendor = xdnsEntry.gatewayVendor.toHuman().toString();
    this.executionLayerType = this.getType(
      xdnsEntry.gatewayVendor.toHuman().toString()
    ) as unknown as ExecutionLayerType;
    this.ticker = xdnsEntry.gatewaySysProps.tokenSymbol.toHuman().toString();
    this.decimals = xdnsEntry.gatewaySysProps.tokenDecimals.toNumber()
    this.addressFormat = xdnsEntry.gatewaySysProps.ss58Format.toNumber()
    this.valueTypeSize = xdnsEntry.gatewayAbi.valueTypeSize.toNumber()
    this.allowedSideEffects = xdnsEntry.allowedSideEffects.map((sfx) => sfx.toHuman().toString())
    this.gatewayType = xdnsEntry.gatewayType.toHuman();
    this.setSfxBindings();
  }

  /**
   * Create a transfer side effect, taking care of target specific encodings
   * @param args - The arguments to create the side effect
   * @param args.from - The address of the sender
   * @param args.to - The address of the receiver
   * @param args.value - The value to transfer
   * @param args.maxReward - The maximum reward for the side effect
   * @param args.insurance - The insurance for the side effect
   * @param args.nonce - The nonce of the side effect
   * @param args.signature - The signature of the side effect
   * @param args.enforceExecutor - The address of the executioner
   */

  createTransferSfx = (args: {
    from: string;
    to: string;
    value: number | BN | string;
    maxReward: number | BN | string;
    insurance: number | BN | string;
    nonce: number;
    signature?: string;
    enforceExecutor?: string;
  }): T3rnTypesSfxSideEffect => {
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
      enforceExecutor: args.enforceExecutor,
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
        return Address.Substrate.addrToPub(addr);
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
     * @param value - The integer value to be converted
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
