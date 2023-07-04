import fetch from "node-fetch";

// Gas amount for actions
export const ETH_TRANSFER_GAS_AMOUNT = 21000;

export type EthSpeedMode = "rapid" | "fast" | "standard" | "slow";
export const EthSpeedModes = {
  Rapid: "rapid",
  Fast: "fast",
  Standard: "standard",
  Slow: "slow",
} as const;

export type EthAction = "tran";
export const EthActions = {
  Transfer: "tran",
} as const;

export type GasPrice = {
  rapid: number;
  fast: number;
  standard: number;
  slow: number;
  timestamp: number;
  priceUsd: number;
};

export type EthTarget = "eth" | "sepl";
export const EthTargets = {
  Eth: "eth",
  Sepolia: "sepl",
};

const getEndpoint = (target: EthTarget) => {
  switch (target) {
    case EthTargets.Eth:
      return "https://beaconcha.in/api/v1/execution/gasnow";
    case EthTargets.Sepolia:
      return "https://sepolia.beaconcha.in/api/v1/execution/gasnow";
  }
};

export const getGasPrice = async (target: EthTarget) => {
  const req = await fetch(getEndpoint(target));

  if (req.status !== 200) {
    throw new Error("Failed to fetch gas price. ERROR_STATUS: " + req.status);
  }

  return ((await req.json()) as {
    data: GasPrice
  }).data;
};

export const getGasAmount = (action: EthAction) => {
  switch (action) {
    case EthActions.Transfer:
    default:
      return ETH_TRANSFER_GAS_AMOUNT;
  }
};

export const calculateGasFee = async (
  target: EthTarget,
  action: EthAction,
  speedMode: EthSpeedMode
) => {
  const gasPrice = (await getGasPrice(target))[speedMode];
  const gasAmount = getGasAmount(action);
  const gasFeeInWei = BigInt(gasPrice) * BigInt(gasAmount);
  const gasFeeInEther = Number(gasFeeInWei) / 1e18;
  return gasFeeInEther;
};
