const { expect } = require("chai");
const { ethers } = require("hardhat");
const ethUtil = require('ethereumjs-util');

const encodedEmptyBatch = '0x00000000'

// Standardized prefix for Ethereum signed messages that has message of 32 bytes
const prefix = '\x19Ethereum Signed Message:\n32'
const prefixBuffer = Buffer.from(prefix)

async function generateSignatures(wallets, messageHash, attestationsVerifier) {
    let signatures = []
    for (let i = 0; i < wallets.length; i++) {
        const wallet = wallets[i];
        const signerAddress = wallet.address;
        const flatSig = await wallet.signMessage(ethers.utils.arrayify(messageHash));
        const signatureBytes = ethers.utils.arrayify(flatSig);

        // Recover the signer's address
        const recovered = await attestationsVerifier.recoverSigner(messageHash, signatureBytes);
        expect(recovered).to.equal(signerAddress);

        signatures.push(signatureBytes);
    }
    return signatures
}

function getMessageHash(batch) {
    const encodedBatch = batchEncodePacked(batch)
    const messageHash = ethers.utils.keccak256(encodedBatch);
    return messageHash;
}

async function parseAllEvents(receipt, contract) {
    const iface = new ethers.utils.Interface(contract.interface.format());

    const logsByTopic = {};

    for (let log of receipt.logs) {
        const parsedLog = iface.parseLog(log);

        // Ignoring event if it's not from the target contract
        if (parsedLog == null) continue;

        if (logsByTopic[parsedLog.name] == null) {
            logsByTopic[parsedLog.name] = [parsedLog.valueOf().args];
        } else {
            logsByTopic[parsedLog.name].push(parsedLog.valueOf().args);
        }
    }

    return logsByTopic
}

function batchEncodePacked(batch) {
    return ethers.utils.solidityPack(
      ['address[]', 'address[]', 'bytes32[]', 'bytes32[]', 'uint32'],
      [batch.nextCommittee, batch.bannedCommittee, batch.committedSfx, batch.revertedSfx, batch.index]
    );
}

function batchDecodePacked(packedBatch) {
    return ethers.utils.defaultAbiCoder.decode(
      ['uint32', 'uint32', 'uint32', 'uint32', 'address[]', 'address[]', 'bytes32[]', 'bytes32[]'],

      packedBatch
    );
}

describe("AttestationSignature", function() {
    it('Should recover the correct signer from the signature with ethers.signMessage', async () => {
        let messageHash32b = "0x58cd0ea9f78f115b381b29bc7edaab46f214968c05ff24b6b14474e4e47cfcdd";
        // Private Key of the signer
        const privateKey = "0x0123456789012345678901234567890123456789012345678901234567890123";
        const wallet = new ethers.Wallet(privateKey);
        expect(wallet.address).to.equal("0x14791697260E4c9A71f18484C9f997B308e59325")
        const flatSig = await wallet.signMessage(ethers.utils.arrayify(messageHash32b));
        const signatureBytes = ethers.utils.arrayify(flatSig);

        expect(signatureBytes.length).to.equal(65);
        expect(flatSig).to.equal("0x534dd0cbadf9a92af5d32533231af6769b3a1e479e5dde49ea4e431028a66e0a2611b13e55034973e1c5f4edcab425af4f164c5a50025204db06f439ad5e977c1c");
        // Recover the signer's address
        const recovered = ethers.utils.recoverAddress(ethers.utils.hashMessage(ethers.utils.arrayify(messageHash32b)), flatSig);

        expect(recovered).to.equal(wallet.address);
    });

    it('Should recover the correct signer from the signature created with signMessage', async () => {
        let messageHash32b = "0x58cd0ea9f78f115b381b29bc7edaab46f214968c05ff24b6b14474e4e47cfcdd";
        const messageWithPrefix = ethers.utils.arrayify(ethers.utils.keccak256(Buffer.concat([ethers.utils.arrayify(prefixBuffer), ethers.utils.arrayify(messageHash32b)])))
        const privateKey = "0x115db6b0c74bef87e28879199e3ab3dda09ed0e7f0c3e1ff6cb92e228b221384";
        const privateKeyBuffer = Buffer.from(ethers.utils.arrayify(privateKey))
        const wallet = new ethers.Wallet(privateKey);
        expect(wallet.address).to.equal("0x3a68c6b6f010017C9b330a7C86D4B19c46ab677a")

        const flatSig = await wallet.signMessage(ethers.utils.arrayify(messageHash32b));
        expect(flatSig).to.equal("0x3c20151678cbbf6c3547c5f911c613e630b0e1be11b24b6b815582db0e47801175421540c660de2a93b46e48f9ff503e5858279ba157fa9b13fbee0a8cf6806e1c");

        const sigObj = ethUtil.ecsign(
          messageWithPrefix,
          privateKeyBuffer
        )

        const signature = ethUtil.toRpcSig(sigObj.v, sigObj.r, sigObj.s)
        expect(signature).to.equal("0x3c20151678cbbf6c3547c5f911c613e630b0e1be11b24b6b815582db0e47801175421540c660de2a93b46e48f9ff503e5858279ba157fa9b13fbee0a8cf6806e1c");

        // Recover the signer's address with ethers
        const recovered = ethers.utils.recoverAddress(messageWithPrefix, signature);
        expect(recovered).to.equal(wallet.address);

        // Recover the signer's address with solidity contract
        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy([], 0);
        await attestationsVerifier.deployed();

        // Contract adds prefix, so we call it without it
        const recoveredByContract = await attestationsVerifier.recoverSigner(ethers.utils.arrayify(messageHash32b), ethers.utils.arrayify(signature));
        expect(recoveredByContract).to.equal(wallet.address);
    });
});

describe("AttestationsVerifier", function() {
    it('Should recover the correct signer from the signature', async () => {
        // Replace these values with the ones used in your test case
        const privateKey = "0x0123456789012345678901234567890123456789012345678901234567890123";
        const message = "Hello, world!";
        const messageHash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(message));

        const wallet = new ethers.Wallet(privateKey);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy([], 0);
        await attestationsVerifier.deployed();

        // Create the signature
        const flatSig = await wallet.signMessage(ethers.utils.arrayify(messageHash));
        const signatureBytes = ethers.utils.arrayify(flatSig);

        // Recover the signer's address
        const recovered = await attestationsVerifier.recoverSigner(messageHash, signatureBytes);
        const expected = ethers.utils.computeAddress(privateKey);

        expect(recovered).to.equal(expected);
    });

    it('Should recover the correct signer from the signature of initialized committee', async () => {
        const message = "Hello, world!";
        const messageHash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(message));

        const wallets = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());
        const [defaultSigner] = await ethers.getSigners();

        for (let i = 0; i < wallets.length; i++) {
            const tx = await defaultSigner.sendTransaction({
                to: wallets[i].address,
                value: ethers.utils.parseEther("1")
            });
            await tx.wait();
        }

        // Create an array of addresses
        const initialCommittee = wallets.map(wallet => wallet.address);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(initialCommittee, 0);
        await attestationsVerifier.deployed();

        for (let i = 0; i < wallets.length; i++) {
            const wallet = wallets[i];
            const signerAddress = wallet.address;
            const flatSig = await wallet.signMessage(ethers.utils.arrayify(messageHash));
            const signatureBytes = ethers.utils.arrayify(flatSig);
            // Recover the signer's address
            const recovered = await attestationsVerifier.recoverSigner(messageHash, signatureBytes);
            expect(recovered).to.equal(signerAddress);
        }
    });


    it("Should produces the correct message hash for empty batch", async function() {
        let batch = {
            nextCommittee: [],
            bannedCommittee: [],
            committedSfx: [],
            revertedSfx: [],
            index: 0
        };

        let messageBytes = batchEncodePacked(batch);

        expect(messageBytes).to.equal(encodedEmptyBatch);
    });

    it("Should produces the correct message hash for filled batch", async function() {
        const nextCommittee = [
            '0x2b7A372d58541c3053793f022Cf28ef971F94EFA',
            '0x60eA580734420A9C23E51C7FdF455b5e0237E07C',
            '0x98DF91EF04A5C0695f8050B7Da4facC0E7d9444e',
            '0x3Cfbc429d7435fD5707390362c210bD272baE8eA',
            '0x66ed579D14Cbad8dFC352a3cEaeeE9711ea65e41',
            '0x786402fa462909785A55Ced48aa5682D99902C57',
            '0x401b7Cb06493eFDB82818F14f9Cd345C01463a81',
            '0xA2E7607A23B5A744A10a096c936AB033866D3bEe',
            '0xac9c643B32916EA52e0fA0c3a3bBdbE120E5CA9e',
            '0xD53d6Af58A2bD8c0f86b25B1309c91f61700144F',
            '0x2feF1f5268d9732CAc331785987d45Fad487fcd6',
            '0xdebc7A55486DbaCB06985ba2415b784e05a35baE',
            '0xd7b33a07Ee05B604138f94335405b55e2b6bbFdD',
            '0x1831c8F78C8b59c1300B79E308BfBf9e4fDd13B0',
            '0x361134E27Af99A288714E428C290d48F82a4895C',
            '0x5897B47E1357eD81B2D85d8f287759502E33f588',
            '0xa880bf7e031ed87d422D31BEBcC9D0339c7b95b4',
            '0xedaB03983D839E6A3a887c3Ee711a724391F8eE1',
            '0x80D80649e13268382ceA3b0a56a57078c2076fE1',
            '0xb0DE4907432a9A4aC92F4988dAa6024CD57D1b27',
            '0x5449D051328dA4cfE8d1eFe7481Ff3B690cF8696',
            '0x4705522d19458a90F06a15d9836A64e45c182c9f',
            '0xB6dE743a22A7A43Edda8b5E21E2f0Aeb70354f5B',
            '0x970c0720316BC03Cd055C5Ec74208Fe0BA3d3c44',
            '0x7905754a5B6A28D1EDf338d9Be06a49aD60D74b6',
            '0x93054A6f5eb0E1978D1e3e27AE758F17480E5988',
            '0xa185b4f947A09286FC028B034f01bAbe53d98301',
            '0x14C74Ce14e833d76dC0190651C0EbA64f3E67c79',
            '0x861fa47e5229C9079d087D6354C1Ede95D233F43',
            '0x6f9925AceFfbe67742257abFf393B123010c4A10',
            '0xA1Ea906c54379032c9857139C6f796Acf88dDb79',
            '0x6219f12779268F8A7ddf0f1E44Fd75253219d639'
        ];

        const committedSfx = [ethers.utils.id("sfx#1"), ethers.utils.id("sfx#2"), ethers.utils.id("sfx#3")];
        const revertedSfx = [ethers.utils.id("sfx#4"), ethers.utils.id("sfx#5")];

        // Constructing the Batch struct
        let batch = {
            nextCommittee,
            bannedCommittee: [
                nextCommittee[0],
                nextCommittee[1],
                nextCommittee[2],
            ],
            committedSfx,
            revertedSfx,
            index: 1
        };

        // Encoding the Batch struct
        const encodedBatchMessage = batchEncodePacked(batch);

        // Hashing the encoded Batch struct
        const batchMessageHash = ethers.utils.keccak256(encodedBatchMessage);

        let expectedMessage = "0x0000000000000000000000002b7a372d58541c3053793f022cf28ef971f94efa00000000000000000000000060ea580734420a9c23e51c7fdf455b5e0237e07c00000000000000000000000098df91ef04a5c0695f8050b7da4facc0e7d9444e0000000000000000000000003cfbc429d7435fd5707390362c210bd272bae8ea00000000000000000000000066ed579d14cbad8dfc352a3ceaeee9711ea65e41000000000000000000000000786402fa462909785a55ced48aa5682d99902c57000000000000000000000000401b7cb06493efdb82818f14f9cd345c01463a81000000000000000000000000a2e7607a23b5a744a10a096c936ab033866d3bee000000000000000000000000ac9c643b32916ea52e0fa0c3a3bbdbe120e5ca9e000000000000000000000000d53d6af58a2bd8c0f86b25b1309c91f61700144f0000000000000000000000002fef1f5268d9732cac331785987d45fad487fcd6000000000000000000000000debc7a55486dbacb06985ba2415b784e05a35bae000000000000000000000000d7b33a07ee05b604138f94335405b55e2b6bbfdd0000000000000000000000001831c8f78c8b59c1300b79e308bfbf9e4fdd13b0000000000000000000000000361134e27af99a288714e428c290d48f82a4895c0000000000000000000000005897b47e1357ed81b2d85d8f287759502e33f588000000000000000000000000a880bf7e031ed87d422d31bebcc9d0339c7b95b4000000000000000000000000edab03983d839e6a3a887c3ee711a724391f8ee100000000000000000000000080d80649e13268382cea3b0a56a57078c2076fe1000000000000000000000000b0de4907432a9a4ac92f4988daa6024cd57d1b270000000000000000000000005449d051328da4cfe8d1efe7481ff3b690cf86960000000000000000000000004705522d19458a90f06a15d9836a64e45c182c9f000000000000000000000000b6de743a22a7a43edda8b5e21e2f0aeb70354f5b000000000000000000000000970c0720316bc03cd055c5ec74208fe0ba3d3c440000000000000000000000007905754a5b6a28d1edf338d9be06a49ad60d74b600000000000000000000000093054a6f5eb0e1978d1e3e27ae758f17480e5988000000000000000000000000a185b4f947a09286fc028b034f01babe53d9830100000000000000000000000014c74ce14e833d76dc0190651c0eba64f3e67c79000000000000000000000000861fa47e5229c9079d087d6354c1ede95d233f430000000000000000000000006f9925aceffbe67742257abff393b123010c4a10000000000000000000000000a1ea906c54379032c9857139c6f796acf88ddb790000000000000000000000006219f12779268f8a7ddf0f1e44fd75253219d6390000000000000000000000002b7a372d58541c3053793f022cf28ef971f94efa00000000000000000000000060ea580734420a9c23e51c7fdf455b5e0237e07c00000000000000000000000098df91ef04a5c0695f8050b7da4facc0e7d9444e6e906f8388de8faea67a770476ade4b76654545002126aa3ea17890fd8acdd7e580032f247eebb5c75889ab42c43dd88a1071c3950f9bbab1f901c47d5331dfae23ab05c5ca561870b6f55d3fcb94ead2b14d8ce49ccf159b8e3449cbd5050c6ff17743a6b48933b94f38f423b15b2fc9ebcd34aab19bd81c2a69d3d052f467f21e5cd2c2f3e32ac4a52543a386821b079711432c2fefd4be3836ed36d129b1100000001";

        expect(encodedBatchMessage).to.equal(expectedMessage);
        expect(batchMessageHash).to.equal("0x92689b8b6360ba49e99b694643ba4c7fedb658496665252ab6de5aed79520a8c");

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy([], 0);
        await attestationsVerifier.deployed();

        let txEncodedBatchOutput = await attestationsVerifier.batchEncodePacked(batch);
        expect(txEncodedBatchOutput).to.equal(encodedBatchMessage);
    });


    it("test_index_only_message_produces_expected_hash", async function() {
        // Constructing the Batch struct
        let batch = {
            nextCommittee: [],
            bannedCommittee: [],
            committedSfx: [],
            revertedSfx: [],
            index: 1
        };

        // Encoding the Batch struct
        const encodedBatchMessage = batchEncodePacked(batch);

        // Hashing the encoded Batch struct
        const batchMessageHash = ethers.utils.keccak256(encodedBatchMessage);

        let expectedMessage = "0x00000001";

        expect(encodedBatchMessage).to.equal(expectedMessage);
        expect(batchMessageHash).to.equal("0x51f81bcdfc324a0dff2b5bec9d92e21cbebc4d5e29d3a3d30de3e03fbeab8d7f");

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy([], 0);
        await attestationsVerifier.deployed();

        let txEncodedBatchOutput = await attestationsVerifier.batchEncodePacked(batch);
        expect(txEncodedBatchOutput).to.equal(encodedBatchMessage);
    });

    it("test_index_and_next_committee_only_message_produces_expected_hash", async function() {
        const nextCommittee = [
            '0x2b7A372d58541c3053793f022Cf28ef971F94EFA',
            '0x60eA580734420A9C23E51C7FdF455b5e0237E07C',
            '0x98DF91EF04A5C0695f8050B7Da4facC0E7d9444e',
            '0x3Cfbc429d7435fD5707390362c210bD272baE8eA',
            '0x66ed579D14Cbad8dFC352a3cEaeeE9711ea65e41',
            '0x786402fa462909785A55Ced48aa5682D99902C57',
            '0x401b7Cb06493eFDB82818F14f9Cd345C01463a81',
            '0xA2E7607A23B5A744A10a096c936AB033866D3bEe',
            '0xac9c643B32916EA52e0fA0c3a3bBdbE120E5CA9e',
            '0xD53d6Af58A2bD8c0f86b25B1309c91f61700144F',
            '0x2feF1f5268d9732CAc331785987d45Fad487fcd6',
            '0xdebc7A55486DbaCB06985ba2415b784e05a35baE',
            '0xd7b33a07Ee05B604138f94335405b55e2b6bbFdD',
            '0x1831c8F78C8b59c1300B79E308BfBf9e4fDd13B0',
            '0x361134E27Af99A288714E428C290d48F82a4895C',
            '0x5897B47E1357eD81B2D85d8f287759502E33f588',
            '0xa880bf7e031ed87d422D31BEBcC9D0339c7b95b4',
            '0xedaB03983D839E6A3a887c3Ee711a724391F8eE1',
            '0x80D80649e13268382ceA3b0a56a57078c2076fE1',
            '0xb0DE4907432a9A4aC92F4988dAa6024CD57D1b27',
            '0x5449D051328dA4cfE8d1eFe7481Ff3B690cF8696',
            '0x4705522d19458a90F06a15d9836A64e45c182c9f',
            '0xB6dE743a22A7A43Edda8b5E21E2f0Aeb70354f5B',
            '0x970c0720316BC03Cd055C5Ec74208Fe0BA3d3c44',
            '0x7905754a5B6A28D1EDf338d9Be06a49aD60D74b6',
            '0x93054A6f5eb0E1978D1e3e27AE758F17480E5988',
            '0xa185b4f947A09286FC028B034f01bAbe53d98301',
            '0x14C74Ce14e833d76dC0190651C0EbA64f3E67c79',
            '0x861fa47e5229C9079d087D6354C1Ede95D233F43',
            '0x6f9925AceFfbe67742257abFf393B123010c4A10',
            '0xA1Ea906c54379032c9857139C6f796Acf88dDb79',
            '0x6219f12779268F8A7ddf0f1E44Fd75253219d639'
        ];
        // Constructing the Batch struct
        let batch = {
            nextCommittee,
            bannedCommittee: [],
            committedSfx: [],
            revertedSfx: [],
            index: 1
        };

        // Encoding the Batch struct
        const encodedBatchMessage = batchEncodePacked(batch);

        // Hashing the encoded Batch struct
        const batchMessageHash = ethers.utils.keccak256(encodedBatchMessage);

        let expectedMessage = "0x0000000000000000000000002b7a372d58541c3053793f022cf28ef971f94efa00000000000000000000000060ea580734420a9c23e51c7fdf455b5e0237e07c00000000000000000000000098df91ef04a5c0695f8050b7da4facc0e7d9444e0000000000000000000000003cfbc429d7435fd5707390362c210bd272bae8ea00000000000000000000000066ed579d14cbad8dfc352a3ceaeee9711ea65e41000000000000000000000000786402fa462909785a55ced48aa5682d99902c57000000000000000000000000401b7cb06493efdb82818f14f9cd345c01463a81000000000000000000000000a2e7607a23b5a744a10a096c936ab033866d3bee000000000000000000000000ac9c643b32916ea52e0fa0c3a3bbdbe120e5ca9e000000000000000000000000d53d6af58a2bd8c0f86b25b1309c91f61700144f0000000000000000000000002fef1f5268d9732cac331785987d45fad487fcd6000000000000000000000000debc7a55486dbacb06985ba2415b784e05a35bae000000000000000000000000d7b33a07ee05b604138f94335405b55e2b6bbfdd0000000000000000000000001831c8f78c8b59c1300b79e308bfbf9e4fdd13b0000000000000000000000000361134e27af99a288714e428c290d48f82a4895c0000000000000000000000005897b47e1357ed81b2d85d8f287759502e33f588000000000000000000000000a880bf7e031ed87d422d31bebcc9d0339c7b95b4000000000000000000000000edab03983d839e6a3a887c3ee711a724391f8ee100000000000000000000000080d80649e13268382cea3b0a56a57078c2076fe1000000000000000000000000b0de4907432a9a4ac92f4988daa6024cd57d1b270000000000000000000000005449d051328da4cfe8d1efe7481ff3b690cf86960000000000000000000000004705522d19458a90f06a15d9836a64e45c182c9f000000000000000000000000b6de743a22a7a43edda8b5e21e2f0aeb70354f5b000000000000000000000000970c0720316bc03cd055c5ec74208fe0ba3d3c440000000000000000000000007905754a5b6a28d1edf338d9be06a49ad60d74b600000000000000000000000093054a6f5eb0e1978d1e3e27ae758f17480e5988000000000000000000000000a185b4f947a09286fc028b034f01babe53d9830100000000000000000000000014c74ce14e833d76dc0190651c0eba64f3e67c79000000000000000000000000861fa47e5229c9079d087d6354c1ede95d233f430000000000000000000000006f9925aceffbe67742257abff393b123010c4a10000000000000000000000000a1ea906c54379032c9857139c6f796acf88ddb790000000000000000000000006219f12779268f8a7ddf0f1e44fd75253219d63900000001";

        expect(encodedBatchMessage).to.equal(expectedMessage);
        expect(batchMessageHash).to.equal("0x571e2e5fc34e6563ebadfc86189cc1b665cefe590fd8015e7d5f3759aaf39ff5");

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy([], 0);
        await attestationsVerifier.deployed();

        let txEncodedBatchOutput = await attestationsVerifier.batchEncodePacked(batch);
        expect(txEncodedBatchOutput).to.equal(encodedBatchMessage);
    });


    it("Should initialize committee and verify signatures for empty batch", async function() {
        const wallets = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());
        const [defaultSigner] = await ethers.getSigners();

        for (let i = 0; i < wallets.length; i++) {
            const tx = await defaultSigner.sendTransaction({
                to: wallets[i].address,
                value: ethers.utils.parseEther("1")
            });
            await tx.wait();
        }

        // Create an array of addresses
        const initialCommittee = wallets.map(wallet => wallet.address);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(initialCommittee, 0);
        await attestationsVerifier.deployed();

        // Constructing the Batch struct with empty arrays
        let batch = {
            nextCommittee: [],
            bannedCommittee: [],
            committedSfx: [],
            revertedSfx: [],
            index: 1
        };

        // Encoding the Batch struct
        const encodedBatchMessage = batchEncodePacked(batch);
        // Hashing the encoded Batch struct
        const batchMessageHash = ethers.utils.keccak256(encodedBatchMessage);

        const signatures = [];

        // Pre-check for the validity of signature before sending the batch message
        for (let i = 0; i < wallets.length; i++) {
            const wallet = wallets[i];
            const signerAddress = wallet.address;
            const flatSig = await wallet.signMessage(ethers.utils.arrayify(batchMessageHash));
            const signatureBytes = ethers.utils.arrayify(flatSig);
            // Recover the signer's address
            const recovered = await attestationsVerifier.recoverSigner(batchMessageHash, signatureBytes);
            expect(recovered).to.equal(signerAddress);
            // console.log("Signature is valid for signer: ", signerAddress);
            signatures.push(signatureBytes);
        }

        // Send the batch message
        let tx = await attestationsVerifier.receiveAttestationBatch(batch.nextCommittee, batch.bannedCommittee, batch.committedSfx, batch.revertedSfx, batch.index, batchMessageHash, signatures);

        // Wait for the transaction to be mined and get the logs
        const receipt = await tx.wait();

        let allEvents = await parseAllEvents(receipt, attestationsVerifier);
        // Get the SignerEmitted events from the logs

        // Check that the correct addresses and indexes were emitted
        const parsedBatchAppliedEvents = allEvents["BatchApplied"];
        expect(parsedBatchAppliedEvents.length).to.equal(1);
    });

    it("Should initialize committee and verify signatures for full batch", async function() {
        const wallets = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());
        const wallets_next_committee = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());

        const [defaultSigner] = await ethers.getSigners();

        for (let i = 0; i < wallets.length; i++) {
            const tx = await defaultSigner.sendTransaction({
                to: wallets[i].address,
                value: ethers.utils.parseEther("1")
            });
            await tx.wait();
        }
        // Create an array of addresses
        const initialCommittee = wallets.map(wallet => wallet.address);
        const nextCommittee = wallets_next_committee.map(wallet => wallet.address);

        const bannedCommittee = [nextCommittee[0], nextCommittee[1], nextCommittee[2]];

        const committedSfx = [ethers.utils.id("sfx#1"), ethers.utils.id("sfx#2"), ethers.utils.id("sfx#3")];

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(initialCommittee, 0);
        await attestationsVerifier.deployed();

        // Constructing the Batch struct
        let batch = {
            // nextCommitteeLen: 32,
            // bannedCommitteeLen: 3,
            // committedSfxLen: 3,
            // revertedSfxLen: 0,
            nextCommittee,
            bannedCommittee,
            committedSfx,
            revertedSfx: [],
            index: 1
        };
        // Encoding the Batch struct
        const encodedBatchMessage = batchEncodePacked(batch);

        // Hashing the encoded Batch struct
        const messageHash = ethers.utils.keccak256(encodedBatchMessage);

        const signatures = await generateSignatures(wallets, messageHash, attestationsVerifier);

        // Send the batch message
        let tx = await attestationsVerifier.receiveAttestationBatch(batch.nextCommittee, batch.bannedCommittee, batch.committedSfx, batch.revertedSfx, batch.index, messageHash, signatures);

        // Wait for the transaction to be mined and get the logs
        const receipt = await tx.wait();
        let allEvents = await parseAllEvents(receipt, attestationsVerifier);
        // Get the SignerEmitted events from the logs

        // Check that the correct addresses and indexes were emitted
        const parsedBatchAppliedEvents = allEvents["BatchApplied"];
        expect(parsedBatchAppliedEvents.length).to.equal(1);
    });
});

describe("AttestationsCommittee", function() {
    it("should correctly calculate committeeSize", async function() {
        const wallets = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());
        const wallets_next_committee = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());

        const initialCommittee = wallets.map(wallet => wallet.address);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(initialCommittee, 0);
        await attestationsVerifier.deployed();

        const initialCommitteeSize = await attestationsVerifier.committeeSize()
        expect(initialCommitteeSize).to.equal(32);

        let batch = {
            nextCommittee: wallets_next_committee.map(wallet => wallet.address),
            bannedCommittee: [],
            committedSfx: [],
            revertedSfx: [],
            index: 1
        };

        const messageHash = getMessageHash(batch)

        // Generate signatures
        const signatures = await generateSignatures(wallets, messageHash, attestationsVerifier);

        await attestationsVerifier.receiveAttestationBatch(
          batch.nextCommittee,
          batch.bannedCommittee,
          batch.committedSfx,
          batch.revertedSfx,
          batch.index,
          ethers.utils.arrayify(messageHash),
          signatures,
        );

        let nextCommitteeSize = await attestationsVerifier.committeeSize()
        expect(nextCommitteeSize.toNumber()).to.equal(32);
    });

    it("should only keep the current committee without old members", async function() {
        const wallets = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());
        const wallets_next_committee = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());

        const expectedInitialCommittee = wallets.map(wallet => wallet.address);
        const expectedNextCommittee = wallets_next_committee.map(wallet => wallet.address);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(expectedInitialCommittee, 0);
        await attestationsVerifier.deployed();

        const initialCommitteeSize = await attestationsVerifier.committeeSize()
        expect(initialCommitteeSize).to.equal(32);

        for (let i = 0; i < initialCommitteeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedInitialCommittee[i])
            expect(currentCommitteeMemberIndex).to.equal(1);
        }

        let batch = {
            nextCommittee: expectedNextCommittee,
            bannedCommittee: [],
            committedSfx: [],
            revertedSfx: [],
            index: 1
        };

        const messageHash = getMessageHash(batch)

        // Generate signatures
        const signatures = await generateSignatures(wallets, messageHash, attestationsVerifier);
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(1);

        let attestationRes = await attestationsVerifier.receiveAttestationBatch(
          batch.nextCommittee,
          batch.bannedCommittee,
          batch.committedSfx,
          batch.revertedSfx,
          batch.index,
          ethers.utils.arrayify(messageHash),
          signatures,
        );
        const receipt = await attestationRes.wait()
        console.log("receiveAttestationBatch gasUsed: ", receipt.cumulativeGasUsed.toString());
        let nextCommitteeSize = await attestationsVerifier.committeeSize()
        expect(nextCommitteeSize.toNumber()).to.equal(32);
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(2);

        for (let i = 0; i < initialCommitteeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedNextCommittee[i])
            expect(currentCommitteeMemberIndex).to.equal(2);
        }
        expect(await attestationsVerifier.totalAttesters()).to.equal(64);
    });


    it("should correctly process consecutive batches", async function() {
        const committeeSize = 32
        const committees = { 
            0: generateWallets(committeeSize),
            1: generateWallets(committeeSize),
            2: generateWallets(committeeSize),
            3: generateWallets(committeeSize),
        }

        let index = 0;

        // initialize contract
        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(committees[index].map(wallet => wallet.address), index);
        await attestationsVerifier.deployed();

        expect(await attestationsVerifier.committeeSize()).to.equal(committeeSize);

        for (let i = 0; i < committeeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(committees[index][i].address)
            expect(currentCommitteeMemberIndex).to.equal(index + 1);
        }

        // first batch
        index = 1
        await sendBatch(attestationsVerifier, index, committees[index-1], committees[index]);

        for (let i = 0; i < committeeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(committees[index][i].address)
            expect(currentCommitteeMemberIndex).to.equal(index + 1);
        }
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(index + 1);
        expect(await attestationsVerifier.totalAttesters()).to.equal((index +1) * committeeSize);

        // second batch
        index = 2
        await sendBatch(attestationsVerifier, index, committees[index-1], committees[index]);
        
        for (let i = 0; i < committeeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(committees[index][i].address)
            expect(currentCommitteeMemberIndex).to.equal(index + 1);
        }
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(index + 1);
        expect(await attestationsVerifier.totalAttesters()).to.equal((index + 1) * committeeSize);
        
        // third batch
        index = 3
        await sendBatch(attestationsVerifier, index, committees[index-1], committees[index]);
        
        for (let i = 0; i < committeeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(committees[index][i].address)
            expect(currentCommitteeMemberIndex).to.equal(index + 1);
        }
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(index + 1);
        expect(await attestationsVerifier.totalAttesters()).to.equal((index + 1) * committeeSize);
    });

    it("should increment current committee transition counter for the same set of members", async function() {
        const wallets = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());

        const expectedInitialCommittee = wallets.map(wallet => wallet.address);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(expectedInitialCommittee, 0);
        await attestationsVerifier.deployed();

        const initialCommitteeSize = await attestationsVerifier.committeeSize()
        expect(initialCommitteeSize).to.equal(32);

        for (let i = 0; i < initialCommitteeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedInitialCommittee[i])
            expect(currentCommitteeMemberIndex).to.equal(1);
        }

        let batch = {
            nextCommittee: expectedInitialCommittee,
            bannedCommittee: [],
            committedSfx: [],
            revertedSfx: [],
            index: 1
        };

        const messageHash = getMessageHash(batch)

        // Generate signatures
        const signatures = await generateSignatures(wallets, messageHash, attestationsVerifier);
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(1);

        let attestationRes = await attestationsVerifier.receiveAttestationBatch(
          batch.nextCommittee,
          batch.bannedCommittee,
          batch.committedSfx,
          batch.revertedSfx,
          batch.index,
          ethers.utils.arrayify(messageHash),
          signatures,
        );
        const receipt = await attestationRes.wait()
        console.log("receiveAttestationBatch gasUsed: ", receipt.cumulativeGasUsed.toString());
        let nextCommitteeSize = await attestationsVerifier.committeeSize()
        expect(nextCommitteeSize.toNumber()).to.equal(32);

        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(2);

        for (let i = 0; i < initialCommitteeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedInitialCommittee[i])
            expect(currentCommitteeMemberIndex).to.equal(2);
        }
        expect(await attestationsVerifier.totalAttesters()).to.equal(32);
    });

    it("should mark banned members among the known attester set", async function() {
        const wallets = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());

        const expectedInitialCommittee = wallets.map(wallet => wallet.address);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(expectedInitialCommittee, 0);
        await attestationsVerifier.deployed();

        const initialCommitteeSize = await attestationsVerifier.committeeSize()
        expect(initialCommitteeSize).to.equal(32);

        for (let i = 0; i < initialCommitteeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedInitialCommittee[i])
            expect(currentCommitteeMemberIndex).to.equal(1);
        }

        let batch = {
            nextCommittee: [],
            bannedCommittee: expectedInitialCommittee,
            committedSfx: [],
            revertedSfx: [],
            index: 1
        };

        const messageHash = getMessageHash(batch)

        // Generate signatures
        const signatures = await generateSignatures(wallets, messageHash, attestationsVerifier);
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(1);

        let attestationRes = await attestationsVerifier.receiveAttestationBatch(
          batch.nextCommittee,
          batch.bannedCommittee,
          batch.committedSfx,
          batch.revertedSfx,
          batch.index,
          ethers.utils.arrayify(messageHash),
          signatures,
        );
        const receipt = await attestationRes.wait()
        console.log("receiveAttestationBatch gasUsed: ", receipt.cumulativeGasUsed.toString());
        let nextCommitteeSize = await attestationsVerifier.committeeSize()
        expect(nextCommitteeSize.toNumber()).to.equal(32);
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(1);

        const MAX_UINT256 = ethers.constants.MaxUint256;
        for (let i = 0; i < initialCommitteeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedInitialCommittee[i])
            expect(currentCommitteeMemberIndex).to.equal(MAX_UINT256);
        }
        expect(await attestationsVerifier.totalAttesters()).to.equal(32);
    });

    it("should discard attestations from banned committee set", async function() {
        const wallets = Array.from({
            length: 32
        }, () => ethers.Wallet.createRandom());

        const expectedInitialCommittee = wallets.map(wallet => wallet.address);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(expectedInitialCommittee, 0);
        await attestationsVerifier.deployed();

        const initialCommitteeSize = await attestationsVerifier.committeeSize()
        expect(initialCommitteeSize).to.equal(32);

        for (let i = 0; i < initialCommitteeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedInitialCommittee[i])
            expect(currentCommitteeMemberIndex).to.equal(1);
        }

        let batch = {
            nextCommittee: [],
            bannedCommittee: expectedInitialCommittee,
            committedSfx: [],
            revertedSfx: [],
            index: 1
        };

        const messageHash = getMessageHash(batch)

        // Generate signatures
        const signatures = await generateSignatures(wallets, messageHash, attestationsVerifier);
        expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(1);

        let attestationRes = await attestationsVerifier.receiveAttestationBatch(
          batch.nextCommittee,
          batch.bannedCommittee,
          batch.committedSfx,
          batch.revertedSfx,
          batch.index,
          ethers.utils.arrayify(messageHash),
          signatures,
        );

        const MAX_UINT256 = ethers.constants.MaxUint256;
        for (let i = 0; i < initialCommitteeSize; i++) {
            let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedInitialCommittee[i])
            expect(currentCommitteeMemberIndex).to.equal(MAX_UINT256);
        }

        let batch_2 = {
            nextCommittee: expectedInitialCommittee,
            bannedCommittee: [],
            committedSfx: [],
            revertedSfx: [],
            index: 2
        };


        await expect(attestationsVerifier.receiveAttestationBatch(
          batch.nextCommittee,
          batch.bannedCommittee,
          batch.committedSfx,
          batch.revertedSfx,
          batch.index,
          ethers.utils.arrayify(messageHash),
          signatures,
        )).to.be.revertedWith("Signatures verification failed")
    });

    describe("AttestationsBatch", function() {
        it("should increment batch index after receiveAttestationBatch", async function() {
            const wallets = Array.from({
                length: 32
            }, () => ethers.Wallet.createRandom());

            const expectedInitialCommittee = wallets.map(wallet => wallet.address);

            const STARTING_BATCH_INDEX = 88;
            const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
            const attestationsVerifier = await AttestationsVerifier.deploy(expectedInitialCommittee, STARTING_BATCH_INDEX);
            await attestationsVerifier.deployed();

            const initialCommitteeSize = await attestationsVerifier.committeeSize()
            expect(initialCommitteeSize).to.equal(32);

            for (let i = 0; i < initialCommitteeSize; i++) {
                let currentCommitteeMemberIndex = await attestationsVerifier.attestersIndices(expectedInitialCommittee[i])
                expect(currentCommitteeMemberIndex).to.equal(1);
            }

            let batch = {
                nextCommittee: expectedInitialCommittee,
                bannedCommittee: [],
                committedSfx: [],
                revertedSfx: [],
                index: STARTING_BATCH_INDEX + 1
            };

            const messageHash = getMessageHash(batch)

            // Generate signatures
            const signatures = await generateSignatures(wallets, messageHash, attestationsVerifier);
            expect(await attestationsVerifier.currentCommitteeTransitionCount()).to.equal(1);
            expect(await attestationsVerifier.currentBatchIndex()).to.equal(STARTING_BATCH_INDEX);

            let _attestationRes = await attestationsVerifier.receiveAttestationBatch(
              batch.nextCommittee,
              batch.bannedCommittee,
              batch.committedSfx,
              batch.revertedSfx,
              batch.index,
              ethers.utils.arrayify(messageHash),
              signatures,
            );
            expect(await attestationsVerifier.currentBatchIndex()).to.equal(STARTING_BATCH_INDEX + 1);
        });
    });
});

async function sendBatch(attestationsVerifier, index, prevCommittee = [], nextCommittee = [], bannedCommittee = [], committedSfx = [], revertedSfx = []) {
    let batch = {
        nextCommittee: nextCommittee.map(wallet => wallet.address),
        bannedCommittee: bannedCommittee.map(wallet => wallet.address),
        committedSfx: committedSfx,
        revertedSfx: revertedSfx,
        index: index
    };
    const messageHash = getMessageHash(batch);
    const signatures = await generateSignatures(prevCommittee, messageHash, attestationsVerifier);

    let result = await attestationsVerifier.receiveAttestationBatch(
        ...Object.values(batch),
        ethers.utils.arrayify(messageHash),
        signatures
    );

    const receipt = await result.wait()
    console.log("receiveAttestationBatch gasUsed: ", receipt.cumulativeGasUsed.toString());

    return result
}

function generateWallets(size = 32) {
    const wallets = Array.from({
        length: size
    }, () => ethers.Wallet.createRandom());

    return wallets
}

