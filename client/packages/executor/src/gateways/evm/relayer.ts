import { EventEmitter } from "events"
import Web3 from "web3";
import { SideEffect, TransactionType } from "../../utils/types"
import { getProof } from "./merkle"
import createDebug from "debug"
import { scaleEncodeProof, scaleEncodeReceipt } from "./encoder";
// import { abi } from './interfaces/ERC20.json'
import { erc20ABI } from 'wagmi'

// In case we need the full ERC20 schema
// import { Convert, Erc20, ABI } from "./interfaces/ERC20";
// const erc20: Erc20 = Convert.toErc20("./interfaces/ERC20.json");

export default class EVMRelayer extends EventEmitter {
    static debug = createDebug("bsc-relayer")

    api: Web3
    id: string
    rpc: string
    signer: any
    nonce: bigint
    sender: any

    async setup(rpc: string, priv: string) {
        this.rpc = rpc;
        this.api = new Web3(rpc);
        this.sender = this.api.eth.accounts.privateKeyToAccount(priv);
        console.log(this.sender)
        this.api.eth.accounts.wallet.add(this.sender);
    }

    async createAssetInstance(asset: string) {
        return new this.api.eth.Contract(
            erc20ABI, // abi, // erc20.abi overloads the function call
            asset,    // FIXME was this the address?
        );
    }

    async executeTx(sideEffect: SideEffect) {
        // FIXME probably don't need this on eth
        const nonce = this.nonce++
        EVMRelayer.debug("executeTx nonce", nonce)

        switch (sideEffect.transactionType) {
            case TransactionType.Erc20Transfer: {
                const [to, asset, amount] = sideEffect.getTransactionArguments()
                const contract = await this.createAssetInstance(asset)
                contract.methods.transfer(to, amount)
                    .send({ from: this.sender.address, gas: this.api.utils.toHex(100000) })
                    .then(async (res: any) => {
                        let [proof, index] = await getProof(res.transactionHash, this.api)
                        const encoded = await scaleEncodeProof(proof, index);
                        let encodedReceipt = await scaleEncodeReceipt(res.transactionHash, this.api)
                        sideEffect.execute(
                            encodedReceipt,
                            res.blockNumber,
                            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                            encoded,
                            res.blockHash,
                            true,
                            false,
                        )

                        this.emit("SideEffectExecuted", sideEffect.getId())
                    })
                    .catch((err: any) => {
                        console.log(err)
                    })
            }
            default:
                EVMRelayer.debug(`Invalid TX type: ${sideEffect.transactionType}`)
        }
    }

    wait() {
        return new Promise((res, _) => {
            setTimeout(() => {
                res
            }, 2000)
        })
    }
}
