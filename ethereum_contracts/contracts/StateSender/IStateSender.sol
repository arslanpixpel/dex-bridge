// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.16;

interface IStateSender {
    event LockedToken(
        uint256 id,
        address indexed depositor,
        bytes32 depositReceiver,
        address indexed rootToken,
        address indexed vault,
        bytes depositData
    );

    event TokenMapAdded(
        uint256 id,
        address indexed rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 indexed tokenType
    );
    event TokenMapRemoved(
        uint256 id,
        address indexed rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 indexed tokenType
    );
    event VaultRegistered(
        uint256 id,
        bytes32 indexed tokenType,
        address indexed vaultAddress
    );
    event WithdrawEvent(
        uint256 id,
        uint64 indexed ccdIndex,
        uint64 indexed ccdSubIndex,
        uint256 amount,
        address indexed userWallet,
        bytes32 ccdTxHash,
        uint64 ccdEventIndex,
        uint64 tokenId
    );
    event MerkleRoot(uint256 id, bytes32 root);

    function emitTokenMapAdd(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) external;

    function emitTokenMapRemove(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) external;

    function emitDeposit(
        address user,
        bytes32 userCCd,
        address rootToken,
        address vault,
        bytes memory depositData
    ) external;

    function emitVaultRegistered(bytes32 tokenType, address vaultAddress)
        external;

    function emitWithdraw(
        uint64 ccdIndex,
        uint64 ccdSubIndex,
        uint256 amount,
        address userWallet,
        bytes32 ccdTxHash,
        uint64 ccdEventIndex,
        uint64 tokenId
    ) external;

    function emitMerkleRoot(bytes32 root) external;
}
