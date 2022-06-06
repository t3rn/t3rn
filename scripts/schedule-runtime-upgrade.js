const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api")
const { readFile, lstat } = require("fs/promises")

async function assertEnv() {
  const stat = await lstat(process.env.WASM)
  if (!stat.isFile()) {
    throw Error(`invalid env var WASM ${process.env.WASM}`)
  }
  if (
    !process.env.PROVIDER.startsWith("ws://") &&
    !process.env.PROVIDER.startsWith("wss://")
  ) {
    throw Error(`invalid env var PROVIDER ${process.env.PROVIDER}`)
  }
  if (!process.env.SUDO) {
    throw Error(`invalid env var SUDO ${process.env.SUDO}`)
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

async function scheduleRuntimeUpgrade(circuit, sudo, code, when) {
  const maybePeriodic = null
  const schedulePriority = 3

  const enactAuthorizedUpgradeCall =
    circuit.tx.parachainSystem.enactAuthorizedUpgrade(code)

  const scheduleCall = circuit.tx.scheduler.schedule(
    when,
    maybePeriodic,
    schedulePriority,
    {
      value: enactAuthorizedUpgradeCall,
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
  const code = await readFile(process.env.WASM)
  const when = BigInt(process.env.WHEN)
  const head = await getBestHead(circuit)

  if (when < head + 3n) {
    throw Error(`when too low => reschedule at a later block`)
  }

  const hash = await scheduleRuntimeUpgrade(circuit, sudo, code, when)

  console.debug(`runtime upgrade scheduled for block ${when} (${hash})`)

  circuit.disconnect()
}

main()
