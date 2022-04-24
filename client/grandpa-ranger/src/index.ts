require('dotenv').config()
import Relayer from './relayer';
import config from "../config.json";
import Relaychain from "./listeners/relaychain";
import Parachain from './listeners/parachain';

class InstanceManager {
	// handles circuit communitcation
  	relayer: Relayer;
	// stores relay/parachains ranger instances
	gateways: {
		[id: string]: any;
	} = {};

	// used for mapping parachain instances to its respective relaychain
	relayLookup: {
		[id: string]: string[];
	} = {};

	constructor() {
		this.relayer = new Relayer();
	}

	async setup() {
		await this.relayer.setup(config.circuit.rpc)
		await this.initializesRelay()
		await this.initialieParachains()
		console.log("Components Initialzed")
	}

	// initialize relaychain instances as defined in config.json
	async initializesRelay() {
		for (let i = 0; i < config.relaychains.length; i++) {
			const entry = config.relaychains[i]
		
			let instance = new Relaychain();
			await instance.setup(entry.rpc, entry.id)

			// forward SubmitFinalityProof request to relayer
			instance.on("SubmitFinalityProof", (data: any) => {
				console.log("Received SubmitFinalityProof")
				this.relayer.submitFinalityProof(
					data.gatewayId,
					data.justification,
					data.anchorHeader,
					data.anchorIndex
				)
			})

			// forward SubmitHeaderRange request to relayer
			instance.on("SubmitHeaderRange", (data: any) => {
				console.log("Received SubmitHeaderRange")
				this.relayer.submitHeaderRange(
					data.gatewayId,
					data.range,
					data.anchorHeader,
					data.anchorIndex
				)
			})

			instance.start()

			// store relaychain instance in mapping
			this.gateways[entry.id] = instance;
			this.relayLookup[entry.id] = [];
		}
	}

	// initialize parachain instances as defined in config.json
	async initialieParachains() {
		for (let i = 0; i < config.parachains.length; i++) {
			const entry = config.parachains[i]

			this.relayLookup[entry.relaychainId].push(entry.id);

			let instance = new Parachain();
			await instance.setup(entry.rpc, entry.id, entry.parachainId)

			// forward SubmitHeaderRange request to relayer
			instance.on("SubmitHeaderRange", (data: any) => {
				console.log("Received SubmitHeaderRange")
				this.relayer.submitHeaderRange(
					data.gatewayId,
					data.range,
					data.anchorHeader,
					data.anchorIndex
				)
			})

			// store instance in mapping
			this.gateways[entry.id] = instance;
		}
	}

	// routes relayer notification to respective function
	async initializeEventListeners() {
		// once a relaychains finality proof has been submitted
		this.relayer.on("FinalityProofSubmitted", (data: any) => {
			console.log("FinalityProofSubmitted")
			// Once the relaychain has called submitFinalityProof, we can add relaychain headers
			this.triggerParachainHeaderVerification(data)
			// the relaychain can submit a header range immediatly
			this.gateways[data.gatewayId].submitHeaderRange(data.anchorIndex)
		})

		// once the headerRange has been submitted, we remove the header from respective instance by using anchorIndex
		this.relayer.on("SubmittedHeaderRange", (data: any) => {
			console.log("SubmittedHeaderRange")
			this.gateways[data.gatewayId].finalize(data.anchorIndex);
		})
		
		// once a parachain has submitted a header, a headerRange can be passed
		this.relayer.on("ParachainHeaderSubmitted", (data: any) => {
			console.log("ParachainHeaderSubmitted");
			this.gateways[data.gatewayId].submitHeaderRange(data.anchorHash)
		})
  	}

	// iterates through a relaychains parachain and submittes header 
	async triggerParachainHeaderVerification(data: any) {
		// we iterate over a relaychains parachains
		let promises: Promise<any>[] = this.relayLookup[data.gatewayId].map(entry => {
			return new Promise(async (res, rej) => {
				//generate a storage read proof for the header we're looking to verify
				let [storageProof, headerHash] = await this.gateways[data.gatewayId].getStorageProof(data.blockHash, this.gateways[entry].parachainId);
				// this.gateways[entry]
				this.relayer.submitParachainHeader(
					entry,
					data.blockHash,
					storageProof.toJSON().proof,
					headerHash //this is the encoded parachain header we are proving with this transaction
				)
				res;
			})
		})

		Promise.all(promises)
			.then(() => console.log("ran promises"))
	}
}

(async () => {
  let exec = new InstanceManager();
  await exec.setup()
  exec.initializeEventListeners()
})()