const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api")

async function assertEnv() {
  if (
    !process.env.PROVIDER.startsWith("ws://") &&
    !process.env.PROVIDER.startsWith("wss://")
  ) {
    throw Error(`invalid env var PROVIDER ${process.env.PROVIDER}`)
  }
  if (!process.env.SUDO) {
    throw Error(`invalid env var SUDO ${process.env.SUDO}`)
  }
  if (!/^0x[0-9a-f]{64}$/.test(process.env.HASH)) {
    throw Error(`invalid env var HASH ${process.env.HASH}`)
  }
  if (!/^\d+$/.test(process.env.AFTER)) {
    throw Error(`invalid env var AFTER ${process.env.AFTER}`)
  }
}

async function main() {
  await assertEnv()

  const provider = new WsProvider(process.env.PROVIDER)
  const circuit = await ApiPromise.create({ provider })
  const keyring = new Keyring({ type: "sr25519" })
  const sudo = keyring.addFromMnemonic(process.env.SUDO)
  const hash = process.env.HASH
  const after = BigInt(process.env.AFTER)

  const maybePeriodic = null
  const schedulePriority = 0

  const authorizeUpgradeCall =
    circuit.tx.parachainSystem.authorizeUpgrade(hash)

  const scheduleCall = circuit.tx.scheduler.scheduleAfter(
    after,
    maybePeriodic,
    schedulePriority,
    {
      value: authorizeUpgradeCall,
    }
  )

  //  await circuit.tx.balances.transfer(freshAccount.address, 1n * TRN)
  await circuit.tx.sudo.sudo(scheduleCall).signAndSend(sudo)

  circuit.disconnect()
}

main()