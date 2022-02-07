# @t3rn/types

This package contains all the necessary types and RPC calls to interact with the t3rn protocol via the @polkadot/api client.

## Usage

Just add this package as a dependency to your project, import it and instantiate a new `ApiPromise` client to a t3rn 
collator node as shown below.

```typescript
import { rpc, types } from '@t3rn/types'
import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';

const circuitApi = new ApiPromise({
  provider: new WsProvider('localhost:9944'),
  types,
  rpc
});

```

## Usage in @polkadot/apps

To get a JSON file ready for use with the https://polkadot.js.org/apps interface, you can use directly the contents of
`dist/types.json` or run `yarn types:json` to generate a fresh one at the same location.

## Development

To build the type definitions, access to a running circuit standalone or collator node is required. This can be either locally 
as documented in the [circuit README](../../node/README.md), or online.

Make any updates to the proper `definitions.ts` file, please make sure to not edit any other file as they are generated
by the `@polkadot/types` tools and your changes will be overwritten.

Finally, after your changes are done, just do a `yarn build`.