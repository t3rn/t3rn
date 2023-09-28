import ora from "ora";
import {Args} from "@/types.js";
import {validate} from "@/utils/fns.js";
import {XcmTransferSchema} from "@/schemas/xcm.ts";
import {colorLogMsg} from "@/utils/log.js";
import {ApiPromise, WsProvider, Utils} from "@t3rn/sdk";
import { generateXcmTransferParameters } from "@/utils/xcm.ts";

export const spinner = ora()

export const handleXcmTransferCommand = async (
    _args: Args<
        | "dest"
        | "recipient"
        | "targetAsset"
        | "targetAmount"
    >,
) => {
    const args = validate(
        XcmTransferSchema,
        {
            ..._args,
            targetAmount: parseFloat(_args?.targetAmount),
        },
        {
            configFileName: "XCM transfer arguments",
        },
    )

    if (!args) {
        process.exit()
    }

    spinner.text = "Submitting XCM Transaction... "
    spinner.start()

    try {
        /*
        // ERROR WHEN CREATING SIGNER
        const keyring = new Keyring({ type: "sr25519" })
        const signer = process.env.CIRCUIT_KEY === undefined
            ? keyring.addFromUri("//Alice")
            : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
        console.log("Creating SDK...\n");
        const sdk = new Sdk("ws://127.0.0.1:9933", signer);

         */
        const targetApi = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9933"), // Rococo Validator on Zombienet
        })
        console.log("Creating XCM message...\n")
        const xcmTransactionParams = generateXcmTransferParameters (
            targetApi,
            args.dest,
            args.recipient,
            "ROC",
            "para"
        );
        console.log("\n")
        console.table(JSON.stringify(xcmTransactionParams))

    } catch (e) {
        spinner.fail(colorLogMsg("ERROR", e))
    }

    spinner.stop()
}
