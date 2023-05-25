const { randomAsU8a, cryptoWaitReady } = require('@polkadot/util-crypto');
const { Keyring } = require('@polkadot/keyring');
const { hexToU8a, u8aToHex } = require('@polkadot/util');

const ethUtil = require('ethereumjs-util');
const fs = require('fs');

async function generate_random_private_keys(count) {
    await cryptoWaitReady();
    let keys = [];
    for (let i = 0; i < count; i++) {
        const seed = randomAsU8a(32);
        const keyring = new Keyring({ type: 'sr25519' });
        keyring.setSS58Format(9935);
        const keypair = keyring.addFromSeed(seed);

        const publicKeyHexSr = u8aToHex(keypair.publicKey);
        const privateKeyHexSr = u8aToHex(seed);

        const keyringEd = new Keyring({ type: 'ed25519' });
        const keypairEd = keyringEd.addFromSeed(seed);

        const privateKeyHexEd = u8aToHex(seed);
        const publicKeyHexEd = u8aToHex(keypairEd.publicKey);

        var ethPrivateKey = Buffer.from(seed);
        var ethPublicKey = ethUtil.privateToPublic(ethPrivateKey);
        var ethAddress = ethUtil.publicToAddress(ethPublicKey);

        keys.push({
            substrate: {
                accountId: keypair.address,
                privateKey: privateKeyHexSr,
                publicKey: publicKeyHexSr,
            },
            ethereum: {
                privateKey: ethPrivateKey.toString('hex'),
                publicKey: ethPublicKey.toString('hex'),
                address: ethAddress.toString('hex'),
            },
            btc: {
                privateKey: privateKeyHexEd,
                publicKey: publicKeyHexEd,
            },
        });
    }
    console.log('saving new keys to keys.json');
    fs.writeFileSync('keys.json', JSON.stringify(keys, null, 2));
    return keys;
}

module.exports = generate_random_private_keys;
