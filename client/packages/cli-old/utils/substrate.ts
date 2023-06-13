import{ ApiPromise, WsProvider } from'@polkadot/api';
import { u8aConcat, u8aToU8a, u8aToHex } from "@polkadot/util"
import { xxhashAsU8a } from "@polkadot/util-crypto"
import BN from "bn.js"

// ToDo update to portal RPC endpoint
export const fetchBestFinalizedHash = async (circuit: any, gatewayId: string) => {
    return circuit.query.rococoBridge.bestFinalizedMap(gatewayId);
}

// this returns the parachain header contained in the specified relaychain header
export const fetchLatestPossibleParachainHeader = async (relaychainRpc: string, relayChainHeaderHash: string, parachainId: string) => {
    const client = await getClient(relaychainRpc, relayChainHeaderHash)
    return client.query.paras.heads(parachainId)
}

export const getStorageProof = async (rpc: string, blockHash: any, parachainId: number) => {
    const client = await getClient(rpc);
    const key = generateArgumentKey("Paras", "Heads", parachainId)
    // @ts-ignore
    return client.rpc.state.getReadProof([key], blockHash)
}

const encodeParachainId = (id: number) => {
    // this is the correct storageKey parameter encoding for u32
    return "0x" + new BN(id).toBuffer("le", 4).toString("hex")
}

const generateArgumentKey = (module: string, variableName: string, parachainid: number) => {
    // lets prepare the storage key for system events.
    let module_hash = xxhashAsU8a(module, 128)
    let storage_value_hash = xxhashAsU8a(variableName, 128)

    let encodedParachainId = encodeParachainId(parachainid)
    let argumenteKey = u8aConcat(
        xxhashAsU8a(encodedParachainId, 64),
        u8aToU8a(encodedParachainId)
    )

    // Special syntax to concatenate Uint8Array
    let final_key = new Uint8Array([
        ...module_hash,
        ...storage_value_hash,
        ...argumenteKey,
    ])

    return u8aToHex(final_key)
}

export const getClient = async (rpc: string, at?: string) => {
    let client = await ApiPromise.create({
        provider: new WsProvider(rpc),
    })

    if(at) {
        return client.at(at);
    }
    return client
}