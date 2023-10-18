import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/**/*.ts"], // Include all TypeScript files in the "src" directory
  splitting: false,
  sourcemap: false,
  clean: true,
  format: ["cjs"],
});
