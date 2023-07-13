import fetch from "node-fetch-commonjs";

export interface CoinInfo {
  id: string,
  name: string,
  symbol: string
}

export const getCoinList = async () => {
  const response = await fetch(`https://api.coingecko.com/api/v3/coins/list`);

  if (response.status !== 200) {
    throw new Error("Failed to fetch coin list. ERROR_STATUS: " + response.status);
  }

  return await response.json() as Array<CoinInfo>
}

export const getCurrencyList = async () => {
  const response = await fetch(`https://api.coingecko.com/api/v3/simple/supported_vs_currencies`);

  if (response.status !== 200) {
    throw new Error("Failed to fetch currencies list. ERROR_STATUS: " + response.status);
  }

  return await response.json() as Array<string>
}

export const getCoinWithSymbol = (symbol: string, list: Array<CoinInfo>) => {
  const result = list.find(entry => entry.symbol === symbol)

  if (!result) {
    throw new Error("Failed to find coin with symbol: " + symbol);
  }

  return result;
}

export const getCoinPrice = async (coinId: string, currency = 'usd') => {
  const response = await fetch(`https://api.coingecko.com/api/v3/simple/price?ids=${coinId}&vs_currencies=${currency}`)

  if (response.status !== 200) {
    throw new Error("Failed to fetch coin value. ERROR_STATUS: " + response.status);
  }

  return (await response.json() as unknown)[coinId][currency] as number
}

/**
 * Get the price of an asset in a given currency
 *
 * @param assetSymbol The asset symbol
 * @param currency The currency
*/

export const getPriceForSymbol = async (assetSymbol: string, currency = 'usd') => {
  const coinList = await getCoinList();
  const currencyList = await getCurrencyList();

  if (!currencyList.includes(currency)) {
    throw new Error("Currency not supported: " + currency);
  }

  const info = getCoinWithSymbol(assetSymbol, coinList);
  const price = await getCoinPrice(info.id, currency);

  if (!price) {
    throw new Error("Failed to fetch price for asset: " + assetSymbol);
  }

  return price;
}

