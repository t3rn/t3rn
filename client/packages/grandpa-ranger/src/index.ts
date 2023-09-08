import {generateRange} from "./collect";

require('dotenv').config()
import { Connection } from './connection';
import { cryptoWaitReady } from "@t3rn/sdk"
import { Prometheus } from "./prometheus";
import { logger } from "./logging";


class GrandpaRanger {
	circuit: Connection;
	target: Connection;
	config: any;
	prometheus: Prometheus;

	constructor(config: any) {
		this.config = config;
		this.prometheus = new Prometheus(this.config.targetGatewayId);
		this.prometheus.rangeInterval.inc({target: this.target}, this.config.rangeInterval);
		this.prometheus.nextSubmission.set({target: this.target}, Date.now() + this.config.rangeInterval * 1000);
	}

	async start() {
		await this.connectClients();
		await new Promise((resolve, _reject) => setTimeout(resolve, 2000)); // wait for the clients to connect
		await this.collectAndSubmit(() => {})
		this.scheduleRangeSubmission();

	}

	async connectClients() {
		await cryptoWaitReady()
		this.circuit = new Connection(this.config.circuit.rpc1, this.config.circuit.rpc2, true, this.prometheus, this.config.targetGatewayId);
		this.circuit.connect();
		this.target = new Connection(this.config.target.rpc1, this.config.target.rpc2, false, this.prometheus, this.config.targetGatewayId);
		this.target.connect();
	}

	async collectAndSubmit(resolve: any) {
		if (!this.circuit.isActive || !this.target.isActive) return resolve() // skip if either client is not active

		let batches = await generateRange(this.config, this.circuit, this.target, this.prometheus, this.config.targetGatewayId)
			.catch((e) => {
				logger.error(e);
				// potentially we want to introduce a retry logic here
				return resolve()
			})

		if(batches.length > 0) {
			if(batches.length > this.config.batches_max) {
				batches.slice(0, this.config.batches_max)
			}

			// calculate the total number of elements in the batches elements
			const totalElements = batches.reduce((acc, curr) => acc + curr.range.length, 0)

			await this.submitToCircuit(batches)
				.then((res) => {
					logger.info({"status": "Submitted", "range_size": totalElements, "circuit_block": res})
					this.prometheus.nextSubmission.set({target: this.config.targetGatewayId}, Date.now() + this.config.rangeInterval * 1000);
					this.prometheus.successesTotal.inc({target: this.config.targetGatewayId}, 1)
					const latestHeight = parseInt(batches[batches.length - 1].signed_header.number)
					this.prometheus.circuitHeight.set(latestHeight)
					return resolve()
				})
				.catch((e) => {
					logger.error(e);
					this.prometheus.nextSubmission.set({target: this.config.targetGatewayId}, Date.now() + this.config.rangeInterval * 1000);
					this.prometheus.errorsTotal.inc({target: this.config.targetGatewayId}, 1)
					return resolve() // resolve, as we don't want to stop the loop
				})
		} else {
			logger.info({"status": "skipped", "range_size": 0, "circuit_block": 0}, "Skipped")
		}
	}

	async submitToCircuit(range: any[]) {
		// limit batches per tx
		if(range.length > 10) {
			range = range.slice(0, 10);
		}

		return new Promise(async (resolve, reject) => {
			try {
				if(this.circuit.sdk && this.circuit.isActive) {
					logger.info(`Creating tx for ${this.config.targetGatewayId}`)
					let tx
					if (this.config.batching) {
						logger.debug("Batching ranges")
						// create a single tx with all the batches
						tx = this.circuit.sdk.circuit.tx.createBatch(range.map(args => {
							let submit;
							// select the correct submit function based on the targetGatewayId
							if(this.config.targetGatewayId === "roco") {
								submit = this.circuit.client.tx.rococoBridge.submitHeaders
							} else if (this.config.targetGatewayId === "kusm") {
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
					} else {
						logger.debug("Batches disabled")
						logger.debug(`Size of range: ${Math.floor(Buffer.from(JSON.stringify(range[0].range)).length/1024)}kB`)
						logger.debug(`Size of signed_header: ${Math.floor(Buffer.from(JSON.stringify(range[0].signed_header)).length/1024)}kB`)
						logger.debug(`Size of justification: ${Math.floor(Buffer.from(JSON.stringify(range[0].justification)).length/1024)}kB`)
							if(this.config.targetGatewayId === "roco") {
								tx = this.circuit.client.tx.rococoBridge.submitHeaders(
									range[0].range,
									range[0].signed_header,
									range[0].justification

								)
							} else if (this.config.targetGatewayId === "kusm") {
								tx = this.circuit.client.tx.kusamaBridge.submitHeaders(

									range[0].range,
									range[0].signed_header,
									range[0].justification

								)
							} else if (this.config.targetGatewayId === "pdot") {
								tx = this.circuit.client.tx.polkadotBridge.submitHeaders(

									range[0].range,
									range[0].signed_header,
									range[0].justification

								)
							} else {
								throw new Error(`Unknown targetGatewayId: ${this.config.targetGatewayId}`)
							}
					}

					const txSize = Math.floor(tx.encodedLength/1024)
					logger.debug(`Tx size: ${txSize}kB`)
					if (tx.encodedLength > 4000000) {
						logger.error(`Tx size is too big: ${txSize}kB`)
						throw new Error(`Tx size is too big: ${txSize}kB`)
					} else if (tx.encodedLength > 1000000) {
						logger.warn(`Tx size is big: ${txSize}kB`)
					}
					let res = await this.circuit.sdk.circuit.tx.signAndSendSafe(tx)
					logger.info({res}, "Tx sent")
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
				logger.info(`Starting new range submission loop`)
				setTimeout(
					() => {
						this.collectAndSubmit(resolve)
							.catch(() => resolve) // we should never get here with the setup above
					},
					this.config.rangeInterval * 1000
				)
			})
		}
	}
}




(async () => {
	let config: any;

	try {
		config = require(`../config/${process.env.PROFILE}.ts`).default;
		logger.info(`Using ${process.env.PROFILE}.ts profile`)
	} catch {
		config = require('../config/local.ts').default;
		logger.info('Using local profile')
	}

	const grandpaRanger = new GrandpaRanger(config);
	await grandpaRanger.start();

})()
