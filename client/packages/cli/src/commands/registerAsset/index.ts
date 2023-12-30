import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { AssetRegistrationSchema } from '@/schemas/registerAsset.ts'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import t3rnSdkUtils from '@t3rn/sdk/utils'
import { signAndSend, signAndSendSudo } from '@/utils/xcm.ts'
const { AssetRegistrationParameters } = t3rnSdkUtils

export const spinner = ora()

export const handleAssetRegistrationCommand = async (
  _args: Args<'endpoint' | 'dest' | 'id' | 'name' | 'symbol' | 'decimals'>,
) => {
  const args = validate(
    AssetRegistrationSchema,
    {
      ..._args,
      id: parseInt(_args?.id),
      decimals: parseInt(_args?.decimals),
    },
    {
      configFileName: 'Asset registration arguments',
    },
  )

  if (!args) {
    process.exit()
  }

  spinner.text = 'Registering Asset... \n'
  spinner.start()

  try {
    const api = await ApiPromise.create({
      provider: new WsProvider(args.endpoint),
    })

    const assetId = AssetRegistrationParameters.createAssetId(api, args.id)
    const assetAdmin = AssetRegistrationParameters.createAdmin(
      api,
      '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d',
    )
    const assetIsSufficient = true
    const assetMinimumBalance =
      AssetRegistrationParameters.createMinimumBalance(api)
    const assetDecimals = AssetRegistrationParameters.createDecimals(
      api,
      args.decimals,
    )
    const assetMultiLocation =
      AssetRegistrationParameters.createAssetMultiLocation(api, args.symbol)

      const keyring = new Keyring({type: 'sr25519'})
      let signer = keyring.addFromUri('//Alice')
      if (process.env.XCM_TEST_SIGNER_KEY != undefined && args.dest != 't0rn') {
          signer = keyring.addFromUri(process.env.XCM_TEST_SIGNER_KEY)
      }
      else if (process.env.CIRCUIT_SIGNER_KEY != undefined && args.dest == 't0rn') {
          signer = keyring.addFromUri(process.env.CIRCUIT_SIGNER_KEY)
      }
      else if (args.dest == 'local') {
          signer = keyring.addFromUri('//Alice')
      }
      else if(args.dest != 'para'){
          throw new Error('Signer not found!')
      }

      if (args.dest == 't0rn' || args.dest == 'local') {
          await signAndSendSudo(
              api.tx.utility.batch([
                  api.tx.assets.forceCreate(
                      assetId,
                      assetAdmin,
                      assetIsSufficient,
                      assetMinimumBalance,
                  ),
                  api.tx.assets.forceSetMetadata(
                      assetId,
                      args.name,
                      args.symbol,
                      assetDecimals,
                      false,
                  ),
                  api.tx.assetRegistry.registerReserveAsset(
                      assetId,
                      assetMultiLocation,
                  ),
              ]),
              api,
              keyring,
          )
    } 
    else if (args.dest == 'para' ) {
        await signAndSend(
              api.tx.utility.batch([
                  api.tx.assets.create(
                      assetId,
                      assetAdmin,
                      assetMinimumBalance,
                  ),
                  api.tx.assets.setMetadata(
                      assetId,
                      args.name,
                      args.symbol,
                      assetDecimals,
                  ),
              ]),
              api,
              signer,
        )
    }
    else {
          throw new Error('Unsupported asset registration destination type!')
    }
    // TO DO: Add support for AssetHub registration of a native token of a parachain
    spinner.succeed(colorLogMsg('SUCCESS', `Asset created`))
    spinner.stopAndPersist({
      symbol: '🎉',
      text: colorLogMsg('SUCCESS', `Asset Created`),
    })
  } catch (e) {
    spinner.fail(colorLogMsg('ERROR', e))
  }
  spinner.stop()
  process.exit(0)
}
