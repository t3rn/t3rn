import { Execution } from "./execution";
import { Notification, NotificationType, SideEffect } from "./sideEffect";
import Estimator from "../gateways/substrate/estimator";
import { SubstrateRelayer } from "../gateways/substrate/relayer";
import { PriceEngine } from "../pricing";
import { StrategyEngine } from "../strategy";
import { Sdk } from "@t3rn/sdk";
import { BiddingEngine } from "../bidding";
import {
  CircuitListener,
  EventData,
  ListenerEventData,
  ListenerEvents,
  ListEventData,
  PropEventData,
} from "../circuit/listener";
import { CircuitRelayer } from "../circuit/relayer";
import { RelayerEventData, RelayerEvents } from "../gateways/types";
import { XtxStatus } from "@t3rn/sdk/side-effects/types";
import { Config, Gateway } from "../../config/config";
import { Logger } from "pino";
import BN from "bn.js";
import { Prometheus } from "../prometheus";
import { logger } from "../logging";

// A type used for storing the different SideEffects throughout their respective life-cycle.
// Please note that waitingForInsurance and readyToExecute are only used to track the progress. The actual logic is handeled in the execution
/**
 * The queue type is used to track the incoming SFXs throughout their life-cycle. Each gateway has its own queue, tracking its height on the
 * light-client. When an SFX was executed, it is moved to the isConfirming queue. Once the gateway has reached the required block height,
 * the SFXs can be confirmed.
 *
 * @group Execution Manager
 */
export type Queue = {
  /** Each gateway has its own queue, which can be accessed via gateway id */
  gateways: {
    /** Stores the latest block height know by the corresponding circuit light client */
    blockHeight: number;
    /** SFXs that are currently in bidding stage */
    isBidding: string[];
    /** SFXs that are currently being executed */
    isExecuting: string[];
    /** SFXs that are waiting to be confirmed */
    isConfirming: {
      [block: number]: string[];
    };
    /** SFXs that are completed */
    completed: string[];
    /** SFXs that are dropped */
    dropped: string[];
    /** SFXs that are reverted */
    reverted: string[];
  };
};

/** Persisted state for JSON de/serialization. WIP */
export interface PersistedState {
  queue: Queue;
  xtx: { [id: string]: Execution };
  sfxToXtx: { [sfxId: string]: string };
  // targetEstimator: { [id: string]: Estimator }
  // relayers: { [key: string]: SubstrateRelayer }
}

/**
 * The ExecutionManager lies at the heart of the t3rn executor. It is responsible for managing and coordinating the execution of incoming
 * XTXs and the corresponding SFXs. It processes incoming events, triggering the creation/execution/confirmation of SFXs.
 *
 * @group Execution Manager
 */
export class ExecutionManager {
  // we map the current state in the queue
  /** Queue used to track SFXs and gateways height */
  queue: Queue = <Queue>{};
  // maps xtxId to Execution instance
  /** Maps XTX id to their corresponding Execution instance */
  xtx: {
    [id: string]: Execution;
  } = {};
  /** Maps SFX id to their corresponding XTX id. Used for lookups */
  sfxToXtx: {
    [sfxId: string]: string;
  } = {};

  /** Tx cost estimator instances for the specific targets */
  targetEstimator: {
    [id: string]: Estimator;
  } = {};
  /** Relayer instances for the specific targets */
  relayers: { [key: string]: SubstrateRelayer } = {};

  priceEngine: PriceEngine;
  strategyEngine: StrategyEngine;
  biddingEngine: BiddingEngine;
  circuitListener: CircuitListener;
  circuitRelayer: CircuitRelayer;
  prometheus: Prometheus;

  constructor(
    public circuitClient: Sdk["client"],
    public sdk: Sdk,
    public logger: Logger,
    public config: Config,
    prometheus: Prometheus,
  ) {
    this.priceEngine = new PriceEngine();
    this.strategyEngine = new StrategyEngine();
    this.biddingEngine = new BiddingEngine(logger, prometheus);
    this.circuitListener = new CircuitListener(this.circuitClient);
    this.circuitRelayer = new CircuitRelayer(sdk);
    this.prometheus = prometheus;
  }

  /** Injects persisted execution state.
   *
   * @param state Persisted state to rebase ontop
   *
   */
  inject(state: undefined | PersistedState): ExecutionManager {
    if (state) {
      this.queue = state.queue;
      this.xtx = state.xtx;
      this.sfxToXtx = state.sfxToXtx;
      // this.targetEstimator = state.targetEstimator
      // this.relayers = state.relayers
    }
    return this;
  }

  /** Setup all instances and listeners for the execution manager */
  async setup(gatewayConfig: Gateway[], vendors: string[]) {
    this.initializeVendors(vendors);
    await this.initializeGateways(gatewayConfig);
    await this.circuitListener.start();
    await this.initializeEventListeners();
    logger.info("ExecutionManager setup successful");
  }

  /** Initiates a shutdown, the promise resolves once all executions are done. */
  async shutdown(): Promise<void> {
    this.circuitListener.stop();

    const recheckQueue = (
      manager: ExecutionManager,
      resolve: (...args: unknown[]) => void,
    ) => {
      const done = Object.entries(manager.sdk.gateways)
        .map(([, gtwy]) => gtwy.id)
        .every(
          (gtwyId) =>
            manager.queue[gtwyId].isBidding.length === 0 &&
            manager.queue[gtwyId].isExecuting.length === 0 &&
            manager.queue[gtwyId].isConfirming.length === 0,
        );

      if (done) {
        resolve(undefined);
      } else {
        manager.circuitListener.once("Event", () =>
          recheckQueue(manager, resolve),
        );
      }
    };

    return new Promise((resolve) => {
      this.circuitListener.once("Event", recheckQueue);
      recheckQueue(this, resolve);
    });
  }

  initializeVendors(vendors: string[]) {
    for (let i = 0; i < vendors.length; i++) {
      this.queue[vendors[i]] = {
        blockHeight: 0,
        isBidding: [],
        isExecuting: [],
        isConfirming: {},
        completed: [],
        dropped: [],
        reverted: [],
      };
    }
  }

  /** Initialize all gateways and their corresponding relayers, event listeners and estimators */
  async initializeGateways(gatewayConfig: Gateway[]) {
    const gatewayKeys = Object.keys(this.sdk.gateways);
    for (let i = 0; i < gatewayKeys.length; i++) {
      const entry = this.sdk.gateways[gatewayKeys[i]];

      const config = gatewayConfig.find((g) => g.id === entry.id);

      if (!config) {
        // skip over gateways we have no configs for
        continue;
      }
      if (entry.executionVendor === "Substrate") {
        // initialize gateway relayer
        const relayer = new SubstrateRelayer();

        await relayer.setup(config, logger);

        this.relayers[entry.id] = relayer;

        // initialize gateway estimator
        this.targetEstimator[entry.id] = new Estimator(relayer);

        relayer.on("Event", async (eventData: RelayerEventData) => {
          switch (eventData.type) {
            case RelayerEvents.SfxExecutedOnTarget:
              {
                const xtxId = this.sfxToXtx[eventData.sfxId];
                const xtx = this.xtx[xtxId];
                const sfx = xtx.sideEffects.get(eventData.sfxId);

                if (!sfx) break;

                const vendor = sfx.vendor;
                this.removeFromQueue("isExecuting", eventData.sfxId, vendor);

                // create array if first for block
                if (!this.queue[vendor].isConfirming[eventData.blockNumber]) {
                  this.queue[vendor].isConfirming[eventData.blockNumber] = [];
                }

                // adds to queue
                this.queue[vendor].isConfirming[eventData.blockNumber].push(
                  eventData.sfxId,
                );

                logger.info({
                  sfxId: eventData.sfxId,
                  xtxId: this.sfxToXtx[eventData.sfxId],
                },
                  "SFX transition from isExecuting to isConfirming",
                );
              }
              break;
            case RelayerEvents.HeaderInclusionProofRequest:
              {
                const proof = await this.relayers[eventData.target]
                  .generateHeaderInclusionProof(
                    eventData.blockNumber,
                    parseInt(eventData.data),
                  )
                  .then((event) => {
                    return event.toJSON().proof;
                  });

                const blockHash = await this.relayers[eventData.target]
                  .getBlockHash(eventData.blockNumber)
                  .then(
                    // @ts-ignore - cannot find property in type never
                    (hash) => hash.toString(),
                  );

                // @ts-ignore - cannot find property in type never
                this.xtx[this.sfxToXtx[eventData.sfxId]].sideEffects
                  .get(eventData.sfxId)
                  ?.addHeaderProof(proof, blockHash);
              }
              break;
            case RelayerEvents.SfxExecutionError:
              // @todo - figure out how to handle this
              break;
          }
        });
      }

      logger.info("Gateways Initialized");
    }
  }

  /** Initialize the circuit listeners */
  async initializeEventListeners() {
    this.circuitListener.on("Event", async (eventData: ListenerEventData) => {
      this.prometheus.events.inc();

      switch (eventData.type) {
        case ListenerEvents.NewSideEffectsAvailable:
          this.addXtx(eventData.data, this.sdk);
          break;
        case ListenerEvents.SFXNewBidReceived:
          this.addBid(eventData.data as ListEventData);
          break;
        case ListenerEvents.XTransactionReadyForExec:
          {
            const xtxId = (
              eventData.data[0] as { toString: () => string }
            ).toString();
            this.xtxReadyForExec(xtxId);
          }
          break;
        case ListenerEvents.HeaderSubmitted:
          {
            const data = eventData.data as PropEventData;

            if (!data.vendor || !data.height) break;

            this.updateGatewayHeight(data.vendor, data.height);
          }
          break;
        case ListenerEvents.SideEffectConfirmed:
          {
            const sfxId = eventData.data[0].toString();
            const xtxId = this.sfxToXtx[sfxId];
            const xtx = this.xtx[xtxId];
            const sfx = xtx.sideEffects.get(sfxId);

            if (!sfx) break;

            sfx.confirmedOnCircuit();
            logger.info({
              sfxId: sfxId,
              xtxId: this.sfxToXtx[sfxId],
            }, "Sfx confirmed");
          }
          break;
        case ListenerEvents.XtxCompleted:
          this.xtx[eventData.data[0].toString()].completed();
          break;
        case ListenerEvents.DroppedAtBidding:
          this.droppedAtBidding(eventData.data[0].toString());
          break;
        case ListenerEvents.RevertTimedOut:
          this.revertTimeout(eventData.data[0].toString());
      }
    });
  }

  /**
   * Add a new XTX to the execution manager. This is triggered when a new XTX is available on the circuit.
   *
   * @param xtxData The SCALE encoded XTX data, as emitted by the circuit
   * @param sdk The SDK instance
   */
  async addXtx(xtxData: EventData, sdk: Sdk) {
    // create the XTX object
    const xtx = new Execution(
      logger,
      sdk.signer.address,
      xtxData,
      sdk,
      this.strategyEngine,
      this.biddingEngine,
      this.prometheus,
    );

    // Run the XTX strategy checks
    try {
      this.strategyEngine.evaluateXtx(xtx);
      logger.info({  xtxId: xtx.id }, "XTX strategy passed!");
    } catch (e) {
      // XTX does not meet strategy requirements
      this.prometheus.executorXtxStrategyRejects.inc();
      logger.warn({
        e: e.toString(),
        xtxId: xtx.id,
      }, "XTX strategy reject!");
      return;
    }
    logger.info(`Received XTX ${xtx.humanId} 🌱`); // XTX is valid for execution

    // add XTX and init required event listeners
    this.xtx[xtx.id] = xtx;
    for (const [sfxId, sfx] of xtx.sideEffects.entries()) {
      this.initSfxListeners(sfx);
      this.sfxToXtx[sfxId] = xtx.id;
      this.queue[sfx.vendor].isBidding.push(sfxId);
      await this.addRiskRewardParameters(sfx);
    }
    logger.info({ xtxId: xtx.id }, "XTX initialized");
  }

  /**
   * Add an incoming bid to the corresponding SFX
   *
   * @param bidData SCALE encoded bid data, as emitted by the circuit
   */
  addBid(bidData: ListEventData) {
    const sfxId = bidData[0].toString();
    const bidder = bidData[1].toString();
    const amt = bidData[2].toNumber();

    const conversionId = this.sfxToXtx[sfxId];
    const sfxFromXtx = this.xtx[conversionId].sideEffects;
    const actualSfx = sfxFromXtx.get(sfxId);
    if (actualSfx !== undefined) {
      actualSfx.processBid(bidder, amt);
    } else {
      throw new Error(`Could not find SFX with id ${sfxId}`);
    }
  }

  /**
   * Update a XTX status to ready. This is triggered by an incoming circuit event Next, this will trigger the execution of SFX the
   * executor has won the bid on.
   *
   * @param xtxId The XTX ID
   */
  async xtxReadyForExec(xtxId: string) {
    if (this.xtx[xtxId]) {
      this.xtx[xtxId].readyToExecute();
      // Get the SFX that the executor has won the bid on and can execute now
      const ready = this.xtx[xtxId].getReadyToExecute();
      if (ready.length > 0) {
        logger.info(
          `Won bids for XTX ${this.xtx[xtxId].humanId}: ${ready.map(
            (sfx) => sfx.humanId,
          )} 🏆`,
        );
      }
      for (const sfx of ready) {
        // move on the queue
        this.removeFromQueue("isBidding", sfx.id, sfx.vendor);
        this.queue[sfx.vendor].isExecuting.push(sfx.id);
        // execute
        this.relayers[sfx.target].executeTx(sfx).then();
      }
    }
  }

  // New header range was submitted on circuit
  // update local params and check which SideEffects can be confirmed
  /**
   * Update the vendor height in the queue. This is triggered by an incoming circuit event. Next, this will trigger the confirmation of
   * SFX that have been executed.
   *
   * @param vendor Id of the gateway
   * @param blockHeight The latest block height
   */
  updateGatewayHeight(vendor: string, blockHeight: number) {
    logger.info({
      vendor: vendor,
      blockHeight: blockHeight,
    }, "Update Gateway Height");
      
    if (this.queue[vendor]) {
      this.queue[vendor].blockHeight = blockHeight;
      this.executeConfirmationQueue(vendor);
    }
  }

  // checks which executed SideEffects can be confirmed on circuit
  /**
   * Trigger the confirmation of SFX that have been executed. When the gateway height is updated, this will check the isConfirming queue
   * for the gateway. The confirmation of any waiting SFXs is now triggered. Requirement for the confirmation is that the circuit has
   * received the corresponding headers, the SFXs where included in.
   *
   * @param gatewayId Of the updated gateway
   */
  async executeConfirmationQueue(vendor: string) {
    // contains the sfxIds of SideEffects that could be confirmed based on the current blockHeight of the light client
    let readyByHeight: string[] = [];
    // stores the block height of the SideEffects that are ready to be confirmed. Needed for clearing the queue
    const batchBlocks: string[] = [];
    const queuedBlocks = Object.keys(this.queue[vendor].isConfirming);
    // we check which headers are available and collect the SFX ids
    for (let i = 0; i < queuedBlocks.length; i++) {
      if (parseInt(queuedBlocks[i]) <= this.queue[vendor].blockHeight) {
        batchBlocks.push(queuedBlocks[i]);
        readyByHeight = readyByHeight.concat(
          this.queue[vendor].isConfirming[queuedBlocks[i]],
        );
      }
    }

    const readyByStep: SideEffect[] = [];

    // In case we have executed SFXs from the next phase already, we ensure that we only confirm the SFXs of the current phase
    for (let i = 0; i < readyByHeight.length; i++) {
      const xtxId = this.sfxToXtx[readyByHeight[i]];
      const xtx = this.xtx[xtxId];
      const sfx = xtx.sideEffects.get(readyByHeight[i]);

      if (!sfx) continue;

      if (sfx.phase === this.xtx[sfx.xtxId].currentPhase) {
        readyByStep.push(sfx);
      }
    }

    // if we found SFXs, we confirm them
    if (readyByStep.length > 0) {
      logger.info({
        gatewayId: vendor,
        sfxIds: readyByStep.map((sfx) => sfx.id),
      },
        "Execute confirmation queue",
      );
      this.circuitRelayer
        .confirmSideEffects(readyByStep)
        .then(() => {
          // remove from queue and update status
          logger.info(
            `Confirmed SFXs: ${readyByStep.map((sfx) => sfx.humanId)} 📜`,
          );
          this.processConfirmationBatch(readyByStep, batchBlocks, vendor);
          logger.info({
            vendor: vendor,
          },
         "Confirmation batch successful");
        })
        .catch((err) => {
          logger.info(
            {
              
              vendor: vendor,
              sfxIds: readyByStep.map((sfx) => sfx.id),
              error: err,
            },
 "Error confirming side effects"
          );
        });
    }
  }

  // updates queue and sfx state after confirmation batch was submitted. This always has the same target
  /**
   * Update the queue and SFX state after a confirmation batch was submitted.
   *
   * @param sfxs Array of sfx objects
   * @param batchBlocks Array of block heights of the sfxs where confirmed for. Needed for cleaning up the queue
   * @param gatewayId Id of the gateway
   */
  processConfirmationBatch(
    sfxs: SideEffect[],
    batchBlocks: string[],
    gatewayId: string,
  ) {
    // remove from queue
    batchBlocks.forEach((block) => {
      delete this.queue[gatewayId].isConfirming[block];
    });

    // add to completed queue and update status
    for (const sfx of sfxs) {
      this.queue[gatewayId].completed.push(sfx.id);
      sfx.confirmedOnCircuit(); // maybe we leave this part and trigger via event, which is done in any case
    }
  }

  /**
   * Initialize SFX event listeners.
   *
   * @param sfx Object of the sfx
   */
  initSfxListeners(sfx: SideEffect) {
    sfx.on("Notification", (notification: Notification) => {
      switch (notification.type) {
        case NotificationType.SubmitBid: {
          this.circuitRelayer
            .bidSfx(
              notification.payload.sfxId,
              notification.payload.bidAmount as BN,
            )
            .then(() => {
              sfx.bidAccepted(notification.payload.bidAmount as number);
            })
            .catch((e) => {
              logger.warn(`Bid rejected for SFX ${sfx.humanId} ❌`);
              sfx.bidRejected(e);
            });
        }
      }
    });
  }

  /**
   * Gather and add the required risk/reward parameters for a new SFX.
   *
   * @param sfx The sfx object
   */
  async addRiskRewardParameters(sfx: SideEffect) {
    // get txCost on target
    const txCostSubject = await this.targetEstimator[
      sfx.target
    ].getNativeTxCostSubject(sfx);
    // get price of native token on target
    const nativeAssetPriceSubject = this.priceEngine.getAssetPrice(
      sfx.gateway.ticker,
    );

    const txOutput = sfx.getTxOutputs();
    // get tx output cost. E.g. tran 1 Eth this returns the current price of Eth
    const txOutputPriceSubject = this.priceEngine.getAssetPrice(txOutput.asset);
    // get price of the reward asset
    const rewardAssetPriceSubject = this.priceEngine.getAssetPrice("TRN");

    sfx.setRiskRewardParameters(
      txCostSubject,
      nativeAssetPriceSubject,
      txOutputPriceSubject,
      rewardAssetPriceSubject,
    );
  }

  /**
   * Update XTX status after it was dropped on circuit. Cleans up queue and updates the SFXs
   *
   * @param xtxId Id of XTX that was dropped
   */
  droppedAtBidding(xtxId: string) {
    const xtx = this.xtx[xtxId];
    if (xtx && !(xtx.status === XtxStatus.DroppedAtBidding)) {
      xtx.droppedAtBidding();
      for (const sfx of xtx.sideEffects.values()) {
        this.removeFromQueue("isBidding", sfx.id, sfx.vendor);
        this.queue[sfx.vendor].dropped.push(sfx.id);
      }
    }
  }

  /**
   * Update XTX status after it was reverted on circuit. Cleans up queue and updates the SFXs
   *
   * @param xtxId Id of XTX that was reverted
   */
  revertTimeout(xtxId: string) {
    const xtx = this.xtx[xtxId];
    if (xtx) {
      for (const sfx of xtx.sideEffects.values()) {
        // sfx could either be in isExecuting or isConfirming
        this.removeFromQueue("isExecuting", sfx.id, sfx.vendor);
        let confirmBatch =
          this.queue[sfx.vendor].isConfirming[
            sfx.targetInclusionHeight.toString()
          ];
        if (!confirmBatch) confirmBatch = [];
        if (confirmBatch.includes(sfx.id)) {
          const index = confirmBatch.indexOf(sfx.id);
          confirmBatch.splice(index, 1);
        }

        // add to reverted queue
        this.queue[sfx.vendor].reverted.push(sfx.id);
      }
      this.xtx[xtxId].revertTimeout();
    }
  }

  // removes sfx from queue
  private removeFromQueue(queue: string, id: string, gatewayId: string) {
    const index = this.queue[gatewayId][queue].indexOf(id);
    if (index > -1) {
      this.queue[gatewayId][queue].splice(index, 1);
    }
    this.biddingEngine.cleanUp(id);
  }
}
