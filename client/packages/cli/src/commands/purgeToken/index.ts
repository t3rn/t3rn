import { Args } from '@/types.ts'
import { log } from '@/utils/log.ts'
import '@t3rn/types'
import ora from 'ora'
import { colorLogMsg } from '@/utils/log.ts'
import { createCircuitContext } from '@/utils/circuit.ts'

export const spinner = ora()

export const handlePurgeTokenCommand = async (
  args: Args<'gateway' | 'export'>,
  options: { [key: string]: any },
) => {
  log('INFO', `Purging ${args} token...`)

  if (!args) {
    log('ERROR', 'No token ID provided!')
    process.exit(1)
  }

  const { circuit, sdk, endpoint, signer } = await createCircuitContext()

  if (
    ![
      'ws://localhost:9944',
      'ws://0.0.0.0:9944',
      'ws://127.0.0.1:9944',
    ].includes(endpoint) &&
    !options.force
  ) {
    log(
      'ERROR',
      `Circuit endpoint is not localhost:9944. We don't want to purge live token! Aborting.`,
    )
    process.exit(1)
  }

  spinner.start()
  try {
    await sdk.circuit.tx.signAndSendSafe(
      sdk.circuit.tx.createSudo(circuit.tx.xdns.purgeTokenRecord(args)),
    )

    spinner.succeed(colorLogMsg('SUCCESS', `Token purged`))
    spinner.stopAndPersist({
      symbol: '🎉',
      text: colorLogMsg('SUCCESS', `Token purged`),
    })
    spinner.stop()

    process.exit(0)
  } catch (error) {
    spinner.fail(colorLogMsg('ERROR', `Token purge failed! ${error}`))
    process.exit(1)
  }
}
