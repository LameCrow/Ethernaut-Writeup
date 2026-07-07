// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import "forge-std/console2.sol";

interface IStake {
    function totalStaked() external view returns (uint256);
    function UserStake(address) external view returns (uint256);
    function Stakers(address) external view returns (bool);
    function WETH() external view returns (address);

    function StakeETH() external payable;
    function StakeWETH(uint256 amount) external returns (bool);
    function Unstake(uint256 amount) external returns (bool);
}

interface IERC20 {
    function balanceOf(address account) external view returns (uint256);
    function allowance(address owner, address spender) external view returns (uint256);

    // function deposit() external payable;
    function approve(address spender, uint256 value) external returns (bool);
}

contract L31_StakeScript is Script {
    address target = 0xa73A13e4EF597D6D2E78083D754fB65d01Eb942b;
    address token = 0xCd8AF4A0F29cF7966C051542905F66F5dca9052f;

    function run() external {
        uint256 alice_key = vm.envUint("alice_key");
        uint256 bob_key = vm.envUint("bob_key");
        address alice = vm.envAddress("alice");
        address bob = vm.envAddress("bob");

        vm.startBroadcast(alice_key);

        IStake stake = IStake(target);
        IERC20 weth = IERC20(token);

        console2.log("[Stake]");
        console2.log("    balance = ", target.balance);
        console2.log("    total Stake = ", stake.totalStaked());
        console2.log("    Alice Stake = ", stake.UserStake(alice));
        console2.log("    Alice Stakers = ", stake.Stakers(alice));
        console2.log("    WETH = ", stake.WETH());

        console2.log("[WETH]");
        console2.log("    Alice balance = ", weth.balanceOf(alice));
        console2.log("    Stake balance = ", weth.balanceOf(target));
        console2.log("    Alice allowance = ", weth.allowance(alice, target));

        stake.StakeETH{value: 0.0011 ether}();

        stake.Unstake(0.0011 ether);

        vm.stopBroadcast();

        vm.startBroadcast(bob_key);

        stake.StakeETH{value: 0.0011 ether}();
        weth.approve(target, 0.0011 ether);
        stake.StakeWETH(0.0011 ether);

        console2.log("[Final]");
        console2.log("    balance = ", target.balance);
        console2.log("    totalStaked = ", stake.totalStaked());
        console2.log("    Alice Stakers = ", stake.Stakers(alice));
        console2.log("    Alice Stake = ", stake.UserStake(alice));

        vm.stopBroadcast();
        
    }
}
