      import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring, Sdk } from "@t3rn/sdk";
import { config } from "../../config/config";
import fs from 'fs';
import Web3 from 'web3';
import { logger } from "../../src/logging";


/**

 * @group Attestions
 */
export class AttestationManager {

    web3: any;
    rpc: string;
    receiveAttestationBatchContract: any
    wallet: any
    client: any
    batches: any



    constructor(client: Sdk["client"]) {
        if (config.ethereum.privateKey === undefined) {
            throw new Error('Ethereum private key is not defined');
        }

        this.client = client
        this.rpc = config.ethereum.rpc;
        this.web3 = new Web3(this.rpc);

        // pub 0xF85A57d965aEcD289c625Cae6161d0Ab5141bC66
        // priv 0x78d3cbf37f1197996246b95bd42af04b14314bc23aa1b607410b0a72b5600156

        this.wallet = this.web3.eth.accounts.privateKeyToAccount(config.ethereum.privateKey); 
        // this.wallet = this.web3.eth.accounts.privateKeyToAccount('0x78d3cbf37f1197996246b95bd42af04b14314bc23aa1b607410b0a72b5600156'); 

        const receiveAttestationBatchAbi = JSON.parse(fs.readFileSync('./src/attestationManager/contracts/AttestationsVerifier.abi.json', 'utf8'))
        const receiveAttestationBatchAddress = config.ethereum.attestationVerifierAddress;
        this.receiveAttestationBatchContract = new this.web3.eth.Contract(receiveAttestationBatchAbi.abi, receiveAttestationBatchAddress);

        // fetch batches
        this.fetchBatches()
    }

    private async fetchBatches() {
      this.client.query.attesters.batches('sepl').then((data) => {
        console.log('Batches: ', data.toHuman())
        this.batches = data
      })
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

    async listenForConfirmedAttestationBatch() {
      throw new Error('Not implemented')
    }
}
