import { registerSubstrate} from "./substrate";
import { registerEth } from "./ethereum";
import {ApiPromise} from "@polkadot/api";

export const register = async (circuit: ApiPromise, gatewayData: any) => {
    return {
        gatewayId: circuit.createType("ChainId", gatewayData.id),
        tokenId: circuit.createType("ChainId", gatewayData.tokenId),
        verificationVendor: circuit.createType('GatewayVendor', gatewayData.registrationData.verificationVendor),
        executionVendor: circuit.createType('ExecutionVendor', gatewayData.registrationData.executionVendor),
        codec: circuit.createType('RuntimeCodec', gatewayData.registrationData.runtimeCodec),
        registrant: null,
        escrowAccounts: null,
        allowedSideEffects: circuit.createType('Vec<([u8; 4], Option<u8>)>', gatewayData.registrationData.allowedSideEffects),
        tokenInfo: circuit.createType('TokenInfo', gatewayData.registrationData.tokenInfo),
        registrationData: await generateRegistrationData(circuit, gatewayData)
    }
}

const generateRegistrationData = async (circuit: ApiPromise, gatewayData: any) => {
    console.log(gatewayData.registrationData.verificationVendor)
     switch(gatewayData.registrationData.verificationVendor) {
        case "Rococo": {
            return registerSubstrate(circuit, gatewayData)
            break;
        }
        case "Ethereum": {
            return registerEth(circuit, gatewayData)
            break;
        }
        default: {
            throw new Error("Registration for VerificationVendor not available!")
        }

    }
}