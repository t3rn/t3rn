//"SPDX-License-Identifier: UNLICENSED"
pragma solidity ^0.8.4;

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
        headers[headerId] = Header(height, root);
    }
}