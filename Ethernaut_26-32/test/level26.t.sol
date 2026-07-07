// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import "forge-std/console.sol";
import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

interface IDoubleEntryPoint {
    function cryptoVault() external view returns(address);
    function player() external view returns(address);
    function delegatedFrom() external view returns(address);
    function forta() external view returns(IForta);
    function balanceOf(address account) external view returns (uint256);
}

interface IForta {
    function usersDetectionBost(address user) external view returns(IDetectionBot);
}

interface IDetectionBot {
    function handleTransaction(address user, bytes calldata msgData) external;
}

interface DelegateERC20 {
    function delegateTransfer(address to, uint256 value, address origSender) external returns (bool);
}

interface ILegacyToken {
    function delegate() external view returns(DelegateERC20);
}

interface ICryptoVault {
    function sweptTokensRecipient() external view returns(address);
    function sweepToken(IERC20 token) external;
}

contract level26Test is Test {
    IDoubleEntryPoint det;

    address constant det_address = 0x046Fa87296A4B9dE9faF53028b510D9a3416CcAE;

    function setUp() public {
        det = IDoubleEntryPoint(det_address);
    }

    function test_print() public {
        ICryptoVault vault = ICryptoVault(det.cryptoVault());
        ILegacyToken legacy = ILegacyToken(det.delegatedFrom());
        DelegateERC20 legacy_dlg = legacy.delegate();
    
        console.log("[DET]");
        console.log("self = ", address(det));
        console.log("vault = ", address(vault));
        console.log("player = ", det.player());
        console.log("delegateFrom = ", address(legacy));
        console.log("forta = ", address(det.forta()));
        console.log("");

        console.log("DET(DET) balance = ", det.balanceOf(det_address));
        console.log("player(DET) balance = ", det.balanceOf(det.player()));
        console.log("vault(DET) balance = ", det.balanceOf(det.cryptoVault()));
        console.log("");

        console.log("[LegacyToken]");
        console.log("self = ", address(legacy));
        console.log("delegate = ", address(legacy_dlg));
        console.log();

        console.log("[Vault]");
        console.log("self = ", address(vault));
        console.log("recipient = ", vault.sweptTokensRecipient());

        vault.sweepToken(IERC20(address(legacy)));
    }
}

// Bot = 0xA670a9aE4EbC61d0802bbc827cD1f5D6A09bBC67
