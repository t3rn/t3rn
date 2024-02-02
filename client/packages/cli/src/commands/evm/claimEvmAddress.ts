import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import Web3 from 'web3'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { signAndSend } from '@/utils/xcm.ts'
import { EvmClaimAddressSchema } from '@/schemas/evm.ts'

export const spinner = ora()

export const handleEvmClaimAaddressCommand = async (
    _args: Args<
        | 'endpoint'
        | 'substrateSigner'
        | 'evmSigner'
    >,
) => {
    const args = validate(
        EvmClaimAddressSchema,
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
        const keyring = new Keyring({type: 'sr25519'})
        const signer =  keyring.addFromUri(args.substrateSigner)
        if ( args.evmSigner == "default" ) {
             await signAndSend(
                api.tx.accountMapping.claimDefaultAccount(),
                api,
                signer,
            )
            spinner.stopAndPersist({
                symbol: '🎉',
                text: colorLogMsg('SUCCESS', `Successfully claimed default EVM account for ${signer.address}`),
            })
        }
        else {
            const evmApi = await new Web3(args.endpoint)
            spinner.stopAndPersist({
                symbol: '\u2713',
                text: colorLogMsg('INFO', `Connected to EVM endpoint ${args.endpoint}`),
            })
            const evmAccount = await evmApi.eth.accounts.privateKeyToAccount(args.evmSigner, false)
            spinner.stopAndPersist({
                symbol: '\u2713',
                text: colorLogMsg('INFO', `The EVM address for the private key is ${evmAccount.address}`),
            })
            const message = "\x19Ethereum Signed Message :\n" + evmAccount.address.toString().toLowerCase()

            spinner.stopAndPersist({
                symbol: '\u2713',
                text: colorLogMsg('INFO', `EVM  message to be signed is:${message}`),
            })
            const evmSign = await evmApi.eth.accounts.sign(message, args.evmSigner)
            spinner.stopAndPersist({
                symbol: '\u2713',
                text: colorLogMsg('INFO', `The EVM signature for private key is ${evmSign.signature}`),
            })

            await signAndSend(
                api.tx.accountMapping.claimEthAccount(evmAccount.address, evmSign.signature),
                api,
                signer,
            )
            spinner.stopAndPersist({
                symbol: '🎉',
                text: colorLogMsg('SUCCESS', `${evmAccount.address} successfully claimed for ${signer.address}`),
            })
        }
    }
    catch (e) {
        spinner.fail(`Claiming EVM account failed: ${e}`)
    }

    spinner.stop()
    process.exit(0)
}