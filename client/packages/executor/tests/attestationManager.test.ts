import { default as chai, expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { AttestationManager } from "../src/attestationManager";
import { mock } from "ts-mockito";
import { Batch } from "../src/attestationManager/batch";
import { Sdk } from "@t3rn/sdk";

chai.use(chaiAsPromised);
chai.should();

describe("AttestationManager", () => {
  it("should return correct message hash for empty batch", () => {
    const attestationManager = new AttestationManager(mock(Sdk).client, mock());
    const batch: Batch = {
      nextCommittee: [],
      bannedCommittee: [],
      committedSfx: [],
      revertedSfx: [],
      index: 0,
    };

    const encodedBatch = attestationManager.batchEncodePacked(batch);
    // console.log("Encoded Batch:", encodedBatch);

    expect(encodedBatch).to.equal("0x00000000");

    const messageHash = attestationManager.getMessageHash(batch);

    expect(messageHash).to.equal(
      "0xe8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c",
    );
  });

  it("should return same message hash as circuit for first batch", () => {
    const attestationManager = new AttestationManager(mock(Sdk).client, mock());
    const batch: Batch = {
      nextCommittee: [
        "0x54e6c56c82a971b6727ae8ebc5d4baab49aa468c",
        "0xf57c633244822ad46f54598b28ec80ac33343e6c",
        "0xc34d6b2b7841b4406457f0132bfe3819e650b26c",
        "0x6ba4fc474d5636cb030b98448d7d8ec4237434a7",
        "0xde23f5f6db5e39f7c0ee091dc5b144469663a67b",
        "0xdf358b1a70e7d236a49d5157424f584eb3240227",
        "0x1e32952b2c7111382fa09211b5731ba68576bc7a",
        "0x82dd70f6bc734abc7315d23ada119bb892d54f6d",
        "0x8d7fd1baf2a5501208cbf93934f7884c4eab7bd1",
        "0x02ace320f93166ec3b1f9c8799446b16cf3b7a33",
        "0xede6fd9dec2dd2f1db3487a2ce576f6269af7e2a",
        "0x379bf3020e9a3cd56ce32e9e8ab7ef84baac3e32",
        "0xbae84cfc2759292ca397c17ba0a10a63c96979df",
        "0x2df305dbc27eb622e41bf1b92241d5cdc6c74c80",
        "0x1caed6f9330c3cff01902bce5470bdd397a1dae2",
        "0x29c8a75732b7f6e36bb2928c8e3b6b9ed83746f3",
      ],
      bannedCommittee: [],
      committedSfx: [],
      revertedSfx: [],
      index: 1,
    };

    // console.log("Batch:", batch);

    const encodedBatch = attestationManager.batchEncodePacked(batch);
    expect(encodedBatch).to.equal(
      "0x00000000000000000000000054e6c56c82a971b6727ae8ebc5d4baab49aa468c000000000000000000000000f57c633244822ad46f54598b28ec80ac33343e6c000000000000000000000000c34d6b2b7841b4406457f0132bfe3819e650b26c0000000000000000000000006ba4fc474d5636cb030b98448d7d8ec4237434a7000000000000000000000000de23f5f6db5e39f7c0ee091dc5b144469663a67b000000000000000000000000df358b1a70e7d236a49d5157424f584eb32402270000000000000000000000001e32952b2c7111382fa09211b5731ba68576bc7a00000000000000000000000082dd70f6bc734abc7315d23ada119bb892d54f6d0000000000000000000000008d7fd1baf2a5501208cbf93934f7884c4eab7bd100000000000000000000000002ace320f93166ec3b1f9c8799446b16cf3b7a33000000000000000000000000ede6fd9dec2dd2f1db3487a2ce576f6269af7e2a000000000000000000000000379bf3020e9a3cd56ce32e9e8ab7ef84baac3e32000000000000000000000000bae84cfc2759292ca397c17ba0a10a63c96979df0000000000000000000000002df305dbc27eb622e41bf1b92241d5cdc6c74c800000000000000000000000001caed6f9330c3cff01902bce5470bdd397a1dae200000000000000000000000029c8a75732b7f6e36bb2928c8e3b6b9ed83746f300000001",
    );

    const messageHash = attestationManager.getMessageHash(batch);
    // console.log("Message Hash:", messageHash);

    // generated by attestationsVerifier.sol
    expect(messageHash).to.equal(
      "0x28163e99017733f93743893089e66fb8ef71699d5624baae04d6daced4ccc150",
    );
  });
});
