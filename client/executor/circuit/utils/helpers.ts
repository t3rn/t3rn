import { ApiPromise } from '@polkadot/api';
import { u8aToHex } from '@polkadot/util';
import { TypeRegistry, createType } from "@polkadot/types"
import { xxhashAsU8a } from '@polkadot/util-crypto';
import { SideEffect } from '../../utils/types';

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

function generateKeyForStorageValue(module: string, variableName: string) {
    // lets prepare the storage key for system events.
    let module_hash = xxhashAsU8a(module, 128);
    let storage_value_hash = xxhashAsU8a(variableName, 128);

    // Special syntax to concatenate Uint8Array
    let final_key = new Uint8Array([...module_hash, ...storage_value_hash]);

    return u8aToHex(final_key);
}

export const getEventProofs = async (api: ApiPromise, blockHash: any) => {
    let key = generateKeyForStorageValue('System', 'Events');
    let proofs = await api.rpc.state.getReadProof([key], blockHash);
    console.log(`getProofs : success : ${blockHash}`);
    return proofs;
}

const typeRegistry = new TypeRegistry()
const sfxType = { type: { type: "Vec<Bytes>" } }
typeRegistry.register(sfxType)

export function decodeSfxArgs(sideEffect: SideEffect) {
    const args = createType(typeRegistry, sfxType.type.type, sideEffect.object.encodedArgs)
    console.log("$$$$$ args", args)
    return args
}