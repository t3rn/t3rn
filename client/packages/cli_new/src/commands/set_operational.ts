import ora from "ora"
import { createType } from "@t3rn/types"
import { Gateway } from "@/schemas/setup.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { getConfig } from "@/utils/config.ts"
import { colorLogMsg, log } from "@/utils/log.ts"

export const spinner = ora()

export const handleSetOperational = async (id: string, enabled: boolean) => {
  const config = getConfig()

  if (!config) {
    process.exit(1)
  }

  const gateway = config.gateways.find((elem) => elem.id === id)

  if (!gateway) {
    log("ERROR", `Gateway with id ${id} not found in config`)
    process.exit(1)
  }

  spinner.start("Setting gateway operational...")

  try {
    const transactionArgs = setGatewayOperational(gateway, enabled)
    const { circuit, sdk } = await createCircuitContext()
    const transaction = circuit.tx.portal.setOperational(
      transactionArgs.gatewayId,
      transactionArgs.operational
    )

    await sdk.circuit.tx.signAndSendSafe(sdk.circuit.tx.createSudo(transaction))
    spinner.stopAndPersist({
      symbol: "🚗",
      text: colorLogMsg(
        "SUCCESS",
        `Gateway ${id} operational status set to ${enabled}`
      ),
    })
  } catch (error) {
    spinner.fail(colorLogMsg("ERROR", error))
    process.exit(1)
  }
}

export const setGatewayOperational = (gateway: Gateway, enabled: boolean) => {
  switch (gateway.registrationData.verificationVendor) {
    case "Rococo":
    case "Substrate":
      return {
        gatewayId: createType("ChainId", gateway.id),
        operational: createType("bool", enabled),
      }
    default:
      throw new Error(
        `SetOperational not available for Vendor ${gateway.registrationData.verificationVendor}`
      )
  }
}
