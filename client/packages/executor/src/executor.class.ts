import { Sdk } from "@t3rn/sdk";
import { Keyring } from "@polkadot/api";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { KeyringPair } from "@polkadot/keyring/types";
import { ExecutionManager } from "./executionManager";
import { config, Config } from "../config/config";
import { getBalanceWithDecimals } from "./utils";
import { Logger } from "pino";
import { Prometheus } from "./prometheus";
import { logger } from "./logging";

/** An executor instance. */
export class Executor {
  name: string;
  circuitClient: Sdk["client"];
  executionManager: ExecutionManager;
  sdk: Sdk;
  signer: KeyringPair;
  config: Config;
  logger: Logger;
  prometheus: Prometheus;

  /**
   * Initializes an executor instance.
   *
   * @param name Display name and config identifier for an instance
   */
  constructor(prometheus: Prometheus) {
    this.prometheus = prometheus;
    this.config = config;
  }

  /**
   * Sets up and configures an executor instance.
   */
  async setup(): Promise<Executor> {
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

    // Set nonce on start
    this.sdk.nonce = (
      await this.circuitClient.rpc.system.accountNextIndex(this.signer.address)
    ).toNumber();
    logger.info(`Set nonce for circuit to ${this.sdk.nonce}`);

    // TODO: print wallet balance on available networks
    const balance = await getBalanceWithDecimals(
      this.circuitClient,
      this.signer.address,
    );

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

    // In case anything goes wrong with nonces we should reset it on each new block to chain state
    this.watchBlock();

    logger.info("Executor setup complete");
    return this;
  }

  async watchBlock() {
    await this.circuitClient.rpc.chain.subscribeNewHeads(async () => {
      // Process the new block header
      this.sdk.nonce = (
        await this.circuitClient.rpc.system.accountNextIndex(
          this.signer.address,
        )
      ).toNumber();
      logger.info(`Set nonce for circuit to ${this.sdk.nonce}`);
    });
  }
}
