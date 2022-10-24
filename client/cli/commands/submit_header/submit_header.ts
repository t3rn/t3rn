import { submitRelaychainHeaders, submitParachainHeaders } from "./substrate";
import{ ApiPromise, WsProvider } from'@polkadot/api';

export const submitHeader = async (circuitApi: any, gatewayData: any, gatewayId: string) => {
    switch(gatewayData.registrationData.gatewayVendor) {
        case "Rococo": {
            const targetApi = await ApiPromise.create({
                provider: new WsProvider(gatewayData.rpc),
            });
            if(gatewayData.registrationData.parachain == null) { // null or undefined
                return submitRelaychainHeaders(circuitApi, targetApi, gatewayId)
            }
            else {
                return submitParachainHeaders(circuitApi, targetApi, gatewayData)
            }
            break;
        }
        default: {
            console.log("Vendor not configured!");
            process.exit(1);
        }
    }
}