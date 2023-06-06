import { EventEmitter } from "events";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { EventMapper, SideEffect } from "../../executionManager/sideEffect";
import { getEventProofs } from "../../utils";
import { SubmittableExtrinsic } from "@polkadot/api/promise/types";
import { SfxType } from "@t3rn/sdk/dist/side-effects/types";
import { InclusionProof, RelayerEventData, RelayerEvents } from "../types";
import Estimator from "./estimator";
import { CostEstimator, Estimate } from "./estimator/cost";
import { Utils } from "@t3rn/sdk";
import { Gateway } from "../../../config/config";

/**
 * Class responsible for submitting transactions to a target chain. Three main tasks are handled by this class:
 *
 * - Build correct TX objects for the target chain
 * - Sign and submit the TXs
 * - Generate inclusion proofs for the TXs
 *
 * @category Substrate
 * @group Gateways
 */
export class SubstrateRelayer extends EventEmitter {
  /** Target chain client */
  client: ApiPromise;
  /** Signer for target */
  signer: any;
  /** Nonce of the signer, tracked locally to enable optimistic nonce increment */
  nonce: number;
  /** Name of the target */
  name: string;
  logger: any;
  nativeId: string;

  async setup(config: Gateway, logger: any) {
    this.client = await ApiPromise.create({
      provider: new WsProvider(config.rpc),
    });
    this.logger = logger;
    this.name = config.name;

    const keyring = new Keyring({ type: "sr25519" });
    this.signer = config.signerKey
      ? keyring.addFromMnemonic(config.signerKey)
      : keyring.addFromUri("//Executor//default");

    if (config.nativeId) this.nativeId = config.nativeId;

    this.nonce = await this.fetchNonce(this.client, this.signer.address);
  }

  /**
   * Builds the TX object for the different types of SFX
   *
   * @param sideEffect Object
   * @returns SubmittableExtrinsic tx that can be submitted to the target
   */
  buildTx(sideEffect: SideEffect): SubmittableExtrinsic | undefined {
    switch (sideEffect.action) {
      case SfxType.Transfer: {
        const data = sideEffect.execute();
        return this.client.tx.balances.transfer(data[0], data[1]);
      }
      default:
        return;
    }
  }

  /**
   * Submit the transaction to the target chain. This function increments the nonce locally to enable optimistic nonce increment. This
   * allows transaction to be submitted in parallel without waiting for the previous one to be included. A successful submission will
   * trigger inclusion proof generation.
   *
   * @param sfx Object to execute
   */
  async executeTx(sfx: SideEffect) {
    this.logger.info(
      `Execution started SFX: ${sfx.humanId} - ${sfx.target} with nonce: ${this.nonce} 🔮`
    );
    const tx = this.buildTx(sfx) as SubmittableExtrinsic;
    const nonce = this.nonce;
    this.nonce += 1; // we optimistically increment the nonce before we go async. If the tx fails, we will decrement it which might be a bad idea
    return new Promise<void>((resolve, reject) =>
      tx.signAndSend(
        this.signer,
        { nonce },
        async ({ dispatchError, status, events }) => {
          if (dispatchError?.isModule) {
            // something went wrong and we can decode the error
            const err = this.client.registry.findMetaError(
              dispatchError.asModule
            );
            this.logger.info(`Execution failed SFX: ${sfx.humanId}`);
            this.emit("Event", <RelayerEventData>{
              type: RelayerEvents.SfxExecutionError,
              data: `${err.section}::${err.name}: ${err.docs.join(" ")}`,
              sfxId: sfx.id,
            });
            // we attempt to restore the correct nonce
            this.nonce = await this.fetchNonce(
              this.client,
              this.signer.address
            );
            reject(Error(`${err.section}::${err.name}: ${err.docs.join(" ")}`));
          } else if (dispatchError) {
            // something went wrong and we can't decode the error
            this.emit("Event", <RelayerEventData>{
              type: RelayerEvents.SfxExecutionError,
              data: dispatchError.toString(),
              sfxId: sfx.id,
            });
            // we attempt to restore the correct nonce
            this.nonce = await this.fetchNonce(
              this.client,
              this.signer.address
            );
            reject(Error(dispatchError.toString()));
          } else if (status.isFinalized) {
            const blockNumber = await this.generateSfxInclusionProof(
              sfx,
              status.asFinalized,
              events
            );
            this.logger.info(
              `Execution complete SFX:  ${sfx.humanId} - #${blockNumber} 🏁`
            );
            this.emit("Event", <RelayerEventData>{
              type: RelayerEvents.SfxExecutedOnTarget,
              sfxId: sfx.id,
              target: this.name,
              data: "",
              blockNumber,
            });
            resolve();
          }
        }
      )
    );
  }

  /**
   * Generates the inclusion proof of an executed transaction/SFX and adds it to the SFX object
   *
   * @param sfx Object
   * @param blockHash Hash of the block in which the transaction was included
   * @param events Events emitted by the transaction
   * @returns Block number in which the transaction was included
   */
  async generateSfxInclusionProof(
    sfx: SideEffect,
    blockHash: any,
    events: any[]
  ): Promise<number> {
    const blockNumber = await this.getBlockNumber(blockHash);
    const event = this.getEvent(sfx.action, events);

    const inclusionProof = await getEventProofs(this.client, blockHash);
    const inclusionData: InclusionProof = {
      encoded_payload: event.toHex(),
      payload_proof: {
        // @ts-ignore
        trieNodes: inclusionProof.toJSON().proof,
      },
      block_hash: blockHash,
    };

    if (sfx.target !== "roco") {
      const blockNumber = await this.fetchCorrespondingRelaychainHeaderNumber(
        blockHash
      ).catch((err) => {
        // this should never happen
        console.log(err);
      });

      this.emit("Event", <RelayerEventData>{
        type: RelayerEvents.HeaderInclusionProofRequest,
        blockNumber,
        target: "roco",
        sfxId: sfx.id,
        data: this.nativeId,
      });

      // Add the inclusion proof to the SFX object
      sfx.executedOnTarget(
        inclusionData,
        this.signer.addressRaw,
        blockNumber as number
      );
      // if we have a parachain SFX we need to submit on the relaychain block height
      return blockNumber as number;
    }

    // Add the inclusion proof to the SFX object
    sfx.executedOnTarget(
      inclusionData,
      this.signer.addressRaw,
      blockNumber.toNumber()
    );

    return blockNumber.toNumber();
  }

  /**
   * Fetches the block number of the relaychain block, containing the parachain block
   *
   * @param parachainBlockHash block hash of the parachain
   * @returns Block number of the relaychain block
   */
  async fetchCorrespondingRelaychainHeaderNumber(
    parachainBlockHash: any
  ): Promise<number> {
    const parachainBlock = await this.client.rpc.chain.getBlock(
      parachainBlockHash
    );

    // ToDo: we should verify that this block is correct. Could be done by running a state query on the relaychain, decoding the block and ensuring the height is correct
    for (let i = 0; i < parachainBlock.block.extrinsics.length; i++) {
      const extrinsic = parachainBlock.block.extrinsics[i];
      if (
        extrinsic.method.method === "setValidationData" &&
        extrinsic.method.section === "parachainSystem"
      ) {
        return (
          // @ts-ignore - TS does not know about the type
          extrinsic.method.args[0].validationData.relayParentNumber.toNumber() +
          2
        ); // im not exactly sure why we need to add 2 here, +1 makes sense as its parent.
      }
    }

    throw Error("Could not find relaychain header number");
  }

  async generateHeaderInclusionProof(
    relaychainBlockNumber: number,
    parachainId: number
  ) {
    const blockHash = await this.client.rpc.chain.getBlockHash(
      relaychainBlockNumber
    );

    return Utils.Substrate.getStorageProof(
      this.client,
      blockHash,
      "Paras",
      "Heads",
      parachainId
    );
  }

  async getBlockHash(blockNumber: number) {
    return this.client.rpc.chain.getBlockHash(blockNumber);
  }

  /**
   * Fetch block number from block hash
   *
   * @param hash Of block
   * @returns Block number
   */
  async getBlockNumber(hash: any) {
    return (await this.client.rpc.chain.getHeader(hash)).number;
  }

  /**
   * Filter events for the event emitted by the transaction
   *
   * @param transactionType Of the transaction
   * @param events Array of events in that block
   * @returns Event emitted by the transaction
   */
  getEvent(transactionType: SfxType, events: any[]) {
    const event = events.find((item) => {
      return item.event.method === EventMapper[transactionType];
    });

    if (event) {
      return event.event;
    } else {
      //Todo: deal with this somehow
      console.log("Event not found");
    }
  }

  /**
   * Fetch nonce of the signer
   *
   * @param api Client
   * @param address Of the signer
   * @returns Nonce of the signer
   */
  async fetchNonce(api: ApiPromise, address: string): Promise<number> {
    return parseInt((await api.rpc.system.accountNextIndex(address)).toHuman());
  }
}

export { Estimator, CostEstimator, Estimate, InclusionProof };
