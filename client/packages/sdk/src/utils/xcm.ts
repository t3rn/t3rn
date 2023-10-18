import { ApiPromise } from '@polkadot/api'
import type {
   InteriorMultiLocation,
   VersionedMultiAssets,
   VersionedMultiLocation,
} from '@polkadot/types/interfaces'
import {u8, u32} from '@polkadot/types'
type ORIGIN = "relay" | "para" | "system" | "t0rn"
type ASSET  = "ROC" | "USDT" | "TRN"

interface ICreateXcmParameters {
   createDestination: (api: ApiPromise, destChainId: string, originType: ORIGIN) => VersionedMultiLocation
   createBeneficiary: (api: ApiPromise, beneficiaryAddress: string) => VersionedMultiLocation
   createAssets: (api: ApiPromise, assetType: ASSET, originType: ORIGIN, amount: string) => VersionedMultiAssets
   createFeeAssetItem: (api: ApiPromise, feeAssetItem: number) => u32
   //createWeightLimit: (api: ApiPromise, isLimited: bool, weight: u32) => u32
}

export const XcmTransferParameters: ICreateXcmParameters = {
   createBeneficiary: (api: ApiPromise, beneficiaryAddress: string): VersionedMultiLocation => {
      const X1 = {AccountId32: {id: beneficiaryAddress}}
      return api.registry.createType('XcmVersionedMultiLocation', {
         V3: {
            parents: "0",
            interior: {
               X1,
            }
         },
      })
   },
   createDestination: (api: ApiPromise, destChainId: string, originType: ORIGIN): VersionedMultiLocation => {
      let destinationInterior: InteriorMultiLocation
      let parentValue: u8 = api.registry.createType("u8", 1)
      if (originType == "relay") {
         parentValue = api.registry.createType("u8", 0)
      }
      if (destChainId != "1") {
         destinationInterior = api.registry.createType('InteriorMultiLocation', {
            X1: {
               parachain: destChainId,
            },
         })
      } else {
         destinationInterior = api.registry.createType('InteriorMultiLocation', {
            Here: '',
         })
      }
      return api.registry.createType('XcmVersionedMultiLocation', {
         V3: {
            parents: parentValue,
            interior: destinationInterior,
         },
      })
   },
   createAssets: (api: ApiPromise, assetType: ASSET, originType: ORIGIN, amount: string)
       : VersionedMultiAssets => {
      let parentValue: u8 = api.registry.createType("u8", 1)
      if ((originType == "relay" && assetType == "ROC")  || (originType == "system" && assetType == "USDT")
          || (originType == "t0rn" && assetType == "TRN") ) {
         parentValue = api.registry.createType("u8", 0)
      }
      let assetInterior: InteriorMultiLocation
      switch (assetType) {
         case "USDT":
            if (originType == "system") {
               assetInterior = api.registry.createType('InteriorMultiLocation', {
                  X2: {
                     PalletInstance: 50,
                     GeneralIndex: 1984,
                  },
               })
            }
            else {
               assetInterior = api.registry.createType('InteriorMultiLocation', {
                  X3: {
                     parachain: 1000,
                     PalletInstance: 50,
                     GeneralIndex: 1984,
                  },
               })
            }
            break
         case "TRN":
            if (originType == "t0rn") {
               assetInterior = api.registry.createType('InteriorMultiLocation', {
                  X1: {
                     parachain: 3333,
                  },
               })
            }
            else {
               assetInterior = api.registry.createType('InteriorMultiLocation', {
                  Here: '',
               })
            }
            break
         default:
            assetInterior = api.registry.createType('InteriorMultiLocation', {
               Here: '',
            })
      }

      return api.registry.createType('XcmVersionedMultiAssets', {
         V3: [{
            fun: {
               Fungible: amount,
            },
            id: {
               Concrete: {
                  interior: assetInterior,
                  parents: parentValue,
               }
            }
         },]
      })
   },
   createFeeAssetItem: (api: ApiPromise, feeAssetItem: number): u32 => {
      return api.registry.createType("u32", feeAssetItem)
   }
}
