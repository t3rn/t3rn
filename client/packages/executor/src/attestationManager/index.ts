import { Sdk } from "@t3rn/sdk";
import { config } from "../../config/config";
import fs from "fs";
import Web3, { Contract } from "web3";
import { logger } from "../../src/logging";
import { Batch, ConfirmationBatch } from "./batch";
import { ethers } from "ethers";

interface IBatch {
  nextCommittee: string[];
  bannedCommittee: string[];
  committedSfx: string[];
  revertedSfx: string[];
  index: number;
  expectedBatchHash: string;
  signatures: [string, string][];
  created: number;
}

/**

 * @group Attestions
 */
export class AttestationManager {
  batches: ConfirmationBatch[] = [];
  rpc: string;
  receiveAttestationBatchContract: Contract<never>;
  web3: Web3;
  wallet: ReturnType<typeof this.web3.eth.accounts.privateKeyToAccount>;

  constructor(public client: Sdk["client"]) {
    if (config.attestations.ethereum.privateKey === undefined) {
      throw new Error("Ethereum private key is not defined");
    }

    this.client = client;
    this.rpc = config.attestations.ethereum.rpc;
    this.web3 = new Web3(this.rpc);

    this.wallet = this.web3.eth.accounts.privateKeyToAccount(
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
    logger.info("Fetching batches from chain...");
    const attesters = this.client.query.attesters;
    const rawBatches = await attesters.batches(
      config.attestations.ethereum.name
    );
    const fetchedData = rawBatches.toJSON() as unknown as Array<IBatch>;
    const batches = fetchedData.map((batch) => {
      return {
        nextCommittee: batch.nextCommittee || [],
        bannedCommittee: batch.bannedCommittee || [],
        committedSfx: batch.committedSfx || [],
        revertedSfx: batch.revertedSfx || [],
        index: batch.index,
        signatures: batch.signatures.map(([, signature]) => signature),
        created: batch.created,
      };
    });
    logger.info("We have " + fetchedData.length + " batches pending");
    return batches as unknown as ConfirmationBatch[];
  }

  async listener() {
    // Subscribe to all events and filter based on the specified event method
    this.client.query.system.events((events: Array<never>) => {
      events.forEach(
        (record: {
          event: { section: string; method: string; data: string[] };
        }) => {
          const { event } = record;

          // Check if the event type matches the specified event
          if (
            event.section === "attesters" &&
            event.method === "NewConfirmationBatch"
          ) {
            logger.debug(
              { data: event.data },
              `Event ${event.section}.${event.method} received`
            );
          }
        }
      );
    });
  }

  getMessageHash(batch: Batch) {
    const encodedBatch = this.batchEncodePacked(batch);
    const messageHash = ethers.keccak256(encodedBatch);
    return messageHash;
  }

  async getBlockHash(blockNumber: number): Promise<string> {
    const blockHash = await this.client.rpc.chain.getBlockHash(blockNumber);
    return blockHash.toHex();
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
    logger.debug("MsgHash with prefix: " + messageHash);

    return messageHash;
  }

  async processPendingAttestationBatches() {
    this.batches = await this.fetchBatches();

    // config.attestations.processPendingBatches is set to process pending batches
    for (const batch of this.batches.slice(
      config.attestations.processPendingBatchesIndex
    )) {
      logger.info(
        `Batch ${
          batch.index
        } processing, created at block ${await this.getBlockHash(
          batch.created
        )}`
      );

      const messageHash = this.getMessageHash(batch);
      logger.debug({ batch, messageHash }, `Batch data`);

      await this.receiveAttestationBatchCall(batch, messageHash);
    }
  }

  async receiveAttestationBatchCall(
    batch: ConfirmationBatch,
    messageHash: string
  ) {
    const committeeSize: number =
      await this.receiveAttestationBatchContract.methods.committeeSize().call();
    const currentBatchIndex = await this.receiveAttestationBatchContract.methods
      .currentBatchIndex()
      .call();
    const currentCommitteeTransitionCount =
      await this.receiveAttestationBatchContract.methods
        .currentCommitteeTransitionCount()
        .call();

    logger.debug(
      { committeeSize, currentBatchIndex, currentCommitteeTransitionCount },
      "Etherum Contract State"
    );
    const contractMethods = Object.keys(
      this.receiveAttestationBatchContract.methods
    );

    // debug signatures recovery
    let signatureErrors: number = 0;
    for (const signature of batch.signatures) {
      const address = await this.receiveAttestationBatchContract.methods
        .recoverSigner(
          // @ts-ignore - ethers.js types are not up to date
          messageHash,
          signature
        )
        .call();

      const result = await this.receiveAttestationBatchContract.methods
        .attestersIndices(
          // @ts-ignore - ethers.js types are not up to date
          ethers.getAddress(address)
        )
        .call();
      if (result != currentCommitteeTransitionCount) {
        logger.error(
          `Signature ${signature} is invalid, address ${address} is not in the current committee`
        );
        signatureErrors++;
      } else {
        logger.info(
          `Signature ${signature} is valid, address ${address} is in the current committee`
        );
      }
    }

    if (
      BigInt(signatureErrors) >
      (BigInt(2) / BigInt(3)) * BigInt(committeeSize)
    ) {
      logger.error(
        `Batch ${batch.index} processing failed, ${
          batch.signatures.length - signatureErrors
        }/${batch.signatures.length} signatures are valid`
      );
      throw new Error();
    } else {
      logger.info(`Batch ${batch.index} processing, all signatures are valid`);
    }

    const contractMethod =
      this.receiveAttestationBatchContract.methods.receiveAttestationBatch(
        // @ts-ignore - ethers.js types are not up to date
        batch.nextCommittee,
        batch.bannedCommittee,
        batch.committedSfx,
        batch.revertedSfx,
        batch.index,
        ethers.toQuantity(messageHash),
        batch.signatures
      );

    const encodedABI = contractMethod.encodeABI();

    const gasPrice = await this.web3.eth.getGasPrice();
    const estimatedGas = await contractMethod.estimateGas({
      from: this.wallet.address,
    });

    const transactionObject = {
      to: this.receiveAttestationBatchContract.options.address,
      from: this.wallet.address,
      data: encodedABI,
      gas: 500000,
      gasPrice: gasPrice,
      estimatedGas: estimatedGas,
    };

    const signedTransaction = await this.wallet.signTransaction(
      transactionObject
    );

    try {
      const transactionReceipt = await this.web3.eth.sendSignedTransaction(
        signedTransaction.rawTransaction
      );
      logger.info(
        { receipt: transactionReceipt },
        `Batch ${batch.index} procesed!`
      );
    } catch (error) {
      logger.warn({ error: error }, "Error sending transaction: ");
      // throw new Error("Error sending transaction: " + error);
    }
  }

  async listenForConfirmedAttestationBatch() {
    throw new Error("Not implemented");
  }
}
