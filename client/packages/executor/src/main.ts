import { Keyring, Sdk, cryptoWaitReady } from "@t3rn/sdk";
import { AttestationManager } from "./attestationManager";
import { Instance } from "./index";

async function main() {
  await cryptoWaitReady();
  const keyring = new Keyring({ type: "sr25519" })
  const signer = keyring.addFromUri("//Bob");

  const sdk = new Sdk(process.env.WS_CIRCUIT_ENDPOINT || "ws://localhost:9944", signer)
  await sdk.init();

  const instance = new Instance(process.env.EXECUTOR);
  await instance.setup();

  new AttestationManager(instance.circuitClient);
}

main();
