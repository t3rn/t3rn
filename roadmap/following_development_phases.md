#### Phase 2: Gateway Circuit & Registry — 1 Month 
##### Registry
- Implement the registry that stores Packages, Standards, Blockchains and Gateways. 
- Provide the Registry API module to post and get the entities to and from the registry.
- Register two example blockchians in the registry. 
##### Circuit
- Implement Gateway Circuit that sends and receives the messages to and from a gateway.  
- Implement the Circuit execution that processes the transaction on all affected gateways.
    - Validate the proofs received from a gateway and store them in the execution context. 
- Implement the execution phases into the circuit. 
- Implement the customizable execution order into the phases.
- Implement the accounts and balances for t3rn packages executioners.
- Provide Circuit API that receives the singed interoperable transactions and stores them as packages in the registry.

#### Phase 3: Consensus & Network — 1 Month 
- Implement consensus and validation system over the Gateway Circuit interoperable execution and proofs.
- Introduce network validators that form the consensus and host the interoperable execution provided by Gateway Circuit.
- Introduce the fee model for interoperable execution.
- Release a test network of t3rn hosting the platform for interoperable execution.

#### Phase 4: Additional Packages — 1 Month 
- Implement the swap package, allowing placing and the swap orders for a limited and market price.
- Implement the SEA package, that consumes the service registered on a different parachain using the swap package. 
- Demonstrate the SEA package usage by deploying the SEA & swap packages into the testnet network and creating a simple GUI.

## Future Steps 
- Implement Gateway with XCMP API. 
- (Opt) Implement Gateway with Bridge API. 
- Release t3rn as a cryptocurrency and integrate with a 3rd party decentralised exchange. 
- Implement the transfer auto package that seamlessly exchanges the cryptocurrencies between parachains affected in the interoperable execution.
- Implement gov module that governs t3rn Registry.

