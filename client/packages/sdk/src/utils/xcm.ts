import { ApiPromise } from '@polkadot/api';
import type {
	InteriorMultiLocation,
	VersionedMultiAssets,
	VersionedMultiLocation,
} from '@polkadot/types/interfaces';
type ASSET  = "ROC" | "USDT";
type DESTINATION_TYPE = "system" | "relay" | "para";

export const generateXcmTransferParameters = (api: ApiPromise, destChainId: string, beneficiaryAddress: string, assetType: ASSET, destinationType: DESTINATION_TYPE): JSON => {
   let parentValue: string = "";
   switch (destinationType) {
      case "relay":
         parentValue = "0";
         break;
      default: 
         parentValue = "1";
         break;
   };
   let destination: VersionedMultiLocation = createDestination(api, destChainId, parentValue);
   let beneficiary: VersionedMultiLocation = createBeneficiary(api, beneficiaryAddress);  
   let assets =  createAssets(api, "ROC", parentValue, "1000000000000");
   let xcmMessage: JSON = JSON.parse(`{
      "dest": ${destination.toString()},
      "beneficiary": ${beneficiary.toString()},
      "assets": ${assets.toString()},
      "feeAssetItem": "0",
      "weightLimit": "Unlimited"
   }`);
   return xcmMessage;
};

export const createDestination = (api: ApiPromise, destChainId: string, parentValue: string): VersionedMultiLocation => {
   let destinationInterior: InteriorMultiLocation
   if (destChainId != "1") {
      destinationInterior = api.registry.createType('InteriorMultiLocation', {
         X1: {
            parachain: destChainId,
         },
      })
   }
   else {
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
            parachain: 1000,
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
               parents: parentValue,
            }
         }
      },]
   })
}

export const createFeeAssetItem = (api: ApiPromise, feeAssetItem: string): U32 => {
   return api.registry.createType("u32", feeAssetItem.toNumber)
}