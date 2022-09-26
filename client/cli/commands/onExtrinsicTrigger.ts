import { ApiPromise } from '@polkadot/api';
import { sideEffect } from "../utils/sideEffects";

export const onExtrinsicTrigger = (circuitApi: ApiPromise, sideEffects: any[], sequential: boolean, sender: any) => {
    return {
        sideEffects: circuitApi.createType("Vec<SideEffect>",
            sideEffects.map(data => sideEffect(circuitApi, data, sender))
        ),
        fee: circuitApi.createType("Currency::Balance", 0),
        sequential: circuitApi.createType("bool", sequential),
    }
}