const { expect } = require("chai");
const { ethers } = require("hardhat");


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


async function generateInitialCommitteeOf32() {
    const wallets = Array.from({ length: 32 }, () => ethers.Wallet.createRandom());
    const [defaultSigner] = await ethers.getSigners();

    for (let i = 0; i < wallets.length; i++) {
        const tx = await defaultSigner.sendTransaction({
            to: wallets[i].address,
            value: ethers.utils.parseEther("1")
        });
        await tx.wait();
    }

    return wallets.map(wallet => wallet.address);
}

describe("AttestationsVerifier", function() {
    it('Should recover the correct signer from the signature', async () => {
        // Replace these values with the ones used in your test case
        const privateKey = "0x0123456789012345678901234567890123456789012345678901234567890123";
        const message = "Hello, world!";
        const messageHash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(message));

        const wallet = new ethers.Wallet(privateKey);

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy([]);
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

        const wallets = Array.from({ length: 32 }, () => ethers.Wallet.createRandom());
        const [defaultSigner] = await ethers.getSigners();

        for (let i = 0; i < wallets.length; i++) {
            const tx = await defaultSigner.sendTransaction({
                to: wallets[i].address,
                value: ethers.utils.parseEther("1")
            });
            await tx.wait();
        }

        // Create an array of Attester structs
        const initialCommittee = wallets.map((wallet, index) => ({ attesterAddress: wallet.address, index }));

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(initialCommittee);
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

    it("Should initialize committee and verify signatures for empty batch", async function() {
        const wallets = Array.from({ length: 32 }, () => ethers.Wallet.createRandom());
        const [defaultSigner] = await ethers.getSigners();

        for (let i = 0; i < wallets.length; i++) {
            const tx = await defaultSigner.sendTransaction({
                to: wallets[i].address,
                value: ethers.utils.parseEther("1")
            });
            await tx.wait();
        }
        // Create an array of Attester structs
        const initialCommittee = wallets.map((wallet, index) => ({ attesterAddress: wallet.address, index }));

        const AttestationsVerifier = await ethers.getContractFactory("AttestationsVerifier");
        const attestationsVerifier = await AttestationsVerifier.deploy(initialCommittee);
        await attestationsVerifier.deployed();

        // Constructing the Batch struct with empty arrays
        let batch = {
            newCommittee: [],
            bannedCommittee: [],
            confirmedSFXs: []
        }

        // Encoding the Batch struct
        const encodedBatchMessage = ethers.utils.defaultAbiCoder.encode(
          ["address[]", "address[]", "bytes32[]"],
          [batch.newCommittee, batch.bannedCommittee, batch.confirmedSFXs]
        );

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
        let tx = await attestationsVerifier.receiveAttestationBatch(encodedBatchMessage, batchMessageHash, signatures);

        // Wait for the transaction to be mined and get the logs
        const receipt = await tx.wait();
        const logs = receipt.logs;
        const signerEmittedEvents = logs.filter(log => log.topics[0] === ethers.utils.id("SignerEmitted(address,uint32)"));
        // Check that the correct number of SignerEmitted events were emitted
        expect(signerEmittedEvents.length).to.equal(32);

        let allEvents = await parseAllEvents(receipt, attestationsVerifier);
        // Get the SignerEmitted events from the logs

        // Check that the correct addresses and indexes were emitted
        const parsedSingerEmittedEvents = allEvents["SignerEmitted"];
        expect(parsedSingerEmittedEvents.length).to.equal(32);

        for (let i = 0; i < parsedSingerEmittedEvents.length; i++) {
            // Expect that parsedSingerEmittedEvents contains the correct signer address and member index from the initial committee
            expect(parsedSingerEmittedEvents[i].signer).to.equal(initialCommittee[i].attesterAddress);
        }
    });
});
