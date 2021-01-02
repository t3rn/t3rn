const fs = require('fs');
const path = require('path');
const {ApiPromise, WsProvider} = require("@polkadot/api");
const {Bytes, u32, u64, u8} = require("@polkadot/types");

const testKeyring = require("@polkadot/keyring/testing").default;
const keyring = testKeyring({type: "sr25519"});

const jsonrpc = require('@polkadot/types/interfaces/jsonrpc').default;

const {ALICE, GAS_REQUIRED, WSURL, DAVE, PHASES} = require("./consts");

let api;

beforeAll(() => {
	// Finalisation of blocks takes ~15 sec.
	jest.setTimeout(20000);
});

// Set "tiny" as a default node. That is used to either set or unset types: { Address: "AccountId", LookupSource: "AccountId" }.
// If set improperly, it would result in the error:
// "Verification Error: Execution: Could not convert parameter `tx` between node and runtime: No such variant in enum MultiSignature"
function pickTypesBasedOnNodeType() {
	const NODE = process.env.NODE || 'tiny';
	const types = {
		EscrowExecuteResult: {result: Bytes},
		DeferredStorageWrite: {
		    dest: Bytes,
			trie_id: Bytes,
			key: Bytes,
			value: Bytes,
		},
		TransferEntry: {
			to: Bytes,
			value: u32,
			data: Bytes,
			gas_left: u64,
		},
		ExecutionStamp: {
			timestamp: u64,
			phase: u8,
			proofs: Bytes,
			call_stamps: Bytes,
			failure: Bytes,
		}
	};
	if (NODE === 'tiny') {
		types['Address'] = 'AccountId';
		types['LookupSource'] = 'AccountId';
	}
	return types;
}

const types = pickTypesBasedOnNodeType();

beforeEach(
	async function (done) {
		api = await ApiPromise.create({
			provider: new WsProvider(WSURL),
			// That's really important for the API not to crash while sending and receiving the transactions.
			// See all of the problems highlighted in https://polkadot.js.org/api/start/FAQ.html
			types, jsonrpc
		});
		return done();
	}
);

describe('Contracts Gateway', function () {
	let origin;
	let phase;
	let requester;
	let targetDest;
	let value;
	let gasLimit;
	let code;
	let inputData;

	describe('when multistep call with topped up ALICE account and sending 500_000 to DAVE', function () {
		beforeEach(function () {
			origin = keyring.getPair(ALICE);
			requester = ALICE;
			value = 500000;
			targetDest = DAVE;
		});
		describe('when attaching a .WASM code that returns = [1, 2, 3, 4] with no inputData and sufficient gas limit', function () {
			beforeEach(function () {
				gasLimit = GAS_REQUIRED;
				inputData = [];
				code = `0x${fs
					.readFileSync(path.join(__dirname, 'fixtures/return_from_start_fn.wasm'))
					.toString("hex")}`;
			});

			describe('when called with topped up ALICE account during EXECUTION phase', function () {
				beforeEach(function () {
					origin = keyring.getPair(ALICE);
					phase = PHASES.EXECUTION;
				});

				it('should be successful & return a bunch of events from runtime', async function (done) {
					const tx = api.tx.contractsGateway.gatewayContractExec(
						requester,
						targetDest,
						phase,
						code,
						value,
						gasLimit,
						inputData,
					);
					tx.signAndSend(origin, {}, ({events = [], status}) => {
						console.info('Transaction status:', status.type);
						if (status.isInBlock) {
							let relevant_event_messages = events.map(({event: {data, method, section}}) => {
								return `${section}.${method} ${data.toString()}`;
							});
							console.warn('Events:');
							console.log(relevant_event_messages);
							// Ignore messsages about treasury deposits that appear on full-node but not on tiny-node.
							relevant_event_messages = relevant_event_messages.filter(msg => !msg.includes('treasury.Deposit'));
							// Check all of the generated events for that call.
							expect(relevant_event_messages).toStrictEqual(
								['contracts.CodeStored ["0x76ff03ad482eb61687f8a158ca68ac9682d83172d4a9175a80eedc539309bea9"]',
									'system.NewAccount ["5FQ3q1Mjoq7RFGZPiEDNwf62XHjYdCD7SXnKs48CACmgvsKo"]',
									'balances.Endowed ["5FQ3q1Mjoq7RFGZPiEDNwf62XHjYdCD7SXnKs48CACmgvsKo",100000000]',
									'balances.Transfer ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5FQ3q1Mjoq7RFGZPiEDNwf62XHjYdCD7SXnKs48CACmgvsKo",100000000]',
									'contracts.ContractExecution ["5FQ3q1Mjoq7RFGZPiEDNwf62XHjYdCD7SXnKs48CACmgvsKo","0x01020304"]',
									'contracts.Instantiated ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5FQ3q1Mjoq7RFGZPiEDNwf62XHjYdCD7SXnKs48CACmgvsKo"]',
									// Use stringContaining partial match for after-execution event, as the exact encoded ExecutionStamp will vary bc of different timestamps.
									expect.stringContaining('contractsGateway.ContractsGatewayExecutionSuccess'),
									'system.ExtrinsicSuccess [{"weight":270000000,"class":"Normal","paysFee":"Yes"}]',
								]
							);
						} else if (status.isFinalized) {
							console.log('Finalized block hash', status.asFinalized.toHex());
							return done();
						} else if (status.isDropped || status.isInvalid) {
							return done('Transaction failed processing multistep_call', status);
						}
					});
				});

				it('should be successful after calling with following COMMIT phase: move funds from escrow to target dest + reveal the contract output = [1, 2, 3, 4] ', async function (done) {
					const tx = api.tx.contractsGateway.gatewayContractExec(
						requester,
						targetDest,
						PHASES.COMMIT,
						code,
						value,
						gasLimit,
						inputData,
					);
					tx.signAndSend(origin, {}, ({events = [], status}) => {
						console.info('Transaction status:', status.type);
						if (status.isInBlock) {
							let relevant_event_messages = events.map(({event: {data, method, section}}) => {
								return `${section}.${method} ${data.toString()}`;
							});
							console.warn('Events:');
							console.log(relevant_event_messages);
							// Check all of the generated events for that call.
							// Ignore messsages about treasury deposits that appear on full-node but not on tiny-node.
							relevant_event_messages = relevant_event_messages.filter(msg => !msg.includes('treasury.Deposit'));
							expect(relevant_event_messages).toEqual(
								expect.arrayContaining([
									// These 2 don't appear on full-node, they're only on tiny-node.
									// 'system.NewAccount ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"]',
									// `balances.Endowed ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",${value}]`,
									`balances.Transfer ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",${value}]`,
									// Use stringContaining partial match for after-commit event, as the exact encoded ExecutionStamp will vary bc of different timestamps.
									expect.stringContaining('contractsGateway.ContractsGatewayCommitSuccess'),
									'system.ExtrinsicSuccess [{"weight":270000000,"class":"Normal","paysFee":"Yes"}]'
								])
							);
						} else if (status.isFinalized) {
							console.log('Finalized block hash', status.asFinalized.toHex());
							return done();
						} else if (status.isDropped || status.isInvalid) {
							return done('Transaction failed processing multistep_call', status);
						}
					});
				});
			});
		});
	});
});

describe('Runtime Gateway', function () {
	let origin;
	let phase;
	let requester;
	let targetDest;
	let value;
	let gasLimit;
	let code;
	let inputData;

	describe('when multistep call with topped up ALICE account and sending 500_000 to DAVE', function () {
		beforeEach(function () {
			origin = keyring.getPair(ALICE);
			requester = ALICE;
			value = 500_000;
			targetDest = DAVE;
		});
		describe('when attaching a .WASM code that returns = [1, 2, 3, 4] with no inputData and sufficient gas limit', function () {
			beforeEach(function () {
				gasLimit = GAS_REQUIRED;
				inputData = [];
				code = `0x${fs
					.readFileSync(path.join(__dirname, 'fixtures/return_from_start_fn.wasm'))
					.toString("hex")}`;
			});

			describe('when called with topped up ALICE account during EXECUTION phase', function () {
				beforeEach(function () {
					origin = keyring.getPair(ALICE);
					phase = PHASES.EXECUTION;
				});

				it('should be successful & return a bunch of events from runtime', async function (done) {
					const tx = api.tx.runtimeGateway.multistepCall(
						requester,
						targetDest,
						phase,
						code,
						value,
						gasLimit,
						inputData,
					);
					tx.signAndSend(origin, {}, ({events = [], status}) => {
						console.info('Transaction status:', status.type);
						if (status.isInBlock) {
							let relevant_event_messages = events.map(({event: {data, method, section}}) => {
								return `${section}.${method} ${data.toString()}`;
							});
							console.warn('Events:');
							console.log(relevant_event_messages);
							// Ignore messsages about treasury deposits that appear on full-node but not on tiny-node.
							relevant_event_messages = relevant_event_messages.filter(msg => !msg.includes('treasury.Deposit'));
							// Check all of the generated events for that call.
							expect(relevant_event_messages).toStrictEqual(
								[
									'versatileWasm.VersatileVMExecution ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","0x01020304"]',
									// Use stringContaining partial match for after-execution event, as the exact encoded ExecutionStamp will vary bc of different timestamps.
									expect.stringContaining('runtimeGateway.RuntimeGatewayVersatileExecutionSuccess'),
									'system.ExtrinsicSuccess [{"weight":270000000,"class":"Normal","paysFee":"Yes"}]',
								]
							);
						} else if (status.isFinalized) {
							console.log('Finalized block hash', status.asFinalized.toHex());
							return done();
						} else if (status.isDropped || status.isInvalid) {
							return done('Transaction failed processing multistep_call', status);
						}
					});
				});

				it('should be successful after calling with following COMMIT phase: move funds from escrow to target dest + reveal the contract output = [1, 2, 3, 4] ', async function (done) {
					const tx = api.tx.runtimeGateway.multistepCall(
						requester,
						targetDest,
						PHASES.COMMIT,
						code,
						value,
						gasLimit,
						inputData,
					);
					tx.signAndSend(origin, {}, ({events = [], status}) => {
						console.info('Transaction status:', status.type);
						if (status.isInBlock) {
							let relevant_event_messages = events.map(({event: {data, method, section}}) => {
								return `${section}.${method} ${data.toString()}`;
							});
							console.warn('Events:');
							console.log(relevant_event_messages);
							// Check all of the generated events for that call.
							// Ignore messsages about treasury deposits that appear on full-node but not on tiny-node.
							relevant_event_messages = relevant_event_messages.filter(msg => !msg.includes('treasury.Deposit'));
							expect(relevant_event_messages).toEqual(
								expect.arrayContaining([
									// Next 2 don't appear on full-node, they're only on tiny-node.
									// 'system.NewAccount ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"]',
									// `balances.Endowed ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",${value}]`,
									'balances.Transfer ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",500000]',
									// Use stringContaining partial match for after-commit event, as the exact encoded ExecutionStamp will vary bc of different timestamps.
									expect.stringContaining('runtimeGateway.RuntimeGatewayVersatileCommitSuccess'),
									'system.ExtrinsicSuccess [{"weight":270000000,"class":"Normal","paysFee":"Yes"}]'

								])
							);
						} else if (status.isFinalized) {
							console.log('Finalized block hash', status.asFinalized.toHex());
							return done();
						} else if (status.isDropped || status.isInvalid) {
							return done('Transaction failed processing multistep_call', status);
						}
					});
				});
			});
		});

		describe('when attaching a .WASM code that demonstrates the demo storage with calls to runtime with input = 16', function () {
			beforeEach(function () {
				gasLimit = 718059880;
				// 16 encoded as u32 on 4 bytes
				inputData = [16, 0, 0, 0];
				code = `0x${fs
					.readFileSync(path.join(__dirname, 'fixtures/32b-account-and-u128-balance/storage_runtime_demo.wasm'))
					.toString("hex")}`;
			});
			describe('when called with topped up ALICE account during EXECUTION phase', function () {
				beforeEach(function () {
					origin = keyring.getPair(ALICE);
					phase = PHASES.EXECUTION;
				});

				it('should be successful & return a bunch of events from runtime', async function (done) {
					const tx = api.tx.runtimeGateway.multistepCall(
						requester,
						targetDest,
						phase,
						code,
						value,
						gasLimit,
						inputData,
					);
					tx.signAndSend(origin, {}, ({events = [], status}) => {
						console.info('Transaction status:', status.type);
						if (status.isInBlock) {
							let relevant_event_messages = events.map(({event: {data, method, section}}) => {
								return `${section}.${method} ${data.toString()}`;
							});
							console.warn('Events:');
							console.log(relevant_event_messages);
							// Ignore messsages about treasury deposits that appear on full-node but not on tiny-node.
							relevant_event_messages = relevant_event_messages.filter(msg => !msg.includes('treasury.Deposit'));
							// Check all of the generated events for that call.
							expect(relevant_event_messages).toStrictEqual(
								[
									// First call to runtime - store_value stores 16 (0x10)
									'versatileWasm.VersatileVMExecution ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","0x10000000"]',
									// First call to runtime - double doubles it - 32 (0x20)
									'versatileWasm.VersatileVMExecution ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","0x20000000"]',
									// First call to runtime - complex_calculations converts it to - 129 (0x81)
									'versatileWasm.VersatileVMExecution ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","0x81000000"]',
									// Use stringContaining partial match for after-execution event, as the exact encoded ExecutionStamp will vary bc of different timestamps.
									expect.stringContaining('runtimeGateway.RuntimeGatewayVersatileExecutionSuccess'),
									'system.ExtrinsicSuccess [{"weight":718059880,"class":"Normal","paysFee":"Yes"}]',
								]
							);
						} else if (status.isFinalized) {
							console.log('Finalized block hash', status.asFinalized.toHex());
							return done();
						} else if (status.isDropped || status.isInvalid) {
							return done('Transaction failed processing multistep_call', status);
						}
					});
				});
			});
		});
	});
});