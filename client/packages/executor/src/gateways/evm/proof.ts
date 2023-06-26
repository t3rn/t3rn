import { BaseTrie as Trie } from "merkle-patricia-tree";
import { encode, receiptToRlp } from "./encoder";
import Web3, { Numbers, TransactionReceipt } from "web3";


/**
 * Construct the proof for a transaction, given its hash
 *
 * @param txHash the identifier of the transaction
 * @return object { proof, tx index }
 * */
export const getProof = async (
  txHash: string,
  client: Web3
) => {
  // Infura's endpoint. If it fails, generate a new one
  // const client = new Web3(getConfig().SEPOLIA);

  // get the transaction (object) receipt given the ID
  const receipt = await getReceipt(txHash, client);

  // get all the receipts from the previous one
  const blockReceipts = await getBlockReceipts(receipt.blockNumber, client);

  // generate the proof with all the receipts
  const proof = await generateProof(blockReceipts, receipt.transactionIndex);

  // get the witness
  // const witness = proof.map(node => node.toString('hex'))

  return { proof: proof, index: receipt.transactionIndex };
};

/**
 * Transactions have a receipt with all the information regarding to them
 *
 * @param txHash the hash as a hex string
 * @param client the Web3 client using to query the info
 */
const getReceipt = async (
  txHash: string,
  client: Web3
): Promise<TransactionReceipt> => {
  const receipt = await client.eth.getTransactionReceipt(txHash);
  return receipt;
};

/**
 * Get all the receipts from a certain block
 *
 * @param blockId the ID as a number | string | bigint
 * @param client the Web3 client using to query the info
 */
const getBlockReceipts = async (blockId: Numbers, client: Web3) => {
  const block = await getBlock(blockId, client);

  const siblings = await Promise.all(
    block.transactions.map(async (tx: any) => {
      // FIXME see if it's `tx.hash` what we need
      return getReceipt(tx.hash, client);
    })
  );

  const encoded = siblings.map((receipt: TransactionReceipt) => {
    return {
      encoded: Buffer.from(receiptToRlp(receipt, client)),
      index: Buffer.from(encode(receipt.transactionIndex)),
    };
  });
  return encoded;
};

/**
 * Dummy interface to not loose typecheck
 */
interface encodedReceipt {
  encoded: Buffer;
  index: Buffer;
}

/**
 * Get all the block for a certain ID
 *
 * @param blockId the ID as Numbers (a number | string | bigint)
 * @param client the Web3 client using to query the info
 */
const getBlock = async (blockId: Numbers, client: Web3) => {
  await sleep(2000);  // wait for RPC to be in sync

  const block = await client.eth.getBlock(blockId);

  return block;
};

/**
 * Generate a proof for a
 */
const generateProof = async (receipts: encodedReceipt[], index: Numbers) => {
  let trie = new Trie();

  await Promise.all(
    receipts.map((entry: any) => {
      return trie.put(entry.index, entry.encoded);
    })
  );
  const proof = await Trie.createProof(trie, Buffer.from(encode(index)));
  console.log("Computed Root: ", trie.root.toString("hex"));
  const value = await Trie.verifyProof(
    trie.root,
    Buffer.from(encode(index)),
    proof
  );
  console.log("Proof Valid?", value !== null);
  return proof;
};

export const sleep = async (time: number) => {
  return new Promise((resolve) => setTimeout(resolve, time));
};

// getProof("0x569a367d4cea332568a00e1fa4389c1fc2d79a7be4224bef179716d768605bae")
