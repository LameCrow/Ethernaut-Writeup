// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "forge-std/console2.sol";

interface IForta {
    function setDetectionBot(address detectionBotAddress) external;
    function usersDetectionBots(address user) external view returns (address);
}

interface ICryptoVault {
    function sweepToken(IERC20 token) external;
}


contract BosScript is Script {
    IForta forta = IForta(0x7F029e42ecdaB670891C34c975888bB1F8C5C3D0);
    address bot = 0xA670a9aE4EbC61d0802bbc827cD1f5D6A09bBC67;
    
    ICryptoVault vault = ICryptoVault(0x3a999B4D2D506444c534f7224902867c35955a33);
    address legacy = 0xa1bd9CC5c6995e87aE8B1BD03c588aC20e7D1DDD;
    
    function run() external {
        address alice = vm.envAddress("alice");
        uint256 alice_key = vm.envUint("alice_key");

        vm.startBroadcast(alice_key);

        console2.log("Bot = ", forta.usersDetectionBots(alice));

        forta.setDetectionBot(bot);

        console2.log("Bot = ", forta.usersDetectionBots(alice));
        // console2.log("");

        // vault.sweepToken(IERC20(legacy));

        vm.stopBroadcast();
    }
}
