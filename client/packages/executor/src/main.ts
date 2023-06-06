import { Instance } from "./index";

async function main() {
  const instance = new Instance(process.env.EXECUTOR);
  await instance.setup();
}

main();
