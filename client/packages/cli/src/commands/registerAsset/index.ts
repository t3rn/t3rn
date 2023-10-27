import ora from "ora"
import {Args} from "@/types.js"
import {validate} from "@/utils/fns.js"
import {AssetRegistrationSchema} from "@/schemas/registerAsset.ts"
import {colorLogMsg} from "@/utils/log.js"
import {ApiPromise, WsProvider, Keyring} from "@t3rn/sdk"
import {XcmTransferParameters} from "@t3rn/sdk/utils"

export const spinner = ora()

export const handleAssetRegistrationCommand = async (
    _args: Args<
        | "endpoint"
        | "dest"
        | "name"
        | "symbol"
        | "decimals"
    >,
) => {
    const args = validate(
        AssetRegistrationSchema,
        {
            ..._args,
            decimals: parseInt(_args?.decimals),
        },
        {
            configFileName: "Asset registration arguments",
        },
    )

    if (!args) {
        process.exit()
    }

    spinner.text = "Registering Asset... \n"
    spinner.start()

    try {
        const api = await ApiPromise.create({
            provider: new WsProvider(args.endpoint),
        })

        const keyring = new Keyring({ type: "sr25519" })
        const signer = keyring.addFromUri("//Alice")
        console.log("Asset Registrateon Command")

        // TO DO: forceCreateAsset - sudo command
        // TO DO: setAssetMetadata
        // TO DO: registerAsset - sudo command


        spinner.stop()
        process.exit(0)

    } catch (e) {
        spinner.fail(colorLogMsg("ERROR", e))
    }
}
