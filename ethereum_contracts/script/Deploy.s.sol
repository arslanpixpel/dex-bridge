// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9 .0;

import "forge-std/console2.sol";
import "forge-std/Script.sol";

import {ProxyAdmin} from "../contracts/proxy/ProxyAdmin.sol";

import {RootChainManager} from "../contracts/root/RootChainManager.sol";
import {RootChainManagerProxy} from "../contracts/root/RootChainManagerProxy.sol";

import {StateSender} from "../contracts/StateSender/StateSender.sol";
import {StateSenderProxy} from "../contracts/StateSender/StateSenderProxy.sol";

import {ERC20Vault} from "../contracts/TokenVault/ERC20Vault.sol";
import {ERC20VaultProxy} from "../contracts/TokenVault/ERC20VaultProxy.sol";

import {EtherVault} from "../contracts/TokenVault/EtherVault.sol";
import {EtherVaultProxy} from "../contracts/TokenVault/EtherVaultProxy.sol";

contract Deploy is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();
        logParams();

        ProxyAdmin admin = new ProxyAdmin();

        RootChainManager rootManager = deployRootChainManager(address(admin));

        EtherVault ethVault = deployEtherVault(address(admin));

        ERC20Vault erc20Vault = deployERC20Vault(address(admin));

        StateSender stateSender = deployStateSender(address(admin));

        // setupStateSender(stateSender, address(rootManager));
        // setupRootManager(ethVault, erc20Vault, rootManager, stateSender);
        // setupEthVault(ethVault, address(rootManager));
        // setupERC20Vault(erc20Vault, address(rootManager));

        console2.log("RootChainManager ", address(rootManager));
        console2.log("EtherVault ", address(ethVault));
        console2.log("ERC20Vault ", address(erc20Vault));
        console2.log("StateSender ", address(stateSender));

        vm.stopBroadcast();
    }

    function deployRootChainManager(
        address admin
    ) public returns (RootChainManager impl) {
        impl = new RootChainManager();

        bytes memory data = abi.encodeWithSelector(
            impl.initialize.selector,
            msg.sender // Our owner eg wallet of deployer
        );

        RootChainManagerProxy proxy = new RootChainManagerProxy(
            address(impl),
            address(admin),
            data
        );

        impl = RootChainManager(payable(address(proxy)));
    }

    function deployEtherVault(address admin) public returns (EtherVault impl) {
        impl = new EtherVault();

        bytes memory data = abi.encodeWithSelector(
            impl.initialize.selector,
            msg.sender // Our owner eg wallet of deployer
        );

        EtherVaultProxy proxy = new EtherVaultProxy(
            address(impl),
            address(admin),
            data
        );

        impl = EtherVault(payable(address(proxy)));
    }

    function deployERC20Vault(address admin) public returns (ERC20Vault impl) {
        impl = new ERC20Vault();

        bytes memory data = abi.encodeWithSelector(
            impl.initialize.selector,
            msg.sender // Our owner eg wallet of deployer
        );

        ERC20VaultProxy proxy = new ERC20VaultProxy(
            address(impl),
            address(admin),
            data
        );

        impl = ERC20Vault(payable(address(proxy)));
    }

    function deployStateSender(
        address admin
    ) public returns (StateSender impl) {
        impl = new StateSender();

        bytes memory data = abi.encodeWithSelector(
            impl.initialize.selector,
            msg.sender // Our owner eg wallet of deployer
        );

        StateSenderProxy proxy = new StateSenderProxy(
            address(impl),
            address(admin),
            data
        );

        impl = StateSender(payable(address(proxy)));
    }

    function setupStateSender(
        StateSender stateSender,
        address rootManagerAddress
    ) public {
        bytes32 role = stateSender.EMITTER_ROLE();
        bool hasRole = stateSender.hasRole(role, rootManagerAddress);
        if (!hasRole) {
            console2.log(
                "Granting emitter role in ",
                address(stateSender),
                " to ",
                address(rootManagerAddress)
            );
            stateSender.grantRole(role, rootManagerAddress);
        }
    }

    function setupRootManager(
        EtherVault ethVault,
        ERC20Vault erc20Vault,
        RootChainManager rootManager,
        StateSender stateSender
    ) public {
        bytes32 ethTokenType = ethVault.TOKEN_TYPE();
        bytes32 erc20TokenType = erc20Vault.TOKEN_TYPE();

        address ethVaultAddress = rootManager.typeToVault(ethTokenType);
        address erc20VaultAddress = rootManager.typeToVault(erc20TokenType);

        address stateSenderAddress = rootManager.stateSenderAddress();

        if (stateSenderAddress != address(stateSender)) {
            console2.log(
                "Wrong state sender address ",
                stateSenderAddress,
                " setting to ",
                address(stateSender)
            );

            rootManager.setStateSender(address(stateSender));
        }

        if (ethVaultAddress != address(ethVault)) {
            console2.log(
                "Wrong ETH Vault address ",
                ethVaultAddress,
                " setting to ",
                address(ethVault)
            );
            rootManager.registerVault(ethTokenType, address(ethVault));
        }

        if (erc20VaultAddress != address(erc20Vault)) {
            console2.log(
                "Wrong ERC20 Vault address ",
                erc20VaultAddress,
                " setting to ",
                address(erc20Vault)
            );

            rootManager.registerVault(erc20TokenType, address(erc20Vault));
        }
    }

    function setupEthVault(
        EtherVault ethVault,
        address rootManagerAddress
    ) public {
        bytes32 role = ethVault.MANAGER_ROLE();
        bool hasRole = ethVault.hasRole(role, rootManagerAddress);

        if (!hasRole) {
            console2.log("");
            ethVault.grantRole(role, rootManagerAddress);
        }
    }

    function setupERC20Vault(
        ERC20Vault erc20Vault,
        address rootManagerAddress
    ) public {
        bytes32 role = erc20Vault.MANAGER_ROLE();
        bool hasRole = erc20Vault.hasRole(role, rootManagerAddress);

        if (!hasRole) {
            console2.log("");
            erc20Vault.grantRole(role, rootManagerAddress);
        }
    }

    function logParams() internal view {}
}
