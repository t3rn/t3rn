import { Connection } from './connection'
import { ApiPromise, Encodings } from '@t3rn/sdk'
const axios = require('axios').default
import { Prometheus } from './prometheus'
import { logger } from './logging'

export const setCheckpointMetrics = async (
  config: any,
  circuitConnection: Connection,
  targetConnection: Connection,
  prometheus: Prometheus,
) => {
  const circuitHeight = await currentGatewayHeight(
    circuitConnection,
    config.targetGatewayId,
  )
  let targetHeight = await currentTargetHeight(targetConnection)

  prometheus.heightDiff = targetHeight - circuitHeight
  prometheus.height.set({ target: 'circuit' }, circuitHeight)
}

export const generateRange = async (
  config: any,
  circuitConnection: Connection,
  targetConnection: Connection,
  target: string,
): Promise<any[]> => {
  return new Promise(async (resolve, reject) => {
    try {
      const circuitHeight = await currentGatewayHeight(
        circuitConnection,
        config.targetGatewayId,
      )
      let targetHeight = await currentTargetHeight(targetConnection)

      logger.debug(
        {
          circuitHeight,
          targetHeight,
        },
        'Current heights',
      )

      if (targetHeight > circuitHeight) {
        let batches = await generateBatchProof(
          circuitConnection.client,
          targetConnection.client,
          config.targetGatewayId,
          circuitHeight + 1,
          targetHeight,
          config.quickSyncLimit,
        )
        return resolve(batches)
      } else {
        throw new Error('No new blocks to submit')
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
  to: number,
  quickSyncLimit: number,
): Promise<any[]> => {
  let transactionArguments: any[] = []

  while (from < to) {
    // get finalityProof element of epoch that contains block #from
    const finalityProof = await targetClient.rpc.grandpa.proveFinality(from)
    // decode finality proof
    let { justification, headers } =
      Encodings.Substrate.Decoders.finalityProofDecode(finalityProof)

    const justificationSize = Math.floor(
      Buffer.from(JSON.stringify(justification)).length / 1024,
    )
    const headersSize = Math.floor(
      Buffer.from(JSON.stringify(headers)).length / 1024,
    )
    logger.debug('Fetched finality proof from target chain')
    logger.debug(`Justification size: ${justificationSize}kb`)
    logger.debug(`Headers size: ${headersSize}kb`)
    const latestHeader = headers.pop()

    if (
      !!quickSyncLimit &&
      headers.length > quickSyncLimit &&
      headers.length > 0
    ) {
      // Switch to quick sync instead - presumably the target chain is too far ahead and there could be too many headers to submit (this issue is a problem for full Polkadot Headers and session of 24H)
      logger.warn('Switching to quick sync')
      // Figure the height of the last block in the epoch is the last block in the headers array.
      // For Quick Sync ask for the last block in the epoch - 101, since the Light Client stores max 100 headers, and the extra 101 is used to compare againt parentHash as substitute for finalized block hash.
      const endOfEpochMinus101 = parseInt(latestHeader.number.toJSON()) - 101
      const finalityProof =
        await targetClient.rpc.grandpa.proveFinality(endOfEpochMinus101)
      const { justification, headers } =
        Encodings.Substrate.Decoders.finalityProofDecode(finalityProof)
      const signedHeaderQuickSync = headers.pop()
      const rangeQuickSync101 = circuitClient.createType('Vec<Header>', headers)
      //push to transaction queue
      transactionArguments = [
        {
          gatewayId: circuitClient.createType('ChainId', targetGatewayId),
          signed_header: signedHeaderQuickSync,
          range: [],
          rangeQuickSync101,
          justification,
        },
      ]

      // No point on dragging the loop further, since we are switching to quick sync
      return transactionArguments
    }

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

    let range = circuitClient.createType('Vec<Header>', headers)
    justification =
      Encodings.Substrate.Decoders.justificationDecode(justification)

    //push to transaction queue
    transactionArguments.push({
      gatewayId: circuitClient.createType('ChainId', targetGatewayId),
      signed_header,
      range,
      rangeQuickSync101: [],
      justification,
    })
  }
  return transactionArguments
}

const currentTargetHeight = async (connection: Connection): Promise<number> => {
  const header = await connection.client.rpc.chain.getHeader(
    await connection.client.rpc.chain.getFinalizedHead(),
  )
  return header.number.toNumber()
}

const currentGatewayHeight = async (
  client: Connection,
  targetGatewayId: string,
) => {
  // client.rpc.portal.
  return axios
    .post(
      client.currentProvider().http,
      {
        jsonrpc: '2.0',
        method: 'portal_fetchHeadHeight',
        params: [Array.from(new TextEncoder().encode(targetGatewayId))],
        id: 1,
      },
      {
        headers: {
          'Content-Type': 'application/json',
        },
      },
    )
    .then(response => {
      if (response.data.error) throw new Error(response.data.error.message)
      return response.data.result
    })
    .catch(error => {
      throw new Error(
        `Gateway height couldnt be fetched! Err: ${error.toString()}`,
      )
    })
}

const getHeader = async (client: ApiPromise, height: number) => {
  return (
    await client.rpc.chain.getHeader(
      await client.rpc.chain.getBlockHash(height),
    )
  ).toJSON()
}
