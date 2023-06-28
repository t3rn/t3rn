import { AttestationManager } from "./attestationManager";
import { Instance } from "./index";

async function main() {
  const instance = new Instance(process.env.EXECUTOR);
  await instance.setup();

  const attestationManager = new AttestationManager(instance.circuitClient);
  // await attestationManager.listener()
  await attestationManager.processPendingAttestationBatches()
}

main();
