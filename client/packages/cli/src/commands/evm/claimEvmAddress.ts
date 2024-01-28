import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { EvmClaimAddressSchema } from '@/schemas/evm.ts'

export const spinner = ora()

export const handleEvmAccountCommand = async (
    _args: Args<
        | 'endpoint'
        | 'substrateSignature'
        | 'evmSignature'
    >,
) => {
    const args = validate(
        EvmTransferSchema,
        {
            ..._args,
        },
        {
            configFileName: 'Claim EVM account arguments',
        },
    )

    if (!args) {
        process.exit()
    }

    spinner.text = 'Claiming EVM account... \n'
    spinner.start()
    try {
        const api = await ApiPromise.create({
            provider: new WsProvider(args.endpoint),
        })

    }
    } catch (e) {
        spinner.fail(`Claiming EVM account failed: ${e}`)
    }

    spinner.stop()
    process.exit(0)
}