// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.16;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ITokenVault} from "./ITokenVault.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract ERC20Vault is Initializable, ITokenVault, AccessControl {
    using SafeERC20 for IERC20;

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
    bytes32 public constant TOKEN_TYPE = keccak256("ERC20");

    event LockedERC20(
        address indexed depositor,
        address indexed depositReceiver,
        bytes32 depositCcdReceiver,
        address indexed rootToken,
        uint256 amount
    );

    event ExitedERC20(
        address indexed exitor,
        address indexed rootToken,
        uint256 amount
    );

    function initialize(address _owner) external initializer {
        _setupRole(DEFAULT_ADMIN_ROLE, _owner);
        _setupRole(MANAGER_ROLE, _owner);
    }

    /**
     * @notice Lock ERC20 tokens for deposit, callable only by manager
     * @param depositor Address who is paying for the tokens. This can be a smart contract
     * @param depositReceiver Address of the user on ETH Chain. This can be different from depositor if depositor is a smart contract
     * @param depositCcdReceiver Address (address) who wants to receive tokens on the CCD chain
     * @param rootToken Token which gets deposited
     * @param depositData ABI encoded amount
     */
    function lockTokens(
        address depositor,
        address depositReceiver,
        bytes32 depositCcdReceiver,
        address rootToken,
        bytes calldata depositData
    ) external override onlyRole(MANAGER_ROLE) {
        uint256 amount = abi.decode(depositData, (uint256));
        emit LockedERC20(
            depositor,
            depositReceiver,
            depositCcdReceiver,
            rootToken,
            amount
        );
        IERC20(rootToken).safeTransferFrom(depositor, address(this), amount);
    }

    /**
     * @notice Sends the correct amount to withdrawer
     * callable only by manager
     * @param userWallet Wallet where the tokens will be send to
     * @param rootToken Token which gets withdrawn
     * @param amount. Amount in u256.
     */
    function exitTokens(
        address payable userWallet,
        address rootToken,
        uint256,
        uint256 amount
    ) external override onlyRole(MANAGER_ROLE) {
        IERC20(rootToken).safeTransfer(userWallet, amount);
        emit ExitedERC20(userWallet, rootToken, amount);
    }
}
