require("@nomicfoundation/hardhat-toolbox");
let ETHEREUM_PRIVATE_KEY

if (process.env.ETHEREUM_PRIVATE_KEY != null) {
  ETHEREUM_PRIVATE_KEY = process.env.ETHEREUM_PRIVATE_KEY
} else {
  ETHEREUM_PRIVATE_KEY = "0x6bfa80bcf02789b719e4b47c26063b1caec900aebc0d09034d3cf2e4f6bfad98"
}

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.18",

  networks: {
    ganache: {
      url: "http://127.0.0.1:7545",
      chainId: 1337, 
      accounts: ["0x6bfa80bcf02789b719e4b47c26063b1caec900aebc0d09034d3cf2e4f6bfad98"], // default ganache-cli mnemonic
    },
    sepolia: {
      url: "https://eth-sepolia.public.blastapi.io", 
      chainId: 11155111, 
      accounts: ["0x78d3cbf37f1197996246b95bd42af04b14314bc23aa1b607410b0a72b5600156"],
    },
  },
};
