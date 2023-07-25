// scripts/deploy.js
const privateKey = process.env.ETHEREUM_PRIVATE_KEY;
// committee on first attestation message
const initialCommittee = [
  "0xde23f5f6db5e39f7c0ee091dc5b144469663a67b",
  "0x54e6c56c82a971b6727ae8ebc5d4baab49aa468c",
  "0x6ba4fc474d5636cb030b98448d7d8ec4237434a7",
  "0xdf358b1a70e7d236a49d5157424f584eb3240227",
  "0x154f571be359982bca6518f4c5af5c9a944eb965",
  "0x76c31c9f8f315f68d21c1bb07a0c25fecb07131b",
  "0x8b3394f80c24f112908847fe64c7288fa4b2751c",
  "0xede6fd9dec2dd2f1db3487a2ce576f6269af7e2a",
  "0x02ace320f93166ec3b1f9c8799446b16cf3b7a33",
  "0x47557d67b3ac57c92e2e49901c07c69bb76e1440",
  "0x14a530582bd8d8076f6d506fc80fe1bfdb950e88",
  "0x379bf3020e9a3cd56ce32e9e8ab7ef84baac3e32",
  "0x1e32952b2c7111382fa09211b5731ba68576bc7a",
  "0x5a03164c6d13e3542f4457e4012c69e362023afd",
  "0xd61bb6b923931b07bd00482891d2e9c862ffef8a",
  "0x8cf51a7d2281a3f9e9074a621390e7e29ef9d486"
  ]

async function main() {
    const Contract = await ethers.getContractFactory('AttestationsVerifier');
    const contract = await Contract.deploy(initialCommittee, 689)
  
    console.log('Contract deployed to address:', contract.address);
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });