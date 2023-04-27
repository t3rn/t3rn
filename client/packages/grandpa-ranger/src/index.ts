require('dotenv').config()
import { Connection } from './connection';
import { ApiPromise, Keyring } from '@polkadot/api';


class GrandpaRanger {
	circuit: Connection;
	target: Connection;
	config: any;

	constructor(config: any) {
		this.config = config;
	}

	async connectClients() {
		this.circuit = new Connection(this.config.circuit.rpc1, this.config.circuit.rpc2);
		await this.circuit.connect();
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
	await grandpaRanger.connectClients();

})()