let { Keyring } = require("@polkadot/api")
// let { WsProvider, ApiPromise } = require("@polkadot/api")
let { cryptoWaitReady } = require("@polkadot/util-crypto")
let assert = require("assert")

async function run(nodeName, networkInfo, args) {
  await cryptoWaitReady()

  let keyring = new Keyring({ type: "sr25519" })
  let alice = keyring.addFromUri("//Alice")
  let bob = keyring.addFromUri("//Bob")

  let { wsUri, userDefinedTypes } = networkInfo.nodesByName[nodeName]
  let api = await zombie.connect(wsUri, userDefinedTypes)
  // let api = await ApiPromise.create({ provider: new WsProvider("ws://localhost:9930") })

  // force_create and set_metadata an asset
  let name = "Asset"
  let symbol = "ASS"
  let decimals = 12
  let id = 1
  let owner = alice.address
  let isSufficient = true
  let minBalance = 1
  await api.tx.assets.forceCreate(id, owner, isSufficient, minBalance).signAndSend(alice)
  // await api.tx.assets.setMetadata(id, name, symbol, decimals).signAndSend(alice)

  // mint some asset balance to bob

  // have bob do a tx paying with the non-native asset
}

module.exports = { run }
