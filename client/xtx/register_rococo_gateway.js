const { ApiPromise, Keyring, WsProvider } = require("@polkadot/api")
const { Bytes } = require("@polkadot/types")
const { TypeRegistry, createType } = require('@polkadot/types');
const { promisify } = require("util")
const { exec: _exec } = require("child_process")
const types = require("./types.json")

const ROCOCO_CHAIN_ID = [114, 111, 99, 111]

async function sleep(ms) {
  return new Promise(res => setTimeout(res, ms))
}

const registry = new TypeRegistry();
const type = { type: 'GrandpaJustification<Header>' }
const grandpaDecode = (data) => {
    registry.register(type);
    const res = createType(registry, type.type, data.toHex())


    return [res.commit.targetNumber.toNumber(), getAuthorities(res.commit.precommits)]
}

const getAuthorities = (precommits) => {
    let res = [];
    for(let i = 0; i < precommits.length; i++) {
        res.push(precommits[i].id)
    }
    return res
}

function createGatewayABIConfig(
  api,
  hash_size,
  address_length,
  block_number_type_size,
  decimals,
  crypto,
  hasher
) {
  return api.createType("GatewayABIConfig", [
    api.createType("u16", block_number_type_size),
    api.createType("u16", hash_size),
    api.createType("HasherAlgo", hasher),
    api.createType("CryptoAlgo", crypto),
    api.createType("u16", address_length),
    api.createType("u16", 16),
    api.createType("u16", decimals),
    api.createType("Vec<StructDecl>", []),
  ])
}

function createGatewayGenesisConfig(metadata, genesisHash, circuitApi) {
  return circuitApi.createType("GatewayGenesisConfig", [
    circuitApi.createType("Option<Bytes>", metadata.asV14.pallets.toHex()),
    metadata.asV14.extrinsic.version,
    genesisHash,
  ])
}

function createGatewaySysProps(api, ss58Format, tokenSymbol, tokenDecimals) {
  return api.createType("GatewaySysProps", [
    api.createType("u16", ss58Format),
    api.createType("Bytes", new Bytes(api.registry, tokenSymbol)),
    api.createType("u8", tokenDecimals),
  ])
}

async function triggerRegister(circuit, params) {
  const {
    authorities,
    url,
    authoritySetId,
    rococoRegistrationHeader,
    metadata,
    genesisHash,
    target,
  } = params

  const registerGateway = await circuit.tx.circuitPortal.registerGateway(
    url,
    String.fromCharCode(...target),
    null,
    createGatewayABIConfig(circuit, 32, 32, 32, 12, "Sr25519", "Blake2"),
    //GatewayVendor: 'Substrate' as rococo is substrate-based
    circuit.createType("GatewayVendor", "Substrate"),
    //GatewayType: we connect as a ProgrammableExternal
    circuit.createType("GatewayType", { ProgrammableExternal: 1 }),
    createGatewayGenesisConfig(metadata, genesisHash, circuit),
    createGatewaySysProps(circuit, 42, "ROC", 12), // GatewaySysProps
    //Initial rococo, acts as gateway activation point
    circuit.createType("Bytes", rococoRegistrationHeader.toHex()),
    //List of current rococo authorities
    circuit.createType("Option<Vec<AccountId>>", authorities),
    circuit.createType("Option<SetId>", authoritySetId),
    circuit.createType("Option<MultiLocation>", "None"),
    //SideEffects that are allowed on gateway instance
    circuit.createType("Vec<AllowedSideEffect>", ["tran"]) // allowed side effects
  )

  const keyring = new Keyring({ type: "sr25519", ss58Format: 60 })
  const alice = keyring.addFromUri("//Alice")
  return circuit.tx.sudo.sudo(registerGateway).signAndSend(alice)
}

async function setOperational(circuit, target) {
  const setOperational = circuit.tx.multiFinalityVerifierDefault.setOperational(
    true,
    target
  )
  const keyring = new Keyring({ type: "sr25519", ss58Format: 60 })
  const alice = keyring.addFromUri("//Alice")

  return new Promise(async (resolve, reject) => {
    await circuit.tx.sudo.sudo(setOperational).signAndSend(alice, result => {
      if (result.isError) {
        reject("submitting setOperational failed")
      } else if (result.isInBlock) {
        console.log(`gateway ${target.toString()} operational`)
        resolve(undefined)
      }
    })
  })
}

async function register(circuit, target) {
  const url = "wss://rococo-rpc.polkadot.io"
  const rococoProvider = new WsProvider(url)
  const api = await ApiPromise.create({ provider: rococoProvider })

  const [metadata, genesisHash] = await Promise.all([
    await api.runtimeMetadata,
    await api.genesisHash,
  ])

  return new Promise(async (resolve, _reject) => {
    let unsub = await api.rpc.grandpa.subscribeJustifications(
      async justification => {
        unsub()

        const [ blockNumber, authorities ] = grandpaDecode(justification)
        console.log("justification block number", blockNumber)

        const rococoRegistrationHeader = await api.rpc.chain.getHeader(
          await api.rpc.chain.getBlockHash(blockNumber)
        )
        const authoritySetId = await api.query.grandpa.currentSetId()

        await triggerRegister(circuit, {
          authorities,
          url,
          authoritySetId,
          rococoRegistrationHeader,
          metadata,
          genesisHash,
          target,
          api,
        })

        return resolve()
      }
    )
  })
}

async function registered(circuit) {
  return Promise.race([
    new Promise((resolve, _reject) => {
      circuit.query.system.events(notifications => {
        notifications.forEach(notification => {
          if (notification.event.method === "NewGatewayRegistered") {
            resolve()
          }
        })
      })
    }),
    sleep(12000).then(() => {
      throw Error("gateway registration timeout")
    }),
  ])
}

async function main() {
  const circuit = await ApiPromise.create({
    provider: new WsProvider("ws://127.0.0.1:9944"),
    types,
  })
  await register(circuit, ROCOCO_CHAIN_ID)
  await registered(circuit)
  await setOperational(circuit, ROCOCO_CHAIN_ID)
  console.log("roco gtwy registered and operational")
  circuit.disconnect()
  process.exit(0)
}

main()
