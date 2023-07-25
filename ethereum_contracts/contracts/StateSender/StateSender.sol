// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.16;
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IStateSender} from "./IStateSender.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract StateSender is Initializable, IStateSender, AccessControl {
    uint256 private id;
    bytes32 public constant EMITTER_ROLE = keccak256("EMITTER_ROLE");

    function initialize(address _owner) external initializer {
        id = 0;
        _setupRole(DEFAULT_ADMIN_ROLE, _owner);
    }

    function emitTokenMapAdd(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) external onlyRole(EMITTER_ROLE) {
        id = id + 1;
        emit TokenMapAdded(
            id,
            rootToken,
            childTokenIndex,
            childTokenSubIndex,
            tokenType
        );
    }

    function emitTokenMapRemove(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) external onlyRole(EMITTER_ROLE) {
        id = id + 1;

        emit TokenMapRemoved(
            id,
            rootToken,
            childTokenIndex,
            childTokenSubIndex,
            tokenType
        );
    }

    function emitDeposit(
        address user,
        bytes32 userCcd,
        address rootToken,
        address vault,
        bytes memory depositData
    ) external onlyRole(EMITTER_ROLE) {
        id = id + 1;

        emit LockedToken(id, user, userCcd, rootToken, vault, depositData);
    }

    function emitVaultRegistered(bytes32 tokenType, address vaultAddress)
        external
        onlyRole(EMITTER_ROLE)
    {
        id = id + 1;
        emit VaultRegistered(id, tokenType, vaultAddress);
    }

    function emitWithdraw(
        uint64 ccdIndex,
        uint64 ccdSubIndex,
        uint256 amount,
        address userWallet,
        bytes32 ccdTxHash,
        uint64 ccdEventIndex,
        uint64 tokenId
    ) external onlyRole(EMITTER_ROLE) {
        id = id + 1;
        emit WithdrawEvent(
            id,
            ccdIndex,
            ccdSubIndex,
            amount,
            userWallet,
            ccdTxHash,
            ccdEventIndex,
            tokenId
        );
    }

    function emitMerkleRoot(bytes32 merkleRoot)
        external
        onlyRole(EMITTER_ROLE)
    {
        id = id + 1;
        emit MerkleRoot(id, merkleRoot);
    }
}
