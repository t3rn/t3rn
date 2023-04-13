const { expect } = require("chai");

describe("PortalPrecompileInterface contract", function () {
    it("encodes read precompile select", async function () {
        const [owner] = await ethers.getSigners();
        const PPI = await ethers.getContractFactory("PortalPrecompileInterface");
        const hardhatPPI = await PPI.deploy();

        // console.log(hardhatPPI);

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
            action: 8, // You can replace this with the desired enum value
            id: ethers.utils.arrayify("0x03030303"), // Replace with the desired bytes4 value
            data: {
                value: ethers.utils.randomBytes(32), // Replace with the desired bytes value
            },
        };

        // Pass the VerifyInclusionPortalOption struct to the encodeVerifyPrecompileSelect function
        const result = await hardhatPPI.encodeVerifyPrecompileSelect(verifyInclusionPortalOption);
        console.log("Encoded verify @ Portal data:", result);
    });

    it("encodes verify precompile select more than 80 bytes long", async function () {
        const [owner] = await ethers.getSigners();
        const PPI = await ethers.getContractFactory("PortalPrecompileInterface");
        const hardhatPPI = await PPI.deploy();

        // Create a VerifyInclusionPortalOption struct
        const verifyInclusionPortalOption = {
            action: 8, // You can replace this with the desired enum value
            id: ethers.utils.arrayify("0x03030303"), // Replace with the desired bytes4 value
            data: {
                value: ethers.utils.randomBytes(81), // Replace with the desired bytes value
            },
        };

        // Pass the VerifyInclusionPortalOption struct to the encodeVerifyPrecompileSelect function
        const result = await hardhatPPI.encodeVerifyPrecompileSelect(verifyInclusionPortalOption);
        console.log("Encoded verify @ Portal data long:", result);
    });
});


