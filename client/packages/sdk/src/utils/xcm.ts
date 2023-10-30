import { ApiPromise } from '@polkadot/api'
import type {
   InteriorMultiLocation,
   VersionedMultiAssets,
   VersionedMultiLocation,
   WeightLimitV2,
} from '@polkadot/types/interfaces'
import {u8, u32, u128} from '@polkadot/types'
type ORIGIN = "relay" | "para" | "system" | "t0rn"
type ASSET  = "ROC" | "USDT" | "TRN"

/*
type WeightLimit = {
   refTime: string,
   proofSize: string,
}
 */

interface ICreateXcmParameters {
   createDestination: (api: ApiPromise, destChainId: string, originType: ORIGIN) => VersionedMultiLocation
   createBeneficiary: (api: ApiPromise, beneficiaryAddress: string) => VersionedMultiLocation
   createAssets: (api: ApiPromise, assetType: ASSET, originType: ORIGIN, amount: string) => VersionedMultiAssets
   createFeeAssetItem: (api: ApiPromise, feeAssetItem: number) => u32
   createNativeAssetAmount: (api: ApiPromise, amount: number) => u128
   createWeightLimit: (api: ApiPromise/*, isLimited: bool, weightLimit: WeightLimit*/) => WeightLimitV2
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
                  X2: [
                     { PalletInstance: 50 },
                     { GeneralIndex: 1984 },
                  ],
               })
            }
            else {
               assetInterior = api.registry.createType('InteriorMultiLocation', {
                  X3: [
                     { Parachain: 1000 },
                     { PalletInstance: 50 },
                     { GeneralIndex: 1984 },
                  ],
               })
            }
            break
         case "TRN":
            assetInterior = api.registry.createType('InteriorMultiLocation', {
               X1: {
                  parachain: 3333,
               },
            })
            break
         case "ROC":
            assetInterior = api.registry.createType('InteriorMultiLocation', {
               Here: '',
            })
            break
         default:
            throw new Error('Unsupported Asset!')
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
   },
   createNativeAssetAmount: (api: ApiPromise, amount: number): u128 => {
      return api.registry.createType("u128", amount)
   },
   createWeightLimit: (api: ApiPromise/*, isLimited: bool, weightLimit: WeightLimit*/): WeightLimitV2 => {
      // if (!isLimited) {
      return api.registry.createType('XcmV3WeightLimit', {
         Unlimited: null
      })
      //}
      //else {
      //   return api.registry.createType('XcmV3WeightLimit', {
      //      Limited: {
      //         refTime: weightLimit.refTime,
      //         proofSize: weightLimit.proofSize,
      //      }
      //  })
      //}
   }
}
