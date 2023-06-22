import { AttestationManager } from "./attestationManager";
import { Instance } from "./index";
import * as fs from 'fs';
async function main() {
  const instance = new Instance(process.env.EXECUTOR);
  const attestationManager = new AttestationManager(instance.sdk);
  attestationManager.receiveAttestationBatchCall();

  // await instance.setup();
}

main();
