const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api")
const { readFile } = require("fs/promises")
const { lstatSync } = require("fs")

if (!lstatSync(process.env.WASM).isFile()) {
  throw Error(`invalid env var WASM ${process.env.WASM}`)
}

if (!process.env.PROVIDER.startsWith("ws")) {
  throw Error(`invalid env var PROVIDER ${process.env.PROVIDER}`)
}

if (!process.env.SUDO) {
  throw Error(`invalid env var SUDO ${process.env.SUDO}`)
}

if (!/^\d+$/.test(process.env.WHEN)) {
  throw Error(`invalid env var WHEN ${process.env.WHEN}`)
}

async function main() {
  const circuit = await ApiPromise.create({
    provider: new WsProvider(process.env.PROVIDER)
  })

  const sudo = new Keyring({ type: "sr25519" }).addFromMnemonic(
    process.env.SUDO
  )

  const wasm = await readFile(process.env.WASM)
  console.debug("wasm buf", wasm)
  const enactAuthorizedUpgrade =
    circuit.tx.parachainSystem.enactAuthorizedUpgrade(wasm)

  const when = Number(process.env.WHEN)

  const head = await Promise.race([
    new Promise(async resolve => {
      const unsub = await circuit.rpc.chain.subscribeNewHeads(header => {
        unsub()
        resolve(header.number.toNumber())
      })
    }),
    new Promise((_, reject) =>
      setTimeout(() => reject(Error("timeout fetching chain head")), 12000)
    ),
  ])

  if (when < head + 3) {
    throw Error(`when too low ${when} => reschedule at a later block`)
  }
  console.debug("enactAuthorizedUpgrade call", enactAuthorizedUpgrade)
  await new Promise(async (resolve, reject) => {
    await circuit.tx.scheduler  
      .schedule(when, null, 1, { value: enactAuthorizedUpgrade })
      .signAndSend(sudo, result => {
        console.debug(result.toHuman())
        if (result.isError) {
          reject(Error("result is error"))
        }
        if (result.isFinalized) {
          console.debug(`runtime upgrade scheduled for block ${when} - ${result.status.asFinalized}`)
          resolve(undefined)
        }
      })
  })
}

main()
