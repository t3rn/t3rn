// this contract is used to add and query t3rn block headers, used by the escrow contract to run inclusion proofs of commit/revert events

struct Header {
    uint height;
    bytes32 root;
}

contract HeaderRegistry {

    mapping(bytes32 => Header) public headers;

    function addHeader(uint height, bytes32 root, bytes32 headerId) 
        external
    {
        // Verify header validity
        // Probably BEEFY is used for this.
        // I really wonder what the gas costs are for this, can't find any good figures.
        // One thing to look out for in my (probably biased) opinion is ZK solutions. 
        // Circom released the PLONK proof construct a while back 
        //      -> result is that you can use N circuits (programs in zk related stuff) with the same verifier, which is amazing
        // They are also working on recursive PLONK. if they pull it of we could verify N headers for pretty much a fixed gas amount (ignoring header storage costs)
        // This is also what Mina uses to do dapps (or zksync)
        headers[headerId] = Header(height, root);
    }
}