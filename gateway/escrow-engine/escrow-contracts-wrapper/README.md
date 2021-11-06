# Escrow Contracts Wrapper

This module re-uses the original Contracts trait but wraps it additionally into escrow execution that may be reverted still be reverted or committed in the later execution phases.

The new functionality of escrow calls is solely contained in [`escrow_exec`](src/escrow_exec.rs).

## Interface

### Functions

* `escrow_call` - Makes a call to an account, optionally transferring some balance. The successful execution is reverted, but all of the internal storage writes and transfers are spied on and added into the deferred collections, that may be released during commit phase or discarded during revert phase.

## Usage

The module is used by `pallet-contracts-gateway`, which integrates with `pallet-contracts` in order to dispatch the calls using the wrapped escrow calls via intermediary escrow account. 

## Related Modules

* [Contracts](https://github.com/paritytech/substrate/blob/master/frame/contracts)

## License

---
Copyright 2020 Maciej Baj.

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
    
---
