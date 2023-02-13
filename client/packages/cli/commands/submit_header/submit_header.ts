import { submitRelaychainHeaders, submitParachainHeaders } from "./substrate";
import{ ApiPromise, WsProvider } from'@polkadot/api';

export const submitHeader = async (circuitApi: any, gatewayData: any, gatewayId: string, logger: any) => {
    switch(gatewayData.registrationData.gatewayVendor) {
        case "Rococo": {
            const targetApi = await ApiPromise.create({
                provider: new WsProvider(gatewayData.rpc),
            });
            if(gatewayData.registrationData.parachain == null) { // null or undefined
                return submitRelaychainHeaders(circuitApi, targetApi, gatewayId, logger)
            }
            else {
                return submitParachainHeaders(circuitApi, targetApi, gatewayData, logger)
            }
            break;
        }
        default: {
            logger.debug({
                success: false,
                msg: "Vendor not configured"
            });
            process.exit(1);
        }
    }
}