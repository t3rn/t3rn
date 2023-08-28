import fetch from "node-fetch";
import { Web3, Bytes } from "web3";
import { Action as SfxAction } from './index'

// DECLARE GAS AMOUNT
export const ETH_TRANSFER_GAS_AMOUNT = 21000;

export type SpeedMode = "rapid" | "fast" | "standard" | "slow";
export const SpeedModes = {
  Rapid: "rapid",
  Fast: "fast",
  Standard: "standard",
  Slow: "slow",
} as const;

export type Action = "transfer" | "cevm";
export const Actions = {
  Transfer: "transfer",
  CallEvm: "cevm"
} as const;

export type GasPrice = {
  rapid: number;
  fast: number;
  standard: number;
  slow: number;
  timestamp: number;
  priceUsd: number;
};

export type Target = "eth" | "sepl";
export const Targets = {
  Eth: "eth",
  Sepolia: "sepl",
} as const;


/**
 * Map t3rn sfx ation to eth action
 *
 * @param action The sfx action
 * @return The eth action
 */

export const mapSfxActionToEthAction = (action: SfxAction) => {
  switch (action) {
    case "tass":
      return Actions.Transfer;
    case "cevm":
      return Actions.CallEvm
    default:
      throw new Error("Unable to map sfx action to eth action");
  }
}

const getGasPriceEstimationEndpoint = (target: Target) => {
  switch (target) {
    case Targets.Eth:
      return "https://beaconcha.in/api/v1/execution/gasnow";
    case Targets.Sepolia:
      return "https://sepolia.beaconcha.in/api/v1/execution/gasnow";
  }
};

/**
 * Gets the gas price for a given target
 * 
 * @param target The execution target
 * @returns The gas price
*/

export const getGasPrice = async (target: Target) => {
  const req = await fetch(getGasPriceEstimationEndpoint(target));

  if (req.status !== 200) {
    throw new Error("Failed to fetch gas price. ERROR_STATUS: " + req.status);
  }

  return ((await req.json()) as {
    data: GasPrice
  }).data;
};

/** 
 * Gets the gas amount for a given action
 *
 * @param action The action
 * @returns The gas amount
 */

export const getGasAmount = (action: Action) => {
  switch (action) {
    case Actions.Transfer:
    default:
      return ETH_TRANSFER_GAS_AMOUNT;
  }
};

/**
 * Calculates the gas fee in ether
 *
 * @param target The execution target
 * @param action The execution action
 * @param speedMode The speed mode
 *
 * @returns The gas fee in ether
*/

export const estimateGasFee = async (
  target: Target,
  action: Action,
  speedMode: SpeedMode
) => {
  const gasPrice = (await getGasPrice(target))[speedMode];
  const gasAmount = getGasAmount(action);
  const gasFeeInWei = BigInt(gasPrice) * BigInt(gasAmount);
  const gasFeeInEther = Number(gasFeeInWei) / 1e18;
  return gasFeeInEther;
};

const getTargetRpcEndpoint = (target: Target) => {
  switch (target) {
    case Targets.Eth:
      return "https://rpc.ankr.com/eth";
    case Targets.Sepolia:
      return "https://rpc.ankr.com/eth_sepolia";
  }
};

export interface EstimateEvmCallGasParams {
  fromAddress: string
  toAddress: string
  data: Bytes
  speedMode?: SpeedMode
}

export const estimateEvmCallGas = async (target: Target, { fromAddress, toAddress, data, speedMode }: EstimateEvmCallGasParams) => {
  const web3 = new Web3(getTargetRpcEndpoint(target));
  const gasAmount = await web3.eth.estimateGas({
    from: fromAddress,
    to: toAddress,
    data,
  })
  const gasPrice = (await getGasPrice(target))[speedMode ?? SpeedModes.Standard];
  const gasFeeInWei = BigInt(gasPrice) * BigInt(gasAmount);
  const gasFeeInEther = Number(gasFeeInWei) / 1e18;
  return gasFeeInEther
}

export type EstimateEthActionParams = EstimateEvmCallGasParams | SpeedMode
export const estimateActionGasFee = <T extends EstimateEthActionParams>(
  target: Target,
  action: Action,
  params: T
) => {
  switch (action) {
    case Actions.CallEvm:
      return estimateEvmCallGas(target, params as EstimateEvmCallGasParams)
    default:
      return estimateGasFee(
        target,
        action,
        (params as SpeedMode) ?? SpeedModes.Standard
      )
  }
}
