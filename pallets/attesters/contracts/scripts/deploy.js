// scripts/deploy.js
const privateKey = process.env.ETHEREUM_PRIVATE_KEY;
// committee on first attestation message
const initialCommittee = [
  "0x1e32952b2c7111382fa09211b5731ba68576bc7a",
  "0x47557d67b3ac57c92e2e49901c07c69bb76e1440",
  "0xf57c633244822ad46f54598b28ec80ac33343e6c",
  "0x8d7fd1baf2a5501208cbf93934f7884c4eab7bd1",
  "0x517c62488386f687d3954a2382df8fb69416d6f3",
  "0xde23f5f6db5e39f7c0ee091dc5b144469663a67b",
  "0xdd3bd377debddc0737fa8811f3f9d78f8eff2f04",
  "0x5a03164c6d13e3542f4457e4012c69e362023afd",
  "0xede6fd9dec2dd2f1db3487a2ce576f6269af7e2a",
  "0x14a530582bd8d8076f6d506fc80fe1bfdb950e88",
  "0xc34d6b2b7841b4406457f0132bfe3819e650b26c",
  "0x2df305dbc27eb622e41bf1b92241d5cdc6c74c80",
  "0xd61bb6b923931b07bd00482891d2e9c862ffef8a",
  "0x1e5cf658e5a248c0057e70b383b0fad897a0f844",
  "0x6ba4fc474d5636cb030b98448d7d8ec4237434a7",
  "0xbae84cfc2759292ca397c17ba0a10a63c96979df"
  ]

async function main() {
    const Contract = await ethers.getContractFactory('AttestationsVerifier');
    const contract = await Contract.deploy(initialCommittee)
  
    console.log('Contract deployed to address:', contract.address);
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });