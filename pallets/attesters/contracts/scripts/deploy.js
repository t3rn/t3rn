// scripts/deploy.js
const privateKey = process.env.ETHEREUM_PRIVATE_KEY;
// committee on first attestation message
const initialCommittee = [
  "0x1caed6f9330c3cff01902bce5470bdd397a1dae2",
  "0x3a68c6b6f010017C9b330a7C86D4B19c46ab677a",
  "0x82dd70f6bc734abc7315d23ada119bb892d54f6d",
  "0x14a530582bd8d8076f6d506fc80fe1bfdb950e88",
  "0xC34D6B2B7841B4406457f0132BFE3819e650b26c",
  "0x8d7fd1baf2a5501208cbf93934f7884c4eab7bd1",
  "0xf07f036C552538377bEDa8341f66B84E26ed0Dfc",
  "0x5A03164C6d13e3542F4457e4012C69E362023AfD",
  "0xede6fd9dec2dd2f1db3487a2ce576f6269af7e2a",
  "0x76c31c9F8F315f68d21C1bB07a0C25FEcb07131b",
  "0x517C62488386f687D3954A2382dF8Fb69416d6F3",
  "0xf57C633244822ad46F54598B28EC80aC33343E6C",
  "0x1e32952B2C7111382FA09211b5731ba68576BC7A",
  "0x6Ba4fC474D5636cb030b98448D7d8EC4237434A7",
  "0x8cf51a7d2281a3f9e9074a621390e7e29ef9d486",
  "0xa2f47cfd4608bda4701d5593f28f119192938158",
  ]

async function main() {
    const Contract = await ethers.getContractFactory('AttestationsVerifier');
    const contract = await Contract.deploy(initialCommittee, 489)
  
    console.log('Contract deployed to address:', contract.address);
  }
  
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });