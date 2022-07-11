import{ ApiPromise, Keyring, WsProvider } from'@polkadot/api';

export const setOperational = async (circuit: ApiPromise, gatewayData: any, argument: boolean) => {
     switch(gatewayData.registrationData.gatewayVendor) {
            case "Substrate": {
                return circuit.tx.multiFinalityVerifierDefault.setOperational(argument, gatewayData.id)
                break;
            }
            default: {
            console.log(`SetOperational not available for Vendor ${gatewayData.registrationData.gatewayVendor}`)
                 return
            }
     }
}