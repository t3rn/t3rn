import fetch from 'node-fetch'
import { ApiPromise, Encodings, WsProvider } from '@t3rn/sdk'
import { Gateway } from '@/schemas/setup.ts'
import { spinner } from '../gateway.ts'
import { colorLogMsg } from '@/utils/log.ts'

export const registerSubstrateVerificationVendor = async (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>,
) => {
  if (!gatewayData.registrationData.parachain) {
    const target = await ApiPromise.create({
      provider: new WsProvider(gatewayData.rpc),
    })
    return registerRelaychain(circuit, target, gatewayData)
  }

  return registerParachain(circuit, gatewayData)
}

const registerRelaychain = async (
  circuit: ApiPromise,
  target: ApiPromise,
  gatewayData: Required<Gateway>,
) => {
  spinner.info(colorLogMsg('INFO', 'Fetching portal consensus data...'))

  const portalConsensusData = await fetchPortalConsensusData(
    circuit,
    target,
    gatewayData,
  )

  if (!portalConsensusData) {
    return
  }

  const { registrationHeader, authorities, authoritySetId } =
    portalConsensusData

  spinner.info(
    colorLogMsg(
      'INFO',
      `Registering Block #${registrationHeader.number.toNumber()}`,
    ),
  )
  spinner.start()

  return circuit
    .createType('RelaychainRegistrationData', [
      registrationHeader.toHex(),
      Array.from(authorities),
      authoritySetId,
      gatewayData.registrationData.owner,
    ])
    .toHex()
}

const registerParachain = async (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>,
) =>
  circuit
    .createType('ParachainRegistrationData', [
      gatewayData.registrationData.parachain.relayChainId,
      gatewayData.registrationData.parachain.id,
    ])
    .toHex()

const fetchPortalConsensusData = async (
  circuit: ApiPromise,
  target: ApiPromise,
  gatewayData: Required<Gateway>,
) => {
  spinner.info(colorLogMsg('INFO', 'Fetching fetchPortalConsensusData ...'))
  const processEnvHeight =
    process.env['REGISTRATION_HEIGHT'] || null
      ? (parseInt(process.env['REGISTRATION_HEIGHT']) as number)
      : null

  spinner.info(colorLogMsg('INFO', 'processEnvHeight ...' + processEnvHeight))
  const registrationHeight =
    processEnvHeight ||
    (await fetchLatestAuthoritySetUpdateBlock(gatewayData.subscan))

  if (!registrationHeight) {
    return
  }

  console.log('registrationHeight', registrationHeight)

  const registrationHeader = await target.rpc.chain.getHeader(
    await target.rpc.chain.getBlockHash(registrationHeight),
  )

  const finalityProof =
    await target.rpc.grandpa.proveFinality(registrationHeight)
  const authorities =
    Encodings.Substrate.Decoders.extractAuthoritySetFromFinalityProof(
      finalityProof,
    )
  const registratationHeaderHash =
    await target.rpc.chain.getBlockHash(registrationHeight)
  const targetAt = await target.at(registratationHeaderHash)
  const authoritySetId = await targetAt.query.grandpa.currentSetId()
  return {
    registrationHeader,
    authorities: circuit.createType('Vec<AccountId>', authorities),
    authoritySetId: circuit.createType('SetId', authoritySetId),
  }
}

export const fetchLatestAuthoritySetUpdateBlock = async (
  subscanIndexerApiEndpoint: string,
) => {
  try {
    const response = await fetch(
      subscanIndexerApiEndpoint + '/api/scan/events',
      {
        method: 'POST',
        headers: {
          'Content-Type': 'text/json',
        },
        body: JSON.stringify({
          row: 20,
          page: 0,
          module: 'grandpa',
          call: 'newauthorities',
        }),
      },
    )
    const responseData = (await response.json()) as {
      data: {
        events: Array<{
          block_num: number
        }>
      }
    }

    if (response.status !== 200) {
      throw new Error('Subscan indexer API error')
    }

    return responseData.data.events.map((entry) => entry.block_num)[0]
  } catch (error) {
    spinner.fail(colorLogMsg('ERROR', error))
    spinner.start()
  }
}
