import { default as chai, expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { AttestationManager } from "../src/attestationManager";
import { mock } from "ts-mockito";
import { Batch } from "../src/attestationManager/batch";
import { logger } from "../src/logging";
import { Sdk } from "@t3rn/sdk";

chai.use(chaiAsPromised);
chai.should();

describe("AttestationManager", () => {
  it("should return correct message hash for empty batch", () => {
    const attestationManager = new AttestationManager(mock(Sdk).client);
    const batch: Batch = {
      nextCommittee: [],
      bannedCommittee: [],
      committedSfx: [],
      revertedSfx: [],
      index: 0,
    };

    const encodedBatch = attestationManager.batchEncodePacked(batch);
    console.log("Encoded Batch:", encodedBatch);

    //   expect(encodedBatch).to.equal('0x00000000');

    const messageHash = attestationManager.getMessageHash(batch);
    console.log("Message Hash:", messageHash);

    expect(messageHash).to.equal(
      "0xe8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c"
    );
  });

  it("should return correct message hash for first batch", () => {
    const attestationManager = new AttestationManager(mock(Sdk).client);
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

    const encodedBatch = attestationManager.batchEncodePacked(batch);
    console.log("Encoded Batch:", encodedBatch);

    //   expect(encodedBatch).to.equal('0x00000000000000000000000054e6c56c82a971b6727ae8ebc5d4baab49aa468c000000000000000000000000f57c633244822ad46f54598b28ec80ac33343e6c000000000000000000000000c34d6b2b7841b4406457f0132bfe3819e650b26c0000000000000000000000006ba4fc474d5636cb030b98448d7d8ec4237434a7000000000000000000000000de23f5f6db5e39f7c0ee091dc5b144469663a67b000000000000000000000000df358b1a70e7d236a49d5157424f584eb32402270000000000000000000000001e32952b2c7111382fa09211b5731ba68576bc7a00000000000000000000000082dd70f6bc734abc7315d23ada119bb892d54f6d0000000000000000000000008d7fd1baf2a5501208cbf93934f7884c4eab7bd100000000000000000000000002ace320f93166ec3b1f9c8799446b16cf3b7a33000000000000000000000000ede6fd9dec2dd2f1db3487a2ce576f6269af7e2a000000000000000000000000379bf3020e9a3cd56ce32e9e8ab7ef84baac3e32000000000000000000000000bae84cfc2759292ca397c17ba0a10a63c96979df0000000000000000000000002df305dbc27eb622e41bf1b92241d5cdc6c74c800000000000000000000000001caed6f9330c3cff01902bce5470bdd397a1dae200000000000000000000000029c8a75732b7f6e36bb2928c8e3b6b9ed83746f300000001');

    const messageHash = attestationManager.getMessageHash(batch);
    console.log("Message Hash:", messageHash);

    expect(messageHash).to.equal(
      "0xa142312886d62e858a0938a329c6bfa398cdb55570a300c636202cc2bdcd6d7d"
    );
  });

  it("should return correct message hash for first batch from pallet tests", () => {
    const attestationManager = new AttestationManager(mock(Sdk).client);
    const batch: Batch = {
      nextCommittee: [
        "0xB4Af5FB69713a28aCCfE461700c15b41246479FD",
        "0x014895A5F20f7fc1f67596ccF052Aa47B8c197eA",
        "0x5ADC71bD4166ac15B497295edd105e87e286B59a",
        "0xBc0E5c16d3c2329721ACCDC446B67B38f43231B8",
        "0xC6A56D001371e98F34a250c9bA8b1FE1b4a82976",
        "0x7222Ac4bd70E936193E48Ee01Ec7Ec3c684173d5",
        "0x2B1CB580592bd158Df8a655518f352E0c51b4ea7",
        "0x6e9BEBAeccbC21663eE55e62fE1cee2968fCd527",
        "0x81F0291FC3b540e35C9F92FCDeF0892223a19136",
        "0x48CE9e4c3213eB0Fe6DFdc1D5a9BceFa2aa46Ee5",
        "0x4ac7947CB68F18069F2201e0b6aE82a41b2Bb2D9",
        "0x02b849460104F855B844eaf55f1f232e322AEE11",
        "0x3d3Fa7cd34fd836827Be14Fe6E31daC03828F908",
        "0xb34A7E7f8ea8a6e13C21B7C9c63FA597F5E5a25B",
        "0xC91f16936d6Bd4D407f56A95d26BCEA99Ad6B012",
        "0x981DB86Bd7D01933f14569E24db88eA27a9Cf84D",
        "0x9d5bA839AA31d1C8095992022E7f7B889F5c5Ee4",
        "0x555B18fB39b0C9E7993979D76dBDA3feE6506c63",
        "0xe91DdB0D6ed792f30A05977b02921E2F2598a112",
        "0x899570d3D482A9CCE4896D7a9E94D571DBC375A6",
        "0x3F71564ec5D5b566a3Ecf71088190013E8DA2139",
        "0xaD01a069b69A35b705bC45228beDf65716C19ca6",
        "0xa59E2615F9054f3B2D3a0E5fFf8D1462D7cD3C25",
        "0x52b551BE6C975d1EbaD662f066b95e71147eD34a",
        "0x8b0A9f698FFb591af87Cd5fd402f405bA573cF89",
        "0x488e862F76f55B982Bc0Bd123288ed74a605Ed71",
        "0x717DD96B3FEBdBF40DEF3D97Cf1576D65FBf2B03",
        "0x7f578dacae1B7a1aFcD44e9BF2FE781215AD1f5e",
        "0x2f958B4B82D8F985C46849a73BBdd3956341f834",
        "0xC668bcec757ee434a4192cdDf58f4a43369dD861",
        "0xAea2e036fc5bf1b12AEc0BC26dfa0f8371Eb5C56",
        "0x6881585C22048AA149B2d0a1AD8d329b2F081DD2",
      ],
      bannedCommittee: [
        "0xB4Af5FB69713a28aCCfE461700c15b41246479FD",
        "0x014895A5F20f7fc1f67596ccF052Aa47B8c197eA",
        "0x5ADC71bD4166ac15B497295edd105e87e286B59a",
      ],
      committedSfx: [
        "0x6e906f8388de8faea67a770476ade4b76654545002126aa3ea17890fd8acdd7e",
        "0x580032f247eebb5c75889ab42c43dd88a1071c3950f9bbab1f901c47d5331dfa",
        "0xe23ab05c5ca561870b6f55d3fcb94ead2b14d8ce49ccf159b8e3449cbd5050c6",
      ],
      revertedSfx: [],
      index: 1,
    };

    const encodedBatch = attestationManager.batchEncodePacked(batch);
    console.log("Encoded Batch:", encodedBatch);

    //   expect(encodedBatch).to.equal('0x00000000000000000000000054e6c56c82a971b6727ae8ebc5d4baab49aa468c000000000000000000000000f57c633244822ad46f54598b28ec80ac33343e6c000000000000000000000000c34d6b2b7841b4406457f0132bfe3819e650b26c0000000000000000000000006ba4fc474d5636cb030b98448d7d8ec4237434a7000000000000000000000000de23f5f6db5e39f7c0ee091dc5b144469663a67b000000000000000000000000df358b1a70e7d236a49d5157424f584eb32402270000000000000000000000001e32952b2c7111382fa09211b5731ba68576bc7a00000000000000000000000082dd70f6bc734abc7315d23ada119bb892d54f6d0000000000000000000000008d7fd1baf2a5501208cbf93934f7884c4eab7bd100000000000000000000000002ace320f93166ec3b1f9c8799446b16cf3b7a33000000000000000000000000ede6fd9dec2dd2f1db3487a2ce576f6269af7e2a000000000000000000000000379bf3020e9a3cd56ce32e9e8ab7ef84baac3e32000000000000000000000000bae84cfc2759292ca397c17ba0a10a63c96979df0000000000000000000000002df305dbc27eb622e41bf1b92241d5cdc6c74c800000000000000000000000001caed6f9330c3cff01902bce5470bdd397a1dae200000000000000000000000029c8a75732b7f6e36bb2928c8e3b6b9ed83746f300000001');

    const messageHash = attestationManager.getMessageHash(batch);
    console.log("Message Hash:", messageHash);

    expect(messageHash).to.equal(
      "0xa142312886d62e858a0938a329c6bfa398cdb55570a300c636202cc2bdcd6d7d"
    );
  });
});
