# t3rn Escrow Contract
Early draft if the t3rn escrow contract.

## Open Questions:
The actual escrow logic is pretty specific, there are however an number of decisions / specifications required regarding the inclusion proofs. For now, I implemented an example function to showcase to potential we this could be done, but it is never called.

### HeaderRegistry:
I'm assuming a contract is available, storing verified t3rn headers. It must be a trustless storage, only providing verified headers. I'm guessing BEEFY will be used in this context. What are the details of this implementation? Are we storing the headers or pass them along to every `COMMIT/REVERT` call? I am very interested how much gas the verification of a header costs.

In that context I also checked the state of zkSNARK, as a proving system with a recursive PLONK construct would be a game changer. Circom has realsed their PLONK proof construct and want to release a recursive version of that soon, which is pretty sick. Quick explaination:

#### PLONK:
Plonk allows us to use the same verifier contract for any zk circuit (programs in zk stuff are called circuits). This is a gamechanger, because in earlier constructs, like GM17, it was required to have a seperate verifier contract for each program. This was a problem, because circuits only work with fixed length inputs. You would have to deploy a seperate verifier for any combination of inputs to the circuit. In PLONK this is not required anymore, allowing circuits to be abstracted to seemingly work with dynamically sized inputs. If we look at header verifications, this would be an important property, as it allows variable number of authorities being checked

#### Recursive:
Proof can be constructed recursivly. The resulting proof can then be prooven on-chain with one transaction. This is cool for 2 main reason:

1. We can compute a large number of proofs in parallel (which is great, it takes a loooong time to generate the proofs) and then aggregate these into a single proof which can be verified on-chain. Conceptually this is simillar to the casper signature scheme being developed for eth2, we can aggregate a large number of proofs into one, enabling parallel generation.
2. We pay a fixed amount for the on-chain verification, around 200k gas.

For verifying block headers, this would be a super interesting application. We could verify N headers, for a fixed cost of around 200k gas (very rough estimation and ignoring header storage costs). In the end that would allow a speed/cost tradeof, which is a good situation to be in. Just wanted to jot this down somewhere, worth keeping an eye on for sure.

### Block inclusion-proof:
Is this done with `SimplifiedMMRVerification`? Essentially we have to prove transactions inclusion in the Patricia tri-tree and verify the root equals the one stored in the corresponding header in the `HeaderRegistry`. I doubt its the correct contract, but am adding it so we have something conceptual. I need some specifiications here.


