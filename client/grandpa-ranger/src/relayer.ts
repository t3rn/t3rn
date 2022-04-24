import { ApiPromise, WsProvider } from '@polkadot/api'
import { createTestPairs } from '@polkadot/keyring/testingPairs'
import { Header } from '@polkadot/types/interfaces'
import { formatEvents } from './util'
import createDebug from 'debug'
import 'dotenv/config'
import types from './types.json'
import { EventEmitter } from 'stream'

const keyring = createTestPairs({ type: 'sr25519' })

export default class Relayer extends EventEmitter {
	static debug = createDebug('relayer')
	api: ApiPromise

	async setup(url: string) {
		this.api = await ApiPromise.create({
			provider: new WsProvider(url),
			types: types as any,
		})
		console.log("Relayer Setup complete")
	}

	submitFinalityProof(
		gatewayId: string,
		justification: any,
		anchorHeader: any,
		anchorIndex: number,
	) {

		const submitFinalityProof =
			this.api.tx.multiFinalityVerifierDefault.submitFinalityProof(
			anchorHeader,
			justification,
			gatewayId
		)

		// as this is event-driven now, we dont need the promises anymore
		submitFinalityProof.signAndSend(keyring.alice, result => {
			if (result.isError) { // this doesn't work for all error in the circuit, but for some
				console.log('FinalityProofSubmitted failed');
			} else if (result.isInBlock) {
				this.emit("FinalityProofSubmitted", {
					gatewayId,
					blockHash: anchorHeader.hash,
					anchorIndex
				})
			}
		})
	}

	// here we pass the anchorHash, instead of anchorIndex. We can use the hash to find the index of the anchor header for the parachain.
	submitParachainHeader(
		gatewayId: string,
		blockHash: any,
		proof: any,
		anchorHash: string
	) {
		console.log("Submitting Parachain Header");
		console.log("blockHash parachain:", blockHash.toHuman());
		console.log("gateway id:", gatewayId);
		let submitParachainHeader = this.api.tx.multiFinalityVerifierDefault.submitParachainHeader(
			blockHash,
			gatewayId,
			proof
		)

		submitParachainHeader.signAndSend(keyring.alice, result => {
			if (result.isError) { // this doesn't work for all error in the circuit, but for some
				console.log('ParachainHeaderSubmitted failed');
			} else if (result.isInBlock) {
				console.log("ParachainHeaderSubmitted - SUCCESS")
				this.emit("ParachainHeaderSubmitted", {
					gatewayId,
					anchorHash
				})
			}
		})
	}

	submitHeaderRange(
		gatewayId: string,
		range: any[],
		anchorHeader: any,
		anchorIndex: number,
	) {
		const submitHeaderRange =
			this.api.tx.multiFinalityVerifierDefault.submitHeaderRange(
				gatewayId,
				range,
				anchorHeader.hash
			)

		// as this is event-driven now, we dont need the promises anymore
		submitHeaderRange.signAndSend(keyring.alice, result => {
			if (result.isError) { // this doesn't work for all error in the circuit, but for some
				console.log("Header Range submissission failed")
			} else if (result.status.isFinalized) {
				console.log("SubmittedHeaderRange - SUCCESS")
				this.emit("SubmittedHeaderRange", {
					gatewayId,
					anchorIndex
				});
			}
		})
	}

//   async submit(
//     gatewayId: Buffer,
//     anchor: Header,
//     reversedRange: Header[],
//     justification: any,
//     offset: number
//   ) {
//     Relayer.debug('submitting finality proof and header range...')
//     Relayer.debug(
//       `submit_finality_proof(\n\t${anchor},\n\t${justification
//         .toString()
//         .slice(0, 10)}...,\n\t${gatewayId}\n)`
//     )

//     const submitFinalityProof =
//       this.circuit.tx.multiFinalityVerifierDefault.submitFinalityProof(
//         anchor,
//         justification,
//         gatewayId
//       )

//     await new Promise(async (resolve, reject) => {
//       await submitFinalityProof.signAndSend(keyring.alice, result => {
//         if (result.isError) {
//           console.error('submitting finality proof failed')
//         } else if (result.isInBlock) {
//           Relayer.debug(
//             'submit_finality_proof events',
//             ...formatEvents(result.events)
//           )
//           return resolve(undefined)
//         }
//       })
//     })

//     Relayer.debug(
//       `submit_header_range(\n\t${gatewayId},\n\t${reversedRange},\n\t${anchor.hash}\n)`
//     )

//     const submitHeaderRange =
//       this.circuit.tx.multiFinalityVerifierDefault.submitHeaderRange(
//         gatewayId,
//         reversedRange,
//         anchor.hash
//       )

//     await new Promise(async (resolve, reject) => {
//       await submitHeaderRange.signAndSend(keyring.alice, result => {
//         if (result.isError) {
//           console.error('submitting header range failed')
//         } else if (result.status.isFinalized) {
//           Relayer.debug(
//             'submit_header_range events',
//             ...formatEvents(result.events)
//           )
//           this.emit("RangeSubmitted", offset);
//           resolve(undefined)
//         }
//       })
//     })
//   }
}
