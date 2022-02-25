import { ApiPromise, WsProvider } from '@polkadot/api';
import { SubstrateListener } from "./listeners/substrate";

class GrandpaRangeRelayer {
    target: SubstrateListener;

    constructor() {
      	this.target = new SubstrateListener("wss://rococo-rpc.polkadot.io", 5);

    }

	init() {
		this.target.initListener()
	}


}


let bla = new GrandpaRangeRelayer();
bla.init()