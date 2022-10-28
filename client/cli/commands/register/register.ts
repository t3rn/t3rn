import {registerPortalSubstrate, registerSubstrate} from "./substrate";

export const register = async (circuitApi: any, gatewayData: any, epochsAgo: number) => {
    switch(gatewayData.registrationData.gatewayVendor) {
        case "Substrate": {
            return registerSubstrate(circuitApi, gatewayData)
        }
        case "Rococo": {
            return registerPortalSubstrate(circuitApi, gatewayData, epochsAgo)
        }
        default: {
            console.log(`Registration not available for Vendor ${gatewayData.registrationData.gatewayVendor}`)
            return
        }
    }
}