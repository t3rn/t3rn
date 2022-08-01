import{ ApiPromise, Keyring, WsProvider } from'@polkadot/api';

export const setOperational = async (circuit: ApiPromise, gatewayData: any, argument: boolean) => {
     switch(gatewayData.registrationData.gatewayVendor) {
            case "Rococo" || "Substrate": {
                return {gatewayId: circuit.createType("ChainId", gatewayData.id), operational: circuit.createType("bool", argument)}
                break;
            }
            default: {
                console.log(`SetOperational not available for Vendor ${gatewayData.registrationData.gatewayVendor}`)
                return
            }
     }
}