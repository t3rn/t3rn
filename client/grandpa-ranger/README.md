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