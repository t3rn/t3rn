import { ApiPromise, Keyring } from "@polkadot/api"
import { Sdk } from "@t3rn/sdk"

export type CircuitContext = {
  circuit: ApiPromise
  sdk: Sdk
  signer: ReturnType<Keyring["addFromMnemonic"]>
}

export const createCircuitContext = async (): Promise<CircuitContext> => {
  const keyring = new Keyring({ type: "sr25519" })
  const signer =
    process.env.CIRCUIT_KEY === undefined
      ? keyring.addFromUri("//Alice")
      : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk(process.env.CIRCUIT_WS || "ws://127.0.0.1:9944", signer)
  const circuit = await sdk.init()

  return {
    circuit,
    sdk,
    signer,
  }
}
