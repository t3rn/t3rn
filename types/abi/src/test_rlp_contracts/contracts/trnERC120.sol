// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract TRN is ERC20, Ownable {
    address private _minterAddress;

    constructor(uint256 initialSupply) ERC20("t3rn", "TRN") {
        _mint(msg.sender, initialSupply);
    }

    function mint(address to, uint256 amount) public {
        require(msg.sender == owner() || msg.sender == _minterAddress, "TRN: must have minter role to mint");
        _mint(to, amount);
    }

    function setMinter(address minter) public onlyOwner {
        _minterAddress = minter;
    }

    function burn(uint256 amount) public {
        _burn(msg.sender, amount);
    }
}