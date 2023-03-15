# @t3rn/types

This package contains all the necessary types and RPC calls to interact with the t3rn protocol via the @polkadot/api client.

## Usage

```typescript
import "@t3rn/types"; // always import for @polkadot/api augmentations
import { createType } from "@t3rn/types"; // function for building typesafe types
import { T3rnTypesSfxSideEffect } from "@polkadot/types/lookup"; // import the specific type that was added via augmentation

let sfx: T3rnTypesSfxSideEffect = createType(
  "T3rnTypesSfxSideEffect",
  // The second parameter is automatically typesafe!
  {
    target: "roco",
    maxReward: 1000000,
    insurance: 100000,
    encodedAction: "tran",
    encodedArgs: ["0x0", "0x1"],
    signature: "",
    enforceExecutor: "",
    rewardAssetId: 1,
  }
);
```

In your project's `tsconfig.json` add the following compiler options:

```json
 "compilerOptions": {
    "module": "esnext" /** Ensures tsc uses esmodules **/,
    "target": "ES2018" /** Use a recent target **/,
    "esModuleInterop": true ,
    "moduleResolution": "node",
    "skipLibCheck": true /** Important: skips type checking for external libraries  **/
  }
```

In your project's `package.json` add the following:

```json
  "type": "module" /** Ensures this package is esm **/,
  "exports": {
    ".": "./index.js"  /** Define your exports **/
  },
```
