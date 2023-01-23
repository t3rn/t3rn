let assert = require("assert")

async function run(nodeName, networkInfo, args) {
  await zombie.util.cryptoWaitReady();

  let { wsUri, userDefinedTypes } = networkInfo.nodesByName[nodeName]
  let api = await zombie.connect(wsUri, userDefinedTypes)

  let keyring = new zombie.Keyring({ type: "sr25519" });
  let alice = keyring.addFromUri("//Alice")
  let bob = keyring.addFromUri("//Bob")

  // force_create and set_metadata an asset
  let name = "Asset"
  let symbol = "ASS"
  let decimals = 12
  let id = 1n
  let owner = alice.address
  let isSufficient = true
  let minBalance = 1n
  console.log("ALICE & BOB", alice.address, bob.address)
  await api.tx.assets.forceCreate(id, owner, isSufficient, minBalance).signAndSend(alice)
  await api.tx.assets.setMetadata(id, name, symbol, decimals).signAndSend(alice)

  // mint some asset balance to bob
  console.log("MADE IT !!!")

  // have bob do a tx paying with the non-native asset
}

module.exports = { run }
