import ora from 'ora'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { signAndSend, signAndSendSafe } from '@/utils/xcm.ts'
import { EvmClaimAddressSchema } from '@/schemas/evm.ts'
import { ethers, Wallet } from 'ethers'

export const spinner = ora()

export const handleEvmClaimAaddressCommand = async (
  _args: Args<'endpoint' | 'substrateSigner' | 'evmSigner'>,
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
    const keyring = new Keyring({ type: 'sr25519' })
    const signer = keyring.addFromUri(args.substrateSigner)
    if (args.evmSigner == 'default') {
      await signAndSendSafe(
        api.tx.accountMapping.claimDefaultAccount(),
        api,
        signer,
        spinner,
      )
      spinner.stopAndPersist({
        symbol: '🎉',
        text: colorLogMsg(
          'SUCCESS',
          `Successfully claimed default EVM account for ${signer.address}`,
        ),
      })
    } else {
      const wallet = new Wallet(args.evmSigner)
      const evmAccount = wallet.address
      const evmAccountBytes = Buffer.from(evmAccount.slice(2), 'hex')
      const evmAccountDigest = ethers.keccak256(evmAccountBytes)
      const evmAccountDigestBytes = Buffer.from(
        evmAccountDigest.slice(2),
        'hex',
      )

      spinner.stopAndPersist({
        symbol: '\u2713',
        text: colorLogMsg('INFO', `Connected to EVM endpoint ${args.endpoint}`),
      })
      spinner.stopAndPersist({
        symbol: '\u2713',
        text: colorLogMsg(
          'INFO',
          `The EVM address for the private key is ${evmAccount}`,
        ),
      })

      const signature = await wallet.signMessage(evmAccountDigestBytes)

      const recoveredAddress = ethers.verifyMessage(
        evmAccountDigestBytes,
        signature,
      )

      spinner.stopAndPersist({
        symbol: '\u2713',
        text: colorLogMsg(
          'INFO',
          `The recovered address is ${recoveredAddress}`,
        ),
      })

      if (recoveredAddress !== evmAccount) {
        spinner.fail(
          `The recovered address ${recoveredAddress} does not match the EVM address ${evmAccount}`,
        )
        process.exit(1)
      }

      spinner.stopAndPersist({
        symbol: '\u2713',
        text: colorLogMsg(
          'INFO',
          `EVM  message to be signed is:${evmAccountDigest}`,
        ),
      })
      // const evmSign = await evmApi.eth.accounts.sign(message, args.evmSigner)
      spinner.stopAndPersist({
        symbol: '\u2713',
        text: colorLogMsg(
          'INFO',
          `The EVM signature for private key is ${signature}`,
        ),
      })

      await signAndSendSafe(
        api.tx.accountMapping.claimEthAccount(evmAccount, signature),
        api,
        signer,
        spinner,
      )

      // Sleep for at least 5 seconds to allow the transaction to be included in the block
      await new Promise((resolve) => setTimeout(resolve, 5000))

      spinner.stopAndPersist({
        symbol: '🎉',
        text: colorLogMsg(
          'SUCCESS',
          `${evmAccount} successfully claimed for ${signer.address}`,
        ),
      })
    }
  } catch (e) {
    spinner.fail(`Claiming EVM account failed: ${e}`)
  }

  spinner.stop()
  process.exit(0)
}
