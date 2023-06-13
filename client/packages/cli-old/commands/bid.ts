import{ ApiPromise} from'@polkadot/api';
import {Sdk} from "@t3rn/sdk/dist";

export const bid = (circuit: ApiPromise, args: any, sdk: Sdk) => {
    return {
        xtxId: circuit.createType("Hash", args.xtxId),
        sfxId: circuit.createType("SideEffectId", args.sfxId),
        bidAmount: circuit.createType("u128", sdk.circuit.floatToBn(args.amount))
    }
}