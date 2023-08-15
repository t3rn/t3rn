import "@polkadot/api-augment";
import * as dotenv from "dotenv";
import { Sdk } from "@t3rn/sdk";
import { Keyring } from "@polkadot/api";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { KeyringPair } from "@polkadot/keyring/types";
import { PathLike } from "fs";
import { mkdir } from "fs/promises";
import { join } from "path";
import "@t3rn/types";
import {
  SubstrateRelayer,
  CostEstimator,
  Estimator,
  Estimate,
  InclusionProof,
} from "./gateways/substrate/relayer";
import { ExecutionManager, PersistedState, Queue } from "./executionManager";
import { config, Config, Circuit, Strategy, Gateway } from "../config/config";
import { BiddingEngine, BiddingStrategy } from "./bidding";
import { PriceEngine, CoingeckoPricing } from "./pricing";
import { StrategyEngine, SfxStrategy, XtxStrategy } from "./strategy";
import {
  SideEffect,
  Notification,
  NotificationType,
  TxOutput,
  TxStatus,
} from "./executionManager/sideEffect";
import { Execution } from "./executionManager/execution";
import {
  CircuitListener,
  ListenerEvents,
  ListenerEventData,
} from "./circuit/listener";
import { CircuitRelayer } from "./circuit/relayer";
import { createLogger } from "./utils";
import { Logger } from "pino";
import { Prometheus } from "./prometheus";
import { logger } from "./logging";
import { homedir } from "os";

dotenv.config();

/** An executor instance. */
class Instance {
  name: string;
  circuitClient: Sdk["client"];
  executionManager: ExecutionManager;
  sdk: Sdk;
  signer: KeyringPair;
  config: Config;
  logger: Logger;
  baseDir: PathLike;
  logsDir: undefined | PathLike;
  stateFile: PathLike;
  prometheus: Prometheus;

  /**
   * Initializes an executor instance.
   *
   * @param name Display name and config identifier for an instance
   * @param logToDisk Write logs to disk within ~/.t3rn-executor-${name}/logs
   */
  constructor(name = "example", logToDisk = false, prometheus: Prometheus) {
    this.name = name;
    this.baseDir = join(homedir(), `.t3rn-executor-${name}`);
    this.logsDir = logToDisk
      ? join(this.baseDir.toString(), "logs")
      : undefined;
    this.stateFile = join(this.baseDir.toString(), "state.json");
    // this.configFile = join(this.baseDir.toString(), "config.json");
    this.prometheus = prometheus;
    this.config = config;
  }

  /**
   * Sets up and configures an executor instance.
   */
  async setup(): Promise<Instance> {
    await this.configureLogging();
    await cryptoWaitReady();

    if (!this.config.circuit.signerKey) {
      logger.error("Missing circuit signer key");
      throw Error;
    }

    this.signer = new Keyring({ type: "sr25519" }).addFromMnemonic(
      this.config.circuit.signerKey,
    );
    this.sdk = new Sdk(this.config.circuit.rpc, this.signer);
    this.circuitClient = await this.sdk.init();
    this.circuitClient.on("disconnected", () => {
      this.prometheus.circuitDisconnects.inc({
        endpoint: this.config.circuit.rpc,
      });
    });

    // TODO: print wallet balance on available networks
    const balance = (
      await this.circuitClient.query.system.account(this.signer.address)
    ).data.free.toNumber();

    // Convert the balance to a human-readable format
    logger.info(
      {
        circuit_signer_address: this.signer.address,
        circuit_signer_balance: balance,
      },
      `Circuit Signer Address`,
    );
    this.prometheus.executorBalance.set(
      {
        signer: this.signer.address,
        target: this.config.circuit.name,
      },
      balance,
    );

    this.executionManager = new ExecutionManager(
      this.circuitClient,
      this.sdk,
      this.logger,
      this.config,
      this.prometheus,
    );
    await this.executionManager.setup(
      this.config.gateways,
      this.config.vendors,
    );
    logger.info("Executor setup complete");
    return this;
  }

  /** Configures the instance's pino logger. */
  async configureLogging(): Promise<Instance> {
    if (this.logsDir) {
      await mkdir(this.logsDir, { recursive: true });
      this.logger = createLogger(this.name, this.logsDir.toString());
    } else {
      this.logger = createLogger(this.name);
    }
    return this;
  }
}

export {
  Instance,
  ExecutionManager,
  Queue,
  Execution,
  SideEffect,
  Notification,
  NotificationType,
  TxOutput,
  TxStatus,
  SubstrateRelayer,
  Estimator,
  CostEstimator,
  Estimate,
  InclusionProof,
  BiddingEngine,
  StrategyEngine,
  SfxStrategy,
  XtxStrategy,
  PriceEngine,
  CoingeckoPricing,
  CircuitListener,
  ListenerEvents,
  ListenerEventData,
  CircuitRelayer,
  Config,
  Circuit,
  Gateway,
  Strategy,
  BiddingStrategy,
};
