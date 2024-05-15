import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import { ethers, Wallet, JsonRpcProvider } from 'ethers'
import { EvmTransferSchema } from '@/schemas/evm.ts'
export const spinner = ora()

export const handleEvmTransferCommand = async (
  _args: Args<'endpoint' | 'sender' | 'receiver' | 'amount'>,
) => {
  const args = validate(
    EvmTransferSchema,
    {
      ..._args,
      amount: parseInt(_args?.amount),
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
  const provider = new JsonRpcProvider(args.endpoint)
  const walletWithProvider = new Wallet(args.sender, provider)
  await walletWithProvider.sendTransaction({
    to: args.receiver,
    value: parseInt(args.amount).toString(),
  }).then((tx) => {
    spinner.stopAndPersist({
      symbol: '\u2713',
      text: colorLogMsg('INFO', `Transaction hash: ${tx.hash}`),
    })
  }).catch((e) => {
      spinner.fail(`Failed to transfer ${args.amount} wei to ${args.receiver}: ${e}`)
      process.exit(1)
  })
  await new Promise((resolve) => setTimeout(resolve, 5000))
  spinner.stopAndPersist({
    symbol: '🎉',
    text: colorLogMsg(
        'SUCCESS',
        `${walletWithProvider.address} successfully sent ${args.amount} wei to ${args.receiver}`,
    ),
  })
  spinner.stop()
  process.exit(0)
}