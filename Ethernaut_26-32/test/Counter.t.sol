// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Counter} from "../src/Counter.sol";
import "forge-std/console.sol";

contract CounterTest is Test {
    Counter public counter;

    function setUp() public {
        counter = new Counter();
        counter.setNumber(0);
    }

    function test_print() public {
        uint256 x = 123;
    
        console.log("x = ", x);
        console.log("before = ", counter.number());

        counter.setNumber(x);

        console.log("after = ", counter.number());
    
        counter.increment();
    }

}
