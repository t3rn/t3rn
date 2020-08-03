const { ApiPromise, SubmittableResult } = require( "@polkadot/api");
const { KeyringPair } = require( "@polkadot/keyring/types");
const { Option } = require( "@polkadot/types");
const { Address, ContractInfo, Hash, StorageData } = require( "@polkadot/types/interfaces");
const { u8aToHex } = require( "@polkadot/util");
const BN = require( "bn.js");
const fs = require( "fs");
const path = require( "path");
const blake = require('blakejs');

const { GAS_LIMIT, GAS_REQUIRED } = require( "./consts");

async function sendAndReturnFinalized(signer, tx) {
	return new Promise(function (resolve, reject) {
		tx.signAndSend(signer, (result) => {
			if (result.status.isInBlock) {
				// Return the result of the submittable extrinsic after the transfer is finalized
				resolve(result);
			}
			if (
				result.status.isDropped ||
				result.status.isInvalid ||
				result.status.isUsurped
			) {
				reject(result);
				console.error("ERROR: Transaction could not be finalized.");
			}
		});
	});
}

async function putCode(
	api,
	signer,
	fileName
) {
	const wasmCode = fs
		.readFileSync(path.join(__dirname, fileName))
		.toString("hex");
	const tx = api.tx.contracts.putCode(`0x${wasmCode}`);
	const result = await sendAndReturnFinalized(signer, tx);
	const record = result.findRecord("contracts", "CodeStored");

	if (!record) {
		console.error("ERROR: No code stored after executing putCode()");
	}
	// Return code hash.
	return record.event.data[0];
}

async function instantiate(
	api,
	signer,
	codeHash ,
	inputData,
	endowment,
	gasRequired = GAS_REQUIRED
) {
	const tx = api.tx.contracts.instantiate(
		endowment,
		gasRequired,
		codeHash,
		inputData
	);
	const result = await sendAndReturnFinalized(signer, tx);
	const record = result.findRecord("contracts", "Instantiated");

	if (!record) {
		console.error("ERROR: No new instantiated contract");
	}
	// Return the Address of  the instantiated contract.
	return record.event.data[1];
}

async function callContract(
	api,
	signer,
	contractAddress,
	inputData,
	gasRequired = GAS_REQUIRED,
	endowment = 0
) {
	const tx = api.tx.contracts.call(
		contractAddress,
		endowment,
		gasRequired,
		inputData
	);

	await sendAndReturnFinalized(signer, tx);
}

async function rpcContract(
	api,
	contractAddres,
	inputData,
	gasLimit= GAS_LIMIT,
) {
	const res = await api.rpc.contracts.call({
		dest: contractAddres,
		gasLimit,
		inputData
	});

	if (!res.isSuccess) {
		console.error("ERROR: rpc call did not succeed");
	}

	return res.asSuccess.data;
}

async function getContractStorage(
	api,
	contractAddress,
	storageKey
) {
	const contractInfo = await api.query.contracts.contractInfoOf(
		contractAddress
	);
	// Return the value of the contracts storage
	const childStorageKey = (contractInfo).unwrap().asAlive.trieId;
	// Add the default child_storage key prefix `:child_storage:default:` to the storage key
	const prefixedStorageKey = '0x3a6368696c645f73746f726167653a64656661756c743a' + u8aToHex(childStorageKey, -1, false);

	console.log(prefixedStorageKey)
	const storageKeyBlake2b = '0x' + blake.blake2bHex(storageKey, null, 32);

	const result = await api.rpc.childstate.getStorage(
		prefixedStorageKey, // childStorageKey || prefixed trieId of the contract
		storageKeyBlake2b // hashed storageKey
	);
	console.log(result.unwrapOrDefault());
	return result.unwrapOrDefault();
}


module.exports = {
	sendAndReturnFinalized,
	getContractStorage,
	putCode,
	callContract,
	instantiate,
};
