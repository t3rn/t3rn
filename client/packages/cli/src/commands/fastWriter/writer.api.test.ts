import { beforeAll, describe } from "@jest/globals"
import { ApiPromise, cryptoWaitReady, Keyring, Sdk } from "@t3rn/sdk"
import { write_single_order } from "@/commands/fastWriter/index.ts"

// @TODO: node fetch is an esm module. We need to update and other tests to support ESM module testing
// for now lets skip this test

// jest.mock("node-fetch", () => ({
//     default: jest.fn(),
// }))

describe("fastWriter API tests against Vacuum @ localhost:9944", () => {
  let circuit: ApiPromise
  let circuitSDK: Sdk
  let signer: ReturnType<Keyring["addFromMnemonic"]>

  beforeAll(async () => {
    // Test Circuit Context connects to localhost:9944
    const signer_in = "//Alice"
    const endpoint = "ws://127.0.0.1:9944"

    await cryptoWaitReady()
    const keyring = new Keyring({ type: "sr25519" })
    signer = keyring.addFromMnemonic(signer_in)
    const sdk = new Sdk(endpoint, signer, false)
    circuit = await sdk.init()
    expect(circuit).toBeDefined()
    circuitSDK = sdk

    // sleep for 2 seconds to allow circuit to connect to localhost:9944
    await new Promise((resolve) => setTimeout(resolve, 2000))
  })

  beforeAll(() => {
    console.log = jest.fn()
  })

  test.only("given established Vacuum connection sends single order as local-local", async () => {
    const localLocalDest = "0x03030303"
    const asset = 1001

    const result = await write_single_order(
      circuitSDK,
      circuit,
      localLocalDest,
      asset,
      signer.address,
      100,
      asset,
      101,
      10,
      3,
    )

    // Give a little time for the transaction to be processed
    await new Promise((resolve) => setTimeout(resolve, 15000))

    expect(result).toBe({ result: "success" })
  })
})
