import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/main.ts", "tests/**/*.ts"],
  splitting: false,
  sourcemap: false,
  clean: true,
  format: ["cjs"],
});
