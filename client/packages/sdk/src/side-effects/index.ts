// @ts-ignore
import {
  // @ts-ignore
  T3rnPrimitivesXdnsXdnsRecord,
  // @ts-ignore
  T3rnTypesSfxSideEffect,
  // @ts-ignore
  u128,
} from '@polkadot/types/lookup'
import * as BN from 'bn.js'

/**
 * A factory function that creates a side effect
 * @param args - The arguments for the side effect
 * @param args.target - The target address of the side effect
 * @param args.signature - The signature of the side effect
 * @param args.nonce - The nonce of the side effect
 * @param args.enforceExecutioner - The address of the executioner
 * @param args.maxReward - The maximum reward for the side effect
 * @param args.insurance - The insurance for the side effect
 * @param args.encodedArgs - The encoded arguments for the side effect
 * @param args.action - The encoded action for the side effect
 * @returns The side effect
 */

export const createSfx = (args: {
  target: number[]
  signature: string | undefined
  nonce: number
  enforceExecutioner: string | undefined
  maxReward: BN
  insurance: BN
  encodedArgs: string[]
  action: string
}): T3rnTypesSfxSideEffect => {
  const sfx: T3rnTypesSfxSideEffect = {
    target: args.target,
    maxReward: args.maxReward,
    insurance: args.insurance,
    action: args.action,
    encodedArgs: args.encodedArgs,
    signature: args.signature,
    nonce: args.nonce,
    enforceExecutioner: args.enforceExecutioner,
  }

  return sfx
}
