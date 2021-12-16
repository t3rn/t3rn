//"SPDX-License-Identifier: UNLICENSED"
pragma solidity ^0.8.4;
pragma experimental ABIEncoderV2; // allows us to pass structs
import "./SimplifiedMMRVerification.sol";
import "./HeaderRegistry.sol";
import "./interface/IERC20.sol";
import "hardhat/console.sol";

contract Escrow is SimplifiedMMRVerification, HeaderRegistry {

    // to save gas we hash the values
    mapping(bytes32 => bytes32) public active;

    modifier noDuplicateXtx(bytes32 xtxId) {
        require(active[xtxId] == 0, "Duplicate XTX ID");
        _;
    }

    event Commit(bytes32 xtxId);
    event Revert(bytes32 xtxId);

    struct CircuitEvent {
        // assumption here is, that the circut will only emit a commit event, if the amounts during execute are correct
        bytes32 xtxId;
        bool shouldCommit;
        // will need more fields for inclusion proof
    }

    // initializes escrowed eth transfer
    function ethTransfer(address to, bytes32 xtxId)
        external
        payable
        noDuplicateXtx(xtxId)
    {
        // we hash the inputs, this has two benefits
        // 1. reduces gas cost, as storage is more expensive then compute
        // 2. we can store all escrow types in same mapping. makes duplicate prevention checks a lot cheaper
        active[xtxId] = _hashEthTransfer(to, msg.value, msg.sender);
    }

    // intializes escrowed token transfer
    function tokenTransfer(address to, address token, uint amount, bytes32 xtxId)
        external
        noDuplicateXtx(xtxId)
    {
        _collectToken(amount, token);
        active[xtxId] = _hashTokenTransfer(to, token, amount, msg.sender);
    }

    function releaseEthTransfer(CircuitEvent memory evnt, address to, uint amount)
        external
    {
        // verify finality of CircuitEvent here. See `_verifyFinality()`

        // ensure the correct inputs where passed
        require(_hashEthTransfer(to, amount, msg.sender) == active[evnt.xtxId], "False inputs passed");
        if(evnt.shouldCommit) {
            // we are commiting
            _sendEth(payable(to), amount);
            emit Commit(evnt.xtxId);
        } else {   
            // reverting
            _sendEth(payable(msg.sender), amount);
            emit Revert(evnt.xtxId);
        }
        delete active[evnt.xtxId]; // gas refund
    }

    function releaseTokenTransfer(CircuitEvent memory evnt, address to, address token, uint amount)
        external
    {
        // verify finality of CircuitEvent here. See `_verifyFinality()`

        // ensure the correct inputs where passed
        require(_hashTokenTransfer(to, token, amount, msg.sender) == active[evnt.xtxId], "False inputs passed");

        if(evnt.shouldCommit) {
            _sendToken(to, token, amount);
            emit Commit(evnt.xtxId);
        } else {   
            _sendToken(payable(msg.sender), token, amount);
            emit Revert(evnt.xtxId);
        }

        delete active[evnt.xtxId]; // gas refund
    }

    // this function is never called and simply a conceptual implementatio until we have the details figured out. 
    function _verifyFinality(bytes32 headerId, CircuitEvent memory evnt, SimplifiedMMRProof memory proof)
        private
        view
    {
         // we assume the existance of a trustless header registry, containing verified t3rn block headers
        bytes32 root = HeaderRegistry.headers[headerId].root;

        // Hash the event/transaction to create leaf. Need more specs here.
        // I'm also assuming the circuit will ensure the correct escrow deposit was made (amount, assets, receiver, etc.).
        bytes32 leafHash = keccak256(abi.encodePacked(evnt.xtxId, evnt.shouldCommit));

        // Run imnclusion proof, prooving the finality and valifity of submitred event
        // we need something that does a patricia-tri tree no?
        require(SimplifiedMMRVerification.verifyInclusionProof(root, leafHash, proof), "MMR verification failed.");
    }

    function _hashEthTransfer(address to, uint amount, address executor)
        private
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(to, amount, executor));
    }

    function _hashTokenTransfer(address to, address asset, uint amount, address executor)
        private
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(to, asset, amount, executor));
    }

    function _sendEth(address payable _to, uint amount)
        private
    {
        _to.call{value: amount}("");
    }

    function _sendToken(address to, address token, uint amount)
        private
    {
        IERC20(token).transfer(to, amount);
    }

    function _collectToken(
        uint256 amount,
        address token
    )
        private
    {
        // escrow contract needs to be approved before
        // This is how erc20 tokens are sent to smart contracts -> Approve contract as spender, then transferFrom to contract address.
        IERC20(token).transferFrom(msg.sender, address(this), amount);
    }
}