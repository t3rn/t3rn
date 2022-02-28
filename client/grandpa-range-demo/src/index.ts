import { ApiPromise, WsProvider } from '@polkadot/api';
import { SubstrateListener } from "./listeners/substrate";
import { fork } from 'child_process';
import * as path from "path"

class GrandpaRangeRelayer {
	batchSize: number;
	targetRpc: string;	
	anchorJustification: any;
	headers: any[];

    constructor() {

    }

	async initTargetListener() {
		let targetListener = fork(path.join(__dirname, 'listeners/substrate' + path.extname(__filename)));

		targetListener.send('init');

		targetListener.on('message', (msg:any) => {
			if (msg.instruction === 'results'){
				console.log(msg)
			}
		})		
	}


}


let bla = new GrandpaRangeRelayer();
bla.initTargetListener()