import { getPriceForSymbol } from "./cg";
import { estimateGasFee as calculateGasFee, mapSfxActionToEthAction, SpeedModes } from "./eth";

/**
 * An execution target
*/

export type Target = "eth" | "sepl" | "roco";
export const Targets = {
  EthereumMainnet: "eth",
  Sepolia: "sepl",
  Rococo: "roco"
} as const;

/**
 * An execution action
 */

export type Action = "tass"
export const Actions = {
  TransferAsset: "tass"
} as const;

/**
 * An interface to configure execution estimation
 */

interface Estimate {
  target: Target,
  action: Action,
  asset: string,
}


export interface EstimateMaxReward extends Estimate {
  targetAsset: string,
  targetAmount: number,
  overSpendPercent: number
}

export interface Asset {
  value: number,
  symbol: string
}

export interface MaxRewardEstimation {
  gasFee: Array<Asset>,
  executorFeeEstimate: Asset,
  maxReward: Asset,
  estimatedValue: number
}

/*
 * Get the native asset of a target
 *
 * @returns The native asset for a given execution target
 */

export const mapTargetToNativeAsset = (target: string) => {
  switch (target) {
    case "eth":
    case "sepl":
      return "eth"
    default:
      throw new Error("Target not yet supported: " + target)
  }
}

/** 
 * Estimate the gas fee required for an execution
 *
 * @param target The execution target
 * @param action The execution action
 *
 * @returns The gas fee in the target's native asset
*/

export const estimateGasFees = async ({ target, action }: Estimate) => {
  switch (target) {
    case "eth":
    case "sepl":
      return calculateGasFee(target, mapSfxActionToEthAction(action), SpeedModes.Standard)
    default:
      throw new Error("Gas fee estimation for this target is not yet implemented!")
  }
}

/**
 * Estimate the a bid amount with a specified profit margin
 *
 * The bid amount is the sum of the gas fee estimate required
 * for an execution and the a profit margin specified by the executor
 *
 * @param input The execution configuration
 * @param profitMargin A function that takes the gas fee estimate and returns the profit margin
 *
 * @returns The bid amount in the target's native asset
*/

export const estimateBidAmount = async (input: Estimate, profitMargin: (gasFee: number) => number) => {
  const targetNativeAsset = mapTargetToNativeAsset(input.target);
  const gasFees = await estimateGasFees(input)
  return { value: gasFees + profitMargin(gasFees), symbol: targetNativeAsset } as Asset
}

/**
 * Estimate the maximum reward for an execution
 *
 * @param input.action The execution action
 * @param input.asset The base asset
 * @param input.target The execution target
 * @param input.targetAmount The amount of the target asset
 * @param input.targetAsset The target asset
 * @overSpentPercent The percentage of the target amount to be used as a profit margin
 *
 * @returns The maximum reward estimate
*/

export const estimateMaxReward = async ({
  action,
  asset: baseAsset,
  target,
  targetAmount,
  targetAsset,
  overSpendPercent = 0.5
}: EstimateMaxReward): Promise<MaxRewardEstimation> => {
  const targetNativeAsset = mapTargetToNativeAsset(target);
  const targetAmountInBaseAsset = await getPriceForSymbol(targetAsset, baseAsset) * targetAmount;

  const gasFeeInTargetNativeAsset = await estimateGasFees({
    target, action, asset: targetAsset,
  });
  const gasFeeInBaseAsset = await getPriceForSymbol(targetNativeAsset, baseAsset) * gasFeeInTargetNativeAsset;

  const executorFeeEstimateInBaseAsset = targetAmountInBaseAsset * overSpendPercent / 100;
  const maxRewardInBaseAsset = targetAmountInBaseAsset + executorFeeEstimateInBaseAsset + gasFeeInBaseAsset;

  return {
    gasFee: [{
      value: gasFeeInTargetNativeAsset,
      symbol: targetNativeAsset
    }, {
      value: gasFeeInBaseAsset,
      symbol: baseAsset
    }],
    executorFeeEstimate: {
      value: executorFeeEstimateInBaseAsset,
      symbol: baseAsset
    },
    maxReward: {
      value: maxRewardInBaseAsset,
      symbol: baseAsset
    },
    estimatedValue: targetAmountInBaseAsset,
  }
}

export * from "./eth";
