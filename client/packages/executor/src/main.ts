import { config } from "../config/config";
import { AttestationManager } from "./attestationManager";
import { Instance } from "./index";
import { logger } from "./logging";
import { Prometheus } from "./prometheus";
import { SingletonComponent } from "./singleton";


async function main() {
  logger.info("Starting prometheus");
  const prometheus = new Prometheus();

  logger.info("Setting up singleton");
  const singleton = new SingletonComponent()
  await singleton.setup(config.gateways)

  logger.info("Starting executor");
  const instance = new Instance(process.env.EXECUTOR, false, prometheus);
  await instance.setup(singleton);

  if (config.attestations.ethereum.privateKey === undefined) {
    logger.warn("Ethereum private key is not defined. Skipping Attestations.");
  } else {
    const attestationManager = new AttestationManager(
      instance.circuitClient,
      prometheus,
    );
    if (config.attestations.processBatches) {
      logger.info("Processing Attestation Batches");
      await attestationManager.processAttestationBatches();
    }
  }
}

main();
