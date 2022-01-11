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
    event EscrowTransfer(address from, address to, uint amount);
    event EscrowMultiTransfer(address from, address to, uint amount, address token);
    event EscrowSwap(address executor, address to, uint amount, address token);
    event EscrowAddLiquidity(address executor, address to, address token, uint amount);
    event EscrowRemoveLiquidity(address executor, address to, address tokenA, uint amountA, address tokenB, uint amountB, bytes32 xtxId);

    struct CircuitEvent {
        // assumption here is, that the circut will only emit a commit event, if the amounts during execute are correct
        bytes32 xtxId;
        bool shouldCommit;
        // will need more fields for inclusion proof
    }

    // function for swapping to ether. Turns out these are very similar to the transfers.
    function ethSwap(address to, bytes32 xtxId)
        external
        payable
        noDuplicateXtx(xtxId)
    {
        active[xtxId] = _hashEthSwap(to, msg.value, msg.sender);
        emit EscrowSwap(msg.sender, to, msg.value, address(0x0));
    }

    function addLiquidity(address to, address token, uint amount, bytes32 xtxId)
        external
        noDuplicateXtx(xtxId)
    {
        // dont think we need to add the source of fund (assets being deposited) as the correctness of this is checked in the circuit
        _collectToken(amount, token); // collects liq token from executor
        active[xtxId] = keccak256(abi.encodePacked(msg.sender, to, token, amount, xtxId));
        emit EscrowAddLiquidity(msg.sender, to, token, amount);
    }

    // can be used for any pool, wont unwrap WETH
    function removeLiquidity(address to, address tokenA, address tokenB, uint amountA, uint amountB, bytes32 xtxId)
        external
        noDuplicateXtx(xtxId)
    {
        require(_collectToken(amountA, tokenA), "tokenA couldn't be collected!");
        require(_collectToken(amountB, tokenB), "tokenB couldn't be collected!");
        active[xtxId] = keccak256(abi.encodePacked(msg.sender, to, tokenA, amountA, tokenB, amountB, xtxId));
        emit EscrowRemoveLiquidity(msg.sender, to, tokenA, amountA, tokenB, amountB, xtxId);
    }
        
    // for swapping to a token
    function tokenSwap(address to, address token, uint amount, bytes32 xtxId)
        external
        noDuplicateXtx(xtxId)
    {
        _collectToken(amount, token);
        active[xtxId] = _hashTokenSwap(to, token, amount, msg.sender);
        emit EscrowSwap(msg.sender, to, amount, token);
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
        emit EscrowTransfer(msg.sender, to, msg.value);
    }

    // intializes escrowed token transfer
    function tokenTransfer(address to, address token, uint amount, bytes32 xtxId)
        external
        noDuplicateXtx(xtxId)
        returns (bool)
    {
        _collectToken(amount, token);
        active[xtxId] = _hashTokenTransfer(to, token, amount, msg.sender);
        emit EscrowMultiTransfer(msg.sender, to, amount, token);
    }

    function settleEthSwap(CircuitEvent memory evnt, address to, uint amount)
        external
    {
        // verify finality of CircuitEvent here. See `_verifyFinality()`

        // ensure the correct inputs where passed
        require(_hashEthSwap(to, amount, msg.sender) == active[evnt.xtxId], "False inputs passed");
        _settleEth(evnt, to, amount);
        delete active[evnt.xtxId]; // gas refund
    }

    function settleTokenSwap(CircuitEvent memory evnt, address to, address token, uint amount)
        external
    {
        // verify finality of CircuitEvent here. See `_verifyFinality()`

        // ensure the correct inputs where passed
        require(_hashTokenSwap(to, token, amount, msg.sender) == active[evnt.xtxId], "False inputs passed");
        _settleToken(evnt, to, amount, token);
        delete active[evnt.xtxId]; // gas refund
    }

    function settleEthTransfer(CircuitEvent memory evnt, address to, uint amount)
        external
    {
        // verify finality of CircuitEvent here. See `_verifyFinality()`

        // ensure the correct inputs where passed
        require(_hashEthTransfer(to, amount, msg.sender) == active[evnt.xtxId], "False inputs passed");
        _settleEth(evnt, to, amount);
        delete active[evnt.xtxId]; // gas refund
    }

    function settleTokenTransfer(CircuitEvent memory evnt, address to, address token, uint amount)
        external
    {
        // verify finality of CircuitEvent here. See `_verifyFinality()`

        // ensure the correct inputs where passed
        require(_hashTokenTransfer(to, token, amount, msg.sender) == active[evnt.xtxId], "False inputs passed");
        _settleToken(evnt, to, amount, token);
        delete active[evnt.xtxId]; // gas refund
    }

    function settleAddLiquidity(CircuitEvent memory evnt, address to, address token, uint amount)
        external
    {
        require(keccak256(abi.encodePacked(msg.sender, to, token, amount, evnt.xtxId)) == active[evnt.xtxId], "False inputs passed");
        _settleToken(evnt, to, amount, token);
        delete active[evnt.xtxId]; // gas refund
    }

    function settleRemoveLiquidity(CircuitEvent memory evnt, address to, address tokenA, address tokenB, uint amountA, uint amountB)
        external
    {
        require(keccak256(abi.encodePacked(msg.sender, to, tokenA, amountA, tokenB, amountB, evnt.xtxId)) == active[evnt.xtxId], "False inputs passed");
        _settleToken(evnt, to, amountA, tokenA);
        _settleToken(evnt, to, amountB, tokenB);
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

    // used for settling eth based transactions
    function _settleEth(CircuitEvent memory evnt, address to, uint amount)
        private
    {
        if(evnt.shouldCommit) {
            // we are commiting
            _sendEth(payable(to), amount);
            emit Commit(evnt.xtxId);
        } else {   
            // reverting
            _sendEth(payable(msg.sender), amount);
            emit Revert(evnt.xtxId);
        }
    }

    // used to settle token based transactions
    function _settleToken(CircuitEvent memory evnt, address to, uint amount, address token)
        private
    {
         if(evnt.shouldCommit) {
            _sendToken(to, token, amount);
            emit Commit(evnt.xtxId);
        } else {   
            _sendToken(payable(msg.sender), token, amount);
            emit Revert(evnt.xtxId);
        }
    }

    // TODO: hashing not dry, need to refactor!
    function _hashEthSwap(address to, uint amount, address executor)
        private
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(to, amount, executor));
    }

    function _hashTokenSwap(address to, address asset, uint amount, address executor)
        private
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(to, asset, amount, executor));
    }

    function _hashAddLiquidity(address to, address asset, uint amount, address executor)
        private
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(to, asset, amount, executor));
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
        returns (bool)
    {
        // escrow contract needs to be approved before
        // This is how erc20 tokens are sent to smart contracts -> Approve contract as spender, then transferFrom to contract address.
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        return true;
    }
}