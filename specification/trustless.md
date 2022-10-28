
## Trust-free
In this subsection we outline how to facilitate the composable transactions over multiple blockchains in a trust-free manner. We design an open network for non-federated actors to join and be incentivised to provide honest & high-quality services. 

Decentralised t3rn network is design to fit into the Polkadot's Parachain framework and become part of Polkadot by leasing a parachain slot. This underlies the following proposal on of how the network's economic is arranged with stipulation on the open-market service providers with zero-block rewards. Such a construct is possible from the point of Blockchain security, as the decision on the branch validity is made by Polkadot's Relay Chain. The Rely Chain provides the Parachains with finality gadget as well. Parachains can lease the security of the Polkadot network by bonding DOT for the parachain slot.

### Actors & Roles

All of the source code and binaries will be actively maintained and published by the team so that there is no restrictions of how many partipants of the network there must be. To provide the secure framework over the composable execution we distinguish additional actors within the system and design economy to incentive an open participation within it:
#### Requesters
Create, sign and submit the interoperable transaction for execution. Requesters submit the order directly to the Circuit or dispatch it via Gateway. 

Expecting the interoperable transactions to be execution requester also pay execution fee that needs to cover all of the necessary fees for collating, validating and relaying necessary data for his transaction. Fees can be dynamically adjusted depending on the network overload and a demanded priority.

#### Interoperable Transactions
If a transaction is dispatched by a requester via native to that foreign chain Gateway, it must have following attributes assigned to the message (that means that all of the blockchains that implement extrinsic  programmable or transaction-only gateway must support a generic opt code attached to a transaction (e.g. "data" field for Ethereum tx). That optional message contains: \\ $code\_hash = contract address on Circuit \\  input = optional input for that contract$ \\ $input\_length = length of input data bytes \\$
    The execution of interoperable transaction has a following lifecycle:
    \begin{enumerate}
        \item Either requester submits a transaction for execution to the native chain A and relayers submit it to the Circuit.
        \item Or requester submits a transaction for execution directly to the Circuit.
        \item Execution Agent is being selected as per rule described in "Selection Method". 
        \item Execution Agent facilitates the execution on appointed chains, following the execution schedule outlined by the requester. It risks its own stake and acts as an intermediate escrow account securing the execution. 
        \item After execution of each step is completed and the transaction with proofs is confirmed by the Relay Chain, Execution Agents can claim the funds deposited by requesters on native chains via gateways.
    \end{enumerate}

#### Collators
Maintainers of t3rn network resources & Circuit operators. They act as a collator of the as per Polkadot's Parachain nomenclature, maintaining a full-node of the parachain, retain all necessary information of the parachain, and produce new block candidates to pass to the Relay Chain validators for verification and inclusion in the shared state of Polkadot. Collators store pending interoperable transactions in a pool and match the proofs provided by relayers with attestations of execution agents.
    Collators produce blocks candidates and submit them to Relay Chain. Collators include the block reward for their services paid in \$t3rn token, which is set by Requester.

#### Validators / Execution Agents
Facilitate interoperable transactions between the Gateways.
Execution Agents can expect rewards (depending on the operating Selection Method) and collect execution fees paid in \$t3rn covered by Requesters.
 
 They compete with other agents on providing attractive services and fee rates and risk loosing a part of bonded stake for not following the protocol. This is reported by either fisherman or collator. 
 
 Accounts on extrinsic chains used by agents to facilitate the interoperable execution are called escrow accounts from the perspective of the protocol. The registry of the circuit, maintained by the collators, holds the list of all escrow accounts controlled by agents and any observed misbehaviour proven by s that compromises requesters' funds is a subject to penalties. Stake bonded by agents must always be x2 higher that the worth of transaction they facilitate.

Execution Agents registering their services must control accounts on at least two chains and submit their addresses to the Registry. Agents with an account active on every chain can compete for every single phase execution of every transaction. That predicates that each agent needs to maintain their own account which will serve as an escrow on each extrinsic chain that the transaction involves. 

Because Execution Agents attest with their stake for the validity of transactions, they incentive is to validate transaction before submitting it to the collators. 
    
#### Relayers
Obserce changes on to the target and escrow accounts on each extrinsic chain involved in the interoperable execution and relay the proofs of correct execution + inclusion to the Circuit. 

#### Fishermen
Watch for misbehaviours and report them to Collators. Collators have authority to include the transaction slashing the bond of validators accordingly. As per initial rule, for each proven misbehaviour . That also means that for validators as a composable transaction executioner, they bond needs to be at least 2x of the total value transferred and used on fees by that transaction. Requesters are therefore ensured against the balance being unrightfully spend by escrow accounts controlled by validators. Fishermen get 10\% out of the violated sum they're reporting.

#### Nominators 
Stake their \$t3rn for validators. In exchange validators share with their gains with nominators. This incentives community to be more active and adds another utility to \$t3rn token model. 
This actor is introduced only in the Development Phase II. Before, the Execution Agents compete only on the open market rules.

#### Selection Method: Open Market vs Golden Ticket

In this subsection we consider two methods for solving a problem of having multiple service providers for a single task. Each actor within the system is assumed to be plural. For achieving a consensus on selecting just one deputy we see two methods that would work with the t3rn protocol.

##### Open Market
First way is being straightforward - a competitive open market. No extra rules are needed to select a leader. Double-spending prevention mechanisms are standard for decentralised projects therefore all extrinsic blockchains are assumed to have a way of selecting only one transaction out of many valid that becomes part of the blockchain.

##### Golden Ticket
The selected for a task leader receives so called "golden ticket". Open race is therefore reduced to a single deputy performing a task in behalf of the whole collective. 
This selection method is more complex and parties must follow a common algorithm of selecting a leader facilitating a single task. The specifics of the algorithm depend on the operative economy system. 

Actors will have the following method to select a single deputy for a task:
- Requesters: Open Market
- Collators: Open Market transition to PoW-like Golden Ticket
- Execution Agents: Open Market transition to PoS-like leader selection
- Relayers: Open Market
- Fisherman: Open Market

The transition is further explained in the Development Path subsection.

### Development path
We foresee a transition of a for method selecting a leader amongst the collators and validators. The open market rule, being an easier one to implement is a great choice for initial launch of the product. Initial launch can therefore assume the Open Market rule for validation (execution) & collation services with zero additional to fees rewards.

We envision t3rn changing its underlying decentralised network model after some time after initial launch. We plan to launch the network on a semi-decentralised Proof of Authority model and transition the network to a fully decentralised model after the community is more engaged and product fine-tuned. As of now now we consider being the network to transition to either a variation of Proof of Stake or Proof of Work. Both would work as the Parachains models and both have their pros and cons. Mechanics for each of the three mentioned models, PoA, PoS & PoA, functioning as a security model for t3rn protocol are introduced in later subsections. 

\par There is several factor we see behind the transition. At of time of writing the development is in its early stage. There will be many new features that will come, some can potentially break the protocol and introduce incompatibility. It is reasonable to assume that it will attract both honest and dishonest actors shortly after launching, causing many extra patch releases to be issued. It is easier to patch the protocol when not too many decentralised parties are involved. It is also much easier to introduce the next protocol upgrades, therefore develop the product faster. 

Why not to wait with the launch for the network to be proven secured on the long testnet time? Early launch & working product attracts community. This in turn allows the network to gain the necessary traction and become more lively. Getting the community to participate in the network's governance is the must-have for decentralised projects. 

The date of transition to the next step also relies on the date and availability of the Polkadot's Parachain slot. Migrating from the Proof of Authority model too early without the Polkadot's slot could compromise the Circuit's security. 

\par To summarize, our plan is to develop the network in three phases:
- Phase I. Release the initial network running on a semi-decentralised Proof of Authority model
- Phase II.  Introduce an execution agent selection mechanism amongst the registered validators working on the Proof of Stake premises.
- Phase III. Introduce an a golden ticket selection mechanism for a single collator amongst the open market of competing collators, working on the Proof of Work premises. Proof of Work is made useful by computing the validation proofs instead of just a math puzzle. 

The transition between phases is based on the community involvement and their opinions in regards to the next protocol phases will be weighted in with on-chain governance. 

Note, that transition between Phase II and III might not even break the protocol. Validators can still submit their attestations of valid transactions directly to the relay chain. They will however most likely be rejected as the chances of obtaining the golden ticket at the random try are relatively low.
 
#### Phase I. Proof of Authority

The transition-only phase. Network is secured by federated authorities responsible for keeping the network security intact. The incentives to become a federated validator are out of intrinsic interest of maintaining the project. There is no rewards for running the federated validation services. Initially, we will run a significant portion of validators ourselves with a plan to distribute the authorities to prominent partnership projects & active community members. To reiterate, this is only a temporary phase. The parties with federated authorities will have no privileges in the next fully decentralised phase. 
 
 During that phase, parachain of t3rn is implemented as a public permissionned blockchain over the smart contract execution platform. 
 The economy for execution agents designed as a competitive open market for execution services with open and equal rules for providers to participate, regardless of their \% in the stake. Zero block reward approach eliminates shortcomings observed in the Proof of Stake systems, where "rich gets richer" and actors with a higher stake are more likely to form "cartels", which can centralize the actual control over data being submitted to blockchain. 
 
 There needs to be at least one honest actor of each category for the system to function correctly. That is enough to collect at least one correct proof per each step of Composable Transaction and deliver it to Relay Chain validators.
 
### Hybrid Approach to Consensus

Our vision of the protocol is to provide a thriving economy ecosystem that incentivizes actors to compete on maximizing the validation security, element present in Proof of Work systems, with large community involvement & voting for competitive validation & execution agents present on the Proof of Stake systems. Proof of Stake is great when it comes to engaging with community and adding the utility of governance to the security. We therefore design a fully decentralised, hybrid consensus protocol that takes what's best of Proof of Stake & Proof of Work systems. 
 
 #### Phase II. Proof of Stake for Execution Agents
 
 ##### Foreward to Proof of Stake
 In proof of stake systems, decentralised validators form a collective consensus over data selecting a leader with permission to add new data to the system. They act as a security guarantee for external actors and for their services claim rewards. New active validators can be selected by being voted in. The more stake is voted in for a validator, the higher chances it has to be selected as a leader.  
  Validators usually risk their bonded stake which can get slashed presented a proven misbehaviour. Individual stake holders are incentivised to nominate, vote their with their stake, for individual or groups of validators. The incentivisation is realised either on-chain, off-chain or both, when validation rewards are shared with nominators. 

 ##### Execution Agent Selection
 
 Validators form a collective and form a consensus selecting individual deputy that executes a transaction in behalf of the the whole group.  
 
 Out of the collective of validators, it's beneficial to select a single representative that facilitates a given transaction. That is due to:
 - avoiding race for dispatching a single transaction by many parties on extrinsic blockchains
 - performing multiple transactions simultaneously by validators by deputing multiple agents
 - collective of agents have bigger collective stake, therefore can offer better services for requesters: higher liquidity and lower fees.
 
 The algorithm weights in the the agent's stake and history of its historical performance.
  The algorithm has a collective random number generation method and the new number is drawn for each task selection. The more stake an actor has & the better performance history, the more chances it gets to be selected for each draw. 
 
 The selection of an execution agents amongst the validators can be approximated with the following steps:
 1. Collators receive an interoperable transaction request.
 2. One or many validators volunteer for execution of the transaction. All of the volunteering validators must have native accounts active on extrinsic blockchains involved in the interoperable execution (escrow accounts)
 3. Validators use the selection algorithm to collectively choose a leader for the transaction - an execution agent.
 4. Execution agent executes the transaction. 
 5. Validators attest for the transaction's correctness. If transaction is proven not to be executed correctly, the requester can claim indemnity from stake bonded by the validators, directly from the collators.
 6. Validators issue a reward for the execution agent.

 ##### Additional Roles of Validators
Validators must achieve a consensus about the selection of an execution agent for a single execution of transaction and submit the completed transaction to collators (Development Phase III) or directly to the Relay Chain (Development Phase II). Validators are now Collators as well. They risk their bonded stake for proven by fishermen misbehaviour. 
 
 
#### Phase III. Useful Proof of Work for collators
Useful Proof of Work can be a subtle extension on top of the t3rn protocol, incentivizing collators to prove their correct validation of interoperable transactions by providing the computation proof. The proof must yield with a unique identifier - that authorizes a collator with such golden ticket to dispatch the transaction with computation proof to the Relay Chain. This adds significantly to the security of overall design, where each transaction step is checked multitude of times by multiple competing collators.
 
#### Concerns surrounding Proof of Stake 

##### Validator Pools
Forming pools of validators seems to be inevitable in decentralised systems. In pure proof of stake systems however, this can be dangerous when the pool becomes too large in de facto stays in control over the system. In common Proof of Stake blockchains validators stay on the top of the "foodchain" being in direct control over the accounts and transactions ledger. In practice, that system works well for popular project with majority of an honest community, that holds dishonest actors accountable. For the smaller projects however, individual and groups with large % of capital invested hold an immense amount of power and in practice solely stay in control of the blockchain. 

As mentioned before, pools could have however a great effect for the end users if incentivised correctly. Validation pools can offer greater liquidity & quality services for interoperable transaction requesters, as collectively they manage larger sums. 

In t3rn we assume the formation of validation pool as natural evolution of decentralised projects. We add additional security layer on top of the validators to mitigate the security concerns and make no distinction between pools and individual agents, as long as they follow a protocol & convince requesters to facilitate execution via services they offer. Particular pool can be corrupt, but as long as the protocol is flexible enough to provide an alternative pool, there is no incentive for the pool to provide dishonest services. In the hybrid approach we propose, we put no boundaries over how validators form themselves within the groups, as long as they follow the common algorithm of selecting a single deputy per execution collectively (explained in Proof of Stake / Execution Agent Selection)   

Another concern we see in arranging network on pure Proof of Stake premises is the lack of assurance that the transaction was actually validated by validators. They attest with their stake for the correctness and requesters or fishermen can claim the x2 refund if they prove the misbehaviour. Validators can still guess the correctness of the transaction without actually checking it. 

To prove that the transaction was validated correctly we therefore introduce the useful proof of work over the computation proofs.

                  

#### The useful Proof of Work Algorithm

The algorithm selects a single collator amongst the competing group - a golden ticket winner. The probability of a collator to become a winner doesn't rely on the stake it has, but the amount of running resources they use for validation instead. The algorithm should take some time to generate the golden ticket. That time difference should be enough for the winner to have the permission to perform a task individually and account the reward.
 
The algorithm for the golden ticket lottery starts from receiving an interoperable transaction attested by validators to be correct. Collators look for the winning validation proofs. The proof consists of:
 - unique hash id 
 - computation log 
 - interoperable transaction
 - random nonce for which the unique hash id yields with a golden ticket
 - execution stamp

The algorithm runs as follows:
- pick a random nonce from 0 to u64.MAX
- run a t3rn-compatible vm for each validation step. The VM takes the nonce as input and produces the trace level debug logs during validation. The VM prepends the next log messages with the nonce. Correctly instrumented VMs will be provided by the team, but there is no requirements on the specific instance that must be used, as long as the produced logs are consistent.
- calculate blake2 hash of the computation log. In order to calculate the unique hash id of the validation proof concat the hash of computation log, encoded interoperable transaction, execution stamp and calculate the blake2 hash of it.
- check whether the calculated uniqe validation hash satisfies the requirements of the golden ticket lottery. The output hash needs to be smaller than a certain number encoded on 32 bytes. This number will be dynamically adjusted based on the current network difficulty, on a similar way that the algorithm implemented in Bitcoin.





