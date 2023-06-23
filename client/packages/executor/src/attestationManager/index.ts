import { Sdk } from "@t3rn/sdk";
import { config } from "../../config/config";
import fs from 'fs';
import Web3 from 'web3';
import { logger } from "../../src/logging";
import { ConfirmationBatch } from "./attestationBatch";
import abi from 'ethereumjs-abi';
import { keccak256 } from 'ethereumjs-util';



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
        if (config.attestations.ethereum.privateKey === undefined) {
            throw new Error('Ethereum private key is not defined');
        }

        this.client = client
        this.rpc = config.attestations.ethereum.rpc;
        this.web3 = new Web3(this.rpc);

        this.wallet = this.web3.eth.accounts.privateKeyToAccount(config.attestations.ethereum.privateKey); 

        const receiveAttestationBatchAbi = JSON.parse(fs.readFileSync('./src/attestationManager/contracts/AttestationsVerifier.abi.json', 'utf8'))
        const receiveAttestationBatchAddress = config.attestations.ethereum.attestationVerifierAddress;
        this.receiveAttestationBatchContract = new this.web3.eth.Contract(receiveAttestationBatchAbi.abi, receiveAttestationBatchAddress);
    }

    async fetchBatches() {
      await this.client.query.attesters.batches('sepl').then((data) => {
        const fetchedData: any = data.toJSON();

        const convertedData: ConfirmationBatch[] = fetchedData.map((batch: any) => {
          return {
            newCommittee: batch.newCommittee || [],
            bannedCommittee: batch.bannedCommittee || [],
            confirmedSFXs: batch.confirmedSFXs || [],
            revertedSFXs: batch.revertedSFXs || [],
            index: batch.index,
            expectedBatchHash: batch.expectedBatchHash,
            signatures: batch.signatures.map(([id, signature]) => signature),
          };
        });

        // console.log('Batches: ', data.toHuman())
        logger.info('We have ' + data.toJSON().length + ' batches')
        // logger.info(['Batch 0: ', data.toJSON()[0]])
        // logger.info(['Batch 1: ', data.toJSON()[1]])
        this.batches = convertedData;
      })
    }

    async receiveAttestationBatchCall() {
      // console.log([this.receiveAttestationBatchContract.methods])
      const committeeSize = await this.receiveAttestationBatchContract.methods.committeeSize().call()
      console.log(['Committee size: ',  committeeSize])
      const currentBatchIndex = await this.receiveAttestationBatchContract.methods.currentBatchIndex().call()
      console.log(['Batch Index: ', currentBatchIndex])

      const blockHash = await this.client.rpc.chain.getBlockHash(this.batches[0].expectedBatchHash)

      logger.error(this.batches[0])

      const batch = {
            newCommittee: this.batches[0].newCommittee,
            bannedCommittee:  this.batches[0].bannedCommittee,
            confirmedSFXs: this.batches[0].confirmedSFXs,
            revertedSFXs: this.batches[0].revertedSFXs,
            index: this.batches[0].index,
            expectedBatchHash: blockHash,
            signatures: this.batches[0].signatures,
      }

      // fetch batch 0 from t0rn 
      logger.error([
        batch
      ])

      const encodedBatch = abi.rawEncode(
        ['address[]', 'address[]', 'bytes32[]', 'bytes32[]', 'uint32'],
        [batch.newCommittee, batch.bannedCommittee, batch.confirmedSFXs, batch.revertedSFXs, batch.index]
      );



      // TODO: align naming of the contract with circuit
      await this.receiveAttestationBatchContract.methods.receiveAttestationBatch(
        this.batches[0].newCommittee,
        this.batches[0].bannedCommittee,
        this.batches[0].confirmedSFXs,
        this.batches[0].revertedSFXs,
        this.batches[0].index,
        Buffer.from(blockHash, 'hex'),
        this.batches[0].signatures,
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
