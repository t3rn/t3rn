import { calculateGasFee, EthSpeedModes } from "./eth";

/**
 * The target on which the execution will run. i.e targe: eth, sepl, roco
*/

export type Target = "eth" | "sepl" | "roco";

/**
 * The action that will be executed. i.e tran - transfer
 */

export type Action = "tran"

interface Estimate {
  target: Target,
  action: Action,
  asset: string,
}

/** 
 * Estimate the gas fee required for an execution
 *
 * @param target The target on which the execution will run. i.e targe: eth, sepl, roco
 * @param action The action that will be executed. i.e tran - transfer
 * @param asset The asset that will be used for the execution. i.e ETH, ROCO
 *
*/

export const estimate = async ({ target, action }: Estimate) => {
  switch (target) {
    case "eth":
    case "sepl":
      return calculateGasFee(target, action, EthSpeedModes.Standard)
    default:
      throw new Error("Price estimation for this target is not yet implemented!")
  }
}

/**
 * Estimate the gas fee required for an execution and add a profit margin
 *
 * @param target The target on which the execution will run. i.e targe: eth, sepl, roco
 * @param action The action that will be executed. i.e tran - transfer
 * @param asset The asset that will be used for the execution. i.e ETH, ROCO
 *
 * @example const maxReward = await estimateForExecutor({ target: "eth", action: "tran", asset: "USDT" }, (gasFee) => gasFee * 1.2)
*/

export const estimateForExecutor = async (input: Estimate, profitMargin: (gasFee: number) => number) => {
  const gasFees = await estimate(input)
  return gasFees + profitMargin(gasFees)
}

interface EstimateMaxReward extends Estimate {
  targetAsset: string,
  targetAmount: number,
  overSpendPercent: number
}

interface MaxRewardEstimation {
  gasFee: number,
  executorFee: number,
  maxReward: number
  asset: string
}

export const estimateMaxReward = async ({
  action,
  asset,
  target,
  targetAmount,
  targetAsset,
  overSpendPercent = 0.5
}: EstimateMaxReward): Promise<MaxRewardEstimation> => {
  const gasFee = await estimate({
    target, action, asset: targetAsset,
  });
  const fxValue = (() => {
    console.log(`Estimate the exchange value i.e Ask CG how much ${targetAmount} is worth in ${asset}`)
    return 0;
  })();
  const executorFee = targetAmount * overSpendPercent / 100;
  const maxReward = gasFee + executorFee + fxValue;

  return {
    gasFee,
    executorFee,
    maxReward,
    asset: targetAsset
  }
}

export * from "./eth";
