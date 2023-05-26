const { ApiPromise, WsProvider } = require('@polkadot/api');
const { hexToU8a, u8aToHex } = require('@polkadot/util');
const { Keyring } = require('@polkadot/keyring');

// use 1.7.0 to access schnorrkelKeypairFromSeed + naclKeypairFromSeed
const { randomAsU8a, naclKeypairFromSeed, schnorrkelKeypairFromSeed, encodeAddress, cryptoWaitReady } = require('@polkadot/util-crypto');
const ethUtil = require('ethereumjs-util');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const fs = require('fs');
const secp256k1 = require('secp256k1');

const argv = yargs(hideBin(process.argv))
    .option('kill-after-5m', { type: 'boolean' })
    .option('generate-random-private-keys', { type: 'number' })// e.g. node offchain-attester --generate-random-private-keys 1 --network t0rn genereates 1 key Substrate SS58Prefix = 42 and saves it to keys.json
    .option('register', { type: 'boolean' })
    .option('commission', { type: 'number' })
    .option('nominate', { nargs: 2, type: 'array', describe: 'nominate <attesterAccountId> <amount>' })
    .option('nomination-amount', { type: 'number' , describe: 'nomination-amount <amount>'})
    .option('deregister', { type: 'boolean' })
    .option('agree-target', { type: 'string' })
    .option('network', { type: 'string' })
    .option('start', { type: 'string' }).argv;

const NODES = {
    local: 'ws://127.0.0.1:9944',
    zombie: 'ws://127.0.0.1:9940',
    t0rn: 'wss://ws.t0rn.io',
    t3rn: 'wss://ws.t3rn.io',
};

async function main() {
    if (argv['register']) {
        console.log(`Registering attester with all pre-generated accounts...`);
        let node = NODES[argv.network] || NODES.local;
        // Connect to the chosen Substrate node
        const wsProvider = new WsProvider(node);
        const api = await ApiPromise.create({ provider: wsProvider });

        console.log(`Connecting to Substrate node ${node}...`);
        await cryptoWaitReady();

        let commission = argv['commission'] ? argv['commission'] : undefined;
        let nominate_amount = argv['nomination-amount'] ? argv['nomination-amount'] : undefined;

        console.log(`commission = ${commission}`);
        console.log(`nominate_amount = ${nominate_amount}`);

        await register_with_each_attester_key(api, commission, nominate_amount);

        await api.disconnect();
    }

    if (argv['agree-target']) {
        let targetId = argv['agree-target'];
        console.log(`Agree to target = ${argv['agree-target']}...`);
        let node = NODES[argv.network] || NODES.local;
        console.log(`Connecting to Substrate node ${node}...`);
        // Connect to the chosen Substrate node
        const wsProvider = new WsProvider(node);
        const api = await ApiPromise.create({ provider: wsProvider });

        await cryptoWaitReady();

        await agree_to_target_with_each_attester_key(api, targetId);

        await api.disconnect();
    }

    if (argv['nominate']) {
        console.log(`Nominate amount to attester = ${argv.nominate}...`);
        let node = NODES[argv.network] || NODES.local;
        // Connect to the chosen Substrate node
        const wsProvider = new WsProvider(node);
        const api = await ApiPromise.create({ provider: wsProvider });

        await cryptoWaitReady();

        console.log('Nominating not implemented yet..');
        await api.disconnect();
    }

    if (argv['deregister']) {
        console.log(`To register connecting to Substrate node ${argv.network}...`);
        let node = NODES[argv.network] || NODES.local;
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

            // Set the SS58 format to 42 for t0rn, 9935 for t3rn
            let ss58Format = NODES[argv.network] === 't3rn' ? 9935 : 42;
            keyring.setSS58Format(ss58Format);
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
            const ethPrivateKeyHexEc = u8aToHex(seed);

            const ethPublicKey = secp256k1.publicKeyCreate(seed, true); // false for uncompressed
            const ethPublicKeyUncompressed65b = secp256k1.publicKeyCreate(seed, false); // false for uncompressed
            const ethPublicKeyBuffer = Buffer.from(ethPublicKey)
            const ethPublicKeyHexEc = u8aToHex(ethPublicKey);
            const ethPublicKeyHexEcUncompressed65b = u8aToHex(ethPublicKeyUncompressed65b);
            const ethAddress = ethUtil.publicToAddress(ethPublicKeyBuffer, true);
            const ethAddressHex = u8aToHex(ethAddress);
            keys.push({
                substrate: {
                    accountId: keypair.address,
                    privateKey: privateKeyHexSr,
                    publicKey: publicKeyHexSr,
                },
                ethereum: {
                    privateKey: ethPrivateKeyHexEc,
                    publicKey: ethPublicKeyHexEc,
                    publicKeyUncompressed: ethPublicKeyHexEcUncompressed65b,
                    address: ethAddressHex,
                },
                ed25519: {
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
            } else if (executionVendor == 'Ed25519') {
                console.log('Ed25519 unhandled yet');
            } else if (executionVendor == 'EVM') {
                // Generate the signature for the message hash
                const privateKey = Buffer.from(hexToU8a(key.ethereum.privateKey));

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
            console.log(`\t\tEd25519 Public Key: ${key.ed25519.publicKey}`);
            console.log(`\t\tSubstrate Public Key: ${key.substrate.publicKey}`);

            const tx = api.tx.attesters
                .registerAttester(nominateAmount, key.ethereum.publicKey, key.ed25519.publicKey, key.substrate.publicKey, commission)
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
