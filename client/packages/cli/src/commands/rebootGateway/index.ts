import { Args } from '@/types.ts'
import { log } from '@/utils/log.ts'
import '@t3rn/types'
import ora from 'ora'
import { colorLogMsg } from '@/utils/log.ts'
import { createCircuitContext } from '@/utils/circuit.ts'
import {
  //@ts-ignore - TS doesn't know about the type
  T3rnPrimitivesGatewayVendor,
} from '@polkadot/types/lookup'
import { createType } from '@t3rn/types'

export const spinner = ora()

export const handleRebootCommand = async (args: Args<'vendor' | 'export'>) => {
  log('INFO', `Rebooting ${args} gateway...`)

  if (!args) {
    log('ERROR', 'No vendor provided!')
    process.exit(1)
  }

  const { circuit, sdk, endpoint } = await createCircuitContext()

  if (
    ![
      'ws://localhost:9944',
      'ws://0.0.0.0:9944',
      'ws://127.0.0.1:9944',
    ].includes(endpoint)
  ) {
    log(
      'ERROR',
      `Circuit endpoint is not localhost:9944. We don't want to reboot live gateway! Aborting.`,
    )
    process.exit(1)
  }

  spinner.start()
  try {
    // @ts-ignore - TS doesn't know about the type
    const verificationVendor: T3rnPrimitivesGatewayVendor = createType(
      'T3rnPrimitivesGatewayVendor',
      args.toLowerCase() as never,
    )
    log('INFO', verificationVendor)
    await sdk.circuit.tx.signAndSendSafe(
      sdk.circuit.tx.createSudo(
        circuit.tx.xdns.rebootSelfGateway(verificationVendor.toJSON()),
      ),
    )

    spinner.succeed(colorLogMsg('SUCCESS', `Gateway rebooted`))
    spinner.stopAndPersist({
      symbol: '🎉',
      text: colorLogMsg('SUCCESS', `Rebooted!`),
    })
    spinner.stop()

    process.exit(0)
  } catch (error) {
    spinner.fail(colorLogMsg('ERROR', `Gateway reboot failed! ${error}`))
    process.exit(1)
  }
}
