import { Sdk } from "@t3rn/sdk";
import { config } from "../../config/config";
import fs from 'fs';
import Web3 from 'web3';


/**

 * @group Attestions
 */
export class AttestationManager {

    web3: any;
    rpc: string;
    receiveAttestationBatchContract: any
    wallet: any
    sdk: Sdk



    constructor(sdk: Sdk) {
        if (config.ethereum.privateKey === undefined) {
            throw new Error('Ethereum private key is not defined');
        }

        this.sdk = sdk
        this.rpc = config.ethereum.rpc;
        this.web3 = new Web3(this.rpc);
        this.wallet = this.web3.eth.accounts.privateKeyToAccount(config.ethereum.privateKey);

        const receiveAttestationBatchAbi = JSON.parse(fs.readFileSync('./src/attestationManager/contracts/AttestationsVerifier.abi.json', 'utf8'))
        const receiveAttestationBatchAddress = config.ethereum.attestationVerifierAddress;
        this.receiveAttestationBatchContract = new this.web3.eth.Contract(receiveAttestationBatchAbi.abi, receiveAttestationBatchAddress);
    }


    async receiveAttestationBatchCall() {
      // console.log([this.receiveAttestationBatchContract.methods])
      const committeeSize = await this.receiveAttestationBatchContract.methods.committeeSize().call()
      const currentBatchIndex = await this.receiveAttestationBatchContract.methods.currentBatchIndex().call()
      console.log(['Committee size: ',  committeeSize])
      console.log(['Batch Index: ', currentBatchIndex])

      // fetch batch 0 from t0rn 

      this.receiveAttestationBatchContract.methods.receiveAttestationBatchCall(
        // address[] memory newCommittee,
        // address[] memory bannedCommittee,
        // bytes32[] memory confirmedSFXs,
        // bytes32[] memory revertedSFXs,
        // uint32 index,
        // bytes32 expectedBatchHash,
        // bytes[] memory signatures
      )
        .send({ from: this.wallet.address, gas: 200000 })
        .on('transactionHash', (hash) => {
          console.log('Transaction hash:', hash);
        })
        .on('receipt', (receipt) => {
          console.log('Receipt:', receipt);
        })
        .on('error', (error) => {
          console.error('Error:', error);
        });

    }


}
