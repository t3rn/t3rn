import { ApiPromise } from '@polkadot/api';
import config from "../config/setup"
import { amountLeArr, optionalInsurance } from "./encoder";
import { addressStringToPubKey } from "./decoder";

export const transferArgs = (target: string, from: string, receiver: string, amount: number, bond: number, reward: number) => {
    const gatewayData: any = getGatewayData(target);
    let encodedAmount = amountLeArr(amount, gatewayData.registrationData.gatewayConfig.decimals, gatewayData?.registrationData.gatewayConfig.valueTypeSize);
    if(receiver === "") receiver = gatewayData.transferData.receiver;
    let res = [addressStringToPubKey(from), addressStringToPubKey(receiver), encodedAmount]
    if (bond !== 0 || reward !== 0) {
        res.push(optionalInsurance(bond, reward, config.circuit.decimals, config.circuit.valueTypeSize));
    }
    return res;
}

export const sideEffect = (circuitApi: ApiPromise, data: any, sender: any) => {
    let encodedArgs: any[] = [];
    if(data.type === "tran") {
        encodedArgs = transferArgs(data.target, addressStringToPubKey(sender), addressStringToPubKey(data.receiver), parseFloat(data.amount), parseFloat(data.bond), parseFloat(data.reward));
    }
    return circuitApi.createType("SideEffect",
    {
            target: circuitApi.createType("TargetId", data.target),
            prize: circuitApi.createType("BalanceOf", 0),
            ordered_at: circuitApi.createType("BlockNumber", 0),
            encoded_action: circuitApi.createType("Bytes", data.type), //tran
            encoded_args: circuitApi.createType("Vec<Bytes>", encodedArgs),
            signature: circuitApi.createType("Bytes", data.signature),
            enforce_executioner: circuitApi.createType("Option<AccountId>", data.executioner)
        })
}

const getGatewayData = (target: string) => {
    return config.gateways.find(entry => entry.id === target);
}

