import fetch from "node-fetch"
import { ApiPromise } from "@t3rn/sdk"
import { Gateway } from "@/schemas/setup.ts"
import { ssz, phase0 } from "@lodestar/types"
import { fromHexString, toHexString } from "@chainsafe/ssz"
import { colorLogMsg, log } from "@/utils/log.ts"
import { spinner } from "../gateway.ts"

const RELAY_ENDPOINT =
  "https://rpc.ankr.com/premium-http/eth_sepolia_beacon/9b5188fb2ebf6f1e050bf1a1b623623759a0108f7a161b3986f3f21329166288"
const LODESTAR_ENDPOINT = "https://lodestar-sepolia.chainsafe.io"

type BLSPubKey = Uint8Array
type Slot = number
type Root = Uint32Array
type ValidatorIndex = number
type U256 = Array<number>
type Bloom = Array<number>


interface SyncCommittee {
    pubs: string[],
    aggr: string
}

interface EpochCheckPoint {
  justified_epoch: number
  justified_execution_height: number
  justified_block_hash: Root
  finalized_epoch: number
  finalized_execution_height: number
}

interface BeaconBlockHeader {
  slot: Slot
  proposer_index: ValidatorIndex
  parent_root: Root
  state_root: Root
  body_root: Root
}

interface ExecutionPayload {
  parent_hash: Root
  fee_recipient: Array<number>
  state_root: Root
  receipts_root: Root
  logs_bloom: Bloom
  prev_randao: Root
  block_number: number
  gas_limit: number
  gas_used: number
  timestamp: number
  extra_data: Array<number>
  base_fee_per_gas: U256
  block_hash: Root
  transactions_root: Root
  withdrawals_root: Root
}

interface EthereumInitializationData {
  current_sync_committe: SyncCommittee
  next_sync_committie: SyncCommittee
  checkpoint: EpochCheckPoint
  beacon_header: BeaconBlockHeader
  execution_payload: ExecutionPayload
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
      pubkeys: Array<BLSPubKey>,
      aggregate_pubkey: BLSPubKey
    }
  }
}


const fetchNextSyncCommittee = async (slot: number): SyncCommittee => {
  const period = (slot / 32) / 256
  const endpoint = `${LODESTAR_ENDPOINT}/eth/v1/beacon/light_client/updates?start_period=${period}&count=1`
  const fetchOptions = {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Accept: "*/*",
    },
  }

  const response = await fetch(endpoint, fetchOptions)
  if (response.status !== 200) {
    throw new Error(
      "Oops! Fetch debug beacon state faulted to error status: " +
      response.status
    )
  }

  const responseData = (await response.json()) as [NextSyncCommitteeResponse]

  const next: SyncCommittee = {
    pubs: responseData[0].data.next_sync_committee.pubkeys,
    aggr: responseData[0].data.next_sync_committee.aggregate_pubkey
  }

  return next
}

export const fetchLastSyncCommitteeUpdateSlot = async () => {
  const fetchOptions = {
    method: "GET",
  }
  const response = await fetch(
    RELAY_ENDPOINT + "/eth/v1/beacon/headers/head",
    fetchOptions
  )

  if (response.status !== 200) {
    throw new Error(
      `Failed to fetch the last sync committee update slot, STATUS: ${response.status
      }, REASON: ${await response.text()}`
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
  slot = slot - (slot % (32 * 256)) // calc first slot of the current committee period
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
      }
      execution_payload_header: unknown
    }
  }
}

export const decodeSignerBits = (signerBits: string): boolean[] => {
  signerBits = signerBits.replace("0x", "")
  // split into byte groups
  const bytes = signerBits.match(/.{1,2}/g) ?? []
  const acc: boolean[][] = []

  for (let i = 0; i < bytes.length; i++) {
    const binaries = parseInt(bytes[i], 16)
      .toString(2)
      .split("")
      .reverse()
      .map((x) => (parseInt(x) === 1 ? true : false))

    // pad remaining 0s
    while (binaries.length < 8) {
      binaries.push(false)
    }

    acc.push(binaries)
  }

  return acc.flat()
}

export async function fetchHeaderData(slot: number) {
  const endpoint = `${RELAY_ENDPOINT}/eth/v2/beacon/blocks/${slot}`
  const fetchOptions = {
    headers: {
      "Content-Type": "application/json",
      Accept: "*/*",
    },
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    throw new Error(
      "Oops! Fetch header data faulted to error status: " + response.status
    )
  }


  const responseData = (await response.json()) as BeaconBlockResponseData

  return responseData
}

async function fetchCheckpointEntry(slot: number) {
  const header = await fetchHeaderData(slot)
  const {root} =  await fetchBeaconBlockHeaderAndRoot(slot)
  // console.log(header.data.message.body.execution_payload.block)
   return {
     beacon: {
       root, epoch: parseInt(header.data.message.slot, 10) / 32
     },
     execution: {
       root: header.data.message.body.execution_payload.block_hash,
       height: parseInt(header.data.message.body.execution_payload.block_number, 10)
     }
   }
}

export async function fetchBeaconBlockHeaderAndRoot(
  slot: number
): Promise<{ header: phase0.BeaconBlockHeader; root: string }> {
  const endpoint = `${RELAY_ENDPOINT}/eth/v1/beacon/headers?slot=${slot}`
  const fetchOptions = {
    method: "GET",
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    throw new Error(
      `Failed to fetch beacon header, STATUS: ${response.status
      }, REASON: ${await response.text()}`
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

export const fetchInitData = async (finalizedSlot: number, finalizedBeaconBlockRoot: string) => {
  const endpoint = `${LODESTAR_ENDPOINT}/eth/v1/beacon/light_client/bootstrap/${finalizedBeaconBlockRoot}`
  const fetchOptions = {
    method: "GET",
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    throw new Error(
      `Failed fetch init data, STATUS: ${response.status
      }, REASON: ${await response.text()}`
    )
  }

  const responseData = (await response.json()) as BootstrapResponse

  const finalized = await fetchCheckpointEntry(finalizedSlot)
  const justified = await fetchCheckpointEntry(finalizedSlot + 32)
  const attested = await fetchCheckpointEntry(finalizedSlot + 64)

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
    finalized_execution: finalized.execution
  }

  return {
    initData: responseData.data,
    currentSyncCommittee,
    nextSyncCommittee,
    checkpoint,
  }
}

export const registerEthereumVerificationVendor = async (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>
) => {
  try {
    const slot = await fetchLastSyncCommitteeUpdateSlot()
    const { header, root } = await fetchBeaconBlockHeaderAndRoot(slot)

    const {initData, checkpoint, currentSyncCommittee, nextSyncCommittee} = await fetchInitData(slot, root)

    return  generateRegistrationData(initData, checkpoint, currentSyncCommittee, nextSyncCommittee, circuit)

  } catch (e) {
    spinner.fail(colorLogMsg("ERROR", e))
  }
}

const generateRegistrationData = (initData: any, checkpoint: any, currentSyncCommittee: any, nextSyncCommittee: any, circuit: ApiPromise,) => {
  return circuit.createType("EthereumInitializationData", {
        current_sync_committee: circuit.createType("SyncCommittee", {
          pubs: circuit.createType("Vec<BLSPubkey>", currentSyncCommittee.pubs),
          aggr: circuit.createType("BLSPubkey", currentSyncCommittee.aggr),
        }),
        next_sync_committee: circuit.createType("SyncCommittee", {
          pubs: circuit.createType("Vec<BLSPubkey>", nextSyncCommittee.pubs),
          aggr: circuit.createType("BLSPubkey", nextSyncCommittee.aggr),
        }),
        checkpoint: circuit.createType("Checkpoint", checkpoint),
        beacon_header: circuit.createType("BeaconBlockHeader", initData.header.beacon),
        execution_header: circuit.createType("ExecutionHeader", initData.header.execution),
    }).toHex()
}