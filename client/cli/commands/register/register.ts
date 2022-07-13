import { registerSubstrate } from "./substrate";

export const register = async (circuitApi: any, gatewayData: any) => {
     switch(gatewayData.registrationData.gatewayVendor) {
            case "Substrate": {
                return registerSubstrate(circuitApi, gatewayData)
            }
             default: {
                console.log(`Registration not available for Vendor ${gatewayData.registrationData.gatewayVendor}`)
                 return
             }
     }
}