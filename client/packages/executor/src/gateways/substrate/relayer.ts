import { EventEmitter } from "events";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { EventMapper, SideEffect } from "../../executionManager/sideEffect";
import { getBalanceWithDecimals, getEventProofs } from "../../utils";
import { SubmittableExtrinsic } from "@polkadot/api/promise/types";
import { SfxType } from "@t3rn/sdk/side-effects/types";
import { InclusionProof, RelayerEventData, RelayerEvents } from "../types";
import Estimator from "./estimator";
import { CostEstimator, Estimate } from "./estimator/cost";
import { Sdk, Utils } from "@t3rn/sdk";
import { Gateway } from "../../../config/config";
import { logger } from "../../logging";
import { Prometheus } from "../../prometheus";

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
  signer: Sdk["signer"];
  /** Nonce of the signer, tracked locally to enable multi tx in single block */
  nonce: number;
  /** Name of the target */
  name: string;
  nativeId: string;
  prometheus: Prometheus;

  async setup(config: Gateway, prometheus: Prometheus) {
    this.client = await ApiPromise.create({
      provider: new WsProvider(config.rpc),
    });
    this.name = config.name;
    this.prometheus = prometheus;

    const keyring = new Keyring({ type: "sr25519" });
    this.signer = config.signerKey
      ? keyring.addFromMnemonic(config.signerKey)
      : keyring.addFromUri("//Executor//default");

    if (config.nativeId) this.nativeId = config.nativeId;

    this.nonce = await this.fetchNonce(this.client, this.signer.address);
    const balance = await getBalanceWithDecimals(
      this.client,
      this.signer.address,
    );
    logger.info(
      {
        signer: this.signer.address,
        nonce: this.nonce,
        target: this.name,
        balance: balance,
        rpc: config.rpc,
      },
      "Relayer setup completed 🏁",
    );
    this.prometheus.executorBalance.set(
      { signer: this.signer.address, target: this.name },
      balance,
    );
  }

  /**
   * Builds the TX object for the different types of SFX
   *
   * @param sideEffect Object
   * @returns SubmittableExtrinsic tx that can be submitted to the target
   */
  buildTx(sideEffect: SideEffect): SubmittableExtrinsic | undefined {
    const data = sideEffect.execute();
    switch (sideEffect.action) {
      case SfxType.Transfer: {
        return this.client.tx.balances.transferKeepAlive(
          data[0] as string,
          data[1],
        );
      }
      case SfxType.TransferAsset: {
        return this.client.tx.assets.transfer(
          data[0], // asset id
          data[1] as string, // to
          data[2], // amount
        );
      }
      case SfxType.CallEVM: {
        return this.client.tx.evm.call(
          sideEffect.executor, // source
          data[0] as string, // target
          data[1] as string, // input
          data[2], // value amount
          data[3], // gas limit
          "100", // max fee per gas
          null, // max priority fee per gas
          null, // nonce
          [], // access list
        );
      }
      case SfxType.CallWASM: {
        return this.client.tx.contracts.call(
          data[0] as string, // target (dest: AccountIdLookupOf<T>)
          data[2], // value amount (value: BalanceOf<T>)
          data[3] as string, // gas limit (gas_limit: OldWeight)
          null, // storage_deposit_limit: Option<<BalanceOf<T> as codec::HasCompact>::Type>
          data[1] as string, // input data (data: Vec<u8>)
        );
      }
      default:
        return;
    }
  }

  /**
   * Submit the transaction to the target chain. A successful submission will
   * trigger inclusion proof generation.
   *
   * @param sfx Object to execute
   */
  async executeTx(sfx: SideEffect) {
    logger.info(
      { sfxId: sfx.id, target: sfx.target, nonce: this.nonce },
      `SFX Execution started 🔮`,
    );

    if (sfx.gateway.executionVendor === "Substrate") {
      this.prometheus.executorTargetBalance.set(
        {
          signer: sfx.arguments[0],
          target: sfx.target,
        },
        await getBalanceWithDecimals(this.client, sfx.arguments[0]),
      );
    }

    const tx = this.buildTx(sfx) as SubmittableExtrinsic;
    const result = new Promise<void>((resolve, reject) =>
      tx.signAndSend(
        this.signer,
        { nonce: this.nonce },
        async ({ dispatchError, status, events }) => {
          if (dispatchError?.isModule) {
            // something went wrong and we can decode the error
            const error = this.client.registry.findMetaError(
              dispatchError.asModule,
            );
            logger.info(
              {
                sfxId: sfx.id,
                sfx: sfx,
                error,
              },
              `SFX Execution failed 🚨`,
            );
            this.emit("Event", <RelayerEventData>{
              type: RelayerEvents.SfxExecutionError,
              data: `${error.section}::${error.name}: ${error.docs.join(" ")}`,
              sfxId: sfx.id,
            });
            // we attempt to restore the correct nonce
            this.nonce = await this.fetchNonce(
              this.client,
              this.signer.address,
            );
            reject(
              Error(`${error.section}::${error.name}: ${error.docs.join(" ")}`),
            );
          } else if (dispatchError) {
            // something went wrong and we can't decode the error
            this.emit("Event", <RelayerEventData>{
              type: RelayerEvents.SfxExecutionError,
              data: dispatchError.toString(),
              sfxId: sfx.id,
            });
            reject(Error(dispatchError.toString()));
          } else if (status.isFinalized) {
            const blockNumber = await this.generateSfxInclusionProof(
              sfx,
              status.asFinalized as never,
              events,
            );
            logger.info(
              {
                sfxId: sfx.id,
                target: sfx.target,
                blockNumber,
                gatewayType: sfx.gateway.gatewayType,
              },
              `SFX Execution completed 🏁`,
            );

            if (sfx.gateway.gatewayType === "Substrate") {
              this.prometheus.executorTargetBalance.set(
                {
                  signer: sfx.arguments[0],
                  target: sfx.target,
                },
                await getBalanceWithDecimals(this.client, sfx.arguments[0]),
              );
            }
            this.prometheus.executorExecutionCompleted.inc({
              target: sfx.target,
            });

            this.emit("Event", <RelayerEventData>{
              type: RelayerEvents.SfxExecutedOnTarget,
              sfxId: sfx.id,
              target: this.name,
              data: "",
              blockNumber,
            });
            resolve();
          }
        },
      ),
    );
    this.nonce++;
    return result;
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
    blockHash: string,
    events: unknown,
  ): Promise<number> {
    const blockNumber = await this.getBlockNumber(blockHash);
    const event = this.getEvent(sfx.action, events as never);
    const inclusionProof = await getEventProofs(this.client, blockHash);
    const inclusionData: InclusionProof = {
      encoded_payload: (event as unknown as { toHex: () => string }).toHex(),
      payload_proof: {
        // @ts-ignore - property does not exist on type
        trieNodes: inclusionProof.toJSON().proof as string,
      },
      block_hash: blockHash,
    };

    if (sfx.target !== "roco") {
      const blockNumber = await this.fetchCorrespondingRelaychainHeaderNumber(
        blockHash,
      ).catch((error) => {
        // this should never happen
        logger.error(
          { error: error.toString() },
          "Failed to fetch corresponding relaychain header number",
        );
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
        blockNumber as number,
      );
      // if we have a parachain SFX we need to submit on the relaychain block height
      return blockNumber as number;
    }

    // Add the inclusion proof to the SFX object
    sfx.executedOnTarget(
      inclusionData,
      this.signer.addressRaw,
      blockNumber.toNumber(),
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
    parachainBlockHash: string,
  ): Promise<number> {
    const number = await this.client.rpc.chain
      .getBlock(
        parachainBlockHash,
        // ToDo: we should verify that this block is correct. Could be done by running a state query on the relaychain, decoding the block and ensuring the height is correct
      )
      .then((parachainBlock) => {
        // @ts-ignore - property does not exist on type
        const block = parachainBlock.block;
        for (let i = 0; i < block.extrinsics.length; i++) {
          const extrinsic = block.extrinsics[i];
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
      })
      .catch((err) => {
        throw Error("Could not find relaychain header number. Error: " + err);
      });

    return number;
  }

  async generateHeaderInclusionProof(
    relaychainBlockNumber: number,
    parachainId: number,
  ) {
    const blockHash = await this.client.rpc.chain.getBlockHash(
      relaychainBlockNumber,
    );
    logger.debug(
      {
        blockHash,
        relaychainBlockNumber,
        parachainId,
        inclusionProof: Utils.Substrate.getStorageProof(
          this.client,
          blockHash,
          "Paras",
          "Heads",
          parachainId,
        ),
      },
      "Generating header inclusion proof with Substrate.getStorageProof",
    );

    return Utils.Substrate.getStorageProof(
      this.client,
      blockHash,
      "Paras",
      "Heads",
      parachainId,
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
  async getBlockNumber(hash: string) {
    return await this.client.rpc.chain
      .getHeader(hash)
      // @ts-ignore - property does not exist on type
      .then((header) => header.number)
      .catch((err) => {
        throw Error("Could not fetch block number. Error: " + err);
      });
  }

  /**
   * Filter events for the event emitted by the transaction
   *
   * @param transactionType Of the transaction
   * @param events Array of events in that block
   * @returns Event emitted by the transaction
   */
  getEvent(
    transactionType: SfxType,
    events: Array<{ event: { method: string } }>,
  ) {
    const event = events.find((item) => {
      return item.event.method === EventMapper[transactionType];
    });

    if (!event) {
      logger.warn("Event not found");
      return;
    }

    return event.event;
  }

  /**
   * Fetch nonce of the signer
   *
   * @param api Client
   * @param address Of the signer
   * @returns Nonce of the signer
   */
  async fetchNonce(api: ApiPromise, address: string): Promise<number> {
    return await api.rpc.system.accountNextIndex(address).then((nextIndex) => {
      // @ts-ignore - property does not exist on type
      return parseInt(nextIndex.toNumber());
    });
  }
}

export { Estimator, CostEstimator, Estimate, InclusionProof };
