// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.9 .0;

// import "forge-std/console2.sol";
// import "forge-std/Script.sol";

// import {ProxyAdmin} from "../contracts/proxy/ProxyAdmin.sol";

// import {RootChainManager} from "../contracts/root/RootChainManager.sol";
// import {RootChainManagerProxy} from "../contracts/root/RootChainManagerProxy.sol";

// import {StateSender} from "../contracts/StateSender/StateSender.sol";
// import {StateSenderProxy} from "../contracts/StateSender/StateSenderProxy.sol";

// import {ERC20Vault} from "../contracts/TokenVault/ERC20Vault.sol";
// import {ERC20VaultProxy} from "../contracts/TokenVault/ERC20VaultProxy.sol";

// import {EtherVault} from "../contracts/TokenVault/EtherVault.sol";
// import {EtherVaultProxy} from "../contracts/TokenVault/EtherVaultProxy.sol";

// contract Deploy is Script {
//     function setUp() public {}

//     function run() public {
//         vm.startBroadcast();
//         logParams();

//         setupStateSender(stateSender, address(rootManger));
//         setupRootManager(ethVault, erc20Vault, rootManger, stateSender);
//         setupEthVault(ethVault, address(rootManger));
//         setupERC20Vault(erc20Vault, address(rootManger));

//         vm.stopBroadcast();
//     }

//     function setupStateSender(
//         StateSender stateSender,
//         address rootManagerAddress
//     ) public {
//         bytes32 role = stateSender.EMITTER_ROLE();
//         bool hasRole = stateSender.hasRole(role, rootManagerAddress);
//         if (!hasRole) {
//             console2.log(
//                 "Granting emitter role in ",
//                 address(stateSender),
//                 " to ",
//                 address(rootManagerAddress)
//             );
//             stateSender.grantRole(role, rootManagerAddress);
//         }
//     }

//     function setupRootManager(
//         EtherVault ethVault,
//         ERC20Vault erc20Vault,
//         RootChainManager rootManager,
//         StateSender stateSender
//     ) public {
//         bytes32 ethTokenType = ethVault.TOKEN_TYPE();
//         bytes32 erc20TokenType = erc20Vault.TOKEN_TYPE();

//         address ethVaultAddress = rootManager.typeToVault(ethTokenType);
//         address erc20VaultAddress = rootManager.typeToVault(erc20TokenType);

//         address stateSenderAddress = rootManager.stateSenderAddress();

//         if (stateSenderAddress != address(stateSender)) {
//             console2.log(
//                 "Wrong state sender address ",
//                 stateSenderAddress,
//                 " setting to ",
//                 address(stateSender)
//             );

//             rootManager.setStateSender(address(stateSender));
//         }

//         if (ethVaultAddress != address(ethVault)) {
//             console2.log(
//                 "Wrong ETH Vault address ",
//                 ethVaultAddress,
//                 " setting to ",
//                 address(ethVault)
//             );
//             rootManager.registerVault(ethTokenType, address(ethVault));
//         }

//         if (erc20VaultAddress != address(erc20Vault)) {
//             console2.log(
//                 "Wrong ERC20 Vault address ",
//                 erc20VaultAddress,
//                 " setting to ",
//                 address(erc20Vault)
//             );

//             rootManager.registerVault(erc20TokenType, address(erc20Vault));
//         }
//     }

//     function setupEthVault(
//         EtherVault ethVault,
//         address rootManagerAddress
//     ) public {
//         bytes32 role = ethVault.MANAGER_ROLE();
//         bool hasRole = ethVault.hasRole(role, rootManagerAddress);

//         if (!hasRole) {
//             console2.log("");
//             ethVault.grantRole(role, rootManagerAddress);
//         }
//     }

//     function setupERC20Vault(
//         ERC20Vault erc20Vault,
//         address rootManagerAddress
//     ) public {
//         bytes32 role = erc20Vault.MANAGER_ROLE();
//         bool hasRole = erc20Vault.hasRole(role, rootManagerAddress);

//         if (!hasRole) {
//             console2.log("");
//             erc20Vault.grantRole(role, rootManagerAddress);
//         }
//     }

//     function logParams() internal view {}
// }
