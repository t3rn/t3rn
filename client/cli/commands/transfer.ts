import { ApiPromise } from '@polkadot/api';

export const transfer = (circuitApi: ApiPromise, gatewayData: any, encodedAmount: number[], encodedBond: number[] | null, sender: string, receiver: string, fee: number, executioner: string) => {
    const encodedArgs = encodedBond === null ? [sender, receiver, encodedAmount] : [sender, receiver, encodedAmount, encodedBond]
    return {
        sideEffects: circuitApi.createType("Vec<SideEffect>",
            [{
            target: circuitApi.createType("TargetId", gatewayData.id),
            prize: circuitApi.createType("BalanceOf", 0),
            ordered_at: circuitApi.createType("BlockNumber", 0),
            encoded_action: circuitApi.createType("Bytes", [116, 114, 97, 110]), //tran
            encoded_args: circuitApi.createType("Vec<Bytes>", encodedArgs),
            signature: circuitApi.createType("Bytes", null),
            enforce_executioner: circuitApi.createType("Option<AccountId>", executioner)
        }]),
        fee: circuitApi.createType("Currency::Balance", fee),
        sequential: circuitApi.createType("bool", false),
    }
}