import type {
    InteriorMultiLocation,
    MultiLocation,
    MultiAddress
} from '@polkadot/types/interfaces'
import { ApiPromise } from '@polkadot/api'
import { u8, u32, u128 } from '@polkadot/types'
//type DESTINATION = "local" | "AssetHub"

interface IRegisterAsset {
    createAssetId: (api: ApiPromise, id: number) => u32
    createMinimumBalance: (api: ApiPromise) => u128
    createAdmin: (api: ApiPromise, address: string) => MultiAddress
    createDecimals: (api: ApiPromise, decimals: number) => u8
    createAssetMultiLocation: (api: ApiPromise, assetSymbol: string) => MultiLocation
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
    },
    createAssetMultiLocation: (api: ApiPromise, assetSymbol: string): MultiLocation => {
        const parentValue: u8 = api.registry.createType("u8", 1)
        let assetInterior: InteriorMultiLocation
        switch (assetSymbol) {
            case "ROC":
                assetInterior = api.registry.createType('InteriorMultiLocation', {
                    Here: '',
                })
                break
            case "USDT":
                assetInterior = api.registry.createType('InteriorMultiLocation', {
                    X3: [
                        { Parachain: 1000 },
                        { PalletInstance: 50 },
                        { GeneralIndex: 1984 },
                    ],
                })
                break
            case "TRN":
                assetInterior = api.registry.createType('InteriorMultiLocation', {
                    X1: {
                        parachain: 3333,
                    },
                })
                break
            default:
                throw new Error('Unsupported Asset!')
        }
        return api.registry.createType('XcmV3MultiLocation', {
                interior: assetInterior,
                parents: parentValue,
        })
    }
}