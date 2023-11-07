import fetch from 'node-fetch'
import { Web3 } from 'web3'
import { ApiPromise } from '@t3rn/sdk'
import { fromHexString } from '@chainsafe/ssz'
import { colorLogMsg } from '@/utils/log.ts'
import {
  ETHEREUM_EPOCHS_PER_PERIOD,
  ETHEREUM_SLOTS_PER_EPOCH,
  EXECUTION_ENDPOINT,
  LODESTAR_ENDPOINT,
  RELAY_ENDPOINT,
} from '@/consts.ts'
import { spinner } from '../gateway.ts'
import {
  BeaconBlockHeader,
  BeaconBlockHeaderAndRoot,
  BeaconBlockHeaderMsgData,
  BootstrapResponse,
  CheckpointEntry,
  InitData,
  NextSyncCommitteeResponse,
  SyncCommittee,
  VendorRegistrationArgs,
} from '@/commands/registerGateway/vendors/types-eth.ts'

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

async function fetchHeaderData(slot: number): Promise<BeaconBlockResponseData> {
  const endpoint = `${RELAY_ENDPOINT}/eth/v2/beacon/blocks/${slot}`
  const fetchOptions = {
    headers: {
      'Content-Type': 'application/json',
      Accept: '*/*',
    },
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    const reason = await response.text()
    throw new Error(
      `Failed to fetch header data, STATUS: ${response.status}, REASON: ${reason}`,
    )
  }

  const responseData = (await response.json()) as {
    data: BeaconBlockResponseData
  }

  return responseData.data
}

async function fetchCheckpointEntry(slot: number): Promise<CheckpointEntry> {
  const header = await fetchHeaderData(slot)
  const { root } = await fetchBeaconBlockHeaderAndRoot(slot)

  return {
    beacon: {
      epoch: slotToEpoch(parseInt(header.message.slot, 10)),
      root,
    },
    execution: {
      height: parseInt(header.message.body.execution_payload.block_number, 10),
      root: header.message.body.execution_payload.block_hash,
    },
  }
}

async function fetchBeaconBlockHeaderAndRoot(
  slot: number,
): Promise<BeaconBlockHeaderAndRoot> {
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
    data: Array<BeaconBlockHeader>
  }

  const { proposer_index, parent_root, state_root, body_root } =
    responseData.data[0].header.message
  const header: BeaconBlockHeaderMsgData = {
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

const fetchInitData = async (
  finalizedSlot: number,
  finalizedBeaconBlockRoot: string,
): Promise<InitData> => {
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

  spinner.info(`Fetching checkpoint data for finalized slot ${finalizedSlot}`)
  const finalized = await fetchCheckpointEntry(finalizedSlot)

  const justifiedSlot = finalizedSlot + ETHEREUM_SLOTS_PER_EPOCH
  spinner.info(`Fetching checkpoint data for justified slot ${justifiedSlot}`)
  const justified = await fetchCheckpointEntry(justifiedSlot)

  const attestedSlot = finalizedSlot + 2 * ETHEREUM_SLOTS_PER_EPOCH
  spinner.info(`Fetching checkpoint data for attested slot ${attestedSlot}`)
  const attested = await fetchCheckpointEntry(attestedSlot)

  const attestedExecutionHeader = await fetchExecutionHeader(
    attested.execution.height - 1,
  )
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
    attestedExecutionHeader,
  }
}

const generateRegistrationData = (
  {
    data,
    checkpoint,
    currentSyncCommittee,
    nextSyncCommittee,
    attestedExecutionHeader,
  }: InitData,
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
        attestedExecutionHeader,
      ),
    })
    .toHex()
}

export const registerEthereumVerificationVendor = async (
  circuit: ApiPromise,
  args: VendorRegistrationArgs,
): Promise<string> => {
  const spinnerMsg = args.slot
    ? `Registering from a predefined slot: ${args.slot}`
    : 'Registering from beacon head'
  spinner.info(spinnerMsg)

  let slot = args.slot ? args.slot : await fetchLastSyncCommitteeUpdateSlot()

  try {
    const { root } = await fetchBeaconBlockHeaderAndRoot(slot)
    const data = await fetchInitData(slot, root)

    return generateRegistrationData(data, circuit)
  } catch (err) {
    spinner.fail(colorLogMsg('ERROR', err))
  }
}

const fetchExecutionHeader = async (blockNumber: number): Promise<any> => {
  const web3 = new Web3(EXECUTION_ENDPOINT as string)
  const blockHeader = await web3.eth.getBlock(blockNumber)

  return {
    parentHash: blockHeader.parentHash,
    ommersHash: blockHeader.sha3Uncles,
    beneficiary: blockHeader.miner.toLowerCase(),
    stateRoot: blockHeader.stateRoot,
    transactionsRoot: blockHeader.transactionsRoot,
    receiptsRoot: blockHeader.receiptsRoot,
    logsBloom: blockHeader.logsBloom,
    difficulty: blockHeader.difficulty,
    number: blockHeader.number,
    gasLimit: blockHeader.gasLimit,
    gasUsed: blockHeader.gasUsed,
    timestamp: blockHeader.timestamp,
    extraData: blockHeader.extraData,
    mixHash: blockHeader.mixHash,
    nonce: blockHeader.nonce,
    // The following two params are not present in the return type in web3js v4.
    // And before we were using v1.
    baseFeePerGas: blockHeader.baseFeePerGas,
    // @ts-ignore
    withdrawalsRoot: blockHeader.withdrawalsRoot,
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
function slotToCommitteePeriod(slot: number): number {
  return (
    (slot - (slot % (ETHEREUM_SLOTS_PER_EPOCH * ETHEREUM_EPOCHS_PER_PERIOD))) /
    (ETHEREUM_SLOTS_PER_EPOCH * ETHEREUM_EPOCHS_PER_PERIOD)
  )
}

/**
 * Calculates, turning a slot into an epoch.
 *
 * @param {number}  slot
 * @return {number} The calculated epoch
 */
function slotToEpoch(slot: number): number {
  const remainder = slot % ETHEREUM_SLOTS_PER_EPOCH

  return (slot - remainder) / ETHEREUM_SLOTS_PER_EPOCH
}
