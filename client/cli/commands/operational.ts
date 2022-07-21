import{ ApiPromise, Keyring, WsProvider } from'@polkadot/api';

export const setOperational = async (circuit: ApiPromise, gatewayData: any, argument: boolean) => {
     switch(gatewayData.registrationData.gatewayVendor) {
            case "Substrate": {
                return circuit.tx.multiFinalityVerifierDefault.setOperational(argument, gatewayData.id)
                break;
            }
            case "Rococo": {
                return circuit.tx.portal.setOperational(gatewayData.id, argument)
                break;
            }
            default: {
                console.log(`SetOperational not available for Vendor ${gatewayData.registrationData.gatewayVendor}`)
                return
            }
     }
}