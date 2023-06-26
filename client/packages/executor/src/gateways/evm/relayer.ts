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

  async setup(rpc: string, priv: string) {
    this.rpc = rpc;
    this.api = new Web3(getConfig().SEPOLIA)  // new Web3(rpc);
    this.sender = this.api.eth.accounts.privateKeyToAccount(priv);
    console.log(this.sender);
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
          .then(async (res: any) => {
            let { proof, index } = await getProof(
              res.transactionHash,
              this.api
            );
            const encoded = await scaleEncodeProof(proof, index);
            let encodedReceipt = await scaleEncodeReceipt(
              res.transactionHash,
              this.api
            );
            sideEffect.execute(
              encodedReceipt,
              res.blockNumber,
              "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
              encoded,
              res.blockHash,
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

