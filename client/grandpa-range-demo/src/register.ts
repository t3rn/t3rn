import { Bytes } from '@polkadot/types'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { createTestPairs } from '@polkadot/keyring/testingPairs'

const keyring = createTestPairs({ type: 'sr25519' })

export default async function registerKusamaGateway(circuit: ApiPromise) {
  const kusama = await ApiPromise.create({
    provider: new WsProvider(process.env.KUSAMA_RPC as string),
  })

  const [currentHeader, metadata, runtimeVersion, genesisHash] =
    await Promise.all([
      kusama.rpc.chain.getHeader(),
      kusama.runtimeMetadata,
      kusama.runtimeVersion,
      kusama.genesisHash,
    ])

  const atGenesis = await kusama.at(genesisHash)
  const initialAuthorities = await atGenesis.query.session.validators()

  await kusama.disconnect()

  const registerGateway = circuit.tx.circuitPortal.registerGateway(
    process.env.KUSAMA_RPC as string,
    Buffer.from(process.env.GATEWAY_ID as string, 'utf8'),
    circuit.createType('GatewayABIConfig', [
      circuit.createType('u16', 32),
      circuit.createType('u16', 32),
      circuit.createType('HasherAlgo', 'Blake2'),
      circuit.createType('CryptoAlgo', 'Sr25519'),
      circuit.createType('u16', 32),
      circuit.createType('u16', 32),
      circuit.createType('u16', 12),
      circuit.createType('Vec<StructDecl>', []),
    ]),
    circuit.createType('GatewayVendor', 'Substrate'),
    circuit.createType('GatewayType', { ProgrammableExternal: 1 }),
    circuit.createType('GatewayGenesisConfig', [
      circuit.createType('Option<Bytes>', metadata.asV14.pallets.toHex()),
      metadata.asV14.extrinsic.version,
      genesisHash,
    ]),
    circuit.createType('GatewaySysProps', [
      circuit.createType('u16', 2),
      circuit.createType('Bytes', 'KSM'),
      circuit.createType('u8', 12),
    ]),
    circuit.createType('Bytes', currentHeader.toHex()),
    circuit.createType('Option<Vec<AccountId>>', initialAuthorities),
    circuit.createType('Vec<AllowedSideEffect>', ['transfer', 'get_storage'])
  )

  return new Promise((resolve, reject) => {
    return circuit.tx.sudo
      .sudo(registerGateway)
      .signAndSend(keyring.alice, result => {
        if (result.isError) {
          reject(new Error('submitting registerGateway failed'))
        } else if (result.isInBlock) {
          resolve(undefined)
        }
      })
  })
}
