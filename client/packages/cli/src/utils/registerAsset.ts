import type {
    InteriorMultiLocation,
    VersionedMultiLocation,
    MultiAddress
} from '@polkadot/types/interfaces'
import {u8, u32, u128} from '@polkadot/types'
type DESTINATION = "local" | "AssetHub"

interface IRegisterAsset {
    //createAssetMultiLocation: (api: ApiPromise, assetSymbol) => VersionedMultiLocation
    createAssetId: (api: ApiPromise, id: number) => u32
    createMinimumBalance: (api: ApiPromise) => u128
    createAdmin: (api: ApiPromise, address: string) => MultiAddress
    createDecimals: (api: ApiPromise, decimals: number) => u8
}

export const AssetRegistrationParameters: IRegisterAsset = {
    createAssetId: (api: ApiPromise, id: number): u32 => {
        return api.registry.createType("u32", id)
    },
    createMinimumBalance: (api: ApiPromise): u128 => {
        return api.registry.createType("u128", 1)
    },
    createAdmin: (api: ApiPromise, address: string): MultiAddress => {
        return api.registry.createType("MultiAddress", {
            Id:  address
        })
    },
    createDecimals: (api: ApiPromise, decimals: number): u8 => {
        return api.registry.createType("u8", decimals)
    }
}