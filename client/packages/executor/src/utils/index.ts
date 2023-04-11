import { ApiPromise } from "@polkadot/api"
import { u8aToHex } from "@polkadot/util"
import { xxhashAsU8a } from "@polkadot/util-crypto"
import { BN } from "@polkadot/util"

export async function getStorage(api: ApiPromise, parameters: any) {
    let res = await api.rpc.state.getStorage(parameters.key)
    return {
        // @ts-ignore
        // { value: '0x1c86d8cbffffffffffffffffffffffff', status: true }
        // We may have to change it later down the line.
        value: res.toHex(),
        status: res !== undefined ? true : false,
    }
}

function generateKeyForStorageValue(module: string, variableName: string) {
    // lets prepare the storage key for system events.
    let module_hash = xxhashAsU8a(module, 128)
    let storage_value_hash = xxhashAsU8a(variableName, 128)

    // Special syntax to concatenate Uint8Array
    let final_key = new Uint8Array([...module_hash, ...storage_value_hash])

    return u8aToHex(final_key)
}

export const getEventProofs = async (api: ApiPromise, blockHash: any) => {
    let key = generateKeyForStorageValue("System", "Events")
    let proofs = await api.rpc.state.getReadProof([key], blockHash)
    return proofs
}

/**
 * Fetches the nonce for a specific account.
 *
 * @param api The api instance
 * @param address The account for which the nonce should be fetched
 * @returns The account nonce
 */
export async function fetchNonce(api: ApiPromise, address: string): Promise<BN> {
    return api.rpc.system.accountNextIndex(address)
}

/**
 * Whether a string looks like a Substrate private key.
 *
 * @param x string in question
 * @returns bool
 */
export function problySubstrateSeed(x: any): boolean {
    return typeof x === "string" && x.startsWith("0x") && x.length === 66
}
