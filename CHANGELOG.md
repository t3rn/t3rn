# Changelog

## `v1.0.0` 2022-10-28
#### Shell Runtime of t3rn
* no custom features
* open collator pool
* upgradable runtime

## `v1.2.0-rc.2` 2022-11-21
* fix: add bad_blocks extension to t0rn specs

## `v1.2.0-rc.1` 2022-11-21
* fix: restore block time to 12s

## `v1.2.0-rc.0` 2022-10-24
## What's Changed

### Features
* feat: consolidate runtimes + Integrate XBI + async reward claims for SFX execution + 3VM LVL 4 by @MaciejBaj in https://github.com/t3rn/t3rn/pull/448
* feat: executors bid for all SFX narrowed to Optimistic and Escrow by @MaciejBaj #477
* feat: utilise the rearchitecture for 3VM and install EVM to the nodes by @AwesomeIbex in https://github.com/t3rn/t3rn/pull/400
* feat: Portal LVL3 - Verify ingress messages finality using pluggable Light Clients by @petscheit in https://github.com/t3rn/t3rn/pull/456

### Maintenance & Docs
* docs: add github and social badges to readme by @chiefbiiko in https://github.com/t3rn/t3rn/pull/409
* fix: Correct received event names after new SFX are available for executors by @petscheit in https://github.com/t3rn/t3rn/pull/391
* docs: add registration part to collator docs by @alexand3rwilke in https://github.com/t3rn/t3rn/pull/411
* fix: fix devnet by unpinning subkey by @chiefbiiko in https://github.com/t3rn/t3rn/pull/414
* docs: update rotate keys part by @alexand3rwilke in https://github.com/t3rn/t3rn/pull/418
* feat: unify side effects between sdk and circuit  by @beqaabu in https://github.com/t3rn/t3rn/pull/413
* chore: enable collator telemetry by @chiefbiiko in https://github.com/t3rn/t3rn/pull/426
* build: Prepare continous collator deployments by @chiefbiiko in https://github.com/t3rn/t3rn/pull/415
* refactor: provide non-sensitive artifacts for collators by @chiefbiiko in https://github.com/t3rn/t3rn/pull/434
* refactor: update TS types to new portal setup and adds step based execution and confirmation logic by @petscheit in https://github.com/t3rn/t3rn/pull/436
* refactor: add common runtime types as a separate crate by @MaciejBaj in https://github.com/t3rn/t3rn/pull/442
* refactor: the collator docker image to incl. all runtime deps by @chiefbiiko in https://github.com/t3rn/t3rn/pull/437
* chore: add DepositReceived event from AccountManager by @MaciejBaj in https://github.com/t3rn/t3rn/pull/455
* refactor: update path to latest XBI Portal by @MaciejBaj in https://github.com/t3rn/t3rn/pull/458
* build: commit linting during pipeline execution by @phpengine in https://github.com/t3rn/t3rn/pull/393
* build: revive devnet run.sh script by @MaciejBaj in https://github.com/t3rn/t3rn/pull/460
* build: update builds and source files to Polkadot v0.9.27 by @MaciejBaj in https://github.com/t3rn/t3rn/pull/467
* refactor: install pallet assets to mock, standalone and t0rn runtimes by @MaciejBaj 

Full Changelog: https://github.com/t3rn/t3rn/compare/v1.1.0-rc.0...v1.2.0-rc.0

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
