import { TestAccountSigningKey, Provider, Signer } from '@acala-network/bodhi';
import { WsProvider } from '@polkadot/api';
import { ApiOptions } from '@polkadot/api/types';
import { createTestPairs } from '@polkadot/keyring/testingPairs';

const DEFAULT_URL = 'ws://127.0.0.1:9944';

export const getTestProvider = async (urlOverwrite?: string, opts?: ApiOptions): Promise<Provider> => {
  const url = urlOverwrite || process.env.ENDPOINT_URL || DEFAULT_URL;

  const provider = new Provider({
    provider: new WsProvider("ws://127.0.0.1:9944"),
  });

  await provider.api.isReady;
  return provider;
};

export const getTestSigners = (provider: Provider): Signer[] => {
    const testPairs = createTestPairs();
    const signingKey = new TestAccountSigningKey(provider.api.registry);
    signingKey.addKeyringPair(testPairs.alice);
    const executor = new Signer(provider, testPairs.alice.address, signingKey)

    const signingKey1 = new TestAccountSigningKey(provider.api.registry);
    signingKey1.addKeyringPair(testPairs.alice);
    const receiver = new Signer(provider, testPairs.bob.address, signingKey)
    return [executor, receiver]
};