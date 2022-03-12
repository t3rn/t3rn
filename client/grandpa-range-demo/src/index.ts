import { ApiPromise, WsProvider } from '@polkadot/api';
import Keyring from '@polkadot/keyring';
import { createTestPairs } from "@polkadot/keyring/testingPairs";
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

		await this.testSubmitSideEffect()
	}

	async testSubmitSideEffect() {
		const keyring = createTestPairs({ type: 'sr25519' });

		let submit_side_effects_request = await this.circuit.tx.circuit
			.onExtrinsicTrigger(
				[{
					target: [0,0,0,0],
					prize: 0,
					ordered_at: 0,
					encoded_action: [176, 203, 123, 178],
					encoded_args: [keyring.alice.address, keyring.charlie.address, [1]],
					signature: [],
					enforce_executioner: false,
				}],
				1,
				true
			)
			.signAndSend(keyring.alice);

		console.log(submit_side_effects_request)
	}


}


let bla = new GrandpaRangeRelayer();
bla.initTargetListener()
