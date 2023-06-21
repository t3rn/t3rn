import { BaseTrie as Trie } from 'merkle-patricia-tree'
import { encode, receiptToRlp } from './encoder';
export const getProof = async (txId: string, instance: any) => {
    const receipt = await getReceipt(txId, instance)
    const blockReceipts = await getBlockReceipts(receipt.blockNumber, instance)
    const proof = (await generateProof(blockReceipts, receipt.transactionIndex))

    return [proof, receipt.transactionIndex]
}

const getReceipt = async (txId: string, instance: any) => {
    const receipt = await instance.eth.getTransactionReceipt(txId)
    return receipt
}

const getBlockReceipts = async (blockId: string, instance: any) => {
    const block = await getBlock(blockId, instance);

    let siblings = await Promise.all(block.transactions.map(async (tx: string) => {
        return getReceipt(tx, instance);
    }))

    let encoded = siblings.map((receipt: any) => {
        return { encoded: Buffer.from(receiptToRlp(receipt, instance)), index: Buffer.from(encode(receipt.transactionIndex)) }
    })

    return encoded
}

const getBlock = async (blockId: string, instance: any) => {
    await sleep(2000); // need to wait for RPC to by synced
    const block = await instance.eth.getBlock(blockId)
        .catch((err: any) => {
            console.log(`Error catched: ${err}`)
        })
    return block
}

const generateProof = async (receipts: any[], index: number) => {
    let trie = new Trie();

    await Promise.all(receipts.map((entry: any) => {
        return trie.put(entry.index, entry.encoded)
    }));
    const proof = await Trie.createProof(trie, Buffer.from(encode(index)))
    console.log("Proof: ", proof)
    const value = await Trie.verifyProof(trie.root, Buffer.from(encode(index)), proof)
    console.log("Proof Valid?", value !== null)
    return proof
}

export const sleep = async (time: number) => {
    return new Promise((resolve) => setTimeout(resolve, time));
}
