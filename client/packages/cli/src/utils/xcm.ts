import { ApiPromise } from '@polkadot/api'
import type {
	InteriorMultiLocation,
	VersionedMultiAssets,
	VersionedMultiLocation,
} from '@polkadot/types/interfaces'
type ASSET  = "ROC" | "USDT"
type DESTINATION_TYPE = "system" | "relay" | "para";

export const generateXcmTransferParameters = (api: ApiPromise, destChainId: string, beneficiaryAddress: string, assetType: ASSET, destinationType: DESTINATION_TYPE): JSON => {
   let parentValue
   switch (destinationType) {
      case "relay":
         parentValue = "0"
         break
      default: 
         parentValue = "1"
         break
   }
   const destination: VersionedMultiLocation = createDestination(api, destChainId, parentValue)
   const beneficiary: VersionedMultiLocation = createBeneficiary(api, beneficiaryAddress)
   const assets =  createAssets(api, "ROC", parentValue, "1000000000000")
   return JSON.parse(`{
      "dest": ${destination.toString()},
      "beneficiary": ${beneficiary.toString()},
      "assets": ${assets.toString()},
      "feeAssetItem": "0",
      "weightLimit": "Unlimited"
   }`)
}

export const createDestination = (api: ApiPromise, destChainId: string, parentValue: string): VersionedMultiLocation => {
   return api.registry.createType('XcmVersionedMultiLocation', {
      V3: {
         parents: parentValue,
         interior: {
            X1: {
               parachain: destChainId,
            },
         },
      },
   })
}

export const createBeneficiary = (api: ApiPromise, beneficiaryAddress: string): VersionedMultiLocation => {
   const X1 = {AccountId32: {id: beneficiaryAddress}}
   return api.registry.createType('XcmVersionedMultiLocation', {
      V3: {
         parents: 0,
         interior: {
            X1,
         },
      },
   })
}

export const createAssets = (api: ApiPromise, assetType: ASSET, parentValue: string, amount: string): VersionedMultiAssets => {
   let assetInterior: InteriorMultiLocation
   switch (assetType) {
      case "USDT":
         assetInterior = api.registry.createType('InteriorMultiLocation', {
            ParachainId: 1000,
            PalletInstance: 50,
            GeneralIndex: 140,
         })
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
                  parent: parentValue,
               }
            }
         },]
      })
}
