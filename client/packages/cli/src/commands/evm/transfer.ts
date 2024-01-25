import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { EvmTransferSchema } from '@/schemas/evm.ts'

export const spinner = ora()

export const handleEvmTransferCommand = async (
    _args: Args<
        | 'endpoint'
        | 'sender'
        | 'receiver'
        | 'amount'
    >,
) => {
    const args = validate(
        EvmTransferSchema,
        {
            ..._args,
            amount: parseFloat(_args?.amount),
        },
        {
            configFileName: 'EVM balance transfer arguments',
        },
    )

    if (!args) {
        process.exit()
    }

    spinner.text = 'EVM balance transfer... \n'
    spinner.start()

    try {
        const targetApi = await ApiPromise.create({
            provider: new WsProvider(args.endpoint),
        })
    } catch (e) {
        spinner.fail(`EVM transfer failed: ${e}`)
    }

    spinner.stop()
    process.exit(0)
}