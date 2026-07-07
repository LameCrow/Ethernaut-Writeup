// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import "forge-std/console2.sol";

interface IGoodSamaritan {
    function wallet() external view returns (address);
    function coin() external view returns (address);

}

interface ICoin {
    function balances(address account) external view returns (uint256);
}


interface INotifyable {
    function notify(uint256 amount) external;
    function run() external;
}

contract L27_SamaritanScript is Script {
    function run() external {
        uint256 alice_key = vm.envUint("alice_key");

        IGoodSamaritan samaritan = IGoodSamaritan(0xe9b2Df07C3B13C4fc262a0cdC3cdf9d983CE2846);

        INotifyable notify = INotifyable(0x8f5eD49B5fD6469D7496F760696507ad3F121b9e);

        // #####
        vm.startBroadcast(alice_key);

        address wallet = samaritan.wallet();
        ICoin coin = ICoin(samaritan.coin());

        console2.log("[GoodSamaritan]");
        console2.log("wallet = ", wallet);
        console2.log("coin = ", address(coin));
        console2.log("");

        console2.log("[Coin]");
        console2.log("samaritan's balance = ", coin.balances(address(samaritan)));
        console2.log("wallet's balance = ", coin.balances(wallet));
        console2.log("notify's balance = ", coin.balances(address(notify)));
        console2.log("");

        notify.run();

        console2.log("[Coin]");
        console2.log("samaritan's balance = ", coin.balances(address(samaritan)));
        console2.log("wallet's balance = ", coin.balances(wallet));
        console2.log("notify's balance = ", coin.balances(address(notify)));
        console2.log("");

        // #####
        vm.stopBroadcast();
    }
}

