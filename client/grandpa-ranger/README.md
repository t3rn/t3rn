# GRANDPA Ranger
PoC of the GRANDPA ranger, which collects and submits headers to the circuit. In its current form, the submission of headers is controlled by the relay-chain. If the relaychains header range is full (set in ENV file), it beginns the submission lifecycle which consists of 3 steps, namely: Submit relaychain GRANDPA justification, verify parachain headers, using verified Relaychain header (storage read proof), submitting header ranges for everything, using the verified headers as anchor headers.

## Quick life-cycle Explaination:
- Collect a GRANDPA justification for the relaychain and submit it via `submit_finality_proof` on circuit
- No we use the relaychain header, to verify its parachains header, using `submit_parachain_header`.
- Now, we can call `submit_header_range` and submit the headers collected by the listeners. (Note, that in reality a relaychains header range us passed in parallel with the parachain header verification)

## To Run:
- run `yarn`
- add gateways in `config.json`
- run `yarn start`

It is important that the defined gateways are registered and operational on the circuit!! (see petscheit/circuit_commands repo on github)

## Current Issues:
A couple of things that need to be fixed for a smooth experience.

### Duplicate Headers Parachains/Relaychains (this is the same isssue for both):
The headerlistener sometimes receives the same block twice. Currently, the second header with the same number is ignored. This sometimes leads to the header-chains to be invalid, and thereby to rejection on the circuit (`submit_header_range`). We need to ensure that the received headers are valid (aka `headers[n].hash() === headers[n + 1].parentHash`). I think the easiest approach here, is to unset a header when its received twice (set to null on array/ storage). We then need an aggregate function of some sort, that iterates through the range, and fills the gaps by querying missing headers from chain via rpc. I tried to implement this on the plane (but reverted because I made a mess because I couldn't test) and one issue I ran into is the following: We need to expect also the first header to be an duplicate. The becomes an issue, because its then difficult to figure out where the range starts (duplicates are unset, and then we need to write a ton of custom logic to figure it out). I think the easiest fix, is to store the header numbers in a seperate array, and ensure that it contains no duplicates and then create a block => header object, to store the actual headers. The array can then be used as it currently is, to easily and in a fault tolarant way remove submitted headers, while making it easy to implement. This is at least what I came up with while working, no need to follow, just wanted to share my findings. 

this is what I mean with the storage, just to make it clear:
```
blockNumbers = [1,2,3,4,5,6,7,8] //no duplicated and is spliced with finalize(). stores keys of headers
headers = {
    1: <Header>,
    2: null // was received twice,
    3: <Header>
} 
```

To construct a range we:
- Find the index of the parachains anchor header in storage (expensive query, but the rest is simple after)
- Iterate thought blockNumber[0, foundIndex] and check if the header is available
- If not, we query from RPC
- We ensure that the range is valid (aka `headers[n].hash() === headers[n + 1].parentHash`)

### Circuit error handling:
Errors are sometimes not handled correctly, which can lead to header gaps to develop. Errors can be handled gracefully by the gateway instance never having `finalize()` called. This prevents header from being removed, and results in them being added with the next range. If the errors are not detected correctly, `finalize()` is called, even though the headers where never added. This needs to be addressed in the relayer file (or potentially the circuit). I have realized that some errors are handled correctly, but not all, which points torwards a problem/inconsistent behaviour on the circuit.

### AuthoritySetChanges:
AuthoritySet cahnges are currently not handled. I had some issues finding the correct justification to handle the update, which is why I changed the approach with the justification in general. In realychain.ts we now store all incoming justifications. We should then be able to detect authoritySetUpdates by decoding them. I have left some comments in the place where this should be implemented. (this should be helpful: <https://docs.substrate.io/rustdocs/latest/src/sc_finality_grandpa/justification.rs.html#121>)

### Submitting header_range for Parachain:
In its current form, we can verify parachain headers via `submit_parachain_header` (using the storage read proof of the relaychain). Once that header is verified, the ranger tries to submit a header range, using the verified header as anchor. This step currently fails. The issue is, that `Parachain:submitHeaderRange()` does not find the anchor header, in the array of headers. I added some logs to see if the header is received, but that is not the case. On top of that, when I search for the verified header on subscan (for acala mandala) it cant be found. I believe the issue here is that the parachain with id 2000 is not actually acala, but something else. To play with this, you have to set a gateways parachain id on registration (petscheit/circuit_commands) and the RPC settings in the config. The anchor header is verified on circuit already, so im pretty certain this is the issue. 


## Run a Demo:
To run the demo, a couple of things are required to run in parallel. Here a quick rundown:

- run circuit
- setup executor and ranger configs, by adding the chains that are supported for the demo. There are some examples.
- Ensure that all chains are registered AND set to operational. Otherwise nothing will work. On github, there is a messy repo that contains yarn commands for these things (petscheit/circuit_commands -> be concious of the difference between relay and parachain registration)
- Once registered, be quick to start the ranger. If a AuthoritySetUpdate occurs without it running, we error out, and need to reregister (or write another script that finds the GRANDPA justification we need and submits it, that also works but is not built)
- now start the executor
- thats pretty much it, you can test a tranfer with the circuit_commands repo by calling `yarn transfer`
