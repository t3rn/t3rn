require("@nomicfoundation/hardhat-toolbox");

export default {
  networks: {
    local: {
      url: `http://localhost:8545`,
      // this mnemonic is already used by lighthouse in https://github.com/sigp/lighthouse so it's not sensitive data
      accounts: {mnemonic: "vast thought differ pull jewel broom cook wrist tribe word before omit"}
    }
  },
};
