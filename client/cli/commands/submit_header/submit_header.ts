import { fetchRelaychainArgs, submitParachainHeader } from "./substrate";
import{ ApiPromise, WsProvider } from'@polkadot/api';

export const submitHeader = async (circuitApi: any, gatewayData: any, gatewayId: string, blockNumber: number) => {
    switch(gatewayData.registrationData.gatewayVendor) {
        case "Rococo": {
            const targetApi = await ApiPromise.create({
                provider: new WsProvider(gatewayData.rpc),
            });
            if(gatewayData.registrationData.parachain == null) { // null or undefined
                return fetchRelaychainArgs(circuitApi, targetApi, gatewayId, blockNumber)
            }
            else {
                console.log("Parachains not implemented yet!")
                process.exit(1)
            }
            break;
        }
        default: {
            console.log("Vendor not configured!");
            process.exit(1);
        }
    }
}