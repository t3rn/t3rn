import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import Web3 from 'web3'
import { EvmTransferSchema } from '@/schemas/evm.ts'

export const spinner = ora()

export const handleEvmTransferCommand = async (
    _args: Args<
        | 'endpoint'
        | 'sender'
        | 'signature'
        | 'receiver'
        | 'amount'
    >,
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

    try {
        const evmApi = new Web3(args.endpoint)
        const transaction = {
            from: args.sender,
            to: args.receiver,
            value: args.amount.toString(),
            gas: 21000
        }
        const signPromise = evmApi.eth.accounts.signTransaction(transaction, args.signature)

        signPromise.then((signedTx) => {
            evmApi.eth.sendSignedTransaction(signedTx.raw || signedTx.rawTransaction)
                .on('error', err => {spinner.fail(colorLogMsg('ERROR', err))})
                .on('sending', transactionToBeSent => console.log(`Sending transaction... \n ${transactionToBeSent}`))
                .on('receipt', transactionReceipt => spinner.stopAndPersist(
                    {
                        symbol: '🎉',
                        text: colorLogMsg('SUCCESS', `EVM transaction completed!\n ${transactionReceipt}`),
                    }
                ))
        }).catch((err) => {
            spinner.fail(`EVM transfer failed: ${err}`)
        })
    } catch (e) {
        spinner.fail(`EVM transfer failed: ${e}`)
    }

    spinner.stop()
    process.exit(0)
}