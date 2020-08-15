const fs = require('fs');
const path = require('path');
const { ApiPromise, WsProvider } = require( "@polkadot/api");
const { Bytes } = require( "@polkadot/types");

const testKeyring = require( "@polkadot/keyring/testing").default;
const keyring = testKeyring({ type: "sr25519" });

const jsonrpc = require('@polkadot/types/interfaces/jsonrpc').default;

const { ALICE, GAS_REQUIRED, WSURL } = require( "./consts");

let api;

beforeAll(() => {
	// Finalisation of blocks takes ~15 sec.
	jest.setTimeout(20000);
});

beforeEach(
	async function (done) {
		api = await ApiPromise.create({ provider: new WsProvider(WSURL),
			// That's really important for the API not to crash while sending and receiving the transactions.
			// See all of the problems highlighted in https://polkadot.js.org/api/start/FAQ.html
			types: {
				Address: "AccountId",
				LookupSource: "AccountId",
				EscrowExecuteResult: { result: Bytes }
			}, jsonrpc });
		return done();
	}
);

describe('Escrow Gateway', function () {
	describe('multistep_call during execution phase',function () {
		let origin;
		let phase;
		let value;
		let gasLimit;
		let code;
		let input_data;

		beforeEach(function () {
			phase = 0;
		});

		describe('when called with topped up ALICE account and sending no value with sufficient gasLimit', function () {
			beforeEach(function () {
				origin = keyring.getPair(ALICE);
				value = 0;
				gasLimit = GAS_REQUIRED;
			});
			describe('when called with valid simple .wasm code with no input_data', function () {
				beforeEach(function () {
					input_data = [];
					code = `0x${fs
						.readFileSync(path.join(__dirname, 'fixtures/return_from_start_fn.wasm'))
						.toString("hex")}`;
				});

				describe('when called with topped up ALICE account', function () {
					beforeEach(function () {
						origin = keyring.getPair(ALICE);
					});

					it('should be successful', async function (done) {
						const tx = api.tx.escrowGateway.multistepCall(
							phase,
							code,
							value,
							gasLimit,
							input_data,
						);
						// tx.sign(origin);
						tx.signAndSend(origin, { }, ({ events = [], status }) => {
							console.log('Transaction status:', status.type);

							if (status.isInBlock) {
								const relevant_event_messages = events.map(({ event: { data, method, section }}) => {
									return `${section}.${method} ${data.toString()}`;
								});
								console.log('Events:');
								console.log(relevant_event_messages);
								expect(relevant_event_messages[0]).toBe('contracts.CodeStored ["0x2b5d3f9b3485f93f43b94f744fac08906393a0b8e32d6634ea7fbcad5b91b473"]');
								expect(relevant_event_messages[1]).toBe('escrowGateway.MultistepExecutionResult [{}]');
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
});
