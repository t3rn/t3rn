import { getAbi } from "./compile";
import { Sdk } from "@t3rn/sdk";
import { ApiPromise, Keyring } from "@polkadot/api";
import { setupSdk } from "./deploy";

const Contract = require('web3-eth-contract');


export const transact = async () => {
	const { sdk, signer } = await setupSdk()
	const abi = getAbi();

	const snorkle = new Contract(abi, "0x55e7925b1015c13cd2c4340d5db1949e6679dadf");
	const tx = snorkle.methods.getHeight();

	const txT3rn = sdk.client.tx.evm.call(
		"0x55e7925b1015c13cd2c4340d5db1949e6679dadf",
		tx.encodeABI(),
		"0",
		"50000000",
		"100",
		null,
		null,
		[]
	)

	sdk.circuit.tx.signAndSendSafe(txT3rn)
	.catch((e) => console.error(e))

}

transact()