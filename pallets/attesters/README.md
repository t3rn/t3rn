# pallet-attesters

`pallet-attesters` is a Substrate module designed for managing attesters in a blockchain network. It provides APIs for registering attesters, nominating them, and handling slash risks.

## API

`pallet-attesters` provides the following extrinsics:

- `register_attester`: Register a new attester.
- `deregister_attester`: Deregister an attester.
- `nominate`: Nominate an attester.
- `submit_attestation`: Submit an attestation.
- `commit_batch`: Commit a batch of attestations.

## Registering an Attester

To register a new attester, call the `register_attester` function. The new attester will then be added to the attesters list.

The function takes in the following parameters:

- `origin`: The origin of the call (i.e., the account of the new attester).
- `self_nominate_amount`: The balance that the attester is willing to nominate themselves with.
- `ecdsa_key`: The ECDSA key of the attester.
- `ed25519_key`: The ED25519 key of the attester.
- `sr25519_key`: The SR25519 key of the attester.
- `custom_commission`: An optional parameter specifying the commission rate of the attester.

Here's an example of how to call `register_attester` using Polkadot.js:

```javascript
const attesterAccountId = ... // The accountId of the attester
const selfNominateAmount = ... // The amount the attester wants to nominate themselves with
const ecdsaKey = ... // The ECDSA key of the attester
const ed25519Key = ... // The ED25519 key of the attester
const sr25519Key = ... // The SR25519 key of the attester
const customCommission = ... // The custom commission rate of the attester

await api.tx.palletAttesters.registerAttester(attesterAccountId, selfNominateAmount, ecdsaKey, ed25519Key, sr25519Key, customCommission).signAndSend(sender);
```

## Deregistering an Attester
To deregister an attester, call the deregister_attester function. This will remove the attester from the attesters list.

Here's an example of how to call deregister_attester using Polkadot.js:

```javascript
await api.tx.palletAttesters.deregisterAttester().signAndSend(attesterAccountId);
``` 

## Nominating an Attester
To nominate an attester, call the nominate function. The nomination will then be added to the attester's nominations list.

The function takes in the following parameters:

- `origin`: The origin of the call (i.e., the account of the nominator).
- `attester`: The account ID of the attester to nominate.
- `amount`: The amount to nominate.

Here's an example of how to call nominate using Polkadot.js:

```javascript
const nominatorAccountId = ... // The accountId of the nominator
const attesterAccountId = ... // The accountId of the attester to nominate
const nominationAmount = ... // The amount to nominate

await api.tx.palletAttesters.nominate(nominatorAccountId, attesterAccountId, nominationAmount).signAndSend(sender);
```

## Committing a Batch of Attestations
To commit a batch of attestations, call the commit_batch function.

The function takes in the following parameters:

origin: The origin of the call (i.e., the account of the committer).
target: The target of the attestations.
target_inclusion_proof_encoded: The encoded inclusion proof of the target.
Here's an example of how to call commit_batch using Polkadot.js:

```javascript
const committerAccountId = ... // The accountId of the committer
const target = ... // The target of the attestations
const targetInclusionProofEncoded = ... // The encoded inclusion proof of the target

await api.tx.palletAttesters.commitBatch(committerAccountId, target, targetInclusionProofEncoded).signAndSend(sender);
```

## Handling Slash Risk

Attesters carry the risk of being slashed for misbehavior. This module provides two functions for handling slash: apply_partial_slash and apply_permanent_slash.

### Partial Slash
The apply_partial_slash function applies a partial slash on an attester based on a given percentage. The slash is applied to both the attester's balance and the nomination balances of their nominators. This function is useful for discouraging misbehavior without completely disincentivizing participation.

### Permanent Slash
The apply_permanent_slash function applies a permanent slash on an attester. The attester's balance and the nomination balances of their nominators are completely slashed. This function is useful for dealing with severe misbehavior.

### Risk of Slashing

Becoming an attester carries the risk of slashing if the attester misbehaves or colludes with others. Slashing reduces the attester's balance and the nomination balances of their nominators. In severe cases, attesters may be permanently slashed, completely eliminating their balance and their nominators' balances. Therefore, it is crucial for attesters to behave honestly and for nominators to carefully choose the attesters they nominate.
