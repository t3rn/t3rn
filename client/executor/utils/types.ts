import "@t3rn/types"
import { AccountId32, H256 } from "@polkadot/types/interfaces/runtime"
import { T3rnTypesSideEffect } from "@polkadot/types/lookup"
import { TextDecoder } from "util"
import { u8aToHex } from "@polkadot/util"
const BN = require("bn.js")

export enum TransactionType {
  Transfer,
  Swap,
}

// maps event names to TransactionType enum;
export const EventMapper = ["Transfer", "MultiTransfer"]

export class SideEffect {
  requester: any
  executor: any
  xtxId: any
  id: Uint8Array
  object: T3rnTypesSideEffect
  confirmedSideEffect: object

  inclusionProof: any
  execBlockHeader: any

  transactionType: TransactionType
  executed: boolean
  confirmed: boolean
  confirmBlockHeader: any

  setRequester(requester: any) {
    this.requester = requester
  }

  setXtxId(xtxId: any) {
    this.xtxId = xtxId
  }

  setSideEffect(sideEffect: T3rnTypesSideEffect) {
    this.object = sideEffect
    this.setTransactionType()
  }

  getTransactionArguments() {
    switch (this.transactionType) {
      case TransactionType.Transfer: {
        return this.getTransferArguments()
      }
      case TransactionType.Swap: {
        return []
      }
    }
  }

  execute(
    encodedEffect: any,
    blockNumber: number,
    executioner: any,
    inclusionProof: any,
    blockHeader: any,
    executed: boolean,
  ) {
    const inclusionData = { // will be encoded by circuit relayer
      encoded_payload: encodedEffect,
      proof: {
            trieNodes: inclusionProof.toJSON().proof
      },
      block_hash: blockHeader
    }

    this.confirmedSideEffect = {
      err: null,
      output: null,
      inclusion_data: inclusionData,
      executioner: executioner,
      receivedAt: blockNumber,
      cost: null,
    }

    this.executed = true
    this.executor = executioner
    this.inclusionProof = inclusionProof
    this.execBlockHeader = blockHeader
  }

  confirm(confirmed: boolean, blockHeader: any) {
    this.confirmed = confirmed
    this.confirmBlockHeader = blockHeader
  }

  setId(id: Uint8Array) {
    this.id = id
  }

  getId() {
    return u8aToHex(this.id)
  }

  /// returns xtxId as string
  getXtxId() {
    return this.xtxId.toString()
  }

  /// returns target as string
  getTarget() {
    return new TextDecoder().decode(this.object.target.toU8a())
  }

  getTargetBlock() {
    if (!this.executed) return null
    // @ts-ignore
    return this.confirmedSideEffect.receivedAt
  }

  private getTransferArguments() {
    return [
      this.object.encodedArgs[1],
      new BN(this.object.encodedArgs[2], "le").toString(),
    ]
  }

  private setTransactionType() {
    switch (this.object.encodedAction.toHuman()) {
      case "tran": {
        this.transactionType = TransactionType.Transfer
        break
      }
      case "swap": {
        this.transactionType = TransactionType.Swap
        break
      }
    }
  }
}
