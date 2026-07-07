// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IDetectionBot {
    function handleTransaction(address user, bytes calldata msgData) external;
}

interface IForta {
    function raiseAlert(address user) external;
}

contract Bot is IDetectionBot {
    address vault = 0x3a999B4D2D506444c534f7224902867c35955a33;
    IForta forta = IForta(0x7F029e42ecdaB670891C34c975888bB1F8C5C3D0);

    function handleTransaction(address user, bytes calldata msgData) external override {
        (
            ,
            ,
            address origSender
        ) = abi.decode(msgData[4:], (address, uint256, address));

        if (origSender == vault) {
            forta.raiseAlert(user);
        }
    }
}
