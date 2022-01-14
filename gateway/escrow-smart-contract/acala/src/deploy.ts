import { ContractFactory } from 'ethers';
import Escrow from '../build/Escrow.json';
import { TestAccountSigningKey, Provider, Signer } from "@acala-network/bodhi";
import { WsProvider } from "@polkadot/api";
import { createTestPairs } from "@polkadot/keyring/testingPairs";

const main = async () => {
  // init connection
  const provider = new Provider({
    provider: new WsProvider("wss://node-6870830370282213376.rz.onfinality.io/ws?apikey=0f273197-e4d5-45e2-b23e-03b015cb7000"),
  });

  await provider.api.isReady;

  // create Alice key and wallet
  const testPairs = createTestPairs();
  let pair = testPairs.alice;

  const signingKey = new TestAccountSigningKey(provider.api.registry);
  signingKey.addKeyringPair(pair);
  const wallet = new Signer(provider, pair.address, signingKey)

  console.log(wallet)
  // deploy contract
  const instance = await ContractFactory.fromSolidity(Escrow).connect(wallet as any).deploy();
  console.log('Escrow address:', instance.address);

  provider.api.disconnect();
}

main()