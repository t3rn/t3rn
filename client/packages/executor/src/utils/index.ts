import { ApiPromise } from "@polkadot/api"
import { u8aToHex } from "@polkadot/util"
import { xxhashAsU8a } from "@polkadot/util-crypto"
import { BN } from "@polkadot/util"

export async function getStorage(api: ApiPromise, parameters: any) {
    const res = await api.rpc.state.getStorage(parameters.key)
    return {
        // { value: '0x1c86d8cbffffffffffffffffffffffff', status: true }
        // We may have to change it later down the line.
        value: res.toHex(),
        status: res !== undefined ? true : false,
    }
}

function generateKeyForStorageValue(module: string, variableName: string) {
    // lets prepare the storage key for system events.
    const module_hash = xxhashAsU8a(module, 128)
    const storage_value_hash = xxhashAsU8a(variableName, 128)

    // Special syntax to concatenate Uint8Array
    const final_key = new Uint8Array([...module_hash, ...storage_value_hash])

    return u8aToHex(final_key)
}

export const getEventProofs = async (api: ApiPromise, blockHash: any) => {
    const key = generateKeyForStorageValue("System", "Events")
    const proofs = await api.rpc.state.getReadProof([key], blockHash)
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
