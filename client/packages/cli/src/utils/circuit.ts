import { Sdk, ApiPromise, Keyring } from "@t3rn/sdk"

export type CircuitContext = {
  circuit: ApiPromise
  sdk: Sdk
  signer: ReturnType<Keyring["addFromMnemonic"]>
}

export const createCircuitContext = async (
  exportMode = false
): Promise<CircuitContext> => {
  const keyring = new Keyring({ type: "sr25519" })
  const signer =
    process.env.CIRCUIT_KEY === undefined
      ? keyring.addFromUri("//Alice")
      : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk(
    process.env.WS_CIRCUIT_ENDPOINT || "ws://127.0.0.1:9944",
    signer,
    exportMode
  )
  const circuit = await sdk.init()

  return {
    circuit,
    sdk,
    signer,
  }
}
