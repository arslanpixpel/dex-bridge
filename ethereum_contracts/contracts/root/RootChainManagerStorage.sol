// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.16;

import {IStateSender} from "../StateSender/IStateSender.sol";

abstract contract RootChainManagerStorage {
    struct CCDAddress {
        uint64 index;
        uint64 subindex;
    }

    address public constant ETHER_ADDRESS =
        0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE;

    bytes32 public constant MAPPER_ROLE = keccak256("MAPPER_ROLE");
    bytes32 public constant MERKLE_UPDATER = keccak256("MERKLE_UPDATER");

    mapping(bytes32 => address) public typeToVault;
    mapping(address => CCDAddress) public rootToChildToken;
    mapping(bytes32 => address) public childToRootToken;
    mapping(address => bytes32) public tokenToType;
    mapping(bytes32 => bool) public processedExits;
    IStateSender internal _stateSender;

    bytes32 public merkleRoot;
    bytes32 public previousMerkleRoot;

    address payable public treasurer;
    uint256 public depositFee;
    uint256 public withdrawFee;
    bool public paused;
}
