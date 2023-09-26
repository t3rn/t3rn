import { config } from "../config/config";
import { AttestationManager } from "./attestationManager";
import { Executor } from "./index";
import { logger } from "./logging";
import { Prometheus } from "./prometheus";

async function main() {
  logger.info("Starting prometheus");
  const prometheus = new Prometheus();

  logger.info("Starting executor");
  const instance = new Executor(prometheus);
  await instance.setup();

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
