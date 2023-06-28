import { Sdk } from "@t3rn/sdk";
import { config } from "../../config/config";
import fs from "fs";
import Web3 from "web3";
import { logger } from "../../src/logging";
import { Batch, ConfirmationBatch } from "./batch";
import { keccakAsHex } from "@polkadot/util-crypto";
import { ethers } from "ethers";
import { t } from "@t3rn/sdk/dist/types-922c6188";
// import { keccak256 } from 'ethereumjs-util';

/**

 * @group Attestions
 */
export class AttestationManager {
  web3: any;
  rpc: string;
  receiveAttestationBatchContract: any;
  wallet: any;
  client: any;
  batches: any;

  constructor(client: Sdk["client"]) {
    if (config.attestations.ethereum.privateKey === undefined) {
      throw new Error("Ethereum private key is not defined");
    }

    this.client = client;
    this.rpc = config.attestations.ethereum.rpc;
    this.web3 = new Web3(this.rpc);

    this.wallet = this.web3.eth.accounts.create(
      config.attestations.ethereum.privateKey
    );
    if (this.wallet.address === undefined) {
      throw new Error("Ethereum wallet address is not defined");
    }
    logger.info("Wallet address: " + this.wallet.address);

    const receiveAttestationBatchAbi = JSON.parse(
      fs.readFileSync(
        "./src/attestationManager/contracts/AttestationsVerifier.abi.json",
        "utf8"
      )
    );
    const receiveAttestationBatchAddress =
      config.attestations.ethereum.attestationVerifierAddress;
    this.receiveAttestationBatchContract = new this.web3.eth.Contract(
      receiveAttestationBatchAbi.abi,
      receiveAttestationBatchAddress
    );
  }

  batchEncodePacked(batch: Batch) {
    return ethers.solidityPacked(
      ["address[]", "address[]", "bytes32[]", "bytes32[]", "uint32"],
      [
        batch.nextCommittee,
        batch.bannedCommittee,
        batch.committedSfx,
        batch.revertedSfx,
        batch.index,
      ]
    );
  }

  async fetchBatches() {
    await this.client.query.attesters.batches("sepl").then((data) => {
      const fetchedData: any = data.toJSON();

      const convertedData: ConfirmationBatch[] = fetchedData.map(
        (batch: any) => {
          return {
            nextCommittee: batch.nextCommittee || [],
            bannedCommittee: batch.bannedCommittee || [],
            committedSfx: batch.committedSfx || [],
            revertedSfx: batch.revertedSfx || [],
            index: batch.index,
            expectedBatchHash: batch.expectedBatchHash,
            signatures: batch.signatures.map(([id, signature]) => signature),
            created: batch.created,
          };
        }
      );

      logger.info("We have " + data.toJSON().length + " batches pending");
      // logger.info(['Batch 0: ', data.toJSON()[0]])
      // logger.info(['Batch 1: ', data.toJSON()[1]])
      this.batches = convertedData;
    });
  }

  async listener() {
    // Subscribe to all events and filter based on the specified event method
    this.client.query.system.events((events) => {
      events.forEach((record) => {
        const { event } = record;

        // Check if the event type matches the specified event
        if (
          event.section === "attesters" &&
          event.method === "NewConfirmationBatch"
        ) {
          logger.debug({data: event.data}, `Event ${event.section}.${event.method} received`);
        }
      });
    });
  }

  getMessageHash(batch: Batch) {
    const encodedBatch = this.batchEncodePacked(batch);
    const messageHash = ethers.keccak256(encodedBatch);
    return messageHash;
  }

  // TODO: not used
  getMessageHashFromString(data: string) {
    const messageHash = ethers.keccak256(data);
    return messageHash;
  }

  // TODO: not used
  getMessageHashWithPrefix(batch: Batch) {
    const prefix = ethers.hexlify(
      ethers.toUtf8Bytes("\x19Ethereum Signed Message:\n32")
    );
    const encodedBatch = ethers.keccak256(this.batchEncodePacked(batch));
    const messageHash = ethers.keccak256(ethers.concat([prefix, encodedBatch]));
    logger.error("MsgHash with prefix: " + messageHash);

    return messageHash;
  }

  async getMessageHashFromCircuit(blockNumber: number) : Promise<string> {
    const blockHash = await this.client.rpc.chain.getBlockHash(blockNumber)
    logger.debug(`Looking for messageHash in ${blockNumber}/${blockHash.toHex()}`)
    const events = await this.client.query.system.events.at(blockHash.toHex())
    logger.debug(`Found ${events.length} events in ${blockHash.toHex()}`); 

    const filteredEvents = events
    .toHuman()
    .filter((event) => event.event.method == 'NewAttestationMessageHash')

    logger.debug(`Found ${filteredEvents.length} NewAttestationMessageHash events`); 

    logger.error(filteredEvents)

    for (const event of filteredEvents) {
        const messageHash = event.event.data[1]
        logger.debug(`Found messageHash: ${messageHash}`)
        return messageHash
    }

    throw new Error(`No messageHash found in block ${blockNumber}`)
  }

  async processPendingAttestationBatches() {
    await this.fetchBatches();

    // iterate over batches, ignore empty 0 batch
    // 0 batch didnt have a message hash in block found in batch.created
    for (const batch of this.batches.slice(1)) {
      logger.info({batch: batch}, `Batch ${batch.index} processing`);
      const messageHash = await this.getMessageHashFromCircuit(batch.created);
      this.receiveAttestationBatchCall(batch, messageHash);
    }
  }

  async receiveAttestationBatchCall(batch: ConfirmationBatch, messageHash: string) {
    const committeeSize = await this.receiveAttestationBatchContract.methods
      .committeeSize()
      .call();
    logger.debug("Committee size: " + committeeSize);
    const currentBatchIndex = await this.receiveAttestationBatchContract.methods
      .currentBatchIndex()
      .call();
    logger.debug("Batch Index: " + currentBatchIndex);

    const contractMethod = this.receiveAttestationBatchContract.methods.receiveAttestationBatch(
      batch.nextCommittee,
      batch.bannedCommittee,
      batch.committedSfx,
      batch.revertedSfx,
      batch.index,
      messageHash,
      batch.signatures,
    );
    
    const encodedABI = contractMethod.encodeABI();
    
    const gasPrice = await this.web3.eth.getGasPrice();
    const estimatedGas = await contractMethod.estimateGas({ from: this.wallet.address });

    const transactionObject = {
      to: this.receiveAttestationBatchContract.options.address,
      from: this.wallet.address,
      data: encodedABI,
      gas: gasPrice, // Set the gas limit accordingly
      estimatedGas: estimatedGas,
    };
    
    const signedTransaction = await this.wallet.signTransaction(transactionObject);
    
    try {
      const transactionReceipt = await this.web3.eth.sendSignedTransaction(signedTransaction.rawTransaction);
      logger.info('Transaction receipt:', transactionReceipt);
    } catch (error) {
      logger.error('Error:', error);
    }
  }

  async listenForConfirmedAttestationBatch() {
    throw new Error("Not implemented");
  }
}
