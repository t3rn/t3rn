import fetch from "node-fetch";

// const GAS_AMOUNT = 10;

// const estimate = () => { };

export const getGasPrice = async () => {
  const endpoint = "https://ethgasstation.info/json/ethgasAPI.json";
  const req = await fetch(endpoint);

  if (req.status !== 200) {
    throw new Error("Failed to fetch gas price. ERROR_STATUS: " + req.status);
  }

  const data = await req.json();

  return data;
};
