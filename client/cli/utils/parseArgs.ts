import{ Keyring }from'@polkadot/api';
import {transferAmount} from "./encoder";

export const parseTransferArgs = (args: string[], gatewayData: any) => {
    const keyring = new Keyring({ type: "sr25519" })
    const signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri("//Alice")
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
    if(!args[4]) {
        console.log("Transfer amount not Specified!")
        process.exit(1);
    }

    if(!gatewayData.transferData && (!args[5] || !args[6]) ) {
        console.log("Missing Transfer Config or Transaction Args")
        process.exit(1)
    }

    const amount = transferAmount(parseFloat(args[4]), gatewayData.registrationData.gatewayConfig.decimals, gatewayData.registrationData.gatewayConfig.valueTypeSize);
    const sender = signer.address
    const receiver = args[5] ? args[5] :gatewayData.transferData.receiver ;
    const fee = args[6] ? args[6] :gatewayData.transferData.fee;
    return [amount, sender, receiver, fee]
}