import { ethers, run, web3, network } from "hardhat";
import {
  BridgeProxyAdmin,
  BridgeProxyAdmin__factory,
  ERC20Vault,
  ERC20VaultProxy__factory,
  ERC20Vault__factory,
  EtherVault,
  EtherVaultProxy__factory,
  EtherVault__factory,
  RootChainManager,
  RootChainManagerProxy__factory,
  RootChainManager__factory,
  StateSender,
  StateSenderProxy__factory,
  StateSender__factory,
} from "../typechain-types";
import { execSync } from "child_process";
import { config } from "dotenv";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
config({
  path: ".env",
});
const sleep = async (milliseconds: number): Promise<void> => {
  // if (network.name !== "hardhat") {
  //   return await new Promise((resolve) => setTimeout(resolve, milliseconds));
  // }
};
async function waitTx(tx: any): Promise<void> {
  let transactionReceipt = null;
  while (transactionReceipt == null) {
    // Waiting expectedBlockTime until the transaction is mined
    transactionReceipt = await web3.eth.getTransactionReceipt(tx.hash);
    await sleep(2500);
    console.log("waiting transaction mined please be patient");
  }
  if (!transactionReceipt.status) {
    console.error(transactionReceipt);
    throw Error("TX failed");
  }
}

let RootChainManagerFactory: RootChainManager__factory;
let EthVaultFactory: EtherVault__factory;
let ERC20VaultFactory: ERC20Vault__factory;
let StateSenderFactory: StateSender__factory;
let RootChainManagerFactoryProxy: RootChainManagerProxy__factory;
let EthVaultFactoryProxy: EtherVaultProxy__factory;
let ERC20VaultFactoryProxy: ERC20VaultProxy__factory;
let StateSenderFactoryProxy: StateSenderProxy__factory;
let BridgeProxyAdminFactory: BridgeProxyAdmin__factory;
let rootManager: RootChainManager;
let ethVault: EtherVault;
let erc20Vault: ERC20Vault;
let stateSender: StateSender;
let proxyAdmin: BridgeProxyAdmin;
let owner: SignerWithAddress;

async function createStateSender(): Promise<StateSender> {
  if (
    process.env.STATE_SENDER_ADDRESS != null &&
    process.env.STATE_SENDER_ADDRESS !== ""
  ) {
    return StateSenderFactory.attach(process.env.STATE_SENDER_ADDRESS);
  }
  console.log("Deploying contract StateSender");

  const stateSender = await StateSenderFactory.deploy();
  console.log(stateSender.address);
  await stateSender.deployed();
  console.log("sleep");
  await sleep(120000);
  if (network.name !== "hardhat") {
    // Verifying stateSender implementation
    try {
      await run("verify:verify", {
        address: stateSender.address,
        constructorArguments: [],
      });
    } catch (err) {
      console.error(err);
    }
  }
  const proxy = await StateSenderFactoryProxy.deploy(
    stateSender.address,
    proxyAdmin.address,
    stateSender.interface.encodeFunctionData("initialize", [owner.address])
  );
  await proxy.deployed();
  console.log("sleep");
  await sleep(120000);
  // if (network.name !== "hardhat") {
  //   // Verifying stateSender proxy
  try {
    await run("verify:verify", {
      contract: "contracts/StateSender/StateSenderProxy.sol:StateSenderProxy",
      address: proxy.address,
      constructorArguments: [
        stateSender.address,
        proxyAdmin.address,
        stateSender.interface.encodeFunctionData("initialize", [owner.address]),
      ],
    });
  } catch (err) {
    console.error(err);
  }

  // console.log("\nLinking stateSender Proxy to implementation contract:");
  // const output = execSync(
  //   `curl -d "address=${proxy.address}" -s "https://api${
  //     network.name !== "mainnet" ? "-" + network.name : ""
  //     // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
  //   }.etherscan.io/api?module=contract&action=verifyproxycontract&apikey=${
  //     process.env.ETHERSCAN_API_KEY
  //   }"`
  // ).toString();
  // console.log(output);
  // }

  console.log(
    `StateSenderProxy at ${proxy.address} with implementation at ${stateSender.address}`
  );
  return StateSenderFactory.attach(proxy.address);
}

async function createProxyAdmin(): Promise<BridgeProxyAdmin> {
  if (
    process.env.PROXY_ADMIN_ADDRESS != null &&
    process.env.PROXY_ADMIN_ADDRESS !== ""
  ) {
    return BridgeProxyAdminFactory.attach(process.env.PROXY_ADMIN_ADDRESS);
  }
  console.log("Deploying contract ProxyAdmin");

  const proxyAdmin = await BridgeProxyAdminFactory.deploy();
  console.log(proxyAdmin.address);
  await proxyAdmin.deployed();
  console.log("sleep");
  await sleep(120000);
  // if (network.name !== "hardhat") {
  //   // Verifying BridgeProxyAdmin
  try {
    await run("verify:verify", {
      contract: "contracts/proxy/ProxyAdmin.sol:BridgeProxyAdmin",
      address: proxyAdmin.address,
      constructorArguments: [],
    });
  } catch (err) {
    console.error(err);
  }
  // }
  console.log(`ProxyAdmin at ${proxyAdmin.address}`);

  return proxyAdmin;
}

async function createEtherVault(): Promise<EtherVault> {
  if (
    process.env.ETHER_PREDICATE_ADDRESS != null &&
    process.env.ETHER_PREDICATE_ADDRESS !== ""
  ) {
    return EthVaultFactory.attach(process.env.ETHER_PREDICATE_ADDRESS);
  }
  console.log("Deploying contract Ether vault");
  const ethVault = await EthVaultFactory.deploy();
  await ethVault.deployed();
  console.log("sleep");
  await sleep(120000);
  // if (network.name !== "hardhat") {
  //   // Verifying ethVault implementation
  try {
    await run("verify:verify", {
      address: ethVault.address,
      constructorArguments: [],
    });
  } catch (err) {
    console.error(err);
  }
  // }
  const proxy = await EthVaultFactoryProxy.deploy(
    ethVault.address,
    proxyAdmin.address,
    ethVault.interface.encodeFunctionData("initialize", [owner.address])
  );
  await proxy.deployed();
  console.log("sleep");
  await sleep(120000);
  // if (network.name !== "hardhat") {
  //   // Verifying ethVault proxy
  try {
    await run("verify:verify", {
      contract: "contracts/TokenVault/EtherVaultProxy.sol:EtherVaultProxy",
      address: proxy.address,
      constructorArguments: [
        ethVault.address,
        proxyAdmin.address,
        ethVault.interface.encodeFunctionData("initialize", [owner.address]),
      ],
    });
  } catch (err) {
    console.error(err);
  }

  //   console.log("\nLinking ethVault Proxy to implementation contract:");
  //   const output = execSync(
  //     `curl -d "address=${proxy.address}" -s "https://api${
  //       network.name !== "mainnet" ? "-" + network.name : ""
  //       // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
  //     }.etherscan.io/api?module=contract&action=verifyproxycontract&apikey=${
  //       process.env.ETHERSCAN_API_KEY
  //     }"`
  //   ).toString();
  //   console.log(output);
  // }

  console.log(
    `EtherVaultProxy at ${proxy.address} with implementation at ${ethVault.address}`
  );

  return EthVaultFactory.attach(proxy.address);
}
async function createErc20Vault(): Promise<ERC20Vault> {
  if (
    process.env.ERC20_PREDICATE_ADDRESS != null &&
    process.env.ERC20_PREDICATE_ADDRESS !== ""
  ) {
    return ERC20VaultFactory.attach(process.env.ERC20_PREDICATE_ADDRESS);
  }
  console.log("Deploying contract Erc20 Vault");
  const erc20Vault = await ERC20VaultFactory.deploy();
  await erc20Vault.deployed();
  console.log("sleep");
  await sleep(120000);
  // if (network.name !== "hardhat") {
  //   // Verifying erc20Vault implementation
  try {
    await run("verify:verify", {
      address: erc20Vault.address,
      constructorArguments: [],
    });
  } catch (err) {
    console.error(err);
  }
  // }
  const proxy = await ERC20VaultFactoryProxy.deploy(
    erc20Vault.address,
    proxyAdmin.address,
    erc20Vault.interface.encodeFunctionData("initialize", [owner.address])
  );
  await proxy.deployed();
  console.log("sleep");
  await sleep(120000);
  // if (network.name !== "hardhat") {
  // Verifying erc20Vault proxy
  try {
    await run("verify:verify", {
      contract: "contracts/TokenVault/ERC20VaultProxy.sol:ERC20VaultProxy",
      address: proxy.address,
      constructorArguments: [
        erc20Vault.address,
        proxyAdmin.address,
        erc20Vault.interface.encodeFunctionData("initialize", [owner.address]),
      ],
    });
  } catch (err) {
    console.error(err);
  }

  // console.log("\nLinking erc20Vault Proxy to implementation contract:");
  // const output = execSync(
  //   `curl -d "address=${proxy.address}" -s "https://api${
  //     network.name !== "mainnet" ? "-" + network.name : ""
  //     // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
  //   }.etherscan.io/api?module=contract&action=verifyproxycontract&apikey=${
  //     process.env.ETHERSCAN_API_KEY
  //   }"`
  // ).toString();
  // console.log(output);
  // }

  console.log(
    `ERC20VaultProxy at ${proxy.address} with implementation at ${erc20Vault.address}`
  );

  return ERC20VaultFactory.attach(proxy.address);
}
async function createRootChainManager(): Promise<RootChainManager> {
  if (
    process.env.ROOT_MANAGER_ADDRESS != null &&
    process.env.ROOT_MANAGER_ADDRESS !== ""
  ) {
    return RootChainManagerFactory.attach(process.env.ROOT_MANAGER_ADDRESS);
  }
  console.log("Deploying contract root manager");

  const rootManager = await RootChainManagerFactory.deploy();
  await rootManager.deployed();
  console.log("sleep");
  await sleep(120000);
  // if (network.name !== "hardhat") {
  //   // Verifying rootManager implementation
  try {
    await run("verify:verify", {
      address: rootManager.address,
      constructorArguments: [],
    });
  } catch (err) {
    console.error(err);
  }
  // }
  const proxy = await RootChainManagerFactoryProxy.deploy(
    rootManager.address,
    proxyAdmin.address,
    rootManager.interface.encodeFunctionData("initialize", [owner.address])
  );
  await proxy.deployed();
  console.log("sleep");
  await sleep(120000);
  // if (network.name !== "hardhat") {
  //   // Verifying rootManager proxy
  try {
    await run("verify:verify", {
      contract:
        "contracts/root/RootChainManagerProxy.sol:RootChainManagerProxy",
      address: proxy.address,
      constructorArguments: [
        rootManager.address,
        proxyAdmin.address,
        rootManager.interface.encodeFunctionData("initialize", [owner.address]),
      ],
    });
  } catch (err) {
    console.error(err);
  }

  // console.log("\nLinking rootManager Proxy to implementation contract:");
  // const output = execSync(
  //   `curl -d "address=${proxy.address}" -s "https://api${
  //     network.name !== "mainnet" ? "-" + network.name : ""
  //     // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
  //   }.etherscan.io/api?module=contract&action=verifyproxycontract&apikey=${
  //     process.env.ETHERSCAN_API_KEY
  //   }"`
  // ).toString();
  // console.log(output);
  // }

  console.log(
    `RootChainManagerProxy at ${proxy.address} with implementation at ${rootManager.address}`
  );

  return RootChainManagerFactory.attach(proxy.address);
}

async function setupRootManager(): Promise<void> {
  const ethTokenType = await ethVault.TOKEN_TYPE();
  const erc20TokenType = await erc20Vault.TOKEN_TYPE();
  const ethVaultAddress = await rootManager.typeToVault(ethTokenType);
  const erc20VaultAddress = await rootManager.typeToVault(erc20TokenType);
  const stateSenderAddress = await rootManager.stateSenderAddress();
  if (stateSenderAddress !== stateSender.address) {
    console.log(
      `Wrong state sender address ${stateSenderAddress} setting to ${stateSender.address}`
    );
    const tx = await rootManager.setStateSender(stateSender.address);
    await waitTx(tx);
  }

  if (ethVaultAddress !== ethVault.address) {
    console.log(
      `Wrong ETH Vault address ${ethVaultAddress} setting to ${ethVault.address}`
    );
    const tx = await rootManager.registerVault(ethTokenType, ethVault.address);
    await waitTx(tx);
  }
  if (erc20VaultAddress !== erc20Vault.address) {
    console.log(
      `Wrong ERC20 Vault address ${erc20VaultAddress} setting to ${erc20Vault.address}`
    );
    const tx = await rootManager.registerVault(
      erc20TokenType,
      erc20Vault.address
    );
    await waitTx(tx);
  }
}
async function setupEthVault(): Promise<void> {
  const role = await ethVault.MANAGER_ROLE();
  const hasRole = await ethVault.hasRole(role, rootManager.address);
  if (!hasRole) {
    console.log(
      `Granting manager role in ${ethVault.address} to ${rootManager.address}`
    );
    const tx = await ethVault.grantRole(role, rootManager.address);
    await waitTx(tx);
  }
}
async function setupErc20Vault(): Promise<void> {
  const role = await erc20Vault.MANAGER_ROLE();
  const hasRole = await erc20Vault.hasRole(role, rootManager.address);
  if (!hasRole) {
    console.log(
      `Granting manager role in ${erc20Vault.address} to ${rootManager.address}`
    );
    const tx = await erc20Vault.grantRole(role, rootManager.address);
    await waitTx(tx);
  }
}

async function setupStateSender(): Promise<void> {
  const role = await stateSender.EMITTER_ROLE();
  const hasRole = await stateSender.hasRole(role, rootManager.address);
  if (!hasRole) {
    console.log(
      `Granting emitter role in ${stateSender.address} to ${rootManager.address}`
    );
    const tx = await stateSender.grantRole(role, rootManager.address);
    await waitTx(tx);
  }
}
async function main(): Promise<void> {
  RootChainManagerFactory = await ethers.getContractFactory("RootChainManager");
  EthVaultFactory = await ethers.getContractFactory("EtherVault");
  ERC20VaultFactory = await ethers.getContractFactory("ERC20Vault");
  StateSenderFactory = await ethers.getContractFactory("StateSender");
  RootChainManagerFactoryProxy = await ethers.getContractFactory(
    "RootChainManagerProxy"
  );
  EthVaultFactoryProxy = await ethers.getContractFactory("EtherVaultProxy");
  ERC20VaultFactoryProxy = await ethers.getContractFactory("ERC20VaultProxy");
  StateSenderFactoryProxy = await ethers.getContractFactory("StateSenderProxy");
  BridgeProxyAdminFactory = await ethers.getContractFactory("BridgeProxyAdmin");
  const signers = await ethers.getSigners();
  owner = signers[0];
  proxyAdmin = await createProxyAdmin();
  rootManager = await createRootChainManager();
  ethVault = await createEtherVault();
  erc20Vault = await createErc20Vault();
  stateSender = await createStateSender();
  await setupStateSender();
  await setupRootManager();
  await setupEthVault();
  await setupErc20Vault();
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
