import { ApiPromise } from "@t3rn/sdk"
import bls from "@chainsafe/bls"
import { createBeaconConfig } from "@lodestar/config"
import { networksChainConfig } from "@lodestar/config/networks"
import { DOMAIN_SYNC_COMMITTEE } from "@chainsafe/lodestar-params"
import { Gateway } from "@/schemas/setup.ts"
import { ssz, phase0 } from "@lodestar/types"
import { ContainerType, Type, ListCompositeType, ValueOf } from "@chainsafe/ssz"
import { fromHexString, toHexString } from "@chainsafe/ssz"
import {
  Tree,
  SingleProofInput,
  ProofType,
  SingleProof,
  createNodeFromProof,
} from "@chainsafe/persistent-merkle-tree"

const RELAY_ENDPOINT =
  "https://rpc.ankr.com/premium-http/eth_sepolia_beacon/9b5188fb2ebf6f1e050bf1a1b623623759a0108f7a161b3986f3f21329166288"
const LODESTAR_ENDPOINT = "https://lodestar-sepolia.chainsafe.io"

type BLSPubKey = Uint8Array
type Slot = number
type Root = Uint32Array
type ValidatorIndex = number
type U256 = Array<number>
type Bloom = Array<number>

export enum Network {
  MAINNET,
  SEPOLIA,
}

interface SyncCommittee {
  pubs: Array<BLSPubKey>
  aggr: BLSPubKey
}

interface SyncAggregate {
  committeeBits: string
  signature: string
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

export const createViewFromContainerType = <
  F extends Record<string, Type<unknown>>,
  V
>(
  type: ContainerType<F>,
  value: V
) => {
  return type.toView(type.fromJson(value))
}

export const createViewFromListCompositeType = <V>(
  type: ListCompositeType<never>,
  value: V
) => {
  return type.toView(type.fromJson(value) as never)
}

export const buildEthereumInitializationData =
  (): EthereumInitializationData => {
    //@todo: implement later
    return {} as never
  }

export async function fetchDebugBeaconState(block: number) {
  const endpoint = `${RELAY_ENDPOINT}/eth/v1/debug/beacon/states/${block}`
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

  const responseData = (await response.json()) as {
    next_sync_committee: {
      pubkeys: Array<BLSPubKey>
      aggregate_pubkey: BLSPubKey
    }
  }

  return responseData
}

export const compressBody = (
  body: BeaconBlockResponseData["message"]["body"]
) => {
  const executionPayload = body.execution_payload
  const transactionRoot = toHexString(
    createViewFromListCompositeType(
      ssz.bellatrix.Transactions as never,
      executionPayload.transactions
    ).hashTreeRoot()
  )

  executionPayload.transactions_root = transactionRoot

  const withdrawalsRoot = toHexString(
    createViewFromListCompositeType(
      ssz.capella.Withdrawals as never,
      executionPayload.withdrawals
    ).hashTreeRoot()
  )
  executionPayload.withdrawals_root = withdrawalsRoot

  delete body.execution_payload
  delete executionPayload.transactions
  delete executionPayload.withdrawals

  body.execution_payload_header = executionPayload

  return body
}

export const beaconHeaderView = <V>(block: V) => {
  const blockValues = ssz.capella.BeaconBlock.fromJson(block)
  return ssz.capella.BeaconBlock.toView(blockValues)
}

export declare type ValueOfFields<
  Fields extends Record<string, Type<unknown>>
> = {
    [K in keyof Fields]: ValueOf<Fields[K]>
  }

export const generateSyncCommitteeInclusionProof = (state: unknown) => {
  const view = createViewFromContainerType(ssz.capella.BeaconState, state)
  const proofInput: SingleProofInput = {
    type: ProofType.single,
    gindex: view.type.getPropertyGindex([
      "next_sync_committee",
    ] as never) as bigint,
  }
  const tree = new Tree(view.node)
  const proof = tree.getProof(proofInput) as SingleProof
  return proof
}

export const fetchSyncCommittee = async (slot: number) => {
  const state = await fetchDebugBeaconState(slot)
  const proof = generateSyncCommitteeInclusionProof(state)
  const syncCommittee: SyncCommittee = {
    pubs: state.next_sync_committee.pubkeys,
    aggr: state.next_sync_committee.aggregate_pubkey,
  }
  return { syncCommittee, proof }
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
      "Oops! Fetch last sync committee update slot responded with an error status: " +
      response.status
    )
  }

  const responseData = (await response.json()) as {
    head: {
      header: {
        message: {
          slot: string
        }
      }
    }
  }
  const slot = parseInt(responseData.head.header.message.slot)

  return slot - (slot % (32 * 256))
}

export const generateExecutionPayloadInclusionProof = (
  block: BeaconBlockResponseData["message"]
) => {
  const beaconHeader = beaconHeaderView(block)
  const compressedBody = compressBody(block.body)
  const executionPayload = createViewFromContainerType(
    ssz.capella.ExecutionPayloadHeader,
    compressedBody.execution_payload_header
  )
  const bodyValues = ssz.capella.BlindedBeaconBlockBody.fromJson(compressedBody)
  const bodyView = ssz.capella.BlindedBeaconBlockBody.toView(bodyValues)
  const proofInput: SingleProofInput = {
    type: ProofType.single,
    gindex: bodyView.type.getPropertyGindex([
      "executionPayloadHeader",
    ] as never) as bigint,
  }
  const tree = new Tree(bodyView.node)
  const proof = tree.getProof(proofInput) as SingleProof
  const proofRes = createNodeFromProof(proof)
  console.log(
    "Proof Valid:",
    toHexString(proofRes.root) === toHexString(bodyView.hashTreeRoot())
  )
  return { executionPayload, beaconHeader, proof }
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

export async function fetchSyncAggregate(slot: number) {
  const endpoint = `${RELAY_ENDPOINT}/eth/v1/beacon/blocks/${slot}`
  const fetchOptions = {
    type: "GET",
    headers: {
      "Content-Type": "application/json",
      Accept: "*/*",
    },
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    throw new Error(
      "Oops! Fetch sync aggregate faulted to error status: " + response.status
    )
  }

  const responseData = (await response.json()) as {
    body: {
      sync_aggregate: {
        sync_committee_bits: string
        sync_committee_signature: string
      }
    }
  }
  const syncAggregate: SyncAggregate = {
    committeeBits: responseData.body.sync_aggregate.sync_committee_bits,
    signature: responseData.body.sync_aggregate.sync_committee_signature,
  }

  return syncAggregate
}

export const fetchValidatorRoot = async () => {
  const response = await fetch(RELAY_ENDPOINT + "/eth/v1/beacon/genesis", {
    method: "GET",
  })

  if (response.status !== 200) {
    throw new Error(
      "Oops! Fetch validator root responded with an error status: " +
      response.status
    )
  }

  const responseData = (await response.json()) as {
    data: {
      genesis_validators_root: string
    }
  }
  return responseData.data.genesis_validators_root
}

export const getDomain = async (
  network: Network,
  slot: number,
  domain: Uint8Array
) => {
  const getConfig = (network: Network) => {
    switch (network) {
      case Network.MAINNET:
        return networksChainConfig.mainnet
        break
      case Network.SEPOLIA:
        return networksChainConfig.sepolia
        break
    }
  }

  const chainConfig = getConfig(network)
  const validatorsRoot = await fetchValidatorRoot()
  const config = createBeaconConfig(chainConfig, fromHexString(validatorsRoot))
  return config.getDomain(slot, domain)
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

export const generateSigningRoot = async (
  network: Network,
  slot: number,
  root: string,
  domainId: Uint8Array
) => {
  const domain = await getDomain(network, slot, domainId)
  const signingRoot = ssz.phase0.SigningData.hashTreeRoot({
    objectRoot: fromHexString(root),
    domain,
  })
  return signingRoot
}

export const aggregatePubkey = (
  pubkeys: string[],
  signerBits: string
): string => {
  const signerBitsArray = decodeSignerBits(signerBits)
  const pubkeysArray = pubkeys.map((x) =>
    Buffer.from(x.replace("0x", ""), "hex")
  )
  const aggPubkey = bls.aggregatePublicKeys(
    pubkeysArray.filter((_, i) => signerBitsArray[i] === true)
  )
  return toHexString(aggPubkey)
}

export const verifyAggregateSignature = (
  aggrPub: string,
  msg: string,
  sig: string
): boolean => {
  const aggrPubBytes = fromHexString(aggrPub)
  const msgBytes = fromHexString(msg)
  const sigBytes = fromHexString(sig)
  return bls.verify(aggrPubBytes, msgBytes, sigBytes)
}

export async function fetchHeaderData(slot: number) {
  const endpoint = `${RELAY_ENDPOINT}/eth/v1/beacon/blocks/${slot}`
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
  const { executionPayload, beaconHeader, proof } =
    generateExecutionPayloadInclusionProof(responseData.message)
  const syncAggregate = await fetchSyncAggregate(
    parseInt(responseData.message.slot) + 1
  )
  // testing the signature:
  const signingRoot = await generateSigningRoot(
    Network.SEPOLIA,
    parseInt(responseData.message.slot),
    toHexString(beaconHeader.hashTreeRoot()),
    DOMAIN_SYNC_COMMITTEE
  )
  const init = await fetchInitData(await fetchLastSyncCommitteeUpdateSlot())
  const aggregate = aggregatePubkey(
    init.current_sync_committee.pubkeys as unknown as string[],
    syncAggregate.committeeBits
  )
  const valid = verifyAggregateSignature(
    aggregate,
    toHexString(signingRoot),
    syncAggregate.signature
  )
  console.log("Signature valid:", valid)
  return { beaconHeader, executionPayload, proof, syncAggregate }
}

export async function fetchHeader(
  slot: number
): Promise<phase0.BeaconBlockHeader> {
  const endpoint = `${RELAY_ENDPOINT}/eth/v1/beacon/headers?slot=${slot}`
  const fetchOptions = {
    method: "GET",
  }

  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    throw new Error(
      "Oops! Fetch header faulted to error status: " + response.status
    )
  }

  const responseData = (await response.json()) as {
    data: Array<{
      header: {
        message: {
          slot: string
          proposer_index: string
          state_root: string
          body_root: string
        }
      }
    }>
  }

  const data = responseData[0]
  const beaconHeader: phase0.BeaconBlockHeader = {
    slot: parseInt(data.header.message.slot),
    proposerIndex: parseInt(data.header.message.proposer_index),
    parentRoot: fromHexString(data.header.message.parent_root),
    stateRoot: fromHexString(data.header.message.state_root),
    bodyRoot: fromHexString(data.header.message.body_root),
  }

  return beaconHeader
}

export const hashHeaderRoot = (header: phase0.BeaconBlockHeader) => {
  return toHexString(ssz.phase0.BeaconBlockHeader.hashTreeRoot(header))
}

export const fetchInitData = async (slot: number) => {
  const header = await fetchHeader(slot)
  const endpoint = `${LODESTAR_ENDPOINT}/eth/v1/beacon/light_client/bootstrap/${hashHeaderRoot(
    header
  )}`
  const fetchOptions = {
    method: "GET",
  }
  const response = await fetch(endpoint, fetchOptions)

  if (response.status !== 200) {
    throw new Error(
      "Oops! Fetch init data faulted to error status: " + response.status
    )
  }

  const responseData = (await response.json()) as {
    header: {
      beacon: { slot: string; proposer_index: string }
      execution: { block_number: string; block_hash: string }
    }
    current_sync_committee: {
      pubkeys: Array<Uint8Array>
      aggregate_pubkey: Uint8Array
    }
  }
  const checkpoint = {
    justified_epoch: parseInt(responseData.header.beacon.slot) / 32,
    justified_execution_height: parseInt(
      responseData.header.execution.block_number
    ),
    justified_block_hash: responseData.header.execution.block_hash,
    finalized_epoch: 0,
    finalized_execution_height: 0,
  }

  return {
    ...responseData,
    checkpoint,
  }
}

export const registerEthereumVerificationVendor = async (
  circuit: ApiPromise,
  gatewayData: Required<Gateway>
) => {
  const slot = await fetchLastSyncCommitteeUpdateSlot()
  const initData = await fetchInitData(slot)
  const headerRaw = await fetchHeader(slot)
  const currentSyncCommittee: SyncCommittee = {
    pubs: initData.current_sync_committee.pubkeys,
    aggr: initData.current_sync_committee.aggregate_pubkey,
  }
  // const response = await circuit.init(headerRaw, currentSyncCommittee)
  // console.log("Init response:", response)

  // const regData: EthereumInitializationData = {
  //   current_sync_committee: currentSyncCommittee,
  //   next_sync_committee: currentSyncCommittee,
  //   checkpoint: initData.checkpoint,
  //   beacon_header: initData.header.beacon,
  //   execution_payload: initData.header.execution,
  // }
  // encode the response
  // return it
}
