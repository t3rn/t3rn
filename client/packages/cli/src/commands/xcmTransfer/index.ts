import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { XcmTransferSchema } from '@/schemas/xcm.ts'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { XcmTransferParameters } from '@t3rn/sdk/utils'
import { signAndSend } from '@/utils/xcm.ts'

export const spinner = ora()

export const handleXcmTransferCommand = async (
  _args: Args<
    | 'signer'
    | 'type'
    | 'endpoint'
    | 'dest'
    | 'recipient'
    | 'targetAsset'
    | 'targetAmount'
  >,
) => {
  const args = validate(
    XcmTransferSchema,
    {
      ..._args,
      dest: parseInt(_args?.dest),
      targetAmount: parseFloat(_args?.targetAmount),
    },
    {
      configFileName: 'XCM transfer arguments',
    },
  )

  if (!args) {
    process.exit()
  }

  spinner.text = 'Submitting XCM Transaction... \n'
  spinner.start()
  try {
    const targetApi = await ApiPromise.create({
      provider: new WsProvider(args.endpoint),
    })
    const xcmBeneficiaryParam = XcmTransferParameters.createBeneficiary(
      targetApi,
      args.recipient,
    )
    const xcmAssetFeeItem = XcmTransferParameters.createFeeAssetItem(
      targetApi,
      0,
    )
    const xcmAssetsParam = XcmTransferParameters.createAssets(
      targetApi,
      args.targetAsset,
      args.type,
      args.targetAmount,
    )
    const xcmDestParam = XcmTransferParameters.createDestination(
      targetApi,
      args.dest,
      args.type,
    )
    const xcmWeightLimitParam =
      XcmTransferParameters.createWeightLimit(targetApi)

    const keyring = new Keyring({type: 'sr25519'})
    let signer = keyring.addFromUri(args.signer)
    if (args.signer == '//Circuit') {
      if (process.env.CIRCUIT_SIGNER_KEY === undefined) {
        console.log('Circuit signer not found... Exit\n')
        spinner.stop()
        process.exit(0)
      }
      signer = keyring.addFromUri(process.env.CIRCUIT_SIGNER_KEY)
    } else if (args.signer == '//Test') {
      if (process.env.XCM_TEST_SIGNER_KEY === undefined) {
        console.log('XCM test signer key not found... Exit\n')
        spinner.stop()
        process.exit(0)
      }
      signer = keyring.addFromUri(process.env.XCM_TEST_SIGNER_KEY)
    }

    if (args.type == 'relay') {
        if (args.dest == 1000) {
            await signAndSend(
                    targetApi.tx.xcmPallet.limitedTeleportAssets(
                        xcmDestParam,
                        xcmBeneficiaryParam,
                        xcmAssetsParam,
                        xcmAssetFeeItem,
                        xcmWeightLimitParam
                    ),
                    targetApi,
                    signer,
            )
        }
        else {
            await signAndSend(
                targetApi.tx.xcmPallet.limitedReserveTransferAssets(
                    xcmDestParam,
                    xcmBeneficiaryParam,
                    xcmAssetsParam,
                    xcmAssetFeeItem,
                    xcmWeightLimitParam
                ),
                targetApi,
                signer,
            )
        }
    }
    else if (args.type == 'para' && args.targetAsset == 'TRN') {
        const xcmNativeAssetAmount = XcmTransferParameters.createNativeAssetAmount(targetApi, args.targetAmount)
        const xcmFeeAsset = XcmTransferParameters.createAssets(targetApi, 'ROC', args.type, 2000000000000)
        await signAndSend(
            targetApi.tx.withdrawTeleport.withdrawAndTeleport(
                xcmDestParam,
                xcmBeneficiaryParam,
                xcmNativeAssetAmount,
                xcmFeeAsset
            ),
            targetApi,
            signer,
        )
    } else if (args.type == 'system' && args.targetAsset == 'TRN') {
        await signAndSend(
          targetApi.tx.polkadotXcm.limitedTeleportAssets(
              xcmDestParam,
              xcmBeneficiaryParam,
              xcmAssetsParam,
              xcmAssetFeeItem,
              xcmWeightLimitParam,
          ),
          targetApi,
          signer,
        )
    } else {
        await signAndSend(
          targetApi.tx.polkadotXcm
              .limitedReserveTransferAssets(
                  xcmDestParam,
                  xcmBeneficiaryParam,
                  xcmAssetsParam,
                  xcmAssetFeeItem,
                  xcmWeightLimitParam,
              ),
              targetApi,
              signer,
        )
    }
    spinner.succeed(colorLogMsg('SUCCESS', `Sent XCM transfer`))
    spinner.stopAndPersist({
        symbol: '🎉',
        text: colorLogMsg('SUCCESS', `Sent XCM transfer`),
    })
  } catch (e) {
    spinner.fail(colorLogMsg('ERROR', e))
  }
  spinner.stop()
  process.exit(0)
}
