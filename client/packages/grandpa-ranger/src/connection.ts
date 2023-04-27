import { ApiPromise, WsProvider } from '@polkadot/api';

export class Connection {
	client: ApiPromise;
	provider: WsProvider;
	rpc1: string;
	usingPrimaryRpc: boolean = true;
	rpc2: string;

	constructor(rpc1: string, rpc2: string) {
		this.rpc1 = rpc1;
		this.rpc2 = rpc2;
		this.usingPrimaryRpc = true;
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
				console.log(`Connected to ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				this.client = await ApiPromise.create({
					provider: this.provider
				})

			})

			this.provider.on('disconnected', () => {
				console.log(`Disconnected from ${this.usingPrimaryRpc ? this.rpc1 : this.rpc2}`)
				this.provider.disconnect()
				if(this.client) {
					this.client.disconnect()
				}
				reject()
			})

			this.provider.on('error',  () => {
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