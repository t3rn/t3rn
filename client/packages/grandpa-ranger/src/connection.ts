import { EventEmitter } from "events"
import { Sdk, ApiPromise, WsProvider, Keyring } from '@t3rn/sdk';

export class Connection extends EventEmitter {
	client: ApiPromise;
	provider: WsProvider;
	rpc1: string;
	usingPrimaryRpc: boolean = true;
	rpc2: string;
	isCircuit: boolean;
	isActive: boolean = false;
	sdk: Sdk | undefined;
	signer: any;

	constructor(rpc1: string, rpc2: string, isCircuit: boolean, signer?: string) {
		super();
		this.rpc1 = rpc1;
		this.rpc2 = rpc2;
		this.usingPrimaryRpc = true;
		this.isCircuit = isCircuit;
		if(signer) {
			const keyring = new Keyring({ type: 'sr25519' });
			this.signer = keyring.addFromMnemonic(signer);
			console.log(`Signer address: ${this.signer.address}`)
		}

	}

	async connect() {
		while(true) {
			try {
				this.provider = this.createProvider();
				await this.setListeners(this.provider);
				break;
			} catch(e) {
				this.usingPrimaryRpc = !this.usingPrimaryRpc; // toggle connection
				console.log(`Retrying in 2 second with ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				await new Promise((resolve, reject) => setTimeout(resolve, 2000));
			}
		}
	}

	async setListeners(provider: WsProvider) {
		// this.provider = new WsProvider(this.usingPrimaryRpc ? this.rpc1 : this.rpc2)
		return new Promise((resolve, reject) => {
			this.provider.on('connected', async () => {
				this.isActive = true;
				console.log(`Connected to ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				if(this.isCircuit) {
					const sdk = new Sdk(this.provider, this.signer);
					this.sdk = sdk;
					this.client = await sdk.init();
				} else {
					this.client = await ApiPromise.create({
						provider: this.provider
					})
				}
			})

			this.provider.on('disconnected', () => {
				this.isActive = false;
				console.log(`Disconnected from ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				this.provider.disconnect()
				if(this.client) {
					this.client.disconnect()
				}
				reject()
			})

			this.provider.on('error',  () => {
				this.isActive = false;
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