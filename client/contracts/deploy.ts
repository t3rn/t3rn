import { Sdk } from "@t3rn/sdk"
import { ApiPromise, Keyring } from'@polkadot/api';
import { cryptoWaitReady } from '@polkadot/util-crypto';

const setup = async () =>  {
	await cryptoWaitReady();
	const keyring = new Keyring({ type: "sr25519" })
	const signer = process.env.CIRCUIT_KEY === undefined
			? keyring.addFromUri("//Alice")
			: keyring.addFromMnemonic(process.env.CIRCUIT_KEY)

	const sdk = new Sdk(process.env.CIRCUIT_WS || "ws://localhost:9944", signer)
	await sdk.init();

	return { sdk, signer }
}

(async () => {
	const { sdk, signer } = await setup()
	console.log(sdk)


})()