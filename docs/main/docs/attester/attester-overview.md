---
sidebar_position: 1
---

# Attester

Attesters occupy a unique space within the t3rn protocol. They are an off-chain bonded entity that locks tokens to sign new transactions. Attesters aggregare finalised transactions from Circuit that can to be released from Escrow Contracts and create a single proof, which then can be send to the Escrow Contracts by Executors to release locked funds.

First or second generation bridges create separate networks relying on trusted parties to attest a transaction has taken place on-chain to trigger transactions on a second chain. The problem? Parties in this type of system are not bonded, which boosts the risk of malicious behavior, fraud, and illicit collusion.

As a result, many previous-generation bridges only have a small number of trusted parties, as signature verification can require immense financial resources. For example, the Ronin bridge has just nine validator nodes to recognize any deposit or withdrawal. The risks of an attack are heightened as only 5/9 validator signatures are needed.

t3rn Attesters are bonded and run the risk of seeing their bond slashed if they submit incorrect signatures. Two-thirds of all Attesters sign to testify to signature correctness, with signatures also checked on the t3rn Circuit.
