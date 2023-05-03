import { ApiPromise } from '@polkadot/api';
import { Sdk } from "@t3rn/sdk";
import "@t3rn/types"
// @ts-ignore
import { Vec, T3rnTypesSfxSideEffect, SpeedMode } from "@polkadot/types/lookup"

export const onExtrinsicTrigger = (circuitApi: ApiPromise, sideEffects: any[], speed_mode: SpeedMode, sender: any, sdk: Sdk) => {
    return {
        sideEffects: circuitApi.createType("Vec<T3rnTypesSfxSideEffect>",
            sideEffects.map(data => {
                console.log("data", data)
                const obj: T3rnTypesSfxSideEffect = sdk.gateways[data.target].createSfx[data.type]({
                    from: sender.toString(),
                    to: data.to,
                    value: sdk.gateways[data.target].floatToBn(data.amount),
                    maxReward: sdk.circuit.floatToBn(data.reward), // CLI accepts floats, so we need to convert
                    insurance: sdk.circuit.floatToBn(data.insurance), // same here
                    nonce: 0,
                })
                return obj
            })
        ),
        speed_mode: circuitApi.createType("SpeedMode", speed_mode),
    }
}