import fetch from 'node-fetch'
import { ApiPromise } from '@t3rn/sdk'
// @ts-ignore - TS doesn't know about the type definition
import { phase0 } from '@lodestar/types'
import { fromHexString } from '@chainsafe/ssz'
import { colorLogMsg } from '@/utils/log.ts'
import {
  ETHEREUM_SLOTS_PER_EPOCH,
  ETHEREUM_EPOCHS_PER_PERIOD,
  LODESTAR_ENDPOINT,
  RELAY_ENDPOINT,
} from '@/consts.ts'
import { spinner } from '../gateway.ts'

type BLSPubKey = Uint8Array

type SyncCommittee = {
  pubs: Array<BLSPubKey | string>
  aggr: BLSPubKey | string
}

interface BootstrapResponse {
  data: {
    header: {
      beacon: {
        slot: string
        proposer_index: string
        parent_root: string
        state_root: string
        body_root: string
      }
      execution: {
        parent_hash: string
        fee_recipient: string
        state_root: string
        receipts_root: string
        logs_bloom: string
        prev_randao: string
        block_number: string
        gas_limit: string
        gas_used: string
        timestamp: string
        extra_data: string
        base_fee_per_gas: string
        block_hash: string
        transactions_root: string
        withdrawals_root: string
      }
      execution_branch: Array<string>
    }
    current_sync_committee: {
      pubkeys: Array<string>
      aggregate_pubkey: string
    }
    current_sync_committe_branch: Array<string>
    version: string
  }
}

interface NextSyncCommitteeResponse {
  data: {
    next_sync_committee: {
      pubkeys: Array<BLSPubKey | string>
      aggregate_pubkey: BLSPubKey | string
    }
  }
}

export interface VendorRegistrationArgs {
  retry?: boolean
  slot?: number
}

const fetchNextSyncCommittee = async (slot: number): Promise<SyncCommittee> => {
  const period = slotToCommitteePeriod(slot)
  const endpoint = `${LODESTAR_ENDPOINT}/eth/v1/beacon/light_client/updates?start_period=${period}&count=1`
  const fetchOptions = {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Accept: '*/*',
    },
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    throw new Error(
      `Could not fetch next sync committee (slot: ${slot}, period: ${period}). Err: ${response.status} ${response.statusText}`,
    )
  }

  const responseData = (await response.json()) as NextSyncCommitteeResponse[]
  const syncCommittee: SyncCommittee = {
    pubs: responseData[0].data.next_sync_committee.pubkeys,
    aggr: responseData[0].data.next_sync_committee.aggregate_pubkey,
  }

  return syncCommittee
}

const fetchLastSyncCommitteeUpdateSlot = async () => {
  const fetchOptions = {
    method: 'GET',
  }
  const response = await fetch(
    RELAY_ENDPOINT + '/eth/v1/beacon/headers/head',
    fetchOptions,
  )

  if (response.status !== 200) {
    const reason = await response.text()
    throw new Error(
      `Failed to fetch the last sync committee update slot, STATUS: ${response.status}, REASON: ${reason}`,
    )
  }

  const responseData = (await response.json()) as {
    data: {
      header: {
        message: {
          slot: string
        }
      }
    }
  }

  let slot = parseInt(responseData.data.header.message.slot)
  slot = slot - (slot % (ETHEREUM_SLOTS_PER_EPOCH * ETHEREUM_EPOCHS_PER_PERIOD)) // calc first slot of the current committee period
  return slot
}

interface BeaconBlockResponseData {
  message: {
    slot: string
    body: {
      execution_payload: {
        transactions: unknown
        transactions_root: string
        withdrawals: unknown
        withdrawals_root: unknown
        block_hash: string
        block_number: string
      }
      execution_payload_header: unknown
    }
  }
}

async function fetchHeaderData(slot: number) {
  const endpoint = `${RELAY_ENDPOINT}/eth/v2/beacon/blocks/${slot}`
  const fetchOptions = {
    headers: {
      'Content-Type': 'application/json',
      Accept: '*/*',
    },
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    throw new Error(
      'Oops! Fetch header data faulted to error status: ' + response.status,
    )
  }

  const responseData = (await response.json()) as {
    data: BeaconBlockResponseData
  }

  return responseData.data
}

async function fetchCheckpointEntry(slot: number) {
  const header = await fetchHeaderData(slot)
  const { root } = await fetchBeaconBlockHeaderAndRoot(slot)

  return {
    beacon: {
      root,
      // slot to epoch number
      epoch: parseInt(header.message.slot, 10) / ETHEREUM_SLOTS_PER_EPOCH,
    },
    execution: {
      root: header.message.body.execution_payload.block_hash,
      height: parseInt(header.message.body.execution_payload.block_number, 10),
    },
  }
}

async function fetchBeaconBlockHeaderAndRoot(
  slot: number,
): Promise<{ header: phase0.BeaconBlockHeader; root: string }> {
  const endpoint = `${RELAY_ENDPOINT}/eth/v1/beacon/headers?slot=${slot}`
  const fetchOptions = {
    method: 'GET',
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    const reason = await response.text()
    throw new Error(
      `Failed to fetch beacon header, STATUS: ${response.status}, REASON: ${reason}`,
    )
  }

  const responseData = (await response.json()) as {
    data: Array<{
      root: string
      header: {
        message: {
          proposer_index: string
          parent_root: string
          state_root: string
          body_root: string
        }
      }
    }>
  }

  const { proposer_index, parent_root, state_root, body_root } =
    responseData.data[0].header.message
  const header = {
    slot,
    proposerIndex: parseInt(proposer_index),
    parentRoot: fromHexString(parent_root),
    stateRoot: fromHexString(state_root),
    bodyRoot: fromHexString(body_root),
  }

  return {
    header,
    root: responseData.data[0].root,
  }
}

type InitData = Awaited<ReturnType<typeof fetchInitData>>

const fetchInitData = async (
  finalizedSlot: number,
  finalizedBeaconBlockRoot: string,
) => {
  const endpoint = `${LODESTAR_ENDPOINT}/eth/v1/beacon/light_client/bootstrap/${finalizedBeaconBlockRoot}`
  const fetchOptions = {
    method: 'GET',
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    const reason = await response.text()
    throw new Error(
      `Failed fetch init data, STATUS: ${response.status}, REASON: ${reason}`,
    )
  }

  const responseData = (await response.json()) as BootstrapResponse

  const finalized = await fetchCheckpointEntry(finalizedSlot)
  const justified = await fetchCheckpointEntry(
    finalizedSlot + ETHEREUM_SLOTS_PER_EPOCH,
  )
  const attested = await fetchCheckpointEntry(
    finalizedSlot + 2 * ETHEREUM_SLOTS_PER_EPOCH,
  )
  // TODO: test if submissions of epochs work w/o this
  // const attestedExecutionHeader = await fetchExecutionHeader(
  //   attested.execution.height - 1
  // )
  const currentSyncCommittee: SyncCommittee = {
    pubs: responseData.data.current_sync_committee.pubkeys,
    aggr: responseData.data.current_sync_committee.aggregate_pubkey,
  }
  const nextSyncCommittee = await fetchNextSyncCommittee(finalizedSlot)
  const checkpoint = {
    attested_beacon: attested.beacon,
    attested_execution: attested.execution,
    justified_beacon: justified.beacon,
    justified_execution: justified.execution,
    finalized_beacon: finalized.beacon,
    finalized_execution: finalized.execution,
  }

  return {
    data: responseData.data,
    currentSyncCommittee,
    nextSyncCommittee,
    checkpoint,
    // attestedExecutionHeader,
  }
}

const generateRegistrationData = (
  { data, checkpoint, currentSyncCommittee, nextSyncCommittee }: InitData,
  circuit: ApiPromise,
) => {
  return circuit
    .createType('EthereumInitializationData', {
      current_sync_committee: circuit.createType('SyncCommittee', {
        pubs: circuit.createType('Vec<BLSPubkey>', currentSyncCommittee.pubs),
        aggr: circuit.createType('BLSPubkey', currentSyncCommittee.aggr),
      }),
      next_sync_committee: circuit.createType('SyncCommittee', {
        pubs: circuit.createType('Vec<BLSPubkey>', nextSyncCommittee.pubs),
        aggr: circuit.createType('BLSPubkey', nextSyncCommittee.aggr),
      }),
      checkpoint: circuit.createType('Checkpoint', checkpoint),
      beacon_header: circuit.createType(
        'BeaconBlockHeader',
        data.header.beacon,
      ),
      execution_header: circuit.createType(
        'ExecutionHeader',
        data.header.execution,
      ),
    })
    .toHex()
}

export const registerEthereumVerificationVendor = async (
  circuit: ApiPromise,
  args: VendorRegistrationArgs,
) => {
  const spinnerMsg = args.slot
    ? `Registering from a predefined slot: ${args.slot}`
    : 'Registering from beacon head'
  spinner.info(spinnerMsg)

  let slot = args.slot ? args.slot : await fetchLastSyncCommitteeUpdateSlot()

  try {
    // Sometimes the slot we fetch here might be missed.
    // In this case, we should subtract 32 * 256 slots (the previous committee term).
    // If one of them returns 404 we should use other one.
    if (args.retry) {
      slot -= 8192
    }

    const { root } = await fetchBeaconBlockHeaderAndRoot(slot)
    const data = await fetchInitData(slot, root)

    return generateRegistrationData(data, circuit)
  } catch (err) {
    if (!args.retry) {
      spinner.info(
        colorLogMsg(
          'INFO',
          `Retrying to fetch init data due to an error: ${err.message}`,
        ),
      )
      return registerEthereumVerificationVendor(circuit, { slot, retry: true })
    } else {
      spinner.fail(colorLogMsg('ERROR', err))
    }
  }
}

/**
 * Calculate the committee period from a given slot.
 * The committee period is set to be 256 epochs long, which is approximately 27 hours (and each epoch has 32 slots).
 * E.g. for slot 3293152 or slot 3305026:
 *    circuit committee period: (3293152 - (3293152 % (32 * 256))) / (32 * 256) = 401
 *    target committee period: (3305026 - (3305026 % (32 * 256))) / (32 * 256) = 403
 *
 * @param {number}  slot
 * @return {number}
 */
export function slotToCommitteePeriod(slot: number): number {
  return (
    (slot - (slot % (ETHEREUM_SLOTS_PER_EPOCH * ETHEREUM_EPOCHS_PER_PERIOD))) /
    (ETHEREUM_SLOTS_PER_EPOCH * ETHEREUM_EPOCHS_PER_PERIOD)
  )
}
