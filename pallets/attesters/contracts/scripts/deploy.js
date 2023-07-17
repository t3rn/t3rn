// scripts/deploy.js
const privateKey = process.env.ETHEREUM_PRIVATE_KEY;
// committee on first attestation message
const initialCommittee = [
  "0xde23f5f6db5e39f7c0ee091dc5b144469663a67b",
  "0x6ba4fc474d5636cb030b98448d7d8ec4237434a7",
  "0x1caed6f9330c3cff01902bce5470bdd397a1dae2",
  "0xdf358b1a70e7d236a49d5157424f584eb3240227",
  "0x2df305dbc27eb622e41bf1b92241d5cdc6c74c80",
  "0x76c31c9f8f315f68d21c1bb07a0c25fecb07131b",
  "0x1e5cf658e5a248c0057e70b383b0fad897a0f844",
  "0x29c8a75732b7f6e36bb2928c8e3b6b9ed83746f3",
  "0xede6fd9dec2dd2f1db3487a2ce576f6269af7e2a",
  "0x47557d67b3ac57c92e2e49901c07c69bb76e1440",
  "0x14a530582bd8d8076f6d506fc80fe1bfdb950e88",
  "0x379bf3020e9a3cd56ce32e9e8ab7ef84baac3e32",
  "0x3a68c6b6f010017c9b330a7c86d4b19c46ab677a",
  "0xbae84cfc2759292ca397c17ba0a10a63c96979df",
  "0xd61bb6b923931b07bd00482891d2e9c862ffef8a",
  "0x8cf51a7d2281a3f9e9074a621390e7e29ef9d486"  
  ]

async function main() {
    const Contract = await ethers.getContractFactory('AttestationsVerifier');
    const contract = await Contract.deploy(initialCommittee, 539)
  
    console.log('Contract deployed to address:', contract.address);
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });