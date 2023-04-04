import { getAbi } from "./compile";
import { Sdk } from "@t3rn/sdk";
import { ApiPromise, Keyring } from "@polkadot/api";
import { setupSdk } from "./deploy";

const Contract = require('web3-eth-contract');


export const transact = async () => {
	const { sdk, signer } = await setupSdk()
	const abi = getAbi();

	const ballot = new Contract(abi, "0xb217046b34543c709f60ba4c4d9d33d787ed616e");
	const tx = ballot.methods.vote(0);

	const txT3rn = sdk.client.tx.evm.call(
		"0xb217046b34543c709f60ba4c4d9d33d787ed616e",
		tx.encodeABI(),
		"0",
		"500000",
		"100",
		null,
		null,
		[]
	)

	sdk.circuit.tx.signAndSendSafe(txT3rn)
	.catch((e) => console.error(e))

}

transact()