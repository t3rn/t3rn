import { Connection } from "./connection"
import { ApiPromise, Encodings } from "@t3rn/sdk"
const axios = require("axios").default
import { Prometheus } from "./prometheus"
import { logger } from "./logging"

export const generateRange = async (
  config: any,
  circuitConnection: Connection,
  targetConnection: Connection,
  prometheus: Prometheus,
  target: string
): Promise<any[]> => {
  return new Promise(async (resolve, reject) => {
    try {
      const circuitHeight = await currentGatewayHeight(
        circuitConnection,
        config.targetGatewayId
      )
      let targetHeight = await currentTargetHeight(targetConnection)

      logger.debug(
        {
          circuitHeight,
          targetHeight,
        },
        "Current heights"
      )

      prometheus.circuitHeight.set({ target }, circuitHeight)

      if (targetHeight > circuitHeight) {
        let batches = await generateBatchProof(
          circuitConnection.client,
          targetConnection.client,
          config.targetGatewayId,
          circuitHeight + 1,
          targetHeight
        )
        return resolve(batches)
      } else {
        throw new Error("No new blocks to submit")
      }
    } catch (error) {
      return reject(error)
    }
  })
}

const generateBatchProof = async (
  circuitClient: ApiPromise,
  targetClient: ApiPromise,
  targetGatewayId: string,
  from: number,
  to: number
): Promise<any[]> => {
  let transactionArguments: any[] = []

  while (from < to) {
    // get finalityProof element of epoch that contains block #from
    const finalityProof = await targetClient.rpc.grandpa.proveFinality(from)
    // decode finality proof
    let { justification, headers } =
      Encodings.Substrate.Decoders.finalityProofDecode(finalityProof)

    const justificationSize = Math.floor(
      Buffer.from(JSON.stringify(justification)).length / 1024
    )
    const headersSize = Math.floor(
      Buffer.from(JSON.stringify(headers)).length / 1024
    )
    logger.debug("Fetched finality proof from target chain")
    logger.debug(`Justification size: ${justificationSize}kb`)
    logger.debug(`Headers size: ${headersSize}kb`)

    let signed_header
    if (headers.length == 0) {
      // Only one block in epoch missing
      signed_header = await getHeader(targetClient, from)
      from = parseInt(signed_header.number) + 1
    } else {
      signed_header = headers.pop()
      headers = [await getHeader(targetClient, from), ...headers]
      from = parseInt(signed_header.number.toJSON()) + 1
    }

    let range = circuitClient.createType("Vec<Header>", headers)
    justification =
      Encodings.Substrate.Decoders.justificationDecode(justification)

    //push to transaction queue
    transactionArguments.push({
      gatewayId: circuitClient.createType("ChainId", targetGatewayId),
      signed_header,
      range,
      justification,
    })
  }
  return transactionArguments
}

const currentTargetHeight = async (connection: Connection): Promise<number> => {
  const header = await connection.client.rpc.chain.getHeader(
    await connection.client.rpc.chain.getFinalizedHead()
  )
  return header.number.toNumber()
}

const currentGatewayHeight = async (
  client: Connection,
  targetGatewayId: string
) => {
  // client.rpc.portal.
  return axios
    .post(
      client.currentProvider().http,
      {
        jsonrpc: "2.0",
        method: "portal_fetchHeadHeight",
        params: [Array.from(new TextEncoder().encode(targetGatewayId))],
        id: 1,
      },
      {
        headers: {
          "Content-Type": "application/json",
        },
      }
    )
    .then(response => {
      if (response.data.error) throw new Error(response.data.error.message)
      return response.data.result
    })
    .catch(error => {
      throw new Error(
        `Gateway height couldnt be fetched! Err: ${error.toString()}`
      )
    })
}

const getHeader = async (client: ApiPromise, height: number) => {
  return (
    await client.rpc.chain.getHeader(
      await client.rpc.chain.getBlockHash(height)
    )
  ).toJSON()
}
