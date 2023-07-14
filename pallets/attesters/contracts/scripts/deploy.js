// scripts/deploy.js
const privateKey = process.env.ETHEREUM_PRIVATE_KEY;
// committee on first attestation message
const initialCommittee = [
  "0x517c62488386f687d3954a2382df8fb69416d6f3",
  "0xf07f036c552538377beda8341f66b84e26ed0dfc",
  "0xdd3bd377debddc0737fa8811f3f9d78f8eff2f04",
  "0x154f571be359982bca6518f4c5af5c9a944eb965",
  "0x2df305dbc27eb622e41bf1b92241d5cdc6c74c80",
  "0x1e5cf658e5a248c0057e70b383b0fad897a0f844",
  "0xa2f47cfd4608bda4701d5593f28f119192938158",
  "0x8b3394f80c24f112908847fe64c7288fa4b2751c",
  "0x29c8a75732b7f6e36bb2928c8e3b6b9ed83746f3",
  "0x02ace320f93166ec3b1f9c8799446b16cf3b7a33",
  "0x47557d67b3ac57c92e2e49901c07c69bb76e1440",
  "0x8d7fd1baf2a5501208cbf93934f7884c4eab7bd1",
  "0x14a530582bd8d8076f6d506fc80fe1bfdb950e88",
  "0xbae84cfc2759292ca397c17ba0a10a63c96979df",
  "0x5a03164c6d13e3542f4457e4012c69e362023afd",
  "0xd61bb6b923931b07bd00482891d2e9c862ffef8a"
  ]

async function main() {
    const Contract = await ethers.getContractFactory('AttestationsVerifier');
    const contract = await Contract.deploy(initialCommittee, 490)
  
    console.log('Contract deployed to address:', contract.address);
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });