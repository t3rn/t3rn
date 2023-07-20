import { Sdk, ApiPromise, Keyring } from "@t3rn/sdk"
import { getConfig } from "./config.ts"

export type CircuitContext = {
  circuit: ApiPromise
  sdk: Sdk
  signer: ReturnType<Keyring["addFromMnemonic"]>
}

export const createCircuitContext = async (
  exportMode = false,
): Promise<CircuitContext> => {
  const config = getConfig()

  const keyring = new Keyring({ type: "sr25519" })
  const signer =
    process.env.CIRCUIT_KEY === undefined
      ? keyring.addFromUri("//Alice")
      : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk(
    config.circuit.ws,
    signer,
    exportMode,
  )
  const circuit = await sdk.init()

  return {
    circuit,
    sdk,
    signer,
  }
}
