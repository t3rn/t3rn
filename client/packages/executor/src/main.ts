import { AttestationManager } from "./attestationManager";
import { Instance } from "./index";
import { logger } from "./logging";

async function main() {
  logger.info("Starting executor");
  const instance = new Instance(process.env.EXECUTOR);
  await instance.setup();

  const attestationManager = new AttestationManager(instance.circuitClient);
  logger.info("Processing Attestation Batches");
  await attestationManager.processAttestationBatches();
}

main();
