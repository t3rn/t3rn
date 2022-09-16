import {registerPortalSubstrate} from "./substrate";

export const register = async (circuitApi: any, gatewayData: any, epochsAgo: number) => {
    switch(gatewayData.registrationData.gatewayVendor) {
        case "Rococo": {
            return registerPortalSubstrate(circuitApi, gatewayData, epochsAgo)
        }
        default: {
            console.log(`Registration not available for Vendor ${gatewayData.registrationData.gatewayVendor}`)
            return
        }
    }
}