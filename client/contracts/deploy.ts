import { Sdk } from "@t3rn/sdk"
import { ApiPromise, Keyring } from'@polkadot/api';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { compile} from "./compile";
const { hexToU8a, u8aToHex } = require('@polkadot/util');
const Web3 = require('web3');
const web3 = new Web3();
export const setupSdk = async () =>  {
	await cryptoWaitReady();
	const keyring = new Keyring({ type: "sr25519" })
	const signer = keyring.addFromUri("//Bob");

	const sdk = new Sdk(process.env.WS_CIRCUIT_ENDPOINT || "ws://localhost:9944", signer)
	await sdk.init();

	return { sdk, signer }
}

export const deploy = async () => {
	const { sdk, signer } = await setupSdk()
	const bytecode = compile()
	let params = web3.eth.abi.encodeParameter('bytes32[]', ['0x11111', '0x22222']).substring(2);

	const tx = sdk.client.tx.evm.create(
		`0x${bytecode}${params}`,
		web3.utils.toWei("0"),
		"1200000",
		"100",
		null,
		null,
		[]
	)

	sdk.circuit.tx.signAndSendSafe(tx)
		.catch((e) => console.error(e))
}

deploy()
