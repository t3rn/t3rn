import { getPriceForSymbol } from "./cg";
import { estimateActionGasFee, EstimateEthActionParams, mapSfxActionToEthAction, SpeedMode } from "./eth";

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

export type Action = "tass" | "cevm"
export const Actions = {
  TransferAsset: "tass",
  CallEvm: "cevm"
} as const;

/**
 * An interface to configure execution estimation
 */

interface Estimate<T> {
  target: Target,
  action: Action,
  asset: string,
  args: T
}

export interface EstimateMaxReward<T> extends Estimate<T> {
  targetAsset: string,
  targetAmount: number,
  overSpendPercent: number
}

export interface Asset {
  value: number,
  symbol: string
}

/**
- **gasFee**: This field represents the cost of computational resources required to execute a transaction on the target blockchain network. It is calculated in the native asset of the target network. For debugging purposes, we also provide the gas fee converted into the base asset.

- **executorFeeEstimate**: This field provides an estimated fee that will be paid to the executor of a transaction. It is calculated as an overspent percentage over the target amount and then converted into the base asset. The executor is the entity that processes and validates the transaction on the blockchain.

- **maxReward**: This field represents the maximum reward for executing the transaction. It is calculated as the sum of the gas fee estimate, the executor fee estimate, and the target amount involved in the transaction. The max reward provides an upper limit on the total cost of the transaction, including all fees and the transaction amount itself. It is estimated in the base asset.

- **estimatedValue**: This field represents the estimated value of the target amount in the base asset. It is included primarily for debugging purposes and provides a way to understand the value of the transaction in terms of the base asset.

Please note that these estimations are subject to change based on the state of the blockchain network at the time of the transaction, and they serve as a guide to understanding the potential costs and rewards associated with a transaction.
*/

export interface MaxRewardEstimation {
  gasFee: Array<Asset>,
  executorFeeEstimate: Asset,
  maxReward: Asset,
  estimatedValue: Asset
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
 * @param input.target The execution target
 * @param input.action The execution action
 * @param input.args Arguments used for estimating gas fees.
 * 
 * If estimating for an ETH transfer:
 * @param {EthSpeedModes} input.args - Specifies the desired speed mode. This should be one of the values from the EthSpeedModes Enum.
 * 
 * If estimating for an EVM call:
 * @param {EstimateEvmCallGasParams} input.args - Parameters necessary for estimating gas in an Ethereum Virtual Machine (EVM) call.
 *
 * @returns The gas fee in the target's native asset
*/

export async function estimateGasFee<T extends EstimateEthActionParams | SpeedMode>({ target, action, args }: Estimate<T>) {
  switch (target) {
    case "eth":
    case "sepl":
      return estimateActionGasFee(target, mapSfxActionToEthAction(action), args as T);
    default:
      throw new Error("Gas fee estimation for this target is not yet implemented!");
  }
}

/**
 * Estimate the a bid amount with a specified profit margin
 *
 * The bid amount is the sum of the gas fee estimate required
 * for an execution and the a profit margin specified by the executor
 *
 * @param input.target The execution target
 * @param input.action The execution action
 * @param input.args Arguments used for estimating gas fees.
 * 
 * If estimating for an ETH transfer:
 * @param {EthSpeedModes} input.args - Specifies the desired speed mode. This should be one of the values from the EthSpeedModes Enum.
 * 
 * If estimating for an EVM call:
 * @param {EstimateEvmCallGasParams} input.args - Parameters necessary for estimating gas in an Ethereum Virtual Machine (EVM) call.
 *
 * @param profitMargin A function that takes the gas fee estimate and returns the profit margin
 *
 * @returns The bid amount in the target's native asset
*/

export async function estimateBidAmount<T extends EstimateEthActionParams | SpeedMode>(input: Estimate<T>, profitMargin: (gasFee: number) => number) {
  const targetNativeAsset = mapTargetToNativeAsset(input.target);
  const gasFees = await estimateGasFee(input);
  return { value: gasFees + profitMargin(gasFees), symbol: targetNativeAsset } as Asset;
}

/**
 * Estimate the maximum reward for an execution
 *
 * @param input.action The execution action
 * @param input.asset The base asset
 * @param input.target The execution target
 * @param input.targetAmount The amount of the target asset
 * @param input.targetAsset The target asset
 * @param input.overSpendPercent The percentage of the target amount to be used as a profit margin
 * @param input.args Arguments used for estimating gas fees.
 * 
 * If estimating for an ETH transfer:
 * @param {EthSpeedModes} input.args - Specifies the desired speed mode. This should be one of the values from the EthSpeedModes Enum.
 * 
 * If estimating for an EVM call:
 * @param {EstimateEvmCallGasParams} input.args - Parameters necessary for estimating gas in an Ethereum Virtual Machine (EVM) call.
 *
 * @returns The maximum reward estimate
*/

export async function estimateMaxReward<T extends EstimateEthActionParams | SpeedMode>({
  action, asset: baseAsset, target, targetAmount, targetAsset, overSpendPercent = 0.5, args
}: EstimateMaxReward<T>): Promise<MaxRewardEstimation> {
  const targetNativeAsset = mapTargetToNativeAsset(target);
  const targetAmountInBaseAsset = await getPriceForSymbol(targetAsset, baseAsset) * targetAmount;
  const gasFeeInTargetNativeAsset = await estimateGasFee({
    target, action, asset: targetAsset, args
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
    estimatedValue: {
      value: targetAmountInBaseAsset,
      symbol: baseAsset
    },
  };
}

export * from "./eth";
