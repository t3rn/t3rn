// @ts-ignore
import { Sdk } from "@t3rn/sdk";
import { config } from "../../config/config";
import fs from "fs";
import Web3, { Contract } from "web3";
import { logger } from "../../src/logging";
import { Batch, ConfirmationBatch } from "./batch";
import { ethers } from "ethers";
import { Prometheus } from "../prometheus";

// TODO: Move to config
const RETRY_DELAY_MS = 30000;

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
  currentCommitteeSize = 0;
  currentBatchIndex = 0;
  currentCommitteeTransitionCount = 0;
  prometheus: Prometheus;

  constructor(
    public client: Sdk["client"],
    prometheus: Prometheus,
  ) {
    if (config.attestations.ethereum.privateKey === undefined) {
      throw new Error("Ethereum private key is not defined");
    }

    this.client = client;
    this.rpc = config.attestations.ethereum.rpc;
    this.web3 = new Web3(this.rpc);

    this.wallet = this.web3.eth.accounts.privateKeyToAccount(
      config.attestations.ethereum.privateKey,
    );

    if (this.wallet.address === undefined) {
      throw new Error("Ethereum wallet address is not defined");
    }

    logger.info("Ethereum Wallet address: " + this.wallet.address);
    // TODO: add balance here

    const receiveAttestationBatchAbi = JSON.parse(
      fs.readFileSync(
        "./src/attestationManager/contracts/AttestationsVerifier.abi.json",
        "utf8",
      ),
    );
    const receiveAttestationBatchAddress =
      config.attestations.ethereum.attestationVerifierAddress;
    this.receiveAttestationBatchContract = new this.web3.eth.Contract(
      receiveAttestationBatchAbi.abi,
      receiveAttestationBatchAddress,
    );
    this.prometheus = prometheus;
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
      ],
    );
  }

  async fetchBatches() {
    logger.info("Fetching batches from chain...");
    const attesters = this.client.query.attesters;
    const rawBatches = await attesters.batches(
      config.attestations.ethereum.name,
    );
    const fetchedData = rawBatches.toJSON() as unknown as Array<IBatch>;
    this.batches = fetchedData.map((batch) => {
      return {
        nextCommittee: batch.nextCommittee || [],
        bannedCommittee: batch.bannedCommittee || [],
        committedSfx: batch.committedSfx || [],
        revertedSfx: batch.revertedSfx || [],
        index: batch.index,
        signatures: batch.signatures.map(([, signature]) => signature),
        created: batch.created,
      } as ConfirmationBatch;
    });
    logger.info("We have " + fetchedData.length + " batches pending");
    this.prometheus.attestationsBatchesPending.set(fetchedData.length);
  }

  async listener() {
    logger.info("Listening for NewConfirmationBatch events...");
    // Subscribe to all events and filter based on the specified event method
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.client.query.system.events(async (events: any) => {
      if (events === undefined || events === null) {
        return;
      }

      logger.debug({ events_count: events.length }, `Received events`);

      const attesterEvents = events
        .toHuman()
        .filter((event) => event.event.section == "attesters");

      await this.processConfirmedBatches(attesterEvents);
    });
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private async processConfirmedBatches(events: any) {
    // Loop through the Vec<EventRecord>
    await Promise.all(
      events.map(async (record) => {
        // Extract the phase, event and the event types
        const { event } = record;

        this.prometheus.attestationEvents.inc({
          event: event.toHuman().method,
        });
        if (record.event.method != "NewConfirmationBatch") {
          return;
        }

        logger.debug(
          {
            event: event.method,
            data: event.data,
          },
          "Received attesters NewConfirmationBatch event",
        );

        const batch = event.data as ConfirmationBatch;
        logger.debug(
          {
            batch,
          },
          "Batch debug",
        );
        const messageHash = this.getMessageHash(batch);

        await this.receiveAttestationBatchCall(batch, messageHash);
      }),
    );
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
  getMessageHashWithPrefix(batch: Batch) {
    const prefix = ethers.hexlify(
      ethers.toUtf8Bytes("\x19Ethereum Signed Message:\n32"),
    );
    const encodedBatch = ethers.keccak256(this.batchEncodePacked(batch));
    const messageHash = ethers.keccak256(ethers.concat([prefix, encodedBatch]));
    logger.debug("MsgHash with prefix: " + messageHash);

    return messageHash;
  }

  async processAttestationBatches() {
    await this.fetchAttesterContractData();
    await this.fetchBatches();

    if (this.batches.length > this.currentBatchIndex) {
      logger.info(
        `We have ${
          this.batches.length - this.currentBatchIndex
        } pending batches to process`,
      );
      this.processPendingAttestationBatches();
    }

    await this.listener();
  }

  async processPendingAttestationBatches() {
    for (const batch of this.batches.slice(
      Number(this.currentBatchIndex) + 1,
    )) {
      logger.info(
        `Batch ${
          batch.index
        } processing, created at block ${await this.getBlockHash(
          batch.created,
        )}`,
      );

      const messageHash = this.getMessageHash(batch);
      logger.debug({ batch, messageHash }, `Batch data`);

      let attestationSubmitSuccess = false;

      while (!attestationSubmitSuccess) {
        try {
          await this.receiveAttestationBatchCall(batch, messageHash);
          attestationSubmitSuccess = true;
        } catch (error) {
          const errorName = error.innerError.message.match(
            /^execution reverted: (.*)/,
          )[1]; // Parse stack trace to get exact error which is not present in error object
          logger.warn({ errorMessage: errorName }, `Error processing batch`);
          this.prometheus.attestatonBatchesFailed.inc({ error: errorName });
          await new Promise((resolve) => setTimeout(resolve, RETRY_DELAY_MS));
        }
      }
    }
  }

  async receiveAttestationBatchCall(
    batch: ConfirmationBatch,
    messageHash: string,
  ) {
    const contractMethod =
      this.receiveAttestationBatchContract.methods.receiveAttestationBatch(
        // @ts-ignore - ethers.js types are not up to date
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

    const signedTransaction =
      await this.wallet.signTransaction(transactionObject);

    const transactionReceipt = await this.web3.eth.sendSignedTransaction(
      signedTransaction.rawTransaction,
    );
    logger.info(
      { receipt: transactionReceipt },
      `Batch ${batch.index} procesed!`,
    );
    this.prometheus.attestationBatchesProcessed.inc();
  }

  private async fetchAttesterContractData() {
    this.currentCommitteeSize = Number(
      await this.receiveAttestationBatchContract.methods.committeeSize().call(),
    );
    this.prometheus.attestationVerifierCurrentCommitteeSize.set(
      this.currentCommitteeSize,
    );

    this.currentBatchIndex = Number(
      await this.receiveAttestationBatchContract.methods
        .currentBatchIndex()
        .call(),
    );
    this.prometheus.attestationVerifierCurrentBatchIndex.set(
      this.currentBatchIndex,
    );

    this.currentCommitteeTransitionCount = Number(
      await this.receiveAttestationBatchContract.methods
        .currentCommitteeTransitionCount()
        .call(),
    );
    this.prometheus.attestationVerifierCurrentCommitteeTransitionCount.set(
      this.currentCommitteeTransitionCount,
    );

    logger.debug(
      {
        currentCommitteeSize: this.currentCommitteeSize,
        currentBatchIndex: this.currentBatchIndex,
        currentCommitteeTransitionCount: this.currentCommitteeTransitionCount,
      },
      "Etherum Contract State",
    );
  }
}
