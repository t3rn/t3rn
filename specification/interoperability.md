## Mission

Our mission is to create a complete / useful / convenient platform to build interoperable solutions using "most wanted blockchain projects" and mixing their functionalities together. We target blockchain teams & individual developers but also dev teams from other tech branches that look for the way to access the power of interoperability :))

Our mission is to create the flexible programming environment that connects the most wanted & popular blockchains and their contract languages into useful & convenient interoperable solutions ready to plug and play on any blockchain. The dev teams by design grow that decentralised contracts repository & share all of the resources.

## Interoperable 

In this section we outline how interoperability is achieved by the open & decentralized network following t3rn protocol. The protocol assumes the network is open for new blockchain to be added and new actors to join and incentives them to behave honestly while slashing their bonded stake for proven misbehaviours. 

#### Foreword to interoperability

By providing an interoperable solution we mean that interaction with external (extrinsic) blockchain systems is possible using the same ways of exchanging messages with common on-chain (intrinsic) entities. 
In other words, from the perspective of intrinsic blockchain entities, processing interoperable transactions looks no different than for any other regular accounts. Interoperable transactions however, trigger actions on external services - dispatch transactions on extrinsic blockchains. This raises a concern - who is authorized to dispatch the messages to extrinsic blockchains and stays in control of funds? In t3rn, that entity is decentralised, open for anyone with a bonding stake to join and with transparent protocol rules outlined in this document.

### Interoperable Execution
// Must edit significantly
It's important to distinguish between the execution type possible on every blockchain involved in the cross-chain execution. Currently, there are many systems that consider interoperability as a possibility to move funds (fungible assets) between chains using a decentralised or semi-decentralised middleman. Semi-decentralised systems rely on federated parties co-signing the transactions locking and releasing on two blockchains involved in the transfers. There aren't many bridges we are aware of that facilitate the message passing relying on the on-chain code deployed on both ends of the bridged blockchains. One of them being a Parity Bridge which is currently under development and can't be considered production ready. The rest of them, federated parties form a consensus for attesting each transaction to be valid (ChainSafe Bridge).

We are not fully satisfied with the status quo and designed a decentralised protocol capable of full-spectrum of interoperable features:
- Type 1. fungible (balance) and non-fungible cross-chain assets transfers
- Type 2. execution of on-chain generic programs registered on extrinsic blockchains (calling smart contracts or on-chain runtime logic)
- Type 3. volatile execution of generic programs (smart contracts) within a context of different blockchains. Executes with a given state and can only access the programs registered on extrinsic chains. It's volatile - removes itself & the assigned storage on extrinsic chains after the execution. See more at "Volatile" subsection of the Composition section. 

#### Distinguishing different types of Blockchain integration
We must set a framework that classifies what types of blockchains we differentiate in order to provide an accurate framework that enables their integration within the same interoperable system. The design should be flexible enough to support the major blockchain mechanisms. 

The main differentiation we make is whether it's possible to program generic logic within a blockchain's runtime and until which extended permission allows access and modification of the accounts ledger to non-owners to users with special privileges (sudo users).

##### Type 1. Programmable blockchains + the owner-only access to accounts ledger
Examples: Ethereum, other smart contract platforms

It's possible to deploy new smart contracts or any other generic logic that oversee the control of the execution results, be responsible for withdrawing reward or refund claims. The access to smart contract is held by its developers and users must trust the code they write. The funds deposited to that smart contract's account can be distributed to those with valid claims as instructed by the code.

##### Type 2. Programmable blockchains + the sudo access to accounts ledger
Examples: Polkadot Parachains with t3rn gateway pallet installed 
The generic logic can be built-in into blockchain granting the full access to accounts ledger to privileged sudo users. 

##### Type 3. Non-programmable (transaction-only) blockchains + owner-only access to accounts ledger
Examples: Polkadot Relay Chain, Lisk main chain

Restricted access to generic logic, usually only balance transfer & consensus-specific transactions are allowed to dispatch by accounts with non-negative balance on those chains. 
Access to those accounts facilitated by a multisignature commitee, that follows particular rules to access funds. Those rules can be better or worse decentralised, therefore the solution is not ideal as always relies on honesty of limited amount of parties involved in forming those multisignatures.

Interoperability with Type 1. & Type 2. can be achieved with actors of t3rn protocol having no direct access to the accounts on connected blockchains. 

Interoperability with Type 3. can only be achieved by actors of t3rn protocol staying in control of the accounts of connected blockchains. This is of course a disadvantage and t3rn protocol aims to decentralize that control the fair degree. This can be done by "fast Multiparty Threshold ECDSA" formed by the t3rn validators and oversee the security by introducing bonding of stake for the multisignature participants.

### Gateways

Execution of a contract (byte code binaries) within the context of a single chain must be secured via a dedicated wrapper -- Gateway. 

In theory any valid account could facilitate the execution on extrinsic blockchains ordered by requesters. In practice, only execution agents approved by the Circuit have incentive to do it - they expect rewards for their honest services. The circuit validators expect the inclusion & execution proofs to be delivered by the relayers. The execution on any extrinsic chain must follow the execution protocol described in details in "execution stamp" subsection. 

t3rn works on the basis of an open & decentralised system allowing parties to use different implementations of gateways as long as they follow the protocol. After all, execution agents are the ones at risk of losing their bonded stake, baring consequences for the gateway's wrongful implementation. We very much encourage the community to create their own more efficient gateways implementation in the spirit of a healthy competition. We however foresee that at the beginning our team will implement the gateways to the most popular blockchains in order to make the product get the traction. 

#### Gateway Protocol
// ToDo: Present in a creative way

-> Execution Request
    - requester
    - target
    - phase
    - value
    // Additionally for programmable gateways 
    - code
    - input
    - ?state?
    
-> Execution Algorithm
// For all gateways (transaction-only)
MATCH phase:
    CASE "execution*": // For all types of execution
        . Transfer fungible or non-fungible assets from requester to escrow account & add to deferred transfers.
    CASE "revert"
        . Cancel deferred transfers from escrow account back to the requester
    CASE "commit"
        . Release deferred transfers from escrow account to target destination

// Additionally for both extrinsic + intrinsic programmable gateways
MATCH phase:
    CASE "execution_*": // For all types of execution
        . Recreate attached byte code with given state (read more at recreate subsection of composability)     
            . Emit EVENT_RECREATED with the instantiated smart contract (or runtime module) identifier
        . Execute the recreated smart contract (or runtime module) with given input. 
            . Allow the forward calls to contracts registered on that chain. For each call emit GATEWAY_EXTERNAL_CALL with input parameters and target contract address.
        . Record all of the fungible or non-fungible transfer throughout the execution to deferred transfers.
        . Revert transaction after successful execution. Expect native to the foreign blockchain to be emitted on that occasion. While registering a new blockchain to the registry the event structure needs to be added as a part of the protocol.
        . Emit EXECUTION_SUCCESS event with encoded EXECUTION_STAMP as a parameter.
    CASE "execution_dirty":
        . Do not revert transaction after successful execution
    CASE "execution_pure":
        . Enforce the state of extrinsic contracts (recreated smart contract from attached code can still be modified) to stay unmodified as part of the execution (`staticcall` for EVM)
// Additionally for intrinsic programmable gateways
    CASE "execution_*":
        . Record all of the deposited events, storage writes throughout the execution.
    CASE "revert"
        . No extra actions required - no changes after "execution_pure" & irreversible changes after "execution_dirty"
    CASE "commit":
        . Release deferred events recorder during execution
        . Apply deferred storage writes (needs sudo access to accounts ledger therefore possible only for intrinsic gateways)

// Additional refunds & rewards claim logic and execution ordering for extrinsic + intrinsic programmable gateways.


. Execute the 
Witness (collected by validators):
    - Block header pre-execution
    - Block header post-execution
    
Proof (collected by relayers or execution agents and delivered to validators):
    - Pre-state & post-state of all accounts involved in execution
    - All events emitted during execution

Execution Stamp:
    // For all gateways
    - deferred balance transfers
    // Additionally for both extrinsic + intrinsic programmable gateways
    - post-execution storage merkle tree root
    - deferred output
    - deferred events emitted
    // Additionally for intrinsic programmable gateways
    - deferred storage writes

Validate Proof:
    - Re-create Execution Stamp by executing the given code + state on a correct VM locally in a sandboxed environment
    - Confirm the valid execution stamp in the events merkle tree of post-execution block
    - Validate phase
        Match phase:
            CASE "execution":
                IF programmable_gateway:
                    - Verify if whether the following events were emitted events with correct parameters: 
                        - VOLATILE_CODE_RECREATED, VOLATILE_CODE_DESTRUCTED, TRANSACTION_REVERTED (if not "execution_dirty")
                    - Scan through all GATEWAY_EXTERNAL_CALL events. For each call to external contract verify that the state wasn't modified by the escrow account. Allow changes only for execution_dirty phase. Note, that it's impossible to verify the correctness of external calls to contracts registed on forensic blockchains. Though we know that contract was actually called as instructed by a requester as the correct event was emitted and included within a block.
            CASE "revert":
                - Confirm that the requester of deferred transfers have their fungible and non-fungible assets reimbursed by escrow account. 
            CASE "commit":
               - Confirm that the target destinations of deferred transfers received have fungible and non-fungible assets from escrow account. 

Extrinsic vs Intrinsic blockchain execution environment from the perspective of t3rn network operators. 
Programmable vs transaction-only differentiates the gateways based on the type of blockchain and its permission to execute generic, programmable logic within it (most probably smart contracts or any other access to blockchain's runtime execution).

### Programmable Gateway

Via programmable gateway a composable contract can be ordered for execution by execution agents. There must be an implementation that follows the operative way of execution on-chain generic logic. Two functionalities are required to be implemented by programmable gateway:
- wrapper over regular execution that adds on (see Composability):
    - Composability: allows for the recreation of attached code within a context of that blockchain
    - Reversibility: shields the target accounts via escrow accounts and defers the changes with respect to the execution phases.

- tracker of the latest t3rn headers which have been submitted to relay chain bridge and the logic that verifies that headers have been finalized. This assures integrity with the Circuit, facilitate rewards & refunds claims in a trust-free manner, with no single entity being in control of the funds

The extrinsic execution is validated locally, by t3rn Circuit validators that have the same Virtual Machine installed.

The examples of these implementation vary between extrinsic and intrinsic gateway:
#### Extrinsic Programmable Gateway
Execution of byte code via extrinsic way achieves compatibility with the t3rn Gateway Protocol using standard methods of execution programmable logic on foreign chains. The execution of the programmable logic is facilitated via Virtual Machine or any other way of generic logic onto on-chain runtime. The gateway like any other smart contracts / binary code on that chain must be written, compiled and successfully instantiated following the deployment process specific for that chain. Both of the required programmable gateway functionalities must be implemented in that way. 

To integrate a blockchain via extrinsic programmable gateway no special access on the part of a dev team behind a project is required.  

Example Virtual Machine: EVM (Ethereum), Solana Runtime 

#### Intrinsic Programmable Gateway
Execution of native byte code via the Intrinsic Gateway becomes an integral part of that blockchain and will be maintained and executed by its operators. This can only be achieved by the inclusion of the gateways by development team & allowing a special sudo access to accounts ledger. The intrinsic programmable gateway offers the full spectrum od t3rn features for the interoperable execution by making all of the execution types possible and reversible (including execution_dirty). 

Example Virtual Machine: Versatile Wasm VM for Polkadot Parachains - both Parachains with Contracts Pallet & without are supported.

#### Transaction-only two-way Gateway 
That's the non-programmable type of gateway that makes it possible to facilitate transfer-only (fungible + non-fungible) transactions through it. The execution, in respect to the protocol and its execution phases can still be reverted, so that a requester could get their assets back in case an interoperable transaction fails.
 
 Transaction-only gateways do not require implementation on the side of foreign chains as an opposite to programmable gateway. However it comes with a cost of the multi-party signature formation for t3rn validators. 
From the perspective of a foreign chain, the gateway looks just like any other regular account. 

That gateway is generally designed to connect t3rn Circuit with Type 3. transaction-only blockchains, like Polkadot Relay Chain. That does not mean however, that blockchains with extended capabilities to add the on-chain programmability, like Ethereum, can't integrate the transaction-only gateway as well. This is further explained in the Circuit subsection.

Transaction-only Gateway is two-way, that means that it comes with two components connected. Relayers dispatch signed transactions from foreign chains to the circuit (execution orders) and the other way around (claims & refunds). On the circuit, an execution agent is chosen to facilitate the transaction. After fulfilling its duty validators generate a transaction with a reward transfer for the execution agent and co-sign it. That is a valid on foreign chain transaction that transfers an amount deposited on the gateway by a requester. Similar transaction with a refund to the requester is created and co-signed by validators in case of dishonest behaviour on the part of the execution agent.


## Circuit
Naming after the electrical circuit isn't by coincidence. The elements are connected as part of 

### Maintainers 

### Registry
\subsubsection{Registry}
Registry of all of the protocol rules, actors, services and contracts involved in composable execution. Registry is maintained by collators. 
\begin{itemize}
    \item Composable Contracts -- each successful execution appears the new contract to the composable contracts. These can be re-used by other developers to help them in building their own decentralised services. Contracts can also be submitted into the registry with no prior-execution. 
    \item Chains -- chains need to be pre-registered in the registry in order to provide the information about their Gateways, which in turn provide the information about fees (to calculate execution costs) and standards available to be called on that chain alongside with the output parameters that are necessary to form execution and inclusion proofs.
    \item Statistics -- statistics about actors of the system that can serve as a base for the reward campaigns instigated by the community via governance.
\end{itemize}

