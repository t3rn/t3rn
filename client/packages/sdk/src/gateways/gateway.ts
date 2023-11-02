import {
  // @ts-ignore
  T3rnPrimitivesXdnsFullGatewayRecord,
  // @ts-ignore
  T3rnTypesSideEffect,
} from '@polkadot/types/lookup'
import * as BN from 'bn.js'
import { AmountConverter } from '../converters/amounts'
import * as Address from '../converters/address'
import { toU8aId } from '../converters/utils'
import { createSfx } from '../side-effects'

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
  id: string
  rpc: string
  vendor: string
  executionVendor: string
  gatewayType: any
  ticker: string
  decimals: number
  tokenId: number
  valueTypeSize: number
  allowedSideEffects: string[]
  createSfx: {} = {}
  tokens: [] = []
  record: T3rnPrimitivesXdnsFullGatewayRecord

  /**
   * Create a Gateway instance
   * @param xdnsEntry - The xdns entry of the gateway
   */

  constructor(record: T3rnPrimitivesXdnsFullGatewayRecord) {
    this.id = record.gateway_record.gateway_id.toHuman()
    this.record = record
    this.vendor = record.gateway_record.verification_vendor.toHuman()
    this.executionVendor = record.gateway_record.execution_vendor.toHuman()
    let tokens: any[] = record.tokens.map((token) => token.toHuman())

    let nativeToken = tokens.filter((token) => token.gateway_id === this.id)[0]
    // @ts-ignore
    this.ticker = Object.values(nativeToken.token_props)[0].symbol
    this.decimals = parseInt(
      // @ts-ignore
      Object.values(nativeToken.token_props)[0].decimals,
    )
    this.tokenId = parseInt(
      // @ts-ignore
      Object.values(nativeToken.token_props)[0].id,
    )
    this.allowedSideEffects = record.gateway_record.allowed_side_effects
      .toHuman()
      .map((entry) => entry[0])
    this.setSfxBindings()
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
   * @param args.enforceExecutioner - The address of the executioner
   */

  createTransferSfx = (args: {
    from: string
    to: string
    value: number | BN | string
    maxReward: number | BN | string
    insurance: number | BN | string
    nonce: number
    signature?: string
    enforceExecutioner?: string
  }): T3rnTypesSideEffect => {
    const encodedArgs: string[] = this.encodeTransferArgs(
      args.from,
      args.to,
      args.value,
      args.insurance,
      args.maxReward,
    )

    const maxReward = new AmountConverter({ value: args.maxReward }).toBn()
    const insurance = new AmountConverter({ value: args.insurance }).toBn()

    return createSfx({
      target: toU8aId(this.id),
      nonce: args.nonce,
      maxReward,
      insurance,
      encodedArgs,
      action: 'tran',
      signature: args.signature,
      enforceExecutioner: args.enforceExecutioner,
    })
  }
  /*
  createXTransferSfx = (args: {
    from: string;
    beneficiary: string;
    destChainId: string;
    asset: string;
    value: number | BN | string;
    maxReward: number | BN | string;
    insurance: number | BN | string;
    nonce: number;
    signature?: string;
    enforceExecutioner?: string;
  }): T3rnTypesSideEffect => {
    const encodedArgs: string[] = this.encodeXTransferArgs(
      args.from,
      args.beneficiary,
      args.destChainId,
      args.asset,
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
      action: "tran",
      signature: args.signature,
      enforceExecutioner: args.enforceExecutioner,
    });
  };
*/
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
    reward: number | BN | string,
  ): string[] {
    if (!this.allowedSideEffects.includes('tran'))
      throw new Error(`Transfer Sfx not supported for ${this.id}`)
    // ensure we pass the correct address encoding (e.g. pub key for substrate)
    to = this.validateAddress(to)

    // convert value to LittleEndian
    const encodedAmount = new AmountConverter({
      value,
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize,
    }).toLeHex()

    return [to, encodedAmount]
  }

  /*
  encodeXTransferArgs(
    from: string,
    beneficiary: string,
    destChainId: number | BN | string,
    asset: string,
    value: number | BN | string,
    insurance: number | BN | string,
    reward: number | BN | string
  ): string[] {
    if (!this.allowedSideEffects.includes("xtran"))
      throw new Error(`XTransfer Sfx not supported for ${this.id}`);
    // ensure we pass the correct address encoding (e.g. pub key for substrate)
    beneficiary = this.validateAddress(beneficiary);

    // convert value to LittleEndian
    const encodedAmount = new AmountConverter({
      value,
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize,
    }).toLeHex();

    return [beneficiary, destChainId, asset, encodedAmount];
  }
  */
  /**
   * Convert an address into t3rn compatible form. For example, we want to ensure we pass the public key for polkadot addresses
   * @param address - The address to convert
   */

  validateAddress(addr: string) {
    switch (this.executionVendor) {
      case 'Substrate':
        return Address.Substrate.addrToPub(addr)
        break
      default:
        return addr
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
    }).floatToBn(value)
  }

  /**
   * Parse LE encoded value to correct integer, accounting for decimals
   * @param value - The value to convert
   */

  parseLe(value: string): BN {
    return new AmountConverter({
      value,
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize,
    }).toBn()
  }

  /**
   * Parse integer to float, accounting for decimals
   * @param value - The integer value to be converted
   */

  toFloat(value: BN | number): number {
    return new AmountConverter({
      value,
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize,
    }).toFloat()
  }

  /**
   * Get gateway type from vendor
   * @param vendor - The vendor of the gateway
   */

  getType(vendor: string) {
    if (['Rococo', 'Kusama', 'Polkadot'].includes(vendor)) {
      return GatewayType.Substrate
    } else if (vendor === 'Ethereum') {
      return GatewayType.Evm
    }
  }

  /**
   * Set the side effect bindings
   */

  setSfxBindings() {
    for (let i = 0; i < this.allowedSideEffects.length; i++) {
      switch (this.allowedSideEffects[i]) {
        case 'tran':
          this.createSfx['tran'] = this.createTransferSfx
      }
    }
  }
}
