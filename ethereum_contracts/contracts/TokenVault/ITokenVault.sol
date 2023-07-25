// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.16;

/// @title Token vault interface for all vaults
/// @notice Abstract interface that defines methods for custom vaults
interface ITokenVault {
    /**
     * @notice Deposit tokens
     * @dev When `depositor` deposits tokens, tokens get locked into vault contract.
     * @param depositor Address who is paying for the tokens. This can be a smart contract
     * @param depositReceiver Address of the user on ETH Chain. This can be different from depositor if depositor is a smart contract
     * @param depositCcdReceiver Address (address) who wants to receive tokens on the CCD chain
     * @param rootToken Token which gets deposited
     * @param depositData Extra data for deposit (amount for ERC20, token id for ERC721 etc.) [ABI encoded]
     */
    function lockTokens(
        address depositor,
        address depositReceiver,
        bytes32 depositCcdReceiver,
        address rootToken,
        bytes calldata depositData
    ) external;

    /**
     * @notice Withdraw tokens for a user after it has been validated by the RootChainManager
     * @dev Processes withdraw based on custom logic. Example: transfer ERC20/ERC721, mint ERC721 if mintable withdraw
     * @param userWallet Address who receives the tokens
     * @param rootToken Token on ETH chain that is withdrawn. Not used for Ether
     * @param tokenId tokenId for ERC721 / ERC1155. Not used for Ether / ERC20
     * @param amount amount to be withdawn.
     */
    function exitTokens(
        address payable userWallet,
        address rootToken,
        uint256 tokenId,
        uint256 amount
    ) external;
}
