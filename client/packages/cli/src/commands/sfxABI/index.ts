import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { AddSfxAbiSchema } from '@/schemas/abi.ts'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'

export const spinner = ora()

export const handleAddSfxAbiCommand = async (
  _args: Args<
    'signer' | 'endpoint' | 'target' | 'sfxId' | 'sfxAbi' | 'palletId' | 'purge'
  >,
) => {
  const args = validate(
    AddSfxAbiSchema,
    {
      ..._args,
    },
    {
      configFileName: 'Add SFX ABI to XDNS arguments',
    },
  )

  if (!args) {
    process.exit()
  }

  console.log('args', args)

  if (args.purge) {
    spinner.text = 'Submitting purge SFX ABI Transaction... \n'
  } else {
    spinner.text = 'Submitting add SFX ABI Transaction... \n'
  }

  spinner.start()

  try {
    const targetApi = await ApiPromise.create({
      provider: new WsProvider(args.endpoint),
    })

    const keyring = new Keyring({ type: 'sr25519' })
    const signer =
      process.env.CIRCUIT_SIGNER_KEY === undefined
        ? keyring.addFromUri(args.signer)
        : keyring.addFromMnemonic(process.env.CIRCUIT_SIGNER_KEY)
    if (
      args.signer == '//Circuit' &&
      process.env.CIRCUIT_SIGNER_KEY === undefined
    ) {
      console.log('Circuit signer not found... Exit\n')
      spinner.stop()
      process.exit(0)
    }

    let tx = undefined

    // Ensure that the effects of the transaction are reflected in the state
    const sfxAbiPrior = await targetApi.query.xdns.sfxabiRegistry(
      args.target,
      null,
    )

    console.info(
      'Updated sfx abi storage prior to transaction',
      sfxAbiPrior.toHuman(),
    )

    if (args.purge) {
      tx = targetApi.tx.xdns.unrollAbiOfSelectedGateway(args.target, args.sfxId)
    } else if (args.sfxAbi) {
      tx = targetApi.tx.xdns.enrollNewAbiToSelectedGateway(
        args.target,
        args.sfxId,
        JSON.parse(args.sfxAbi),
        args.palletId,
      )
    } else {
      // Assume SFX is built-in - lookup in StandardSFXRegistry
      const isKnownABI = await targetApi.query.xdns.standardSFXABIs(args.sfxId)
      if (!isKnownABI) {
        throw new Error(
          'SFX ABI descriptor is required - optional, but required if SFX ID is non-standard (not one of the built-in SFXs)',
        )
      } else {
        console.log(
          `SFX ABI descriptor is not provided, but SFX ID is known - using the built-in ABI descriptor:`,
        )
        console.log(JSON.stringify(isKnownABI, null, 2))

        tx = targetApi.tx.xdns.enrollNewAbiToSelectedGateway(
          args.target,
          args.sfxId,
          null,
          args.palletId,
        )
      }
    }

    if (tx === undefined) {
      throw new Error('No transaction to submit')
    }

    await targetApi.tx.sudo
      .sudo(tx)
      .signAndSend(signer, ({ status, events, dispatchError }) => {
        if (
          status.isDropped ||
          status.isInvalid ||
          status.isUsurped ||
          status.isRetracted ||
          !!dispatchError
        ) {
          if (dispatchError) {
            spinner.fail(colorLogMsg('ERROR', dispatchError))
            process.exit(1)
          }

          spinner.fail(colorLogMsg('ERROR', status.type))
          process.exit(1)
        } else if (status.isInBlock || status.isFinalized || status.isReady) {
          // check if we have an error event in a custom module
          events.forEach((eventEntry) => {
            const eventEntryParsed = JSON.parse(JSON.stringify(eventEntry))
            if (
              eventEntryParsed &&
              eventEntryParsed.event &&
              eventEntryParsed.event.data &&
              Array.isArray(eventEntryParsed.event.data) &&
              eventEntryParsed.event.data.length > 0 &&
              eventEntryParsed.event.data[0].err
            ) {
              const pallet =
                eventEntryParsed.event.data[0].err.module.index ||
                'Un-parsed pallet index'
              const error =
                eventEntryParsed.event.data[0].err.module.error ||
                'Un-parsed error index'
              spinner.fail(
                colorLogMsg(
                  'ERROR',
                  `Pallet of index = ${pallet} returned an error of index = ${error}`,
                ),
              )
              process.exit(1)
            }
          })
        }
      })

    // Sleep for 15 seconds to wait for the transaction to be included in the block
    await new Promise((r) => setTimeout(r, 15000))

    // Ensure that the effects of the transaction are reflected in the state
    const sfxAbi = await targetApi.query.xdns.sfxabiRegistry(args.target, null)

    console.info('Updated sfx abi storage after transaction', sfxAbi.toHuman())

    if (args.purge) {
      spinner.stopAndPersist({
        symbol: '🚩',
        text: colorLogMsg(
          'SUCCESS',
          `SFX ABI Successfully purged ${args.sfxId} for ${args.target}`,
        ),
      })
    } else {
      spinner.stopAndPersist({
        symbol: '🚩',
        text: colorLogMsg(
          'SUCCESS',
          `SFX ABI Successfully added ${args.sfxId} for ${args.target}`,
        ),
      })
    }

    process.exit(0)
  } catch (e) {
    spinner.fail(colorLogMsg('ERROR', e))
  }
}
