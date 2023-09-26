import { ApiPromise } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { xxhashAsU8a } from "@polkadot/util-crypto";
import { BN } from "@polkadot/util";
import { Sdk } from "@t3rn/sdk";

export async function getStorage(
  api: ApiPromise,
  parameters: Record<string, unknown>,
) {
  const res = (await api.rpc.state.getStorage(parameters.key)) as unknown as {
    toHex: () => string;
  };
  return {
    value: res.toHex(),
    status: res !== undefined ? true : false,
  };
}

function generateKeyForStorageValue(module: string, variableName: string) {
  // lets prepare the storage key for system events.
  const module_hash = xxhashAsU8a(module, 128);
  const storage_value_hash = xxhashAsU8a(variableName, 128);

  // Special syntax to concatenate Uint8Array
  const final_key = new Uint8Array([...module_hash, ...storage_value_hash]);

  return u8aToHex(final_key);
}

export const getEventProofs = async (api: ApiPromise, blockHash: string) => {
  const key = generateKeyForStorageValue("System", "Events");
  const proofs = await api.rpc.state.getReadProof([key], blockHash);
  return proofs;
};

/**
 * Fetches the nonce for a specific account.
 *
 * @param api The api instance
 * @param address The account for which the nonce should be fetched
 * @returns The account nonce
 */
export async function fetchNonce(
  api: ApiPromise,
  address: string,
): Promise<BN> {
  return api.rpc.system.accountNextIndex(address);
}

/**
 * Whether a string looks like a Substrate private key.
 *
 * @param x string in question
 * @returns bool
 */
export function problySubstrateSeed(x: string): boolean {
  return /^0x[0-9a-f]{64}$/.test(x);
}

export async function getBalanceWithDecimals(
  client: Sdk["client"] | ApiPromise,
  address: string,
  decimals: number = 12,
): Promise<number> {
  const balance = await client.query.system.account(address);
  // @ts-ignore Property 'data' does not exist on type 'Codec | FrameSystemAccountInfo'.
  const balanceBN = new BN(balance.data.free.toString());

  // Assuming `balance.data.free` is the balance in the lowest unit (smallest decimal)
  // For example, if you have 123.456789 TKN, and decimals: 12, then `balance.data.free` might be 123456789000000
  // We always want to display the balance with two numbers after decimal
  const divisor = new BN(10).pow(new BN(decimals - 2));

  const balanceNumber = balanceBN.div(divisor).toNumber() / 100;

  return balanceNumber;
}
