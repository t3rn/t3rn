import ora from "ora";
import {Args} from "@/types.js";
import {validate} from "@/utils/fns.js";
import {XcmTransferSchema} from "@/schemas/xcm.ts";
import {colorLogMsg} from "@/utils/log.js";
import {ApiPromise, WsProvider } from "@t3rn/sdk";
import { generateXcmTransferParameters } from "@t3rn/sdk/utils";

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
        const targetApi = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9933"), // Rococo Validator on Zombienet
        })
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
