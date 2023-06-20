// import "@t3rn/types"
import { AccountId32, H256 } from "@polkadot/types/interfaces/runtime"
import { T3rnPrimitivesSideEffect } from "@polkadot/types/lookup"
import { TextDecoder } from "util"
import { u8aToHex } from "@polkadot/util"
const BN = require("bn.js")

export enum TransactionType {
  Transfer,
  Swap,
  Erc20Transfer
}

// maps event names to TransactionType enum;
export const EventMapper = ["Transfer", "MultiTransfer"]

export class SideEffect {
  requester: any
  executor: any
  xtxId: any
  id: Uint8Array
  object: any
  confirmedSideEffect: any

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

  setSideEffect(sideEffect: any) {
    this.object = sideEffect
    this.setTransactionType()
  }

  getTransactionArguments() {
    switch (this.transactionType) {
      case TransactionType.Transfer: {
        return this.getTransferArguments()
      }
      case TransactionType.Erc20Transfer: {
        return this.getErc20TransferArguments()
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
    substrate: boolean = true,
  ) {
    if(substrate) {
      console.log("in substrate")
      this.confirmedSideEffect = {
        err: null,
        output: null,
        encodedEffect: encodedEffect,
        inclusionProof: null,
        executioner: executioner,
        receivedAt: blockNumber,
        cost: null,
      }
      this.executed = executed
      this.executor = executioner
      this.inclusionProof = inclusionProof
      this.execBlockHeader = blockHeader
    } else {
      console.log("in bsc")
      this.confirmedSideEffect = {
        err: null,
        output: null,
        encodedEffect: encodedEffect,
        inclusionProof: inclusionProof,
        executioner: executioner,
        receivedAt: blockNumber,
        cost: null,
      }
      this.executed = executed
      this.executor = executioner
      this.inclusionProof = null
      this.execBlockHeader = blockHeader
    }

    console.log("confirmed:", this.confirmedSideEffect)

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

  private getErc20TransferArguments() {
    return [
      this.object.encodedArgs[1].toHex(), // to
      this.object.encodedArgs[2].toHex(), // asset
      new BN(this.object.encodedArgs[3], "le").toString(), // amount
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
      case "etra": {
        this.transactionType = TransactionType.Erc20Transfer
        break
      }
    }
  }
}
