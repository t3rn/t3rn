import { Sdk, ApiPromise, WsProvider, Keyring } from '@t3rn/sdk';
import { Prometheus } from "./prometheus";

export class Connection {
	client: ApiPromise;
	provider: WsProvider;
	rpc1: string;
	usingPrimaryRpc: boolean = true;
	rpc2: string;
	isCircuit: boolean;
	isActive: boolean = false;
	sdk: Sdk | undefined;
	signer: any;
	prometheus: Prometheus;

	constructor(rpc1: string, rpc2: string, isCircuit: boolean, prometheus: Prometheus, signer?: string) {
		this.rpc1 = rpc1;
		this.rpc2 = rpc2;
		this.usingPrimaryRpc = true;
		this.isCircuit = isCircuit;
		this.prometheus = prometheus;
		if(signer) {
			const keyring = new Keyring({ type: 'sr25519' });
			this.signer = keyring.addFromMnemonic(signer);
		}
	}

	async connect() {
		while(true) {
			try {
				this.provider = this.createProvider();
				await this.setListeners();
				break;
			} catch(e) {
				const endpoint = this.usingPrimaryRpc ? this.rpc1 : this.rpc2
				this.isCircuit ?
					this.prometheus.circuitDisconnected.inc({endpoint, timestamp: Date.now() }) :
					this.prometheus.targetDisconnected.inc({endpoint, timestamp: Date.now()});

				this.usingPrimaryRpc = !this.usingPrimaryRpc; // toggle connection
				console.log(`Retrying in 2 second with ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				await new Promise((resolve, _reject) => setTimeout(resolve, 2000));
			}
		}
	}

	async setListeners() {
		return new Promise((resolve, reject) => {
			this.provider.on('connected', async () => {
				this.isActive = true;
				console.log(`Connected to ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				if(this.isCircuit) {
					this.prometheus.circuitActive = true;
					const sdk = new Sdk(this.provider, this.signer);
					this.sdk = sdk;
					this.client = await sdk.init();
				} else {
					this.prometheus.targetActive = true;
					this.client = await ApiPromise.create({
						provider: this.provider
					})

					// update prometheus metrics with incoming blocks
					this.client.derive.chain.subscribeNewHeads(header => {
						this.prometheus.targetHeight.set(header.number.toNumber());
					})
				}
			})

			this.provider.on('disconnected', () => {
				this.isActive = false;
				this.isCircuit ? this.prometheus.circuitActive = false : this.prometheus.targetActive = false;
				console.log(`Disconnected from ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				this.provider.disconnect()
				if(this.client) {
					this.client.disconnect()
				}
				reject()
			})

			this.provider.on('error',  () => {
				this.isActive = false;
				this.isCircuit ? this.prometheus.circuitActive = false : this.prometheus.targetActive = false;
				console.log(`Error from ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				this.provider.disconnect()
				if(this.client) {
					this.client.disconnect()
				}
				reject()
			})
		})

	}

	createProvider() {
		return new WsProvider(this.usingPrimaryRpc ? this.rpc1 : this.rpc2)
	}
}