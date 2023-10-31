import ora from "ora"
import {Args} from "@/types.js"
import {validate} from "@/utils/fns.js"
import {AssetRegistrationSchema} from "@/schemas/registerAsset.ts"
import {colorLogMsg} from "@/utils/log.js"
import {ApiPromise, WsProvider, Keyring} from "@t3rn/sdk"
import {AssetRegistrationParameters} from "@t3rn/sdk/utils"

export const spinner = ora()

export const handleAssetRegistrationCommand = async (
    _args: Args<
        | "endpoint"
        | "dest"
        | "id"
        | "name"
        | "symbol"
        | "decimals"
    >,
) => {
    const args = validate(
        AssetRegistrationSchema,
        {
            ..._args,
            id: parseInt(_args?.id),
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
        const adminId = await api.query.sudo.key()
        const keyring = new Keyring({ type: "sr25519" })
        keyring.addFromUri("//Alice")
        const adminPair = keyring.getPair(adminId.toString())

        const assetId = AssetRegistrationParameters.createAssetId(api, args.id)
        const assetAdmin = AssetRegistrationParameters.createAdmin(
            api,
            "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        )
        const assetIsSufficient = true
        const assetMinimumBalance = AssetRegistrationParameters.createMinimumBalance(api)
        const assetDecimals = AssetRegistrationParameters.createDecimals(api, args.decimals)
        const assetMultiLocation = AssetRegistrationParameters.createAssetMultiLocation(api, args.symbol)

        const create = await api.tx.sudo.sudo(
            api.tx.utility.batch([
                api.tx.assets.forceCreate(assetId, assetAdmin, assetIsSufficient, assetMinimumBalance),
                api.tx.assets.forceSetMetadata(assetId, args.name, args.symbol, assetDecimals, false ),
                api.tx.assetRegistry.registerReserveAsset(assetId, assetMultiLocation)
            ])
        ).signAndSend(adminPair, ({ status, events }) => {
                if (status.isInBlock || status.isFinalized) {
                    events
                        // We know this tx should result in `Sudid` event.
                        .filter(({ event }) =>
                            api.events.sudo.Sudid.is(event)
                        )
                        // We know that `Sudid` returns just a `Result`
                        .forEach(({ event : { data: [result] } }) => {
                            // Now we look to see if the extrinsic was actually successful or not...
                            if (result.isError) {
                                const error = result.asError
                                if (error.isModule) {
                                    // for module errors, we have the section indexed, lookup
                                    const decoded = api.registry.findMetaError(error.asModule)
                                    const { docs, name, section } = decoded

                                    console.log(`${section}.${name}: ${docs.join(' ')}`)
                                } else {
                                    // Other, CannotLookup, BadOrigin, no extra info
                                    console.log(error.toString())
                                }
                            }
                        })
                    create()
                }
            }
        )
        console.log("Asset Created!\n")

        spinner.stop()
        process.exit(0)

    } catch (e) {
        spinner.fail(colorLogMsg("ERROR", e))
    }
}
