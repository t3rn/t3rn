const { ApiPromise, WsProvider } = require('@polkadot/api');
const { hexToU8a, u8aToHex } = require('@polkadot/util');
const { Keyring } = require('@polkadot/keyring');
const generate_random_private_keys = require('./generateKeys');

const { randomAsU8a, naclKeypairFromSeed, schnorrkelKeypairFromSeed, encodeAddress, cryptoWaitReady } = require('@polkadot/util-crypto');
const ethUtil = require('ethereumjs-util');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const fs = require('fs');

const argv = yargs(hideBin(process.argv))
    .option('kill-after-5m', { type: 'boolean' })
    .option('generate-random-private-keys', { type: 'number' })
    .option('register', { type: 'string' })
    .option('commission', { type: 'number' })
    .option('nomination-amount', { type: 'number' })
    .option('deregister', { type: 'string' })
    .option('start', { type: 'string' }).argv;

const NODES = {
    local: 'ws://127.0.0.1:9944',
    zombie: 'ws://127.0.0.1:9940',
    t0rn: 'wss://ws.t0rn.io',
    t3rn: 'wss://ws.t3rn.io',
};

async function main() {
    if (argv['register']) {
        console.log(`To register connecting to Substrate node ${argv.register}...`);
        let node = NODES[argv.start] || NODES.local;
        // Connect to the chosen Substrate node
        const wsProvider = new WsProvider(node);
        const api = await ApiPromise.create({ provider: wsProvider });

        await cryptoWaitReady();

        let commission = argv['commission'] ? argv['commission'] : undefined;
        let nominate_amount = argv['nomination-amount'] ? argv['nomination-amount'] : undefined;
        await register_with_each_attester_key(api, commission, nominate_amount);

        await api.disconnect();
    }

    if (argv['deregister']) {
        console.log(`To register connecting to Substrate node ${argv.start}...`);
        let node = NODES[argv.start] || NODES.local;
        // Connect to the chosen Substrate node
        const wsProvider = new WsProvider(node);
        const api = await ApiPromise.create({ provider: wsProvider });

        await cryptoWaitReady();

        await deregister_with_each_attester_key(api);

        await api.disconnect();
    }

    if (argv['start']) {
        console.log(`Connecting to Substrate node ${argv.start}...`);
        let node = NODES[argv.start] || NODES.local;

        // Connect to the chosen Substrate node
        const wsProvider = new WsProvider(node);
        const api = await ApiPromise.create({ provider: wsProvider });

        // Subscribe to the NewAttestationMessageHash event
        api.query.system.events(async (events) => {
            console.log(`\nReceived ${events.length} events:`);
            // Loop through the Vec<EventRecord>
            await Promise.all(
                events.map(async (record) => {
                    // Extract the phase, event and the event types
                    const { event } = record;

                    if (event.section == 'attesters') {
                        console.log(`\t${event.section}:${event.method}:: (phase=${record.phase})`);

                        switch (event.method) {
                            case 'NewAttestationMessageHash': {
                                const [targetId, messageHash, executionVendor] = event.data;
                                console.log(`Received the attestation message hash request to sign`);
                                console.log(`\t\tTarget ID: ${targetId.toString()}`);
                                console.log(`\t\tMessage Hash: ${messageHash.toHex()}`);
                                console.log(`\t\tExecution Vendor: ${executionVendor.toString()}`);
                                // Submit the attestation for the given target ID for the given message hash for each attester's key in the keys.json file
                                await attest_with_each_attester_key(api, targetId, messageHash.toHex(), executionVendor);

                                break;
                            }
                            case 'NewTargetProposed': {
                                console.log(`Received the new target proposed event`);
                                break;
                            }
                            default: {
                                break;
                            }
                        }
                    }
                }),
            );
        });

        // Infinite loop to keep the script running
        while (true) {
            await new Promise((resolve) => setTimeout(resolve, 1000));
        }
    }
    // Set a timeout to exit after 5 minutes if the `--kill-after-5m` option was given
    if (argv['kill-after-5m']) {
        setTimeout(() => process.exit(), 5 * 60 * 1000);
    }

    // Generate random private keys if the `--generate-random-private-keys` option was given
    if (argv['generate-random-private-keys']) {
        await cryptoWaitReady();
        let keys = [];
        let count = argv['generate-random-private-keys'];
        console.log(`Generating ${count} random private keys...`);
        // await generate_random_private_keys(count);

        for (let i = 0; i < count; i++) {
            // Substrate keys
            const seed = randomAsU8a(32);

            // Create a new keyring and add a keypair from the seed
            const keyring = new Keyring({ type: 'sr25519' });
            keyring.setSS58Format(9935);
            const keypair = keyring.addFromSeed(seed);

            // Substrate keys
            const publicKeyHexSr = u8aToHex(keypair.publicKey);
            const privateKeyHexSr = u8aToHex(seed); // use the seed as a private key

            // ED25519 keys
            const keyringEd = new Keyring({ type: 'ed25519' });
            const keypairEd = keyringEd.addFromSeed(seed);

            const privateKeyHexEd = u8aToHex(seed); // use the seed as a private key
            const publicKeyHexEd = u8aToHex(keypairEd.publicKey);

            // Ethereum keys
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

        console.log('Saving new keys to keys.json');

        fs.writeFileSync('keys.json', JSON.stringify(keys, null, 2));
    }

    // TODO: Listen to events and respond to them
}

main().catch(console.error);

async function attest_with_each_attester_key(api, targetId, messageHash, executionVendor) {
    let keys = JSON.parse(fs.readFileSync('keys.json'));
    await cryptoWaitReady();

    return await Promise.all(
        keys.map(async (key) => {
            if (executionVendor == 'Substrate') {
                console.log('Substrate unhandled yet');
            } else if (executionVendor == 'Btc') {
                console.log('Ed25519 unhandled yet');
            } else if (executionVendor == 'EVM') {
                // Generate the signature for the message hash
                const privateKey = Buffer.from(key.ethereum.privateKey, 'hex');

                const sigObj = ethUtil.ecsign(hexToU8a(messageHash), privateKey);
                const signature = ethUtil.toRpcSig(sigObj.v, sigObj.r, sigObj.s);
                // Create the Keyring pair from the private key
                const keyring = new Keyring({ type: 'sr25519' });
                const pair = keyring.addFromSeed(hexToU8a(key.substrate.privateKey));

                // Now use this pair to sign and send the transaction
                const tx = api.tx.attesters.submitAttestation(messageHash, signature, targetId).signAndSend(pair, ({ events = [], status }) => {
                    if (status.isInBlock) {
                        console.log(`Included in ${status.asInBlock}`);
                    } else {
                        console.log(`Current status: ${status}`);
                    }
                });
            }
        }),
    );
}

async function register_with_each_attester_key(api, commission, nominateAmount) {
    let keys = JSON.parse(fs.readFileSync('keys.json'));
    await cryptoWaitReady();

    return await Promise.all(
        keys.map(async (key) => {
            // Create the Keyring pair from the private key
            const keyring = new Keyring({ type: 'sr25519' });
            const pair = keyring.addFromSeed(hexToU8a(key.substrate.privateKey));
            // Now use this pair to sign and send the transaction
            console.log('Registering with each attester key');
            console.log(`\t\tNominate Amount: ${nominateAmount}`);
            console.log(`\t\tCommission: ${commission}`);
            console.log(`\t\tEthereum Public Key: ${key.ethereum.publicKey}`);
            console.log(`\t\tBTC Public Key: ${key.btc.publicKey}`);
            console.log(`\t\tSubstrate Public Key: ${key.substrate.publicKey}`);

            const tx = api.tx.attesters
                .registerAttester(nominateAmount, key.ethereum.publicKey, key.btc.publicKey, key.substrate.publicKey, commission)
                .signAndSend(pair, ({ events = [], status }) => {
                    if (status.isInBlock) {
                        console.log(`Included in ${status.asInBlock}`);
                    } else {
                        console.log(`Current status: ${status}`);
                    }
                });
        }),
    );
}

async function agree_to_target_with_each_attester_key(api, targetId) {
    let keys = JSON.parse(fs.readFileSync('keys.json'));
    await cryptoWaitReady();

    return await Promise.all(
        keys.map(async (key) => {
            // Create the Keyring pair from the private key
            const keyring = new Keyring({ type: 'sr25519' });
            const pair = keyring.addFromSeed(hexToU8a(key.substrate.privateKey));
            // Now use this pair to sign and send the transaction
            console.log('Agreeing to new target attester key');
            console.log(`\t\tEthereum Address: ${key.ethereum.address}`);

            const tx = api.tx.attesters.agreeToNewAttestationTarget(targetId, key.ethereum.address).signAndSend(pair, ({ events = [], status }) => {
                if (status.isInBlock) {
                    console.log(`Included in ${status.asInBlock}`);
                } else {
                    console.log(`Current status: ${status}`);
                }
            });
        }),
    );
}

async function deregister_with_each_attester_key(api) {
    let keys = JSON.parse(fs.readFileSync('keys.json'));
    await cryptoWaitReady();

    return await Promise.all(
        keys.map(async (key) => {
            const keyring = new Keyring({ type: 'sr25519' });
            const pair = keyring.addFromSeed(hexToU8a(key.substrate.privateKey));

            // Now use this pair to sign and send the transaction
            const tx = api.tx.attesters.deregisterAttester().signAndSend(pair, ({ events = [], status }) => {
                if (status.isInBlock) {
                    console.log(`Included in ${status.asInBlock}`);
                } else {
                    console.log(`Current status: ${status}`);
                }
            });
        }),
    );
}

module.exports.register_with_each_attester_key = register_with_each_attester_key;
module.exports.deregister_with_each_attester_key = deregister_with_each_attester_key;
module.exports.attest_with_each_attester_key = attest_with_each_attester_key;
module.exports.agree_to_target_with_each_attester_key = agree_to_target_with_each_attester_key;
