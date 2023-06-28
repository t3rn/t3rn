import { EventEmitter } from "events";
import Web3 from "web3";
import { SideEffect, TransactionType } from "../../utils/types";
import { getProof } from "./proof";
import createDebug from "debug";
import { scaleEncodeProof, scaleEncodeReceipt } from "./encoder";
import { erc20ABI } from "wagmi";
import { getConfig } from "./utils";

export class EVMRelayer extends EventEmitter {
  static debug = createDebug("evm-relayer");

  api: Web3;
  id: string;
  rpc: string;
  signer: any;
  nonce: bigint;
  sender: any;

  /**
   * Setup the instance of the relayer.
   *
   * Default value for the API endpoints is loaded
   * from config if not provided.
   *
   * @param priv Private key to the eth account
   * @param rpc? (Optional) Endpoint for the API. Defaults to Sepolia.
   * */
  async setup(priv: string, rpc?: string) {
    if (rpc) {
      this.rpc = rpc;
      this.api = new Web3(rpc)
    } else {

      this.api = new Web3(getConfig().SEPOLIA)  // new Web3(rpc);
    }
    this.sender = this.api.eth.accounts.privateKeyToAccount(priv);
    this.api.eth.accounts.wallet.add(this.sender);
  }

  /**
   * Create a contract instance from an asset using the ABI
   *
   * @param asset the address of the asset
   * @return the contract instance
   */
  async createAssetInstance(asset: string) {
    return new this.api.eth.Contract(erc20ABI, asset);
  }

  /**
   * Given a SFX, send it, get a proof of it, encode it and execute the SFX.
   * Only works for ERC20 transfers.
   *
   * @param sideEffect the sfx to be executed.
   * @return None
   */
  async executeTx(sideEffect: SideEffect): Promise<void> {
    switch (sideEffect.transactionType) {
      case TransactionType.Erc20Transfer: {
        const [to, asset, amount] = sideEffect.getTransactionArguments();
        const contract = await this.createAssetInstance(asset);
        contract.methods
          .transfer(to, amount)
          .send({
            from: this.sender.address,
            gas: this.api.utils.toHex(100000),
          })
          .then(async (response: any) => {
            let { proof, index } = await getProof(
              response.transactionHash,
              this.api
            );
            const encodedProof = await scaleEncodeProof(proof, index);
            let encodedReceipt = await scaleEncodeReceipt(
              response.transactionHash,
              this.api
            );
            sideEffect.execute(
              encodedReceipt,
              response.blockNumber,
              "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
              encodedProof,
              response.blockHash,
              true,
              false
            );
            this.emit("SideEffectExecuted", sideEffect.getId());
            EVMRelayer.debug(`Excuted SFX with ID: ${sideEffect.getId()}`);
          })
          .catch((err: any) => {
            console.log(err);
          });
      }
      default:
        EVMRelayer.debug(`Invalid TX type: ${sideEffect.transactionType}`);
    }
  }
}

