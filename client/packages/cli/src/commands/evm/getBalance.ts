import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { EvmGetBalanceSchema } from '@/schemas/evm.ts'

export const spinner = ora()

export const handleEvmGetBalanceCommand = async (
    _args: Args<
        | 'endpoint'
        | 'account'
    >,
) => {
    const args = validate(
        EvmGetBalanceSchema,
        {
            ..._args,
        },
        {
            configFileName: 'EVM get balance arguments',
        },
    )

    if (!args) {
        process.exit()
    }

    spinner.text = 'Getting EVM account balance... \n'
    spinner.start()

    try {
        const targetApi = await ApiPromise.create({
            provider: new WsProvider(args.endpoint),
        })
    } catch (e) {
        spinner.fail(`Getting EVM balance for account failed: ${e}`)
    }

    spinner.stop()
    process.exit(0)
}