// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.16;

import {IRootChainManager} from "./IRootChainManager.sol";
import {IStateSender} from "../StateSender/IStateSender.sol";
import {RootChainManagerStorage} from "./RootChainManagerStorage.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import {ITokenVault} from "../TokenVault/ITokenVault.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract RootChainManager is
    Initializable,
    IRootChainManager,
    RootChainManagerStorage,
    AccessControl
{
    /**
     * @notice Disable direct eth transfers
     */
    receive() external payable {
        require(false, "RootChainManager: no direct ETH deposits");
    }

    function initialize(address _owner) external initializer {
        _setupRole(DEFAULT_ADMIN_ROLE, _owner);
        _setupRole(MAPPER_ROLE, _owner);
        treasurer = payable(_owner);
        paused = false;
    }

    modifier isNotPaused() {
        require(!paused, "RootChainManager: bridge is paused");
        _;
    }

    function setPaused(bool _paused) external onlyRole(DEFAULT_ADMIN_ROLE) {
        paused = _paused;
    }

    function setTreasurer(address payable newTreasurer)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        require(
            newTreasurer != address(0),
            "RootChainManager: treasurer can not be address(0)"
        );
        treasurer = newTreasurer;
    }

    function setDepositFee(uint256 newDepositFee)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        depositFee = newDepositFee;
    }

    function setWithdrawFee(uint256 newWithdrawFee)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        withdrawFee = newWithdrawFee;
    }

    /**
     * @notice Set the state sender, callable only by admins
     * @param newStateSender address of state sender contract
     */
    function setStateSender(address newStateSender)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        require(
            newStateSender != address(0),
            "RootChainManager: stateSender can not be address(0)"
        );
        _stateSender = IStateSender(newStateSender);
    }

    /**
     * @notice Get the address of contract set as state sender
     * @return The address of state sender contract
     */
    function stateSenderAddress() external view returns (address) {
        return address(_stateSender);
    }

    function setMerkleRoot(bytes32 _merkleRoot)
        external
        onlyRole(MERKLE_UPDATER)
    {
        previousMerkleRoot = merkleRoot;
        merkleRoot = _merkleRoot;
        _stateSender.emitMerkleRoot(_merkleRoot);
    }

    function getMerkleRoot() external view returns (bytes32) {
        return merkleRoot;
    }

    /**
     * @notice Register a token vault address against its type, callable only by ADMIN
     * @dev A vault is a contract responsible to process the token specific logic while locking or exiting tokens
     * @param tokenType bytes32 unique identifier for the token type
     * @param vaultAddress address of token vault address
     */
    function registerVault(bytes32 tokenType, address vaultAddress)
        external
        override
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        typeToVault[tokenType] = vaultAddress;
        _stateSender.emitVaultRegistered(tokenType, vaultAddress);
    }

    /**
     * @notice Map a token to enable it on the bridge, callable only by mappers
     * @param rootToken address of token on root chain
     * @param childTokenIndex address of token on child chain
     * @param childTokenSubIndex address of token on child chain
     * @param tokenType bytes32 unique identifier for the token type
     */
    function mapToken(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) external override onlyRole(MAPPER_ROLE) {
        bytes32 childToken = hashChild(childTokenIndex, childTokenSubIndex);
        // explicit check if token is already mapped to avoid accidental remaps
        require(
            rootToChildToken[rootToken].index == 0 &&
                childToRootToken[childToken] == address(0),
            "RootChainManager: already mapped"
        );
        _mapToken(
            rootToken,
            childToken,
            childTokenIndex,
            childTokenSubIndex,
            tokenType
        );
    }

    function hashChild(uint64 childTokenIndex, uint64 childTokenSubIndex)
        public
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(childTokenIndex, childTokenSubIndex));
    }

    /**
     * @notice Remove a token mapping.
     * @param rootToken address of token on root chain.
     * @param childTokenIndex index of the child contract on the CCD chain
     * @param childTokenSubIndex subindex of the child contract on the CCD chain
     */
    function cleanMapToken(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex
    ) external override onlyRole(DEFAULT_ADMIN_ROLE) {
        bytes32 childToken = hashChild(childTokenIndex, childTokenSubIndex);
        rootToChildToken[rootToken] = CCDAddress(0, 0);
        childToRootToken[childToken] = address(0);
        tokenToType[rootToken] = bytes32(0);
        _stateSender.emitTokenMapRemove(
            rootToken,
            childTokenIndex,
            childTokenSubIndex,
            tokenToType[rootToken]
        );
    }

    /**
     * @notice Remap a token that has already been mapped, properly cleans up old mapping
     * Callable only by ADMIN
     * @param rootToken address of token on root chain
     * @param childTokenIndex address of token on child chain
     * @param childTokenSubIndex address of token on child chain
     * @param tokenType bytes32 unique identifier for the token type
     */
    function remapToken(
        address rootToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) external override onlyRole(DEFAULT_ADMIN_ROLE) {
        bytes32 childToken = hashChild(childTokenIndex, childTokenSubIndex);

        // cleanup old mapping
        CCDAddress memory oldChildTokenAddress = rootToChildToken[rootToken];
        bytes32 oldChildToken = hashChild(
            oldChildTokenAddress.index,
            oldChildTokenAddress.subindex
        );
        address oldRootToken = childToRootToken[childToken];
        if (
            rootToChildToken[oldRootToken].index != 0 ||
            rootToChildToken[oldRootToken].subindex != 0
        ) {
            rootToChildToken[oldRootToken] = CCDAddress(0, 0);
            tokenToType[oldRootToken] = bytes32(0);
        }
        if (childToRootToken[oldChildToken] != address(0)) {
            childToRootToken[oldChildToken] = address(0);
        }
        _mapToken(
            rootToken,
            childToken,
            childTokenIndex,
            childTokenSubIndex,
            tokenType
        );
    }

    function _mapToken(
        address rootToken,
        bytes32 childToken,
        uint64 childTokenIndex,
        uint64 childTokenSubIndex,
        bytes32 tokenType
    ) private {
        require(
            typeToVault[tokenType] != address(0x0),
            "RootChainManager: not supported token type"
        );
        rootToChildToken[rootToken] = CCDAddress({
            index: childTokenIndex,
            subindex: childTokenSubIndex
        });
        childToRootToken[childToken] = rootToken;
        tokenToType[rootToken] = tokenType;

        _stateSender.emitTokenMapAdd(
            rootToken,
            childTokenIndex,
            childTokenSubIndex,
            tokenType
        );
    }

    /**
     * @notice Move ether from root to child chain, accepts ether transfer
     * @param user address of account that should receive ETH on child chain
     * @param ccdUser The 32 bytes address of the ccdUser, without b58checksum. Sample conversion snippet:
        '0x' + bs58check.decode('43A45q6ohC5tsZ9nTZ7T5EniADtzgFBBYPE9A41Mjeq1vz2hev').slice(1).toString('hex')
     */
    function depositEtherFor(address user, bytes32 ccdUser)
        external
        payable
        override
        isNotPaused
    {
        require(
            msg.value >= depositFee,
            "RootChainManager: ETH send needs to be at least depositFee"
        );
        if (depositFee > 0) {
            _sendFee(depositFee);
        }
        _depositEtherFor(user, ccdUser, msg.value - depositFee);
    }

    /**
     * @notice Move tokens from root to child chain
     * @dev This mechanism supports arbitrary tokens as long as its vault has been registered and the token is mapped
     * @param user address of account that should receive this deposit on child chain
     * @param rootToken address of token that is being deposited
     * @param ccdUser The 32 bytes address of the ccdUser, without b58checksum. Sample conversion snippet:
         '0x' + bs58check.decode('43A45q6ohC5tsZ9nTZ7T5EniADtzgFBBYPE9A41Mjeq1vz2hev').slice(1).toString('hex')
     * @param depositData bytes data that is sent to vault and child token contracts to handle deposit
     */
    function depositFor(
        address user,
        bytes32 ccdUser,
        address rootToken,
        bytes calldata depositData
    ) external payable override isNotPaused {
        require(
            rootToken != ETHER_ADDRESS,
            "RootChainManager: invalid root token"
        );
        require(msg.value >= depositFee, "RootChainManager: ETH send needs to be at least depositFee");
        if (depositFee > 0) {
            _sendFee(depositFee);
        }
        _depositFor(user, ccdUser, rootToken, depositData);
    }

    function _sendFee(uint256 amount) private {
        treasurer.transfer(amount);
    }

    function _depositEtherFor(
        address user,
        bytes32 ccdUser,
        uint256 amount
    ) private {
        bytes memory depositData = abi.encode(amount);
        _depositFor(user, ccdUser, ETHER_ADDRESS, depositData);
        payable(typeToVault[tokenToType[ETHER_ADDRESS]]).transfer(amount);
    }

    function _depositFor(
        address user,
        bytes32 ccdUser,
        address rootToken,
        bytes memory depositData
    ) private {
        bytes32 tokenType = tokenToType[rootToken];
        require(
            rootToChildToken[rootToken].index != 0 && tokenType != 0,
            "RootChainManager: token not mapped"
        );
        address vaultAddress = typeToVault[tokenType];
        require(
            vaultAddress != address(0),
            "RootChainManager: invalid token type"
        );
        require(user != address(0), "RootChainManager: invalid user");
        ITokenVault(vaultAddress).lockTokens(
            _msgSender(),
            user,
            ccdUser,
            rootToken,
            depositData
        );
        _stateSender.emitDeposit(
            user,
            ccdUser,
            rootToken,
            vaultAddress,
            depositData
        );
    }

    /**
     * @notice exit tokens by providing proof
     * @dev This function verifies if the transaction actually happened on child chain
     */
    function withdraw(
        WithdrawParams calldata withdrawParam,
        bytes32[] calldata proof
    ) external payable override isNotPaused {
        require(msg.value >= withdrawFee, "RootChainManager: ETH send needs to be at least withdrawFee");
        if (withdrawFee > 0) {
            _sendFee(withdrawFee);
        }

        bytes32 exitHash = keccak256(
            abi.encode(
                withdrawParam.ccdIndex,
                withdrawParam.ccdSubIndex,
                withdrawParam.amount,
                withdrawParam.userWallet,
                withdrawParam.ccdTxHash,
                withdrawParam.ccdEventIndex,
                withdrawParam.tokenId
            )
        );
        require(
            processedExits[exitHash] == false,
            "RootChainManager: exit already processed"
        );

        require(
            MerkleProof.verify(proof, merkleRoot, exitHash) ||
                MerkleProof.verify(proof, previousMerkleRoot, exitHash),
            "RootChainManager: transaction proof verification failed"
        );
        processedExits[exitHash] = true;
        // log should be emmited only by the child token
        bytes32 childKey = hashChild(
            withdrawParam.ccdIndex,
            withdrawParam.ccdSubIndex
        );
        address rootToken = childToRootToken[childKey];
        require(rootToken != address(0), "RootChainManager: token not mapped");
        address vaultAddress = typeToVault[tokenToType[rootToken]];
        ITokenVault(vaultAddress).exitTokens(
            withdrawParam.userWallet,
            rootToken,
            withdrawParam.tokenId,
            withdrawParam.amount
        );
        _stateSender.emitWithdraw(
            withdrawParam.ccdIndex,
            withdrawParam.ccdSubIndex,
            withdrawParam.amount,
            withdrawParam.userWallet,
            withdrawParam.ccdTxHash,
            withdrawParam.ccdEventIndex,
            withdrawParam.tokenId
        );
    }
}
