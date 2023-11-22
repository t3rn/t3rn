import '@t3rn/types'
import ora from 'ora'
import { ApiPromise } from '@t3rn/sdk'
import { createType } from '@t3rn/types'
import {
  T3rnAbiRecodeCodec,
  T3rnPrimitivesExecutionVendor,
  T3rnPrimitivesGatewayVendor,
} from '@polkadot/types/lookup'
import { Gateway } from '@/schemas/setup.ts'
import { colorLogMsg, log } from '@/utils/log.ts'
import { createCircuitContext } from '@/utils/circuit.ts'
import { getConfig } from '@/utils/config.ts'
import { registerSubstrateVerificationVendor } from './vendors/substrate.ts'
import { registerEthereumVerificationVendor } from './vendors/eth.ts'

export const spinner = ora()

export const handleRegisterGateway = async (
  gatewayId: string,
  exportMode: boolean,
  slot?: number,
): Promise<void> => {
  const config = getConfig()
  if (!config) {
    process.exit(1)
  }

  const foundGateway = config.gateways.find(
    (gateway) => gateway.id.toLowerCase() === gatewayId.toLowerCase(),
  )

  if (!foundGateway) {
    log('ERROR', `Gateway ID ${gatewayId} not found in config file`)
    process.exit(1)
  }

  log('INFO', `Registering ${foundGateway.name} gateway...`)
  spinner.start()

  await registerGateway(foundGateway as Required<Gateway>, exportMode, slot)
}

const registerGateway = async (
  gatewayData: Required<Gateway>,
  exportMode: boolean,
  slot?: number,
): Promise<void> => {
  const { circuit, sdk } = await createCircuitContext(exportMode)

  const gatewayId = createType('[u8; 4]', gatewayData.id)
  const tokenId = createType('u32', gatewayData.tokenId)
  const verificationVendor: T3rnPrimitivesGatewayVendor = createType(
    'T3rnPrimitivesGatewayVendor',
    gatewayData.registrationData.verificationVendor as never,
  )
  const executionVendor: T3rnPrimitivesExecutionVendor = createType(
    'T3rnPrimitivesExecutionVendor',
    gatewayData.registrationData.executionVendor as never,
  )
  const codec: T3rnAbiRecodeCodec = createType(
    'T3rnAbiRecodeCodec',
    gatewayData.registrationData.runtimeCodec as never,
  )
  const registrant = null
  const escrowAccounts = null
  const allowedSideEffects = circuit.createType(
    'Vec<AllowedSideEffect>',
    gatewayData.registrationData.allowedSideEffects,
  )
  const tokenInfo = circuit.createType(
    'TokenInfo',
    gatewayData.registrationData.tokenInfo,
  )

  try {
    const registrationData = await getRegistrationData(
      circuit,
      gatewayData,
      slot,
    )

    if (!registrationData) {
      throw new Error(
        `${gatewayData.name} gateway registration data is not present!`,
      )
    }

    spinner.succeed(colorLogMsg('SUCCESS', 'Fetched registration data'))
    spinner.start()

    const tx = circuit.tx.portal.registerGateway(
      gatewayId,
      tokenId,
      //@ts-ignore - TS doesn't know about the type
      verificationVendor.toJSON(),
      //@ts-ignore - TS doesn't know about the type
      executionVendor.toJSON(),
      //@ts-ignore - TS doesn't know about the type
      codec.toJSON(),
      registrant,
      escrowAccounts,
      allowedSideEffects,
      tokenInfo,
      registrationData,
    )
    const response = await sdk.circuit.tx.signAndSendSafe(
      sdk.circuit.tx.createSudo(tx),
    )

    spinner.succeed(
      colorLogMsg('SUCCESS', `Gateway registration tx sent ${response}`),
    )
    spinner.stopAndPersist({
      symbol: '🎉',
      text: colorLogMsg('SUCCESS', `${gatewayData.name} gateway registered!`),
    })
    spinner.stop()

    process.exit(0)
  } catch (error) {
    spinner.fail(
      colorLogMsg(
        'ERROR',
        `${gatewayData.name} gateway registration failed! REASON: ${error}`,
      ),
    )
    process.exit(1)
  }
}

const getRegistrationData = (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>,
  slot?: number,
) => {
  switch (gatewayData.registrationData.verificationVendor) {
    case 'Kusama':
      return registerSubstrateVerificationVendor(circuit, gatewayData)
    case 'Rococo':
      return registerSubstrateVerificationVendor(circuit, gatewayData)
    case 'Polkadot':
      return registerSubstrateVerificationVendor(circuit, gatewayData)
    case 'Bittensor':
      return registerSubstrateVerificationVendor(circuit, gatewayData)
    case 'Ethereum':
      return registerEthereumVerificationVendor(circuit, { slot })
    case 'Sepolia':
      return registerEthereumVerificationVendor(circuit, { slot })
    default:
      throw new Error('Registration for verification vendor not available!')
  }
}
