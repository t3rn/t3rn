#### Milestone 1: Initial Structure & Gateway API — 2 Weeks
##### Initial Structure
- Document and standardise Gateway Standards, Engine, API.
- Create initial Substrate modules & structure for Gateway Standards, Engine, API filled with mocked functions.
- Create the structure for the multi-step transaction. 
##### Gateway API
- Implement `multistep_call` RPC endpoint that receives the multi-step transctions, validates them and passes into Gateway Engine.
- Implement the `rent_projection` function that passes the query about the execution projection to Gateway Engine. 
- Create an example, already compiled, package and demonstrating sending Gateway API. Update the documentation and readme files accordingly.

#### Milestone 2: Gateway Engine — 6 weeks
- Execute received from Gateway API packages and pass the current execution results back to the API. 
- Implement Escrow Accounts:
    - Implement Escrow Account submodule that holds the multi-step transaction executed on that parachain within its storage.
    - Adds the authorization layer on top of changes to Escrow Accounts granting the write access only to authorized accounts (eventually t3rn validators, in the meantime to trusted parties).
    - Calculates the merkle path out of storage transitions to Escrow Accounts as execution proofs of the `Execution` phase.
- Implement the `Revert` and `Commit` phases handlers, that either move the state out of Escrow Account into the target accounts or invalidate the operation. Validate whether the accounts were mutated and provide convenient configuration to deal with this special case.
- Present the proofs of execution and inclusion that accesses the state of a parachain to calculate merkle paths out of state and extrinsic tree.
- Integrate the Engine with Parachains supporting both Balance and Contract pallet. Adhere to the operative fee strategy.
- Prepare functional tests checking validity of execution results and their proofs for a few example packages.
- Update the documentation and readme files to ensure they're aligned with implementation. 

#### Milestone 3: Gateway Standards — 4 Weeks
- Prepare Standards - the equivalent of Contract pallet external execution context to work in multiple phases with the use of Escrow Account.
- Allow modules to be executed via Gateway as host functions, in order to support non-standard functionalities hosted by parachains that do not include Contracts pallet.
- Update the documentation and readme files to ensure they're aligned with implementation. 
- Write integration test checking the functionality of standalone Gateway as a whole.
- Provide a Docker image with standalone Gateway.

#### Milestone 4: Package Compilation Tools — 4 Weeks
- Create the compilation tool that translates the contracts and modules into the packages, as described in details in "Package, Contract, Module" subsection. 
The compiler divides the business logic into several chunks which can be executed separately on gateways, but make sense as a whole in the context of overall multi-step transaction execution.
- Create the tool for signing and sending the interoperable transactions to the Gateway.
- Demonstrate the execution on standalone gateway by preparing a few examples, which will be included in the documentation.
- Create the docker image with preloaded modules and the example of sending packages into the Gateway.
