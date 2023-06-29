import { AttestationManager } from "./attestationManager";
import { Instance } from "./index";
import { config } from "../config/config";

async function main() {
  const instance = new Instance(process.env.EXECUTOR);
  await instance.setup();

  const attestationManager = new AttestationManager(instance.circuitClient);
  if (config.attestations.processPendingBatches) {
    await attestationManager.fetchBatches()
    await attestationManager.processPendingAttestationBatches()
  }
  // await attestationManager.listener()
}

main();
