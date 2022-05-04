import { ApiPromise } from '@polkadot/api';
// import { u8aToHex, stringToU8a } from '@polkadot/util';
// import { xxhashAsU8a, xxhashAsHex } from '@polkadot/util-crypto';
import { decodeCustomType } from "./typeDecoder"

import { u8aConcat, u8aToU8a, u8aToHex } from '@polkadot/util';
import { xxhashAsU8a } from '@polkadot/util-crypto';
const BN = require("bn.js");

async function getStorage(api: ApiPromise, parameters: any) {
    let res = await api.rpc.state.getStorage(parameters.key);
    return {
        // @ts-ignore
        // { value: '0x1c86d8cbffffffffffffffffffffffff', status: true }
        // We may have to change it later down the line.
        value: res.toHex(),
        status: res !== undefined ? true : false,
    }
}

function encodeParachainId(id: number) {
    // this is the correct encoding for u32
    return "0x" + new BN(id).toBuffer("le", 4).toString("hex")
}

function generateKeyForStorageValue(module: string, variableName: string, parachainid: number) {
    // lets prepare the storage key for system events.
    let module_hash = xxhashAsU8a(module, 128);
    let storage_value_hash = xxhashAsU8a(variableName, 128);

    let encodedParachainId = encodeParachainId(parachainid)
    let argumenteKey = u8aConcat(xxhashAsU8a(encodedParachainId, 64), u8aToU8a(encodedParachainId))

    // Special syntax to concatenate Uint8Array
    let final_key = new Uint8Array([...module_hash, ...storage_value_hash, ...argumenteKey]);

    return u8aToHex(final_key);
}

export const getHeaderProof = async (api: ApiPromise, blockHash: any, parachainId: number) => {
    let key = generateKeyForStorageValue('Paras', 'Heads', parachainId); // these are correct!
    // 0xcd710b30bd2eab0352ddcc26417aa1941b3c252fcb29d88eff4f3de5de4476c39f434b9dae0bfb8ed4070000
    console.log("key:", key); 
    const proof = await api.rpc.state.getReadProof([key], blockHash);
    let header = await api.rpc.state.getStorage(key, blockHash);
    
    // console.log("HEADER FROM STORAGE:", header.toHuman())

    console.log(proof.toJSON())

    return [proof.toJSON(), key];
}