let { Keyring } = require("@polkadot/api")
let { cryptoWaitReady } = require("@polkadot/util-crypto")
const assert = require("assert")

async function run(nodeName, networkInfo, args) {
  await cryptoWaitReady()

  let keyring = new Keyring({ type: "sr25519" })
  let alice = keyring.addFromUri("//Alice")
  let bob = keyring.addFromUri("//Bob")

  const { wsUri, userDefinedTypes } = networkInfo.nodesByName[nodeName]
  const api = await zombie.connect(wsUri, userDefinedTypes)

  console.log("ALICE", alice.address)

  // force_create and set_metadata an asset

  // mint some asset balance to bob

  // have bob do a tx paying with the non-native asset
}

module.exports = { run }
