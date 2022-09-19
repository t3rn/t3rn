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

export const parseSubmitHeaderArgs = (args: string[]): [string] => {
    const gatewayId = process.argv[3]

    if(!gatewayId) {
        console.log("GatewayId not Specified!")
        process.exit(1);
    }

    return [gatewayId]
}

export const parseRegisterArgs = (args: string[]): [string, number] => {
    const gatewayId = process.argv[3]

    if(!gatewayId) {
        console.log("GatewayId not Specified!")
        process.exit(1);
    }

    let epochsAgo = 0
    if(args[4]) {
        let parsed = parseInt(args[4])
        if(isNaN(parsed)) {
            console.log("Can't parse epochsAgo argument! Using latest epoch for registration!")
        } else {
            epochsAgo = parsed;
        }
    }
    return [gatewayId, epochsAgo]
}