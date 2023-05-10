import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/**/*"],
  splitting: false,
  sourcemap: true,
  clean: true,
  format: ["cjs", "esm"],
  external: [
    "@polkadot/keyring",
    "@polkadot/util",
    "@polkadot/util-crypto",
    "@polkadot/wasm-crypto",
    "@polkadot/types",
  ],
});
