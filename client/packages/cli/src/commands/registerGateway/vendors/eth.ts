import fetch from 'node-fetch'
import { ApiPromise } from '@t3rn/sdk'
import { fromHexString } from '@chainsafe/ssz'
import { colorLogMsg } from '@/utils/log.ts'
import { spinner } from '../gateway.ts'
import Web3 from 'web3'
import {
  AttestedExecutionHeader,
  BeaconBlockHeader,
  BeaconBlockHeaderAndRoot,
  BeaconBlockHeaderMsgData,
  BeaconBlockResponseData,
  BootstrapResponse,
  CheckpointEntry,
  InitData,
  NextSyncCommitteeResponse,
  SyncCommittee,
  VendorRegistrationArgs,
} from '@/commands/registerGateway/vendors/types-eth.ts'
import { config } from '@/config/config.ts'

export const registerEthereumVerificationVendor = async (
  circuit: ApiPromise,
  args: VendorRegistrationArgs,
) => {
  const spinnerMsg = args.slot
    ? `Registering from a predefined slot: ${args.slot}`
    : 'Registering from beacon head'
  spinner.info(spinnerMsg)

  const slot = args.slot ? args.slot : await fetchLastSyncCommitteeUpdateSlot()

  try {
    const { root } = await fetchBeaconBlockHeaderAndRoot(slot)
    const data = await fetchInitData(slot, root)

    return generateRegistrationData(data, circuit)
  } catch (err) {
    spinner.fail(colorLogMsg('ERROR', err))
  }
}

const fetchLastSyncCommitteeUpdateSlot = async (): Promise<number> => {
  const fetchOptions = {
    method: 'GET',
  }
  const res = await fetch(
    `${config().targetChain.relayEndpoint}/eth/v1/beacon/headers/head`,
    fetchOptions,
  )

  if (res.status !== 200) {
    throw new Error(
      `Failed to fetch the last sync committee update slot, STATUS: ${
        res.status
      }, REASON: ${await res.text()}`,
    )
  }

  const responseData = (await res.json()) as {
    data: {
      header: {
        message: {
          slot: string
        }
      }
    }
  }

  let slot = parseInt(responseData.data.header.message.slot)
  // calc first slot of the current committee period
  slot =
    slot -
    (slot %
      (config().eth.consts.slotsPerEpoch *
        config().eth.consts.epochsPerCommitteePeriod))
  return slot
}

async function fetchBeaconBlockHeaderAndRoot(
  slot: number,
): Promise<BeaconBlockHeaderAndRoot> {
  const endpoint = `${
    config().targetChain.relayEndpoint
  }/eth/v1/beacon/headers?slot=${slot}`
  const fetchOptions = {
    method: 'GET',
  }
  const res = await fetch(endpoint, fetchOptions)

  if (res.status !== 200) {
    const reason = await res.text()
    throw new Error(
      `Failed to fetch beacon header, STATUS: ${res.status}, REASON: ${reason}`,
    )
  }

  const responseData = (await res.json()) as {
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

/**
 * Makes external calls to different Ethereum nodes to fetch beacon and execution headers data,
 * and constructs an object out of it.
 *
 * @param {number}  finalizedSlot
 * @param {number}  finalizedBeaconBlockRoot
 * @return {Promise<InitData>}
 */
const fetchInitData = async (
  finalizedSlot: number,
  finalizedBeaconBlockRoot: string,
): Promise<InitData> => {
  const endpoint = `${
    config().targetChain.lodestarEndpoint
  }/eth/v1/beacon/light_client/bootstrap/${finalizedBeaconBlockRoot}`
  const fetchOptions = {
    method: 'GET',
  }
  const res = await fetch(endpoint, fetchOptions)

  if (res.status !== 200) {
    const reason = await res.text()
    throw new Error(
      `Failed fetch init data, STATUS: ${res.status}, REASON: ${reason}`,
    )
  }

  const responseData = (await res.json()) as BootstrapResponse

  spinner.info(`Fetching checkpoint data for finalized slot ${finalizedSlot}`)
  const finalized = await fetchCheckpointEntry(finalizedSlot)

  const justifiedSlot = finalizedSlot + config().eth.consts.slotsPerEpoch
  spinner.info(`Fetching checkpoint data for justified slot ${justifiedSlot}`)
  const justified = await fetchCheckpointEntry(justifiedSlot)

  const attestedSlot = finalizedSlot + 2 * config().eth.consts.slotsPerEpoch
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

/**
 * Generate registration data hex, from the init data object, that is to be submitted as tx to Circuit
 *
 * @param {InitData}  initData
 * @param {ApiPromise}  circuit
 */
const generateRegistrationData = (initData: InitData, circuit: ApiPromise) => {
  const {
    data,
    checkpoint,
    currentSyncCommittee,
    nextSyncCommittee,
    attestedExecutionHeader,
  } = initData

  return circuit
    .createType('EthereumInitializationData', {
      current_sync_committee: circuit.createType(
        'PalletEth2FinalityVerifierSyncCommittee',
        {
          pubs: circuit.createType('Vec<BLSPubkey>', currentSyncCommittee.pubs),
          aggr: circuit.createType('BLSPubkey', currentSyncCommittee.aggr),
        },
      ),
      next_sync_committee: circuit.createType(
        'PalletEth2FinalityVerifierSyncCommittee',
        {
          pubs: circuit.createType('Vec<BLSPubkey>', nextSyncCommittee.pubs),
          aggr: circuit.createType('BLSPubkey', nextSyncCommittee.aggr),
        },
      ),
      checkpoint: circuit.createType(
        'PalletEth2FinalityVerifierCheckpoint',
        checkpoint,
      ),
      beacon_header: circuit.createType(
        'PalletEth2FinalityVerifierBeaconBlockHeader',
        data.header.beacon,
      ),
      execution_header: circuit.createType(
        'PalletEth2FinalityVerifierExecutionHeader',
        attestedExecutionHeader,
      ),
    })
    .toHex()
}

const fetchNextSyncCommittee = async (slot: number): Promise<SyncCommittee> => {
  const period = slotToCommitteePeriod(slot)
  const endpoint = `${
    config().targetChain.lodestarEndpoint
  }/eth/v1/beacon/light_client/updates?start_period=${period}&count=1`
  const fetchOptions = {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Accept: '*/*',
    },
  }
  const res = await fetch(endpoint, fetchOptions)

  if (res.status !== 200) {
    throw new Error(
      `Could not fetch next sync committee (slot: ${slot}, period: ${period}). Err: ${res.status} ${res.statusText}`,
    )
  }

  const responseData = (await res.json()) as [NextSyncCommitteeResponse]
  return {
    pubs: responseData[0].data.next_sync_committee.pubkeys,
    aggr: responseData[0].data.next_sync_committee.aggregate_pubkey,
  }
}

async function fetchHeaderData(slot: number): Promise<BeaconBlockResponseData> {
  const endpoint = `${
    config().targetChain.relayEndpoint
  }/eth/v2/beacon/blocks/${slot}`
  const fetchOptions = {
    headers: {
      'Content-Type': 'application/json',
      Accept: '*/*',
    },
  }
  const res = await fetch(endpoint, fetchOptions)

  if (res.status !== 200) {
    const reason = await res.text()
    throw new Error(
      `Failed to fetch header data, STATUS: ${res.status}, REASON: ${reason}`,
    )
  }

  const responseData = (await res.json()) as {
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

const fetchExecutionHeader = async (
  blockNumber: number,
): Promise<AttestedExecutionHeader> => {
  // @ts-ignore - TS doesn't know about this type
  const web3 = new Web3(
    // @ts-ignore - TS doesn't know about this type
    new Web3.providers.HttpProvider(config().targetChain.executionEndpoint),
  )
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
    baseFeePerGas: blockHeader.baseFeePerGas,
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
    (slot -
      (slot %
        (config().eth.consts.slotsPerEpoch *
          config().eth.consts.epochsPerCommitteePeriod))) /
    (config().eth.consts.slotsPerEpoch *
      config().eth.consts.epochsPerCommitteePeriod)
  )
}

/**
 * Calculates, turning a slot into an epoch.
 *
 * @param {number}  slot
 * @return {number} The calculated epoch
 */
function slotToEpoch(slot: number): number {
  const remainder = slot % config().eth.consts.slotsPerEpoch

  return (slot - remainder) / config().eth.consts.slotsPerEpoch
}
