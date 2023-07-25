// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.16;

interface IRootChainManager {
    function registerVault(bytes32 tokenType, address vaultAddress) external;

    function mapToken(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) external;

    function cleanMapToken(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex
    ) external;

    function remapToken(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) external;

    function depositEtherFor(address user, bytes32 ccdAddress) external payable;

    function depositFor(
        address user,
        bytes32 ccdAddress,
        address rootToken,
        bytes calldata depositData
    ) external payable;

    struct WithdrawParams {
        uint64 ccdIndex;
        uint64 ccdSubIndex;
        uint256 amount;
        address payable userWallet;
        bytes32 ccdTxHash;
        uint64 ccdEventIndex;
        uint64 tokenId;
    }

    function withdraw(
        WithdrawParams calldata withdrawParams,
        bytes32[] calldata proof
    ) external payable;
}
