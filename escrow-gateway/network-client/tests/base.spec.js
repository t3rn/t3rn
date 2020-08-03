// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

const { ApiPromise, SubmittableResult, WsProvider } = require( "@polkadot/api");
const testKeyring = require( "@polkadot/keyring/testing").default;
const { u8aToHex } = require( "@polkadot/util");
const { randomAsU8a } = require( "@polkadot/util-crypto");
// const { KeyringPair } = require( "@polkadot/keyring/types");
const { Option } = require( "@polkadot/types");
const { Address, ContractInfo, Hash } = require( "@polkadot/types/interfaces");

const jsonrpc = require('@polkadot/types/interfaces/jsonrpc').default;

const { ALICE, CREATION_FEE, WSURL } = require( "./consts");
const {
	callContract,
	instantiate,
	getContractStorage,
	putCode
} = require( "./utils");

// This is a test account that is going to be created and funded each test.
const keyring = testKeyring({ type: "sr25519" });
const alicePair = keyring.getPair(ALICE);
let testAccount;
let api;

beforeAll(() => {
	jest.setTimeout(30000);
});




beforeEach(
	async done => {
		const types = {
			"CUSTOM_TYPES": {
				"Address": "AccountId",
				"LookupSource": "AccountId"
			}
		};

		api = await ApiPromise.create({ provider: new WsProvider(WSURL), types, jsonrpc });
		// api = await ApiPromise.create();
		testAccount = keyring.addFromSeed(randomAsU8a(32));

		return api.tx.balances
			.transfer(testAccount.address, CREATION_FEE.muln(3))
			.signAndSend(alicePair, (result) => {
				if (
					result.status.isInBlock &&
					result.findRecord("system", "ExtrinsicSuccess")
				) {
					console.log("New test account has been created.");
					done();
				}
			});
	}
);

describe("Rust Smart Contracts", () => {
	test("Flip contract", async (done) => {
		// The next two lines are a not so pretty workaround until the new metadata format has been fully implemented
		const metadata = require("../lib/ink/examples/flipper/target/metadata.json");

		const selector = metadata.contract.constructors[1].selector;
		const flipAction = metadata.contract.messages[0].selector;

		const STORAGE_KEY = (new Uint8Array(32)).fill(0);

		// Deploy contract code on chain and retrieve the code hash
		const codeHash = await putCode(
			api,
			testAccount,
			"../lib/ink/examples/flipper/target/flipper.wasm"
		);
		expect(codeHash).toBeDefined();

		// Instantiate a new contract instance and retrieve the contracts address
		// The selector `0x0222FF18` is copied over = require( the generated ink! contract metadata
		const address = await instantiate(
			api,
			testAccount,
			codeHash,
			selector,
			CREATION_FEE
		);
		expect(address).toBeDefined();

		const initialValue = await getContractStorage(
			api,
			address,
			STORAGE_KEY
		);
		expect(initialValue).toBeDefined();
		expect(initialValue.toString()).toEqual("0x00");

		// The selector `0x8C97DB39` is copied over = require( the generated ink! contract metadata
		await callContract(api, testAccount, address, flipAction);

		const newValue = await getContractStorage(api, address, STORAGE_KEY);
		expect(newValue.toString()).toEqual("0x01");

		done();
	});
});
