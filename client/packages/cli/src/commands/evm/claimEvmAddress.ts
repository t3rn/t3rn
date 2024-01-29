import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { signAndSend } from '@/utils/xcm.ts'
import { EvmClaimAddressSchema } from '@/schemas/evm.ts'

export const spinner = ora()

export const handleEvmClaimAaddressCommand = async (
    _args: Args<
        | 'endpoint'
        | 'substrateSignature'
        | 'evmAddress'
        | 'evmSignature'
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
        const signer =  keyring.addFromUri(args.substrateSignature)
        if ( args.evmAddress == "default" ) {
             await signAndSend(
                api.tx.accountMapping.claimDefaultAccount(),
                api,
                signer,
            )
            spinner.stopAndPersist({
                symbol: '🎉',
                text: colorLogMsg('SUCCESS', `Successfully claimed default EVM account!`),
            })
        }
        else {
            await signAndSend(
                api.tx.accountMapping.claimEthAccount(args.evmAddress, args.evmSignature),
                api,
                signer,
            )
            spinner.stopAndPersist({
                symbol: '🎉',
                text: colorLogMsg('SUCCESS', `${args.evmAddress} successfully claimed!`),
            })
        }
    }
    catch (e) {
        spinner.fail(`Claiming EVM account failed: ${e}`)
    }

    spinner.stop()
    process.exit(0)
}