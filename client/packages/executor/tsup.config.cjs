import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/main.ts"],
  splitting: false,
  sourcemap: false,
  clean: true,
  format: ["cjs"],
});
