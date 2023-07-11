import { AttestationManager } from "./attestationManager";
import { Instance } from "./index";
import { config } from "../config/config";
import { logger } from "./logging";

async function main() {
  logger.info("Starting executor");
  const instance = new Instance(process.env.EXECUTOR);
  await instance.setup();

  const attestationManager = new AttestationManager(instance.circuitClient);
  if (config.attestations.processPendingBatches) {
    logger.info("Processing Pending Attestation Batches");
    await attestationManager.processPendingAttestationBatches();
  }
  // await attestationManager.listener()
}

main();
