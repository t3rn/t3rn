import { ApiPromise, WsProvider } from '@polkadot/api';
import { SubstrateListener } from "./listeners/substrate";
import { types } from '@t3rn/types';
import { fork } from 'child_process';
import * as path from "path"

class GrandpaRangeRelayer {
	batchSize: number;
	targetRpc: string;	
	circuit: ApiPromise;

		
		
	constructor() {
		
	}

	
	async initTargetListener() {
		const circuitProvider = new WsProvider("ws://127.0.0.1:9944");
		this.circuit = await ApiPromise.create({ provider: circuitProvider, types });

		console.log(this.circuit)	
	}


}


let bla = new GrandpaRangeRelayer();
bla.initTargetListener()