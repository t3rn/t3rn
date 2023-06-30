require("@nomicfoundation/hardhat-toolbox");

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
      accounts: [process.env.ETHEREUM_PRIVATE_KEY], 
    },
  },

};
