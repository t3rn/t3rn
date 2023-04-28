import {generateRange} from "./collect";

require('dotenv').config()
import { Connection } from './connection';
import { cryptoWaitReady } from "@t3rn/sdk"
import { Prometheus } from "./prometheus";

class GrandpaRanger {
	circuit: Connection;
	target: Connection;
	config: any;
	prometheus: Prometheus;

	constructor(config: any) {
		this.config = config;
		this.prometheus = new Prometheus();
		this.prometheus.rangeBreak.inc(this.config.rangeBreak);
		this.prometheus.nextSubmission.set(Date.now() + this.config.rangeBreak * 1000);
	}

	async start() {
		await this.connectClients();
		this.scheduleRangeSubmission();
	}

	async connectClients() {
		await cryptoWaitReady()
		this.circuit = new Connection(this.config.circuit.rpc1, this.config.circuit.rpc2, true, this.prometheus, this.config.circuitSigner);
		this.circuit.connect();
		this.target = new Connection(this.config.target.rpc1, this.config.target.rpc2, false, this.prometheus);
		this.target.connect();
	}

	async collectAndSubmit(resolve: any) {
		if (!this.circuit.isActive || !this.target.isActive) return resolve() // skip if either client is not active

		let batches = await generateRange(this.config, this.circuit, this.target, this.prometheus)
			.catch((e) => {
				console.log(e);
				// potentially we want to introduce a retry logic here
				// console.log("-resolving collectAndSubmit on range generation error")
				return resolve()
			})

		if(batches.length > 0) {
			// calculate the total number of elements in the batches elements
			const totalElements = batches.reduce((acc, curr) => acc + curr.range.length, 0)
			// console.log(`Submitting ${batches?.length} ranges with ${totalElements} elements`)

			this.submitToCircuit(batches)
				.then((res) => {
					console.log({"status": "Submitted", "range_size": totalElements, "circuit_block": res})
					this.prometheus.nextSubmission.set(Date.now() + this.config.rangeBreak * 1000);
					this.prometheus.successes.inc({rangeSize: totalElements, circuitBlock: res, timestamp: Date.now()})
					this.prometheus.successCount.inc(1)
					const latestHeight = parseInt(batches[batches.length - 1].signed_header.number)
					this.prometheus.circuitHeight.set(latestHeight)
					return resolve()
				})
				.catch((e) => {
					console.log(e);
					this.prometheus.nextSubmission.set(Date.now() + this.config.rangeBreak * 1000);
					this.prometheus.errors.inc({rangeSize: totalElements, timestamp: Date.now()})
					this.prometheus.errorCount.inc(1)
					return resolve() // resolve, as we don't want to stop the loop
				})
		} else {
			console.log({"status": "skipped", "range_size": 0, "circuit_block": 0})
		}
	}

	async submitToCircuit(range: any[]) {
		// limit to 10 batches per tx
		if(range.length > 10) {
			range = range.slice(0, 10);
		}
		return new Promise(async (resolve, reject) => {
			try {
				if(this.circuit.sdk && this.circuit.isActive) {
					let tx = this.circuit.sdk.circuit.tx.createBatch(range.map(args => {
						let submit;
						// select the correct submit function based on the targetGatewayId
						if(this.config.targetGatewayId === "roco") {
							submit = this.circuit.client.tx.rococoBridge.submitHeaders
						} else if (this.config.targetGatewayId === "ksma") {
							submit = this.circuit.client.tx.kusamaBridge.submitHeaders
						} else if (this.config.targetGatewayId === "pdot") {
							submit = this.circuit.client.tx.polkadotBridge.submitHeaders
						} else {
							throw new Error(`Unknown targetGatewayId: ${this.config.targetGatewayId}`)
						}
						return submit(
							args.range,
							args.signed_header,
							args.justification
						)
					}))

					let res = await this.circuit.sdk.circuit.tx.signAndSendSafe(tx)
					resolve(res)
				} else {
					// we should prob have some retry logic here instead
					throw new Error(`Circuit client is not active!`)
				}
			} catch(err) {
				reject(err)
			}
		})
	}

	async scheduleRangeSubmission() {
		while(true) {
			await new Promise((resolve, _reject) => {
				console.log(`Starting new range submission loop: ${new Date().toISOString()}`)
				setTimeout(
					() => {
						this.collectAndSubmit(resolve)
							.catch(() => resolve) // we should never get here with the setup above
					},
					this.config.rangeBreak * 1000
				)
			})
		}
	}
}




(async () => {
	let config: any;
	if(process.env.PROFILE === 'prod') {
		config = require('../config/prod.ts').default;
	} else {
		config = require('../config/local.ts').default;
	}
	const grandpaRanger = new GrandpaRanger(config);
	await grandpaRanger.start();

})()