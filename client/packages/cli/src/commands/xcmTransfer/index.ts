import ora from "ora"
import {Args} from "@/types.js"
import {validate} from "@/utils/fns.js"
import {XcmTransferSchema} from "@/schemas/xcm.ts"
import {colorLogMsg} from "@/utils/log.js"
import {ApiPromise, WsProvider, Keyring } from "@t3rn/sdk"
import {createAssets, createBeneficiary, createDestination, createFeeAssetItem} from "@/utils/xcm.ts"//"@t3rn/sdk/utils"

export const spinner = ora()

export const handleXcmTransferCommand = async (
    _args: Args<
        | "signer"
        | "type"
        | "endpoint"
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
            provider: new WsProvider(args.endpoint),
        })
        const xcmBeneficiaryParam = createBeneficiary(targetApi, args.recipient)
        const xcmAssetFeeItem = createFeeAssetItem(targetApi, "0")
        console.log("Sending XCM Transfer... \n")

        const keyring = new Keyring({ type: "sr25519" })
        const signer = process.env.CIRCUIT_SIGNER_KEY === undefined
            ? keyring.addFromUri(args.signer)
            : keyring.addFromMnemonic(process.env.CIRCUIT_SIGNER_KEY)
        if (args.signer == "//Circuit" && process.env.CIRCUIT_SIGNER_KEY === undefined) {
            console.log("Circuit signer not found... Exit\n")
            spinner.stop()
            process.exit(0)
        }
        if (args.type == "relay") {
            const xcmDestParam = createDestination(targetApi, args.dest, "0")
            const xcmAssetsParam = createAssets(targetApi, args.targetAsset, "0", args.targetAmount)
            await targetApi.tx.xcmPallet
                .reserveTransferAssets(xcmDestParam,xcmBeneficiaryParam, xcmAssetsParam, xcmAssetFeeItem)
                .signAndSend(signer, ({ status, events }) => {
                    if (status.isInBlock || status.isFinalized) {
                        events
                            // find/filter for failed events
                            .filter(({ event }) =>
                                api.events.system.ExtrinsicFailed.is(event)
                            )
                            // we know that data for system.ExtrinsicFailed is
                            // (DispatchError, DispatchInfo)
                            .forEach(({ event: { data: [error, info] } }) => {
                                if (error.isModule) {
                                    // for module errors, we have the section indexed, lookup
                                    const decoded = api.registry.findMetaError(error.asModule)
                                    const { docs, method, section } = decoded

                                    console.log(`${section}.${method}: ${docs.join(' ')}`)
                                } else {
                                    // Other, CannotLookup, BadOrigin, no extra info
                                    console.log(error.toString())
                                }
                            })
                    }
                })
        }
        else if (args.type == "para") {
            const xcmDestParam = createDestination(targetApi, args.dest, "1")
            const xcmAssetsParam = createAssets(targetApi, args.targetAsset, "1", args.targetAmount)
            await targetApi.tx.polkadotXcm
                .reserveTransferAssets(xcmDestParam,xcmBeneficiaryParam, xcmAssetsParam, xcmAssetFeeItem)
                .signAndSend(signer, ({ status, events }) => {
                    if (status.isInBlock || status.isFinalized) {
                        events
                            // find/filter for failed events
                            .filter(({ event }) =>
                                api.events.system.ExtrinsicFailed.is(event)
                            )
                            // we know that data for system.ExtrinsicFailed is
                            // (DispatchError, DispatchInfo)
                            .forEach(({ event: { data: [error, info] } }) => {
                                if (error.isModule) {
                                    // for module errors, we have the section indexed, lookup
                                    const decoded = api.registry.findMetaError(error.asModule)
                                    const { docs, method, section } = decoded

                                    console.log(`${section}.${method}: ${docs.join(' ')}`)
                                } else {
                                    // Other, CannotLookup, BadOrigin, no extra info
                                    console.log(error.toString())
                                }
                            })
                    }
                })
        }
        console.log("XCM Transfer Completed\n")
        spinner.stop()
        process.exit(0)

    } catch (e) {
        spinner.fail(colorLogMsg("ERROR", e))
    }
}
