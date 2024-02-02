import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import Web3 from 'web3'
import { EvmGetBalanceSchema } from '@/schemas/evm.ts'

export const spinner = ora()

export const handleEvmGetBalanceCommand = async (
  _args: Args<'endpoint' | 'account'>,
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
    const evmApi = new Web3(args.endpoint)
    const balance = await evmApi.eth.getBalance(args.account)
    await new Promise((f) => setTimeout(f, 10000))
    spinner.stopAndPersist({
      symbol: '🎉',
      text: colorLogMsg('SUCCESS', `${args.account} balance: ${balance}`),
    })
  } catch (e) {
    spinner.fail(`Getting EVM balance for account failed: ${e}`)
  }

  spinner.stop()
  process.exit(0)
}
