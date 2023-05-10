import "@t3rn/types"
import ora from "ora"
import { Encodings, ApiPromise, WsProvider } from "@t3rn/sdk"
import fetch from "node-fetch"
import { createType } from "@t3rn/types"
import {
  T3rnPrimitivesGatewayVendor,
  T3rnPrimitivesExecutionVendor,
  T3rnAbiRecodeCodec,
} from "@polkadot/types/lookup"
import { Config, Gateway } from "@/schemas/setup.ts"
import { colorLogMsg, log } from "@/utils/log.ts"
import { createCircuitContext } from "@/utils/circuit.ts"

export const spinner = ora()

export const handleRegisterGateway = async (
  config: Config,
  gatewayId: string
) => {
  const foundGateway = config.gateways.find(
    (g) => g.id.toLowerCase() === gatewayId.toLowerCase()
  )

  if (!foundGateway) {
    log("ERROR", `Gateway ID ${gatewayId} not found in config file`)
    process.exit(1)
  }

  spinner.text = `Registering ${foundGateway.name} gateway...`
  spinner.start()
  await registerGateway(foundGateway as Required<Gateway>)
}

const registerGateway = async (gatewayData: Required<Gateway>) => {
  const { circuit, sdk } = await createCircuitContext()

  const gatewayId = createType("[u8; 4]", gatewayData.id)
  const tokenId = createType("[u8; 4]", gatewayData.tokenId)
  const verificationVendor: T3rnPrimitivesGatewayVendor = createType(
    "T3rnPrimitivesGatewayVendor",
    gatewayData.registrationData.verificationVendor as never
  )
  const executionVendor: T3rnPrimitivesExecutionVendor = createType(
    "T3rnPrimitivesExecutionVendor",
    gatewayData.registrationData.executionVendor as never
  )
  const codec: T3rnAbiRecodeCodec = createType(
    "T3rnAbiRecodeCodec",
    gatewayData.registrationData.runtimeCodec as never
  )
  const registrant = null
  const escrowAccounts = null
  const allowedSideEffects = circuit.createType(
    "Vec<AllowedSideEffect>",
    gatewayData.registrationData.allowedSideEffects
  )
  const tokenInfo = circuit.createType(
    "TokenInfo",
    gatewayData.registrationData.tokenInfo
  )
  const registrationData = await getRegistrationData(circuit, gatewayData)

  if (!registrationData) {
    spinner.fail(
      colorLogMsg("ERROR", `${gatewayData.name} gateway registration failed!`)
    )
    process.exit(1)
  }

  spinner.succeed(colorLogMsg("SUCCESS", "Fetched registration data"))
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
    registrationData
  )

  const response = await sdk.circuit.tx.signAndSendSafe(
    sdk.circuit.tx.createSudo(tx)
  )

  spinner.succeed(
    colorLogMsg("SUCCESS", `Gateway registration tx sent ${response}`)
  )
  spinner.stopAndPersist({
    symbol: "🎉",
    text: colorLogMsg("SUCCESS", `${gatewayData.name} gateway registered!`),
  })
  spinner.stop()
  process.exit(0)
}

const getRegistrationData = (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>
) => {
  switch (gatewayData.registrationData.verificationVendor) {
    case "Rococo":
      return registerSubstrateVerificationVendor(circuit, gatewayData)
    default:
      spinner.fail(
        colorLogMsg(
          "ERROR",
          "Registration for verification vendor not available!"
        )
      )
      return
  }
}

export const registerSubstrateVerificationVendor = async (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>
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
  gatewayData: Required<Gateway>
) => {
  const portalConsensusData = await fetchPortalConsensusData(
    circuit,
    target,
    gatewayData
  )

  if (!portalConsensusData) {
    return
  }

  const { registrationHeader, authorities, authoritySetId } =
    portalConsensusData

  spinner.info(
    colorLogMsg(
      "INFO",
      `Registering Block #${registrationHeader.number.toNumber()}`
    )
  )
  spinner.start()

  return circuit
    .createType("RelaychainRegistrationData", [
      registrationHeader.toHex(),
      Array.from(authorities),
      authoritySetId,
      gatewayData.registrationData.owner,
    ])
    .toHex()
}

const registerParachain = async (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>
) =>
  circuit
    .createType("ParachainRegistrationData", [
      gatewayData.registrationData.parachain.relayChainId,
      gatewayData.registrationData.parachain.id,
    ])
    .toHex()

const fetchPortalConsensusData = async (
  circuit: ApiPromise,
  target: ApiPromise,
  gatewayData: Required<Gateway>
) => {
  const registrationHeight = await fetchLatestAuthoritySetUpdateBlock(
    gatewayData.subscan
  )

  if (!registrationHeight) {
    return
  }

  const registrationHeader = await target.rpc.chain.getHeader(
    await target.rpc.chain.getBlockHash(registrationHeight)
  )
  const finalityProof = await target.rpc.grandpa.proveFinality(
    registrationHeight
  )
  const authorities =
    Encodings.Substrate.Decoders.extractAuthoritySetFromFinalityProof(
      finalityProof
    )
  const registratationHeaderHash = await target.rpc.chain.getBlockHash(
    registrationHeight
  )
  const targetAt = await target.at(registratationHeaderHash)
  const authoritySetId = await targetAt.query.grandpa.currentSetId()
  return {
    registrationHeader,
    authorities: circuit.createType("Vec<AccountId>", authorities),
    authoritySetId: circuit.createType("SetId", authoritySetId),
  }
}

export const fetchLatestAuthoritySetUpdateBlock = async (
  subscanIndexerApiEndpoint: string
) => {
  try {
    const response = await fetch(
      subscanIndexerApiEndpoint + "/api/scan/events",
      {
        method: "POST",
        headers: {
          "Content-Type": "text/json",
        },
        body: JSON.stringify({
          row: 20,
          page: 0,
          module: "grandpa",
          call: "newauthorities",
        }),
      }
    )
    const responseData = (await response.json()) as {
      data: {
        events: Array<{
          block_num: number
        }>
      }
    }

    if (response.status !== 200) {
      throw new Error("Subscan indexer API error")
    }

    return responseData.data.events.map((entry) => entry.block_num)[0]
  } catch (error) {
    spinner.fail(colorLogMsg("ERROR", error))
    spinner.start()
  }
}
