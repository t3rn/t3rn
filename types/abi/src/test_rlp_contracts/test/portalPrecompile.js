const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("PortalPrecompileInterface contract", function () {
  it("encodes read precompile select", async function () {

    const [owner] = await ethers.getSigners();
    const PPI = await ethers.getContractFactory("PortalPrecompileInterface");
    const hardhatPPI = await PPI.deploy();

    // Create a ReadPortalOption struct
    const readPortalOption = {
      action: 4, // You can replace this with the desired enum value
      id: ethers.utils.arrayify("0x03030303"), // Replace with the desired bytes4 value
    };

    // Pass the ReadPortalOption struct to the encodeReadPrecompileSelect function
    const result = await hardhatPPI.encodeReadPrecompileSelect(readPortalOption);
    console.log("Encoded read @ Portal data:", result);
  });

  it("encodes verify precompile select", async function () {
    const [owner] = await ethers.getSigners();
    const PPI = await ethers.getContractFactory("PortalPrecompileInterface");
    const hardhatPPI = await PPI.deploy();

    // Create a VerifyInclusionPortalOption struct
    const verifyInclusionPortalOption = {
      action: 4, // You can replace this with the desired enum value
      id: ethers.utils.arrayify("0x03030303"), // Replace with the desired bytes4 value
      executionSource: ethers.utils.arrayify("0x0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a"), // Replace with the desired contract address of MAX 32 Bytes on target
      speedMode: 1, // Replace with the desired speed mode
      data: {
        value: ethers.utils.arrayify("0xffffffffffffffffffffffffffffffffffffffffffffffffffff"), // Replace with the desired bytes value
      },
    };

    // Pass the VerifyInclusionPortalOption struct to the encodeVerifyPrecompileSelect function
    const result = await hardhatPPI.encodeVerifyPrecompileSelect(verifyInclusionPortalOption);
    console.log("Encoded verify @ Portal data long:", result);

    expect(result).equal("0x0403030303010a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0affffffffffffffffffffffffffffffffffffffffffffffffffff");
  });

  it("encodes verify precompile select more than 80 bytes long", async function () {
    const [owner] = await ethers.getSigners();
    const PPI = await ethers.getContractFactory("PortalPrecompileInterface");
    const hardhatPPI = await PPI.deploy();

    // Create a VerifyInclusionPortalOption struct
    const verifyInclusionPortalOption = {
      action: 4, // You can replace this with the desired enum value
      id: ethers.utils.arrayify("0x03030303"), // Replace with the desired bytes4 value
      executionSource: ethers.utils.arrayify("0x0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a"), // Replace with the desired contract address of MAX 32 Bytes on target
      speedMode: 1, // Replace with the desired speed mode
      data: {
        value: ethers.utils.arrayify("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"), // Replace with the desired bytes value
      },
    };

    // Pass the VerifyInclusionPortalOption struct to the encodeVerifyPrecompileSelect function
    const result = await hardhatPPI.encodeVerifyPrecompileSelect(verifyInclusionPortalOption);
    console.log("Encoded verify @ Portal data long:", result);

    expect(result).equal("0x0403030303010a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0affffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

  });


  it("encodes arguments with encodeVerifyEventInclusion", async function () {
    const [owner] = await ethers.getSigners();
    const PPI = await ethers.getContractFactory("PortalPrecompileInterface");
    const hardhatPPI = await PPI.deploy();

    // Create a VerifyInclusionPortalOption struct
    const verifyInclusionPortalOption = {
      action: 4, // You can replace this with the desired enum value
      id: ethers.utils.arrayify("0x03030303"), // Replace with the desired bytes4 value
      executionSource: ethers.utils.arrayify("0x0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a"), // Replace with the desired contract address of MAX 32 Bytes on target
      speedMode: 1, // Replace with the desired speed mode
      data: {
        value: ethers.utils.arrayify("0xffffffffffffffffffffffffffffffffffffffffffffffffffff"), // Replace with the desired bytes value
      },
    };

    let chainId = ethers.utils.arrayify("0x03030303");
    let speedMode = ethers.utils.arrayify("0x01");
    const eventData = {
      value: ethers.utils.arrayify("0xffffffffffffffffffffffffffffffffffffffffffffffffffff"), // Replace with the desired bytes value
    }

    // Pass the VerifyInclusionPortalOption struct to the encodeVerifyPrecompileSelect function
    const result = await hardhatPPI.encodeVerifyEventInclusion(chainId, speedMode, eventData);
    console.log("Encoded encodeVerifyEventInclusion @ Portal data long:", result);

    // Encodes executionSource as 0-bytes if no given
    expect(result).equal("0x0403030303010000000000000000000000000000000000000000000000000000000000000000ffffffffffffffffffffffffffffffffffffffffffffffffffff");
  });

  it("encodes arguments with encodeVerifyEventInclusionWithSource", async function () {
    const [owner] = await ethers.getSigners();
    const PPI = await ethers.getContractFactory("PortalPrecompileInterface");
    const hardhatPPI = await PPI.deploy();


    let chainId = ethers.utils.arrayify("0x03030303");
    let speedMode = ethers.utils.arrayify("0x01");
    const eventData = {
      value: ethers.utils.arrayify("0xffffffffffffffffffffffffffffffffffffffffffffffffffff"), // Replace with the desired bytes value
    }

    // Pass the VerifyInclusionPortalOption struct to the encodeVerifyEventInclusionWithSource function
    const result = await hardhatPPI.encodeVerifyEventInclusionWithSource(chainId, speedMode, eventData, owner.address);
    console.log("Encoded encodeVerifyEventInclusion @ Portal data long:", result);

    // Encodes executionSource as 0-bytes if no given
    expect(result).equal("0x040303030301f39fd6e51aad88f6f4ce6ab8827279cfffb92266000000000000000000000000ffffffffffffffffffffffffffffffffffffffffffffffffffff");
  });

});