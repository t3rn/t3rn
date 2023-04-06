import { submitRelaychainHeaders } from "./substrate";
import{ ApiPromise, WsProvider } from'@polkadot/api';

export const submitHeader = async (circuitApi: any, gatewayData: any, gatewayId: string, logger: any) => {
    switch(gatewayData.registrationData.verificationVendor) {
        case "Rococo": {
            const targetApi = await ApiPromise.create({
                provider: new WsProvider(gatewayData.rpc),
            });
            if(!gatewayData.registrationData.parachain) { // null or undefined
                return submitRelaychainHeaders(circuitApi, targetApi, gatewayId, logger)
            }
            else {
                console.log("Headers can only be submitted for the relaychain, not for parachains");
                process.exit(1);
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