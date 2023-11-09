import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { XcmTransferSchema } from '@/schemas/xcm.ts'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { XcmTransferParameters } from '@t3rn/sdk/utils'

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
    }
    else if (args.signer == '//Test') {
      if (process.env.XCM_TEST_SIGNER_KEY === undefined) {
          console.log('XCM test signer key not found... Exit\n')
          spinner.stop()
          process.exit(0)
      }
      signer = keyring.addFromUri(process.env.XCM_TEST_SIGNER_KEY)
    }

    if (args.type == 'relay') {
        if (args.dest == 1000) {
            await targetApi.tx.xcmPallet
                .limitedTeleportAssets(
                    xcmDestParam,
                    xcmBeneficiaryParam,
                    xcmAssetsParam,
                    xcmAssetFeeItem,
                    xcmWeightLimitParam
                )
                .signAndSend(signer, ({ status, events }) => {
                    if (status.isInBlock || status.isFinalized) {
                        events
                            // find/filter for failed events
                            .filter(({ event }) =>
                                targetApi.events.system.ExtrinsicFailed.is(event)
                            )
                            // we know that data for system.ExtrinsicFailed is
                            // (DispatchError, DispatchInfo)
                            .forEach(({ event: { data: [error, info] } }) => {
                                if (error.isModule) {
                                    // for module errors, we have the section indexed, lookup
                                    const decoded = targetApi.registry.findMetaError(error.asModule)
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
        else {
            await targetApi.tx.xcmPallet
                .limitedReserveTransferAssets(
                    xcmDestParam,
                    xcmBeneficiaryParam,
                    xcmAssetsParam,
                    xcmAssetFeeItem,
                    xcmWeightLimitParam
                )
                .signAndSend(signer, ({status, events}) => {
                    if (status.isInBlock || status.isFinalized) {
                        events
                            // find/filter for failed events
                            .filter(({event}) =>
                                targetApi.events.system.ExtrinsicFailed.is(event)
                            )
                            // we know that data for system.ExtrinsicFailed is
                            // (DispatchError, DispatchInfo)
                            .forEach(({event: {data: [error, info]}}) => {
                                if (error.isModule) {
                                    // for module errors, we have the section indexed, lookup
                                    const decoded = targetApi.registry.findMetaError(error.asModule)
                                    const {docs, method, section} = decoded

                                    console.log(`${section}.${method}: ${docs.join(' ')}`)
                                } else {
                                    // Other, CannotLookup, BadOrigin, no extra info
                                    console.log(error.toString())
                                }
                            })
                    }
                })
        }
    }
    else if (args.type == 'para' && args.targetAsset == 'TRN') {
        const xcmNativeAssetAmount = XcmTransferParameters.createNativeAssetAmount(targetApi, args.targetAmount)
        const xcmFeeAsset = XcmTransferParameters.createAssets(targetApi, 'ROC', args.type, 2000000000000)
        await targetApi.tx.withdrawTeleport
            .withdrawAndTeleport(xcmDestParam, xcmBeneficiaryParam, xcmNativeAssetAmount, xcmFeeAsset)
            .signAndSend(signer, ({ status, events }) => {
                if (status.isInBlock || status.isFinalized) {
                    events
                        // find/filter for failed events
                        .filter(({ event }) =>
                            targetApi.events.system.ExtrinsicFailed.is(event)
                        )
                        // we know that data for system.ExtrinsicFailed is
                        // (DispatchError, DispatchInfo)
                        .forEach(({ event: { data: [error, info] } }) => {
                            if (error.isModule) {
                                // for module errors, we have the section indexed, lookup
                                const decoded = targetApi.registry.findMetaError(error.asModule)
                                const { docs, method, section } = decoded

                console.log(`${section}.${method}: ${docs.join(' ')}`)
              } else {
                // Other, CannotLookup, BadOrigin, no extra info
                console.log(error.toString())
              }
            },
          )
        }
     })
    } else if (args.type == 'system' && args.targetAsset == 'TRN') {
      await targetApi.tx.polkadotXcm
        .limitedTeleportAssets(
          xcmDestParam,
          xcmBeneficiaryParam,
          xcmAssetsParam,
          xcmAssetFeeItem,
          xcmWeightLimitParam,
        )
        .signAndSend(signer, ({ status, events }) => {
          if (status.isInBlock || status.isFinalized) {
            events
              // find/filter for failed events
              .filter(({ event }) =>
                targetApi.events.system.ExtrinsicFailed.is(event),
              )
              // we know that data for system.ExtrinsicFailed is
              // (DispatchError, DispatchInfo)
              .forEach(
                ({
                  event: {
                    data: [error, info],
                  },
                }) => {
                  if (error.isModule) {
                    // for module errors, we have the section indexed, lookup
                    const decoded = targetApi.registry.findMetaError(
                      error.asModule,
                    )
                    const { docs, method, section } = decoded

                    console.log(`${section}.${method}: ${docs.join(' ')}`)
                  } else {
                    // Other, CannotLookup, BadOrigin, no extra info
                    console.log(error.toString())
                  }
                },
              )
          }
        })
    } else if(args.type == 'system' || args.type == 'para' ) {
      console.log('Start XCM Transfer \n')
      await targetApi.tx.polkadotXcm
        .limitedReserveTransferAssets(
          xcmDestParam,
          xcmBeneficiaryParam,
          xcmAssetsParam,
          xcmAssetFeeItem,
          xcmWeightLimitParam,
        )
        .signAndSend(signer, ({ status, events }) => {
          if (status.isInBlock || status.isFinalized) {
            events
              // find/filter for failed events
              .filter(({ event }) =>
                targetApi.events.system.ExtrinsicFailed.is(event),
              )
              // we know that data for system.ExtrinsicFailed is
              // (DispatchError, DispatchInfo)
              .forEach(
                ({
                  event: {
                    data: [error, info],
                  },
                }) => {
                  if (error.isModule) {
                    // for module errors, we have the section indexed, lookup
                    const decoded = targetApi.registry.findMetaError(
                      error.asModule,
                    )
                    const { docs, method, section } = decoded

                    console.log(`${section}.${method}: ${docs.join(' ')}`)
                  } else {
                    // Other, CannotLookup, BadOrigin, no extra info
                    console.log(error.toString())
                  }
                },
              )
          }
        })
    }
    else {
        throw new Error('Unsupported transaction type!')
    }
    console.log('XCM Transfer Completed\n')
    spinner.stop()
    process.exit(0)
  } catch (e) {
    spinner.fail(colorLogMsg('ERROR', e))
  }
}
