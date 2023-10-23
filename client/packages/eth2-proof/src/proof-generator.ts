import RLP from 'rlp'
import { BaseTrie as Trie } from 'merkle-patricia-tree'
import Web3, { TransactionReceipt } from 'web3'
import { TransactionFactory, TypedTransaction } from 'web3-eth-accounts'
import { HexString32Bytes, TransactionInfo } from 'web3-types'
import * as utils from 'web3-utils'
import { sleep } from './helpers'

export class ProofGenerator {
  private readonly web3: Web3

  constructor(rpcUrl: string) {
    this.web3 = new Web3(rpcUrl)
  }

  static encode(input: any): Uint8Array {
    return input === '0xs0' ? RLP.encode(Buffer.alloc(0)) : RLP.encode(input)
  }

  static receiptToRlp(receipt: TransactionReceipt): Uint8Array {
    let encodedLegacy = RLP.encode([
      receipt.status ? '0x1' : '0x',
      (receipt.cumulativeGasUsed as number) > 0
        ? utils.toHex(receipt.cumulativeGasUsed)
        : '0x',
      receipt.logsBloom,
      receipt.logs.map((log: any) => [log.address, log.topics, log.data]),
    ])

    if (!!receipt.type && receipt.type !== '0x0') {
      const transactionType = parseInt(receipt.type.toString())
      const concat = new Uint8Array(encodedLegacy.byteLength + 1)
      const version = new Uint8Array([transactionType])
      concat.set(version, 0)
      concat.set(new Uint8Array(encodedLegacy), 1)
      return concat
    }

    return encodedLegacy
  }

  async generateTxReceiptProof(
    txHash: string,
  ): Promise<{ proof: string[]; root: string; value: string }> {
    // assume event attached will be taken from index = 0 if exists
    const eventIndex = 0
    const receipt: TransactionReceipt =
      await this.web3.eth.getTransactionReceipt(txHash)

    console.log('⬅️ found receipt for tx: ', txHash)
    console.log('🔃 parsed receipt to hex form') // console.log if u will (seems too long to show in command line output) utils.toHex(receiptToRlp(receipt))
    const block = await this.web3.eth.getBlock(
      receipt.blockHash as HexString32Bytes,
    )
    console.log('⬅️ found block for receipt: ', block.hash, block.number)
    console.log(`🔃 fetch sibling tx receipts: ${block.transactions.length}`)

    // let siblings: TransactionReceipt[] = await Promise.all(
    //   // @ts-ignore
    //   block.transactions.map(async (txId: string) => {
    //     let sibling: TransactionReceipt =
    //       await this.web3.eth.getTransactionReceipt(txId)
    //     return sibling
    //   }),
    // )
    let siblings: TransactionReceipt[] = []
    // @ts-ignore
    for (let txHash of block.transactions) {
      const sibling: TransactionReceipt =
        await this.web3.eth.getTransactionReceipt(txHash as string)
      siblings.push(sibling)
    }
    console.log(`⬅️fetched all ${siblings.length} sibling transaction receipts`)
    const proofOutput = await ProofGenerator.calculateReceiptProof(
      siblings,
      receipt.transactionIndex as number,
    )

    console.log(receipt.logs)
    const event0 = receipt.logs[eventIndex]
    const eventAsUint8Array = !!event0
      ? ProofGenerator.encode([event0.address, event0.topics, event0.data])
      : Uint8Array.from([])

    const proofOutputHex = {
      proof: proofOutput.proof.map((node: Buffer) => node.toString('hex')),
      root: proofOutput.root.toString('hex'),
      index: ProofGenerator.encode(receipt.transactionIndex as number),
      value: proofOutput.value.toString('hex'),
      event: Buffer.from(eventAsUint8Array).toString('hex'),
    }
    // console.log("🧮generated proof for tx: ", proofOutputHex.proof);
    console.log(
      '🧮proof-calculated receipts root vs block receipts root: ',
      '0x' + proofOutputHex.root,
      block.receiptsRoot,
    )

    return proofOutputHex
  }

  /**
   * The IPLD block is the consensus encoding of the transaction:
   * Legacy transaction encoding: RLP([AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data, V, R, S]).
   * The V, R, S elements of this transaction either represent a secp256k1 signature over KECCAK_256(RLP([AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data])) OR over KECCAK_256(RLP([AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data, ChainID, 0, 0])) as described by EIP-155.
   * Access list (EIP-2930) transaction encoding: 0x01 || RLP([ChainID, AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data, AccessList, V, R, S].
   * The V, R, S elements of this transaction represent a secp256k1 signature over KECCAK_256(0x01 || RLP([ChainID, AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data, AccessList]).
   * || is the byte/byte-array concatenation operator.
   * Dynamic fee (EIP-1559) transaction encoding: 0x02 || RLP([ChainID, AccountNonce, GasTipCap, maxFeePerGas, GasFeeCap, Recipient, Amount, Data, AccessList, V, R, S]
   * The V, R, S elements of this transaction represent a secp256k1 signature over KECCAK_256(0x02 || RLP([ChainID, AccountNonce, GasTipCap, maxFeePerGas, GasFeeCap, Recipient, Amount, Data, AccessList]
   */
  static encodeTxAsValidRLP(tx: TransactionInfo): Buffer {
    if (tx.type == 0 || tx.type === undefined) {
      // Legacy transaction encoding: RLP([AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data, V, R, S]).
      // The V, R, S elements of this transaction either represent a secp256k1 signature over KECCAK_256(RLP([AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data])) OR over KECCAK_256(RLP([AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data, ChainID, 0, 0])) as described by EIP-155.
      let legacyTransactionEncoded: Uint8Array = ProofGenerator.encode([
        tx.nonce,
        tx.gasPrice,
        tx.gas,
        tx.to || undefined,
        tx.value,
        tx.input,
        tx.v,
        tx.r,
        tx.s,
      ])

      return Buffer.from(legacyTransactionEncoded)
    } else if (tx.type == 1) {
      // Access list (EIP-2930) transaction encoding: 0x01 || RLP([ChainID, AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data, AccessList, V, R, S].
      // The V, R, S elements of this transaction represent a secp256k1 signature over KECCAK_256(0x01 || RLP([ChainID, AccountNonce, GasPrice, GasLimit, Recipient, Amount, Data, AccessList]).
      // || is the byte/byte-array concatenation operator.
      let accessListTransactionEncoded: Uint8Array = ProofGenerator.encode([
        tx.chainId,
        tx.nonce,
        tx.gasPrice,
        tx.gas,
        tx.to || undefined,
        tx.value,
        tx.input,
        tx.accessList,
        tx.v,
        tx.r,
        tx.s,
      ])

      const TRANSACTION_TYPE = 1
      const TRANSACTION_TYPE_BUFFER = Buffer.from(
        TRANSACTION_TYPE.toString(16).padStart(2, '0'),
        'hex',
      )

      return Buffer.concat([
        TRANSACTION_TYPE_BUFFER,
        accessListTransactionEncoded,
      ])
    } else if (tx.type == 2) {
      // Dynamic fee (EIP-1559) transaction encoding: 0x02 || RLP([ChainID, AccountNonce, GasTipCap, maxFeePerGas, GasFeeCap, Recipient, Amount, Data, AccessList, V, R, S]
      // The V, R, S elements of this transaction represent a secp256k1 signature over KECCAK_256(0x02 || RLP([ChainID, AccountNonce, GasTipCap, maxFeePerGas, GasFeeCap, Recipient, Amount, Data, AccessList]
      let eip1559TransactionEncoded: Uint8Array = ProofGenerator.encode([
        tx.chainId,
        tx.nonce,
        tx.maxPriorityFeePerGas,
        tx.maxFeePerGas,
        tx.gas, // gasFeeCap
        tx.to || undefined,
        tx.value,
        tx.input,
        tx.accessList,
        tx.v,
        tx.r,
        tx.s,
      ])

      const TRANSACTION_TYPE = 2
      const TRANSACTION_TYPE_BUFFER = Buffer.from(
        TRANSACTION_TYPE.toString(16).padStart(2, '0'),
        'hex',
      )

      return Buffer.concat([TRANSACTION_TYPE_BUFFER, eip1559TransactionEncoded])
    }

    return Buffer.from([])
  }

  async generateStateProof(
    accountId: string,
    storageId: string,
    blockNumber: number | string,
  ) {
    // Get the state root hash
    const block = await this.web3.eth.getBlock(blockNumber)
    const blockHash = block.hash

    console.log('⬅️found block matching block number: ', blockNumber, blockHash)
    console.log(`🔃block's state_root = ${block.stateRoot}`)

    let rpcProof = await this.web3.eth.getProof(
      accountId,
      [storageId],
      blockNumber,
    )

    // @ts-ignore
    rpcProof.blockStateRoot = block.stateRoot
    return rpcProof
  }

  async generateTransactionProof(txId: string) {
    const tx: TransactionInfo = await this.web3.eth.getTransaction(txId)

    console.log('⬅️found transaction matching ID: ', txId)
    const typedTransaction: TypedTransaction = TransactionFactory.fromTxData({
      nonce: tx.nonce,
      gasPrice: tx.gasPrice,
      gasLimit: tx.gas,
      to: tx.to || undefined,
      value: tx.value,
      data: tx.input,
      v: tx.v,
      r: tx.r,
      s: tx.s,
      type: tx.type,
    })
    console.log('🔃serialized transaction to RLP form') // console.log if u will (seems too long to show in command line output) utils.toHex(typedTransaction.serialize())
    const block = await this.web3.eth.getBlock(tx.blockHash as HexString32Bytes)
    console.log('⬅️found block for receipt: ', block.hash, block.number)
    let siblings: TransactionInfo[] = await Promise.all(
      // @ts-ignore
      block.transactions.map(async (txId: string) => {
        let sibling: TransactionInfo = await this.web3.eth.getTransaction(txId)
        return sibling
      }),
    )
    console.log(`⬅️fetched all ${siblings.length} sibling transactions`)
    let proofOutput = await ProofGenerator.calculateTransactionProof(
      siblings,
      tx.transactionIndex as number,
    )
    const proofOutputHex = {
      proof: proofOutput.proof.map((node: Buffer) => node.toString('hex')),
      root: proofOutput.root.toString('hex'),
      index: ProofGenerator.encode(tx.transactionIndex as number),
      value: proofOutput.value.toString('hex'),
    }

    console.log(
      '🧮proof-calculated transactions root vs block transactions root: ',
      '0x' + proofOutputHex.root,
      block.transactionsRoot,
    )

    return proofOutputHex
  }

  async getBlock(blockId: string) {
    await sleep(2) // need to wait for RPC to by synced
    const block = await this.web3.eth.getBlock(blockId).catch((err: any) => {
      console.log('errrrr')
      console.log(err)
    })
    return block
  }

  static async calculateReceiptProof(
    receipts: TransactionReceipt[],
    index: number,
  ): Promise<{ proof: Buffer[]; root: Buffer; value: Buffer }> {
    let trie = new Trie()

    for (let i = 0; i < receipts.length; i++) {
      const entry = receipts[i]
      const keyAsRlpEncodedTxIndex = ProofGenerator.encode(
        entry.transactionIndex as number,
      )
      const valueAsRlpEncodedReceipt = ProofGenerator.receiptToRlp(entry)
      await trie.put(
        Buffer.from(keyAsRlpEncodedTxIndex),
        Buffer.from(valueAsRlpEncodedReceipt),
      )
    }

    const proof = await Trie.createProof(
      trie,
      Buffer.from(ProofGenerator.encode(index)),
    )
    console.log('Computed Root: ', trie.root.toString('hex'))
    const verifyResult = await Trie.verifyProof(
      trie.root,
      Buffer.from(ProofGenerator.encode(index)),
      proof,
    )
    if (verifyResult === null) {
      throw new Error('Proof is invalid')
    }
    const value: Buffer = verifyResult

    return {
      proof,
      root: trie.root,
      value,
    }
  }

  static async calculateTransactionProof(
    transactions: TransactionInfo[],
    index: number,
  ): Promise<{ proof: Buffer[]; root: Buffer; value: Buffer }> {
    let trie = new Trie()

    for (let i = 0; i < transactions.length; i++) {
      const entry = transactions[i]
      const keyAsRlpEncodedTxIndex = ProofGenerator.encode(
        entry.transactionIndex as number,
      )
      const valueAsRlpEncodedTransaction =
        ProofGenerator.encodeTxAsValidRLP(entry)
      await trie.put(
        Buffer.from(keyAsRlpEncodedTxIndex),
        Buffer.from(valueAsRlpEncodedTransaction),
      )
    }

    const proof = await Trie.createProof(
      trie,
      Buffer.from(ProofGenerator.encode(index)),
    )
    const verifyResult = await Trie.verifyProof(
      trie.root,
      Buffer.from(ProofGenerator.encode(index)),
      proof,
    )
    if (verifyResult === null) {
      throw new Error('💣💣💣 Proof is invalid 💣💣💣')
    }
    const value: Buffer = verifyResult

    return {
      proof,
      root: trie.root,
      value,
    }
  }
}
