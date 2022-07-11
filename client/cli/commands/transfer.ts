import { transferAmount } from "../utils/encoder";
import {ApiPromise} from "@polkadot/api";

export const transfer = async (circuit: ApiPromise, gatewayData: any, amount: number, sender: string, receiver: string, fee: number) => {
    return circuit.tx.circuit
        .onExtrinsicTrigger(
            [
                {
                    target: gatewayData.id,
                    prize: 0,
                    ordered_at: 0,
                    encoded_action: [116, 114, 97, 110], //tran
                    encoded_args: [sender, receiver, amount],
                    signature: null,
                    enforce_executioner: null,
                }
            ],
            fee,
            false
        )


}