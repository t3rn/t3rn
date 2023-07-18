import { config } from "../config/config";
import { AttestationManager } from "./attestationManager";
import { Instance } from "./index";
import { logger } from "./logging";

async function main() {
  logger.info("Starting executor");
  const instance = new Instance(process.env.EXECUTOR);
  await instance.setup();

  if (config.attestations.ethereum.privateKey === undefined) {
    logger.error("Ethereum private key is not defined.");
  } else {
    const attestationManager = new AttestationManager(instance.circuitClient);
    if (config.attestations.processBatches) {
      logger.info("Processing Attestation Batches");
      await attestationManager.processAttestationBatches();
    }
  }
}

main();
