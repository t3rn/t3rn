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

export const getGasPrice = async () => {
  const endpoint = "https://beaconcha.in/api/v1/execution/gasnow";
  const req = await fetch(endpoint);

  if (req.status !== 200) {
    throw new Error("Failed to fetch gas price. ERROR_STATUS: " + req.status);
  }

  return (await req.json()).data as GasPrice;
};

export const getGasAmount = (action: EthAction) => {
  switch (action) {
    case EthActions.Transfer:
    default:
      return ETH_TRANSFER_GAS_AMOUNT;
  }
};

export const calculateGasFee = async (
  action: EthAction,
  speedMode: EthSpeedMode
) => {
  const gasPrice = (await getGasPrice())[speedMode];
  const gasAmount = getGasAmount(action);
  const gasFeeInWei = BigInt(gasPrice) * BigInt(gasAmount);
  const gasFeeInEther = Number(gasFeeInWei) / 1e18;
  return gasFeeInEther;
};
