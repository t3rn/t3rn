import { transferAmount } from "../utils/encoder";
import{ ApiPromise, Keyring, WsProvider }from'@polkadot/api';

export const transfer = async (circuit: ApiPromise, gatewayData: any, amount: number, sender: string, receiver: string, fee: number) => {
    const keyring = new Keyring({ type: "sr25519" })
    const signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri("//Alice")
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
    return new Promise((res, rej) => {
        circuit.tx.circuit
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
            .signAndSend(signer, async result => {
                // @ts-ignore
                if (result && result.toHuman().dispatchError !== undefined) { // The pallet doesn't return a proper error
                    // @ts-ignore
                    rej(result.toHuman().dispatchError)
                } else if (result.isInBlock) {
                    res(true)
                }
            })
    })


}