// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.16;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract MockStateSenderUpgrade is Initializable, AccessControl {
    uint256 private id;
    bytes32 public constant EMITTER_ROLE = keccak256("EMITTER_ROLE");

    event TestEvent(uint256 id, uint64 payload);

    function emitTest(uint64 payload) external onlyRole(EMITTER_ROLE) {
        id = id + 1;
        emit TestEvent(id, payload);
    }
}
