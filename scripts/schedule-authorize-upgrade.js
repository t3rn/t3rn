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
  if (!/^\d+$/.test(process.env.WHEN)) {
    throw Error(`invalid env var WHEN ${process.env.WHEN}`)
  }
}

async function getBestHead(circuit) {
  return Promise.race([
    new Promise(async resolve => {
      const unsub = await circuit.rpc.chain.subscribeNewHeads(header => {
        unsub()
        resolve(BigInt(header.number.toString()))
      })
    }),
    new Promise((_, reject) =>
      setTimeout(() => reject(Error("timeout fetching chain head")), 12000)
    ),
  ])
}

async function scheduleAuthorizeUpgrade(circuit, sudo, hash, when) {
  const maybePeriodic = null
  const schedulePriority = 0

  const authorizeUpgradeCall =
    circuit.tx.parachainSystem.authorizeUpgrade(hash)

  const scheduleCall = circuit.tx.scheduler.schedule(
    when,
    maybePeriodic,
    schedulePriority,
    {
      value: authorizeUpgradeCall,
    }
  )

  return circuit.tx.sudo.sudo(scheduleCall).signAndSend(sudo)
}

async function main() {
  await assertEnv()

  const provider = new WsProvider(process.env.PROVIDER)
  const circuit = await ApiPromise.create({ provider })
  const keyring = new Keyring({ type: "sr25519" })
  const sudo = keyring.addFromMnemonic(process.env.SUDO)
//   const code = await readFile(process.env.WASM)
  const hash = process.env.HASH
  const when = BigInt(process.env.WHEN)
  const head = await getBestHead(circuit)

  if (when < head + 5n) { // ~1m assuming 12s block time
    throw Error(`when too low => reschedule at a later block`)
  }

  const xt = await scheduleAuthorizeUpgrade(circuit, sudo, hash, when)

  circuit.disconnect()
}

main()
