// scripts/deploy.js
const privateKey = process.env.ETHEREUM_PRIVATE_KEY;
// committee on first attestation message
const initialCommittee = [
  "0x2cb0d650cc7146bc9967115fba4d77f5f32d0cab",
  "0xde23f5f6db5e39f7c0ee091dc5b144469663a67b",
  "0xc34d6b2b7841b4406457f0132bfe3819e650b26c",
  "0xdf358b1a70e7d236a49d5157424f584eb3240227",
  "0x76c31c9f8f315f68d21c1bb07a0c25fecb07131b",
  "0xa2f47cfd4608bda4701d5593f28f119192938158",
  "0x8b3394f80c24f112908847fe64c7288fa4b2751c",
  "0x2a3d6324efa1f889407400d391258a62368ead9e",
  "0x02ace320f93166ec3b1f9c8799446b16cf3b7a33",
  "0x47557d67b3ac57c92e2e49901c07c69bb76e1440",
  "0x14a530582bd8d8076f6d506fc80fe1bfdb950e88",
  "0x379bf3020e9a3cd56ce32e9e8ab7ef84baac3e32",
  "0x3a68c6b6f010017c9b330a7c86d4b19c46ab677a",
  "0x5a03164c6d13e3542f4457e4012c69e362023afd",
  "0xd61bb6b923931b07bd00482891d2e9c862ffef8a",
  "0x8cf51a7d2281a3f9e9074a621390e7e29ef9d486"
  ]

async function main() {
    const Contract = await ethers.getContractFactory('AttestationsVerifier');
    const contract = await Contract.deploy(initialCommittee, 729)
  
    console.log('Contract deployed to address:', contract.address);
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });