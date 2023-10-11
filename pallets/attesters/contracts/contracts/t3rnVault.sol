pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";


interface It3rnVault {

    function deposit(address asset, uint256 amount) external payable;

    function withdraw(address asset, uint256 amount, address to) external payable;
}

contract t3rnVault {
    using SafeERC20 for IERC20;

    address public escrow; // Only this address can withdraw
    address public owner; // Only this address can withdraw

    modifier onlyOwner() {
        require(msg.sender == owner, "Only Owner can call this function");
        _;
    }

    function assignEscrow(address _escrow) public onlyOwner {
        escrow = _escrow;
    }

    modifier onlyEscrow() {
        require(msg.sender == escrow, "Only Escrow can call this function");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function deposit(address asset, uint256 amount) external payable {
        if (asset == address(0)) {
            require(msg.value == amount, "Mismatched deposit amount");
        } else {
            IERC20(asset).safeTransferFrom(msg.sender, address(this), amount);
        }
    }

    function withdraw(address asset, uint256 amount, address to) external payable onlyEscrow {
        if (asset == address(0)) {
            payable(to).transfer(amount);
        } else {
            IERC20(asset).safeTransfer(to, amount);
        }
    }
}