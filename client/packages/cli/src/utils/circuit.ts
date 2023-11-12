import { ApiPromise, Keyring, Sdk } from '@t3rn/sdk'
import ora from 'ora'
import { config } from '@/config/config.ts'
import { colorLogMsg } from '@/utils/log.ts'

const spinner = ora()

export type CircuitContext = {
  circuit: ApiPromise
  sdk: Sdk
  signer: ReturnType<Keyring['addFromMnemonic']>
  endpoint: string
}

export const createCircuitContext = async (
  exportMode = false,
): Promise<CircuitContext> => {
  const keyring: Keyring = new Keyring({ type: 'sr25519' })
  const signerKey = config().circuit.signerKey
  let signer

  if (signerKey === undefined || signerKey.length === 0) {
    spinner.info(
      colorLogMsg('INFO', `Init Circuit signer: Using default '//Alice' user`),
    )
    signer = keyring.addFromUri('//Alice')
  } else {
    spinner.info(
      colorLogMsg(
        'INFO',
        `Init Circuit signer: Using CIRCUIT_SIGNER_KEY from .env`,
      ),
    )
    signer = keyring.addFromMnemonic(signerKey)
  }

  const sdk = new Sdk(config().circuit.rpc.ws, signer, exportMode)
  const circuit = await sdk.init()

  return {
    circuit,
    sdk,
    signer,
    endpoint: config().circuit.rpc.ws,
  }
}
