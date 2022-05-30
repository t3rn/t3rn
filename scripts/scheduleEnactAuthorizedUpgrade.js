const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api")

async function main() {
  const circuit = await ApiPromise.create({
    provider: new WsProvider(process.env.PROVIDER),
  })

  const when = BigInt(process.env.WHEN)

  const enactAuthorizedUpgrade = circuit.tx.parachainSystem.enactAuthorizedUpgrade(
    process.env.WASM_RUNTIME
  )

  const sudo = new Keyring({ type: "sr25519" }).addFromMnemonic(
    process.env.SUDO_SECRET
  )

  const txHash = await circuit.tx.scheduler
    .schedule(when, null, 1, enactAuthorizedUpgrade)
    .signAndSend(sudo)

  console.debug(`runtime upgrade scheduled for block ${when} - ${txHash}`)
}

main()
