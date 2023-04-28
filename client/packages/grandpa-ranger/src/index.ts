import {generateRange} from "./collect";

require('dotenv').config()
import { Connection } from './connection';
import { cryptoWaitReady } from "@t3rn/sdk"

class GrandpaRanger {
	circuit: Connection;
	target: Connection;
	config: any;

	constructor(config: any) {
		this.config = config;
	}

	async start() {
		await this.connectClients();
		this.scheduleRangeSubmission();
	}

	async connectClients() {
		await cryptoWaitReady()
		this.circuit = new Connection(this.config.circuit.rpc1, this.config.circuit.rpc2, true, this.config.circuitSigner);
		this.circuit.connect();
		this.target = new Connection(this.config.target.rpc1, this.config.target.rpc2, false);
		this.target.connect();
		this.collectAndSubmit(() => {})
	}

	async collectAndSubmit(resolve: any) {
		let range = await generateRange(this.config, this.circuit, this.target)
			.then(range => range)
			.catch((e) => {
				console.log(e);
				// potentially we want to introduce a retry logic here
				return [];
			})

		if(range.length > 0) {
			console.log(`Submitting ${range.length} ranges`)
			this.submitToCircuit(range)
				.then(() => resolve())
				.catch((e) => {
					console.log(e);
					resolve() // resolve anyway
				})
		}
	}

	async submitToCircuit(range: any[]) {
		// limit to 10 batches per tx
		if(range.length > 10) {
			range = range.slice(0, 10);
		}
		new Promise((resolve, reject) => {
			if(this.circuit.sdk && this.circuit.isActive) {
				let tx = this.circuit.sdk.circuit.tx.createBatch(range.map(args => {
					let submit;
					if(this.config.targetGatewayId === "roco") {
						submit = this.circuit.client.tx.rococoBridge.submitHeaders
					} else if (this.config.targetGatewayId === "ksma") {
						submit = this.circuit.client.tx.kusamaBridge.submitHeaders
					} else {
						throw new Error(`Unknown targetGatewayId ${this.config.targetGatewayId}`)
					}
					return submit(
						args.range,
						args.signed_header,
						args.justification
					)
				}))

				this.circuit.sdk.circuit.tx.signAndSendSafe(tx)
					.then((result) => {
						console.log(`Submitted ${range.length} ranges to circuit`)
						console.log(`Tx hash: ${result}`)
					})

				resolve
			} else {
				// we should prob have some retry logic here instead
				reject(new Error(`Circuit client is not active!`))
			}
		})

	}

	async scheduleRangeSubmission() {
		while(true) {
			await new Promise((resolve, reject) => {
				setTimeout(
					() => {
						this.collectAndSubmit(resolve)
							.catch(() => reject) // we should never get here with the setup above
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