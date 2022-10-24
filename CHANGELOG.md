# Changelog

## `v1.2.0-rc.0` 2022-10-24

## `v1.1.0-rc.0` 2022-07-14

### Features
- [**3VM**: repatriate gas fees to contract authors](https://github.com/t3rn/t3rn/pull/295)

- [**3VM**: create pallet account manager](https://github.com/t3rn/t3rn/pull/273)


### Minor Features
- **3VM**: Introduce system contracts to 3VM 

- **3VM**: Provide the hardened side effects in LocalState for SDK  

- **3VM**: Start handling posted signals asynchronously

- [**Circuit**: Execute optimistic side effects in batches](https://github.com/t3rn/t3rn/pull/306)
   - _Add support for execution and confirmation of multiple side effects_

- [**Portal**: Add parachain header support](https://github.com/t3rn/t3rn/commit/aa1eb714bf9e70dde3822bb3a2533d59ddc54a30)

- [**Portal**: Update ranger architecture to support dynamic instances and new proofs](https://github.com/t3rn/t3rn/commit/b2bb50d8fd73503ff83e8fe012f392e9d63f36ac)

- [**Portal**: Add parachain type to XDNS entry and decodes parachain header correctly](https://github.com/t3rn/t3rn/commit/27e62f14817c7b8839d0f354877069c82de0f700)

- [**Portal**: Combine grandpa-ranger and executor](https://github.com/t3rn/t3rn/commit/92c12564977a0db951369c3fdc6e1f948bb1ddb5)


- [**Portal**: Expose read and compare best available height]()


### Maintenance  

- **Portal**: Seed the XDNS registry with all Rococo parachains

- **Community**: Create t0rn collator docs introduction

- **Portal**: Fix invalid header ranges by refetching finalized headers before subm… 

- **Portal**: Restore correct bridges init and event verify from Portal to MFV

- **Portal**: Reconnect MFV to runtimes after Portal is dep no longer

- **Portal**: Fix hitting an invalid justification target

- **Circuit**: Fix creating new received side effect object to avoid override

- **Circuit**: Fix xtx cancel timeout by adding current block offset

- **Circuit**: Emit new side effects before Xtx status

- **Circuit**: Emit side effect ids next to new side effects

## `v1.0.0-rc.2` 2022-04-28

Adds pallets scheduler and preimage to the parachain runtime.

## `v1.0.0-rc.1` 2022-04-26

Covers the same sub products as `v1.0.0-rc.0` but fixes the collator setup.

## `v1.0.0-rc.0` 2022-04-25
#### Release Candidate to Rococo

Includes the following sub products:

- #####  [Circuit LVL 2 - Xtx Lifecycle and steps auto-assignment for both Gateways & 3VM](https://github.com/t3rn/t3rn/pull/279)
- #####  [Portal LVL 1 - Dynamic on-chain registration of new Substrate-based chains based on GRANDPA range proofs](https://github.com/t3rn/t3rn/tree/development/pallets/circuit-portal)
- #####  [3VM LVL 1 - Volatile WASM Contracts execution from Contracts Registry](https://github.com/t3rn/t3rn/pull/270)
