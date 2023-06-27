import { Sdk } from "@t3rn/sdk";
import { config } from "../../config/config";
import fs from "fs";
import Web3 from "web3";
import { logger } from "../../src/logging";
import { Batch, ConfirmationBatch } from "./batch";
import { keccakAsHex } from "@polkadot/util-crypto";
import { ethers } from "ethers";
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

    this.wallet = this.web3.eth.accounts.privateKeyToAccount(
      config.attestations.ethereum.privateKey
    );
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
          };
        }
      );

      // console.log('Batches: ', data.toHuman())
      logger.info("We have " + data.toJSON().length + " batches");
      // logger.info(['Batch 0: ', data.toJSON()[0]])
      // logger.info(['Batch 1: ', data.toJSON()[1]])
      this.batches = convertedData;
    });
  }

  async listener() {
    // Subscribe to all events and filter based on the specified event type
    this.client.query.system.events((events) => {
      // Iterate through the events and filter based on the event type
      events.forEach((record) => {
        const { event } = record;

        // Check if the event type matches the specified event type
        if (
          event.section === "attesters" &&
          event.method === "NewConfirmationBatch"
        ) {
          // Event of the specified type found, process it
          logger.error(`Event: ${event.section}.${event.method}`);
          logger.error(`Data: ${event.data}`);
        }
      });
    });
  }

  getMessageHash(batch: Batch) {
    const encodedBatch = this.batchEncodePacked(batch);
    const messageHash = ethers.keccak256(encodedBatch);
    return messageHash;
  }

  getMessageHashWithPrefix(batch: Batch) {
    const prefix = ethers.hexlify(
      ethers.toUtf8Bytes("\x19Ethereum Signed Message:\n32")
    );
    const encodedBatch = ethers.keccak256(this.batchEncodePacked(batch));
    const messageHash = ethers.keccak256(ethers.concat([prefix, encodedBatch]));
    logger.error("MsgHash with prefix: " + messageHash);

    return messageHash;
  }

  async receiveAttestationBatchCall() {
    // console.log([this.receiveAttestationBatchContract.methods])
    const committeeSize = await this.receiveAttestationBatchContract.methods
      .committeeSize()
      .call();
    console.log(["Committee size: ", committeeSize]);
    const currentBatchIndex = await this.receiveAttestationBatchContract.methods
      .currentBatchIndex()
      .call();
    console.log(["Batch Index: ", currentBatchIndex]);

    const batch = {
      nextCommittee: this.batches[1].nextCommittee,
      bannedCommittee: this.batches[1].bannedCommittee,
      committedSfx: this.batches[1].committedSfx,
      revertedSfx: this.batches[1].revertedSfx,
      index: this.batches[0].index,
    };
    const messageHash = this.getMessageHash(batch);
    // const messageHash = this.getMessageHash(batch)

    // fetch batch 0 from t0rn
    logger.error([batch, messageHash, this.batches[1].signatures]);

    throw Error("stop");
    // TODO: align naming of the contract with circuit
    // await this.receiveAttestationBatchContract.methods.receiveAttestationBatch(
    //     this.batches[1].nextCommittee,
    //     this.batches[1].bannedCommittee,
    //     this.batches[1].committedSfx,
    //     this.batches[1].revertedSfx,
    //     this.batches[1].index,
    //     messageHash,
    //     this.batches[1].signatures,
    // )
    //   .send({ from: this.wallet.address, gas: 2000000 })
    //   .on('transactionHash', (hash) => {
    //     logger.info('Transaction hash:', hash);
    //   })
    //   .on('receipt', (receipt) => {
    //     logger.info('Receipt:', receipt);
    //   })
    //   .on('error', (error) => {
    //     logger.error({error: error, request: error.request}, 'Error:');
    //   });
  }

  async listenForConfirmedAttestationBatch() {
    throw new Error("Not implemented");
  }
}
