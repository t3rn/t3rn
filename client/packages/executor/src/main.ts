import { config } from "../config/config";
import { AttestationManager } from "./attestationManager";
import { Executor } from "./executor.class";
import { logger } from "./logging";
import { Prometheus } from "./prometheus";

async function main() {
  logger.info("Starting prometheus");
  const prometheus = new Prometheus();

  logger.info("Starting executor");
  const instance = new Executor(prometheus);
  await instance.setup();

  // TODO: attestions for now should be disabled by default until ready
  if (config.attestations.enableAttestations) {
    if (config.attestations.ethereum.privateKey === undefined) {
      logger.warn("Ethereum private key is not defined. Skipping Attestations.");
    } else {
      const attestationManager = new AttestationManager(
        instance.circuitClient,
        prometheus,
      );
      await attestationManager.processAttestationBatches();
    }
  }
}

main();
