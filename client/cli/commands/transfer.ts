import { ApiPromise } from '@polkadot/api';

export const transfer = (circuitApi: ApiPromise, gatewayData: any, amount: number, sender: string, receiver: string, fee: number) => {
    return {
        sideEffects: circuitApi.createType("Vec<SideEffect>",
            [{
            target: circuitApi.createType("TargetId", gatewayData.id),
            prize: circuitApi.createType("BalanceOf", 0),
            ordered_at: circuitApi.createType("BlockNumber", 0),
            encoded_action: circuitApi.createType("Bytes", [116, 114, 97, 110]), //tran
            encoded_args: circuitApi.createType("Vec<Bytes>", [sender, receiver, amount]),
            signature: circuitApi.createType("Bytes", null),
            enforce_executioner: circuitApi.createType("Option<AccountId>", null)
        }]),
        fee: circuitApi.createType("Currency::Balance", fee),
        sequential: circuitApi.createType("bool", false),
    }
}