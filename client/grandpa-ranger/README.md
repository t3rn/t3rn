# GRANDPA Ranger
PoC of the GRANDPA ranger, which collects and submits headers to the circuit.

## Quick life-cycle Explaination:
- Collect a GRANDPA justification for the relaychain and submit it via `submit_finality_proof` on circuit
- Now, we can use `submit_header_range` and pass an aribrary sized header_range, using the GRANDPA justification header, as an anchor and going backwards with the range
- At the same time, we can use the relaychain header, to verify its parachains header, using `submit_parachain_header`.
- Once that is complete, we can use the new header, as an anchor header for submitting via `submit_header_range`

## To Run:
- run `yarn`
- add gateways in `config.json`
- run `yarn start`

It is important that the defined gateways are registered and operational on the circuit!!

## Current Issues:
A couple of things that need to be fixed for a smooth experience.

### Duplicate Headers:
The headerlistener sometimes receives the same block twice. Currently, the second header with the same number is ignored. This sometimes leads to the header-chains to be invalid, and thereby to rejection on the circuit (`submit_header_range`). We need to ensure that the received headers are valid (aka `headers[n].hash() === headers[n + 1].parentHash`)

### Circuit error handling:
Errors are sometimes not handled correctly, which can lead to header gaps to develop. Errors can be handled gracefully by the gateway instance never having `finalize()` called. This prevents header from being removed, and results in them being added with the next range. If the errors are not detected correctly, `finalize()` is called, even though the headers where never added. This needs to be addressed in the relayer file (or potentially the circuit)

### AuthoritySetChanges:
AuthoritySet cahnges are currently not handled. I had some issues finding the correct justification to handle the update, which is why I changed the approach with the justification in general. In realychain.ts we now store all incoming justifications. We should then be able to detect authoritySetUpdates by decoding them. I have left some comments in the place where this should be implemented.

### Submitting header_range for parachain:
Currently doesn't work, because the header we submit via `submitParachainHeader` is not found be the listener. I suspect that the RPC we use points to a different chain, then the parachain.

## Run a Demo:
To run the demo, a couple of things are required to run in parallel. Here a quick rundown:

- run circuit
- setup executor and ranger configs, by adding the chains that are supported for the demo. There are some examples.
- Ensure that all chains are registered AND set to operational. Otherwise nothing will work. On github, there is a messy repo that contains yarn commands for these things (petscheit/circuit_commands -> be concious of the difference between relay and parachain registration)
- Once registered, be quick to start the ranger. If a AuthoritySetUpdate occurs without it running, we error out, and need to reregister (or write another script that finds the GRANDPA justification we need and submits it, that also works but is not built)
- now start the executor
- thats pretty much it, you can test a tranfer with the circuit_commands repo by calling `yarn transfer`
