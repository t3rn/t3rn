import { ContractFactory } from 'ethers';
import Escrow from '../build/Escrow.json';
import { TestAccountSigningKey, Provider, Signer } from "@acala-network/bodhi";
import { WsProvider } from "@polkadot/api";
import { createTestPairs } from "@polkadot/keyring/testingPairs";

const main = async () => {
  // init connection
  const provider = new Provider({
    provider: new WsProvider("ws://127.0.0.1:9944"),
  });

  await provider.api.isReady;

  // create Alice key and wallet
  const testPairs = createTestPairs();
  let pair = testPairs.alice;
  const signingKey = new TestAccountSigningKey(provider.api.registry);
  signingKey.addKeyringPair(pair);
  const wallet = new Signer(provider, pair.address, signingKey)


  // deploy contract
  const instance = await ContractFactory.fromSolidity(Escrow).connect(wallet as any).deploy();
  console.log('Escrow address:', instance.address);

  provider.api.disconnect();
}

main()