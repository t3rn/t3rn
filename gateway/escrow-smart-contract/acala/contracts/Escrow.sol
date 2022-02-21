//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2; // allows us to pass structs as args
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract Escrow {

    // to save gas we hash the values
    mapping(bytes32 => bytes32) public active;

    modifier noDuplicateXtx(bytes32 xtxId) {
        require(active[xtxId] == 0, "Duplicate XTX ID");
        _;
    }

    event Execute(bytes32 xtxId, address executor, address to, address token, uint amount);
    event ExecuteRemoveLiquidity(bytes32 xtxId, address executor, address to, address tokenA, address tokenB, uint amountA, uint amountB);

    event Commit(bytes32 xtxId);
    event Revert(bytes32 xtxId);

    struct CircuitEvent {
        // assumption here is, that the circut will only emit a commit event, if the amounts during execute are correct
        bytes32 xtxId;
        bool shouldCommit;
        // will need more fields for inclusion proof
    }

    // intializes escrowed token transfer, token swap or liquidity provision. In all three cases contract is holding the token to receive until commit
    function execute(bytes32 xtxId, address to, address token, uint amount)
        external
        noDuplicateXtx(xtxId)
    {
        _collect(amount, token);
        active[xtxId] = keccak256(abi.encodePacked(xtxId, msg.sender, to, token, amount));
        emit Execute(xtxId, msg.sender, to, token, amount);
    }

    // settles token transaction (transfer, swap or addLiquidity)
    function settle(CircuitEvent memory evnt, address to, address token, uint amount)
        external
    {
        // verify finality of CircuitEvent here. See `_verifyFinality()`

        require(keccak256(abi.encodePacked(evnt.xtxId, msg.sender, to, token, amount)) == active[evnt.xtxId], "False inputs passed");
        
        if(evnt.shouldCommit) {
            _send(to, token, amount);
            emit Commit(evnt.xtxId);
        } else {   
            _send(payable(msg.sender), token, amount);
            emit Revert(evnt.xtxId);
        }
        
        delete active[evnt.xtxId]; // gas refund
    }

    // can be used for any pool, wont unwrap WETH though. Do we want a version that unwraps WETH?
    function executeRemoveLiquidity(bytes32 xtxId, address to, address tokenA, address tokenB, uint amountA, uint amountB)
        external
        noDuplicateXtx(xtxId)
    {
        require(_collect(amountA, tokenA), "tokenA couldn't be collected!");
        require(_collect(amountB, tokenB), "tokenB couldn't be collected!");
        active[xtxId] = keccak256(abi.encodePacked(xtxId, msg.sender, to, tokenA, tokenB, amountA, amountB));
        emit ExecuteRemoveLiquidity(xtxId, msg.sender, to, tokenA, tokenB, amountA, amountB);
    }

    function settleRemoveLiquidity(CircuitEvent memory evnt, address to, address tokenA, address tokenB, uint amountA, uint amountB)
        external
    {
        require(keccak256(abi.encodePacked(evnt.xtxId, msg.sender, to, tokenA, tokenB, amountA, amountB)) == active[evnt.xtxId], "False inputs passed");

        if(evnt.shouldCommit) {
            _send(to, tokenA, amountA);
            _send(to, tokenB, amountB);
            emit Commit(evnt.xtxId);
        } else {   
            _send(payable(msg.sender), tokenA, amountA);
            _send(payable(msg.sender), tokenB, amountB);
            emit Revert(evnt.xtxId);
        }
        
        delete active[evnt.xtxId]; // gas refund
    }

    function _send(address to, address token, uint amount)
        private
    {
        IERC20(token).transfer(to, amount);
    }

    function _collect(
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