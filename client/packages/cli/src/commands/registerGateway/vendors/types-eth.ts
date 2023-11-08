import { phase0 } from '@lodestar/types'

export type BLSPubKey = Uint8Array

export type SyncCommittee = {
  pubs: Array<BLSPubKey | string>
  aggr: BLSPubKey | string
}

export interface CheckpointEntry {
  beacon: {
    epoch: number
    root: string
  }
  execution: {
    height: number
    root: string
  }
}

export interface Checkpoint {
  attested_beacon: {
    epoch: number
    root: string
  }
  attested_execution: {
    height: number
    root: string
  }
  justified_beacon: {
    epoch: number
    root: string
  }
  justified_execution: {
    height: number
    root: string
  }
  finalized_beacon: {
    epoch: number
    root: string
  }
  finalized_execution: {
    height: number
    root: string
  }
}

export interface InitData {
  checkpoint: Checkpoint
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
  attestedExecutionHeader: AttestedExecutionHeader
  currentSyncCommittee: SyncCommittee
  nextSyncCommittee: SyncCommittee
}

export interface BootstrapResponse {
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

export interface NextSyncCommitteeResponse {
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

export interface BeaconBlockHeader {
  root: string
  header: {
    message: {
      proposer_index: string
      parent_root: string
      state_root: string
      body_root: string
    }
  }
}

export type BeaconBlockHeaderMsgData = phase0.BeaconBlockHeader
export interface BeaconBlockHeaderAndRoot {
  header: BeaconBlockHeaderMsgData
  root: string
}

export interface BeaconBlockResponseData {
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

export interface AttestedExecutionHeader {
  parentHash: string
  ommersHash: string
  beneficiary: string
  stateRoot: string
  transactionsRoot: string
  receiptsRoot: string
  logsBloom: string
  difficulty: string // a number
  number: number
  gasLimit: number
  gasUsed: number
  timestamp: number
  extraData: string // a number
  mixHash: string
  nonce: string
  baseFeePerGas: number
  withdrawalsRoot: string
}
