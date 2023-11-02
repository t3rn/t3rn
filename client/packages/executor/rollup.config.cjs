import typescript from "@rollup/plugin-typescript";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";

export default {
  input: "src/main.ts",
  output: {
    dir: "bin",
    format: "cjs",
  },
  plugins: [
    typescript({
      compilerOptions: {
        module: "ESNext",
      },
      sourceMap: false,
      outDir: "bin",
    }),
    nodeResolve(),
    commonjs(),
    json(),
  ],
};
