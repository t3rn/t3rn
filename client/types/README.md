# @t3rn/types

This package contains all the necessary types and RPC calls to interact with the t3rn protocol via the @polkadot/api client.

## Installation 

Just add this package as a dependency to your project, import it and instantiate a new `ApiPromise` client to a t3rn 
collator node as shown below.

Since `@polkadot/typegen@^7.0.0`, all custom types are injected to the `@polkadot/*` namespaces.

```typescript
import '@t3rn/types'
import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';

const circuitApi = new ApiPromise({ provider: new WsProvider('localhost:9944') });

```

## Example extrinsic call with types 

Given you have already imported the types, you can try calling an extrinsic as follows: 
```typescript
const keyring = createTestPairs({ type: 'sr25519' });
return circuitApi.tx.circuit
.onExtrinsicTrigger(
  [{
    target: 'abcd', // [97, 98, 99, 100] 
    prize: 0,
    orderedAt: 0,
    encodedAction: 'tran', 
    encodedArgs: [
      keyring.alice.address, 
      keyring.charlie.address, 
      10000000
    ],
    signature: [],
    enforceExecutioner: false,
  }],
  0, // fee must be set to 0
  true
).signAndSend(keyring.alice)
.catch(err => console.error(err));
```

## Usage in @polkadot/apps

To get a JSON file ready for use with the https://polkadot.js.org/apps interface, you can run `yarn load:meta` with a 
circuit node running, to generate the latest metadata.

## Development

To build the type definitions, access to a running circuit standalone or collator node is required. This can be either locally 
as documented in the [circuit README](../../node/README.md), or online.

Past that, all you need to do is run `yarn generate`, make sure everything generation was completed successfully and then `yarn build`.

Make any updates to the proper `definitions.ts` file, please make sure to not edit any other file as they are generated
by the `@polkadot/types` tools and your changes will be overwritten.

Finally, after your changes are done, just do `yarn build`.