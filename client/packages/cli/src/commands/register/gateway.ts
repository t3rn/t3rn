import "@t3rn/types"
import ora from "ora"
import { ApiPromise } from "@t3rn/sdk"
import { createType } from "@t3rn/types"
//@ts-ignore - TS doesn't know about the type
import {
  //@ts-ignore - TS doesn't know about the type
  T3rnPrimitivesGatewayVendor,
  //@ts-ignore - TS doesn't know about the type
  T3rnPrimitivesExecutionVendor,
  //@ts-ignore - TS doesn't know about the type
  T3rnAbiRecodeCodec,
  //@ts-ignore - TS doesn't know about the type
} from "@polkadot/types/lookup"
import { Gateway } from "@/schemas/setup.ts"
import { colorLogMsg, log } from "@/utils/log.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { getConfig } from "@/utils/config.ts"
import { registerSubstrateVerificationVendor } from "./vendors/substrate.ts"
import { registerEthereumVerificationVendor } from "./vendors/eth.ts"

export const spinner = ora()

export const handleRegisterGateway = async (
  gatewayId: string,
  exportMode: boolean,
) => {
  const config = getConfig()
  if (!config) {
    process.exit(1)
  }

  const foundGateway = config.gateways.find(
    (g) => g.id.toLowerCase() === gatewayId.toLowerCase(),
  )

  if (!foundGateway) {
    log("ERROR", `Gateway ID ${gatewayId} not found in config file`)
    process.exit(1)
  }
  console.log("Found gateway!", foundGateway)

  spinner.text = `Registering ${foundGateway.name} gateway...`
  spinner.start()

  await registerGateway(foundGateway as Required<Gateway>, exportMode)
}

const registerGateway = async (
  gatewayData: Required<Gateway>,
  exportMode: boolean,
) => {
  const { circuit, sdk } = await createCircuitContext(exportMode)

  const gatewayId = createType("[u8; 4]", gatewayData.id)
  const tokenId = createType("u32", gatewayData.tokenId)
  const verificationVendor: T3rnPrimitivesGatewayVendor = createType(
    "T3rnPrimitivesGatewayVendor",
    gatewayData.registrationData.verificationVendor as never,
  )
  const executionVendor: T3rnPrimitivesExecutionVendor = createType(
    "T3rnPrimitivesExecutionVendor",
    gatewayData.registrationData.executionVendor as never,
  )
  const codec: T3rnAbiRecodeCodec = createType(
    "T3rnAbiRecodeCodec",
    gatewayData.registrationData.runtimeCodec as never,
  )
  const registrant = null
  const escrowAccounts = null
  const allowedSideEffects = circuit.createType(
    "Vec<AllowedSideEffect>",
    gatewayData.registrationData.allowedSideEffects,
  )
  const tokenInfo = circuit.createType(
    "TokenInfo",
    gatewayData.registrationData.tokenInfo,
  )

  try {
    const registrationData = await getRegistrationData(circuit, gatewayData)

    // need to call xdns.rebootSelfGateway to add 0x03030303
    await sdk.circuit.tx.signAndSendSafe(
      sdk.circuit.tx.createSudo(
        circuit.tx.xdns.rebootSelfGateway(gatewayData.registrationData.verificationVendor as never)
      )
    )

    if (!registrationData) {
      throw new Error(`${gatewayData.name} gateway registration failed!`)
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
      //@ts-ignore - TS doesn't know about the type
      allowedSideEffects,
      tokenInfo,
      registrationData,
    )
    const response = await sdk.circuit.tx.signAndSendSafe(
      sdk.circuit.tx.createSudo(tx),
    )

    spinner.succeed(
      colorLogMsg("SUCCESS", `Gateway registration tx sent ${response}`),
    )
    spinner.stopAndPersist({
      symbol: "🎉",
      text: colorLogMsg("SUCCESS", `${gatewayData.name} gateway registered!`),
    })
    spinner.stop()

    process.exit(0)
  } catch (error) {
    spinner.fail(
      colorLogMsg(
        "ERROR",
        `${gatewayData.name} gateway registration failed! ${error}`,
      ),
    )
    process.exit(1)
  }
}

const getRegistrationData = (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>,
) => {
  switch (gatewayData.registrationData.verificationVendor) {
    case "Rococo":
      return registerSubstrateVerificationVendor(circuit, gatewayData)
    case "Ethereum":
      return registerEthereumVerificationVendor(circuit)
    default:
      throw new Error("Registration for verification vendor not available!")
  }
}
