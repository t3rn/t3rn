---
sidebar_position: 1
---

# Collator

Collators maintain parachains by collecting parachain transactions from users and producing state transition proofs for Relay Chain validators. In other words, collators maintain parachains by aggregating parachain transactions into parachain block candidates and producing state transition proofs for validators based on those blocks.

Collators maintain a full node for the Relay Chain and a full node for their particular parachain; meaning they retain all necessary information to be able to author new blocks and execute transactions in much the same way as miners do on current PoW blockchains. Under normal circumstances, they will collate and execute transactions to create an unsealed block and provide it, together with a proof of state transition, to one or more validators responsible for proposing a parachain block.

Unlike validators, collator nodes do not secure the network. If a parachain block is invalid, it will get rejected by validators. Therefore the assumption that having more collators is better or more secure is not correct. On the contrary, too many collators may slow down the network. The only nefarious power collators have is transaction censorship. To prevent censorship, a parachain only needs to ensure that there exist some neutral collators - but not necessarily a majority. Theoretically, the censorship problem is solved with having just one honest collator.

<p align="center">
    <img height="150" src="/img/collator.png?raw=true"/>
</p>
