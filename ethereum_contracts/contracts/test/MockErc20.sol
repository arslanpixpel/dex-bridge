// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.16;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockErc20 is ERC20 {
    constructor() ERC20("MOCK", "MO") {
        _decimals = 18;
    }

    uint8 private _decimals;

    function mint(address to, uint256 amount) public {
        _mint(to, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    function setDecimals(uint8 decimalsValue) public {
        _decimals = decimalsValue;
    }
}
