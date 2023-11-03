import { ApiPromise, Keyring, Sdk } from '@t3rn/sdk'

export type CircuitContext = {
  circuit: ApiPromise
  sdk: Sdk
  signer: ReturnType<Keyring['addFromMnemonic']>
  endpoint: string
}

export const createCircuitContext = async (
  exportMode = false,
): Promise<CircuitContext> => {
  const keyring = new Keyring({ type: 'sr25519' })
  const signer =
    process.env.CIRCUIT_SIGNER_KEY === undefined
      ? keyring.addFromUri('//Alice')
      : keyring.addFromMnemonic(process.env.CIRCUIT_SIGNER_KEY)
  const circuitWs = process.env.CIRCUIT_WS_ENDPOINT || 'ws://127.0.0.1:9944'
  const sdk = new Sdk(circuitWs, signer, exportMode)
  const circuit = await sdk.init()

  return {
    circuit,
    sdk,
    signer,
    endpoint: circuitWs,
  }
}
