// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.16;

import {ITokenVault} from "./ITokenVault.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract EtherVault is Initializable, ITokenVault, AccessControl {
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
    bytes32 public constant TOKEN_TYPE = keccak256("Ether");

    event LockedEther(
        address indexed depositor,
        address indexed depositReceiver,
        bytes32 depositCcdReceiver,
        uint256 amount
    );

    event ExitedEther(address indexed exitor, uint256 amount);

    function initialize(address _owner) external initializer {
        _setupRole(DEFAULT_ADMIN_ROLE, _owner);
        _setupRole(MANAGER_ROLE, _owner);
    }

    /**
     * @notice Receive Ether to lock for deposit, callable only by manager
     */
    // solhint-disable-next-line no-empty-blocks
    receive() external payable onlyRole(MANAGER_ROLE) {}

    /**
     * @notice handle ether lock, callable only by manager
     * @param depositor Address who is paying for the tokens. This can be a smart contract
     * @param depositReceiver Address of the user on ETH Chain. This can be different from depositor if depositor is a smart contract
     * @param depositCcdReceiver Address (address) who wants to receive tokens on the CCD chain
     * @param depositData ABI encoded amount
     */

    function lockTokens(
        address depositor,
        address depositReceiver,
        bytes32 depositCcdReceiver,
        address,
        bytes calldata depositData
    ) external override onlyRole(MANAGER_ROLE) {
        uint256 amount = abi.decode(depositData, (uint256));
        emit LockedEther(
            depositor,
            depositReceiver,
            depositCcdReceiver,
            amount
        );
    }

    /**
     * @notice Sends the correct amount to withdrawer
     * callable only by manager
     * @param userWallet Wallet where the tokens will be send to
     * @param amount. Amount in u256
     */
    function exitTokens(
        address payable userWallet,
        address,
        uint256,
        uint256 amount
    ) external override onlyRole(MANAGER_ROLE) {
        userWallet.transfer(amount);
        emit ExitedEther(userWallet, amount);
    }
}
