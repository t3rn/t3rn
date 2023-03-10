
const { mnemonicGenerate, mnemonicValidate } = require('@polkadot/util-crypto');
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const keyring = new Keyring({ type: 'sr25519' });

// Connect to the node
const connect = async () => {
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = new ApiPromise({ provider: wsProvider });
    return api.isReady;
};

// Create new accounts based on mnemonics
const createAccount = (mnemonic: string) => {
    mnemonic = mnemonic && mnemonicValidate(mnemonic)
        ? mnemonic
        : mnemonicGenerate();
    const account = keyring.addFromMnemonic(mnemonic);
    return { account, mnemonic };
}

// Entry point
const main = async (api: any) => {
    console.log(`Our client is connected: ${api.isConnected}`);

    const mnemonic = 'cruel leader remember night skill clump question focus nurse neck battle federal';
    const { account: medium1 } = createAccount(mnemonic);

    const balance = await api.derive.balances.all(medium1.address);
    const available = balance.availableBalance.toNumber();
    const dots = available / (10 ** api.registry.chainDecimals);
    const print = dots.toFixed(4);

    console.log(`Address ${medium1.address} has ${print} DOT`);
};

connect().then(main).catch((err) => {
    console.error(err)
}).finally(() => process.exit());