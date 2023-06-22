require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.18",

  networks: {
    sepolia: {
      url: "https://eth-sepolia.public.blastapi.io", // Replace with the Sepolia testnet URL
      chainId: 11155111, // Replace with the Sepolia chain ID
      accounts: ["0x026725ded690042c65163d263432d79b293a8a10108c7e7a32ff185f1926fb61"], // Replace with your Ethereum private keys for Sepolia
    },
  },

};
