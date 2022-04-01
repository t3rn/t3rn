import { ApiPromise, WsProvider } from '@polkadot/api'
import { createTestPairs } from '@polkadot/keyring/testingPairs'
import { JustificationNotification } from '@polkadot/types/interfaces'
import { tmpdir } from 'os'
import { join } from 'path'
import { writeFile } from 'fs/promises'
import { exec, formatEvents } from './util'

const keyring = createTestPairs({ type: 'sr25519' })

export default async function registerKusamaGateway(
  circuit: ApiPromise,
  log = console.log
) {
  const kusama = await ApiPromise.create({
    provider: new WsProvider(process.env.KUSAMA_RPC as string),
  })

  const justification: JustificationNotification = await new Promise(
    async resolve => {
      const unsubJustifications =
        await kusama.rpc.grandpa.subscribeJustifications(
          async justification => {
            unsubJustifications()
            resolve(justification)
          }
        )
    }
  )

  const tmpFile: string = join(tmpdir(), justification.toString().slice(0, 10))

  await writeFile(tmpFile, justification.toString())

  const { authoritySet, blockNumber } = await exec(
    './justification-decoder/target/release/justification-decoder ' + tmpFile
  ).then(cmd => JSON.parse(cmd.stdout))

  const blockHash = await kusama.rpc.chain.getBlockHash(blockNumber)
  const kusamaAt = await kusama.at(blockHash)
  const registrationHeader = await kusama.rpc.chain.getHeader(blockHash)
  const authoritySetId = await kusamaAt.query.grandpa.currentSetId()

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
      circuit.createType(
        'Option<Bytes>',
        kusama.runtimeMetadata.asV14.pallets.toHex()
      ),
      kusama.runtimeMetadata.asV14.extrinsic.version,
      kusama.genesisHash,
    ]),
    circuit.createType('GatewaySysProps', [
      circuit.createType('u16', 2),
      circuit.createType('Bytes', 'KSM'),
      circuit.createType('u8', 12),
    ]),
    circuit.createType('Bytes', registrationHeader.toHex()),
    circuit.createType('Option<Vec<AccountId>>', authoritySet),
    circuit.createType('Option<SetId>', authoritySetId),
    circuit.createType('Vec<AllowedSideEffect>', ['tran'])
  )

  return new Promise((resolve, reject) => {
    return circuit.tx.sudo
      .sudo(registerGateway)
      .signAndSend(keyring.alice, result => {
        if (result.isError) {
          reject(new Error('submitting registerGateway failed'))
        } else if (result.isInBlock) {
          log('register_gateway events', ...formatEvents(result.events))
          resolve(undefined)
        }
      })
  })
}
