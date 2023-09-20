import { u8aConcat, u8aToU8a, u8aToHex } from "@polkadot/util"
import { xxhashAsU8a } from "@polkadot/util-crypto"
import BN from "bn.js"
const generateArgumentKey = (module: string, variableName: string, arg?: number | string | Uint8Array | Buffer ) => {
    // lets prepare the storage key for system events.
    let module_hash = xxhashAsU8a(module, 128)
    let storage_value_hash = xxhashAsU8a(variableName, 128)

	if(arg){
		//if arg is a number, we need to encode it as a hex string
		if(typeof arg === "number") {
			arg = "0x" + new BN(arg).toBuffer("le", 4).toString("hex")
		}

		let argumentKey = u8aConcat(
			xxhashAsU8a(arg, 64),
			u8aToU8a(arg)
		)
		// Special syntax to concatenate Uint8Array
		let final_key = new Uint8Array([
			...module_hash,
			...storage_value_hash,
			...argumentKey,
		])
		return u8aToHex(final_key)
	} else {
		// Special syntax to concatenate Uint8Array
		let final_key = new Uint8Array([
			...module_hash,
			...storage_value_hash,
		])
		return u8aToHex(final_key)
	}
}

export const getStorageProof = async (client: any, blockHash: any, module: string, variableName: string, arg?: number | string | Uint8Array | Buffer) => {
	const key = generateArgumentKey(module, variableName, arg)
    // @ts-ignore
    return client.rpc.state.getReadProof([key], blockHash)
}

export const getStorage = async (client: any, blockHash: any, module: string, variableName: string, arg?: number | string | Uint8Array | Buffer) =>{
	const key = generateArgumentKey(module, variableName, arg)
    return client.rpc.state.getStorage(key)
}