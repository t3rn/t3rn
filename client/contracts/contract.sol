// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.7.0 <0.9.0;
/// @title Voting with delegation.
contract Snorkle {
    // event interface
    string private eventInterface = "Transfer(address indexed from, address indexed to, uint256 value)";

    address private constant PORTAL = address(0x000000000000000000000000000000000000000A);

    bytes4 public SEPL = 0x7365706C; // gateway id for sepolia
    event Adress(address addr);
    event PortalHeight(bytes height);
    event Success(bool success);

    // This is a type for a single proposal.
    struct Transfer {
        address contractAddr;
        address from;
        address to;
        uint256 value;
    }

    /// Create a new ballot to choose one of `proposalNames`.
    constructor() {
    }

    function getHeight() public  {
        bytes32 arguments = bytes32(0x027365706C000000000000000000000000000000000000000000000000000000);
        
        (bool success, bytes memory returnData) = PORTAL.staticcall(
            abi.encodePacked(arguments)
        );
        emit Success(success);
        emit Adress(msg.sender);
        emit PortalHeight(returnData);
    }

}