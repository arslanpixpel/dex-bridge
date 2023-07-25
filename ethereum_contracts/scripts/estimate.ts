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
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { BigNumber, ContractFactory } from "ethers";

let RootChainManagerFactory: RootChainManager__factory;
let EthVaultFactory: EtherVault__factory;
let ERC20VaultFactory: ERC20Vault__factory;
let StateSenderFactory: StateSender__factory;
let RootChainManagerFactoryProxy: RootChainManagerProxy__factory;
let EthVaultFactoryProxy: EtherVaultProxy__factory;
let ERC20VaultFactoryProxy: ERC20VaultProxy__factory;
let StateSenderFactoryProxy: StateSenderProxy__factory;
let BridgeProxyAdminFactory: BridgeProxyAdmin__factory;

const estimate = async () => {
  const contracts: ContractFactory[] = [
    RootChainManagerFactory,
    EthVaultFactory,
    ERC20VaultFactory,
    StateSenderFactory,
  ];

  let impls = [];

  const proxy: ContractFactory[] = [
    RootChainManagerFactoryProxy,
    EthVaultFactoryProxy,
    ERC20VaultFactoryProxy,
    StateSenderFactoryProxy,
    BridgeProxyAdminFactory,
  ];

  const signer = (await ethers.getSigners())[0];

  let totalGas = BigNumber.from(0);
  for (const contract of contracts) {
    const deployTx = contract.getDeployTransaction();
    const estimatedGas = await signer.estimateGas(deployTx);
    totalGas = totalGas.add(estimatedGas);
  }

  const alice = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
  const bob = "0x90F79bf6EB2c4f870365E785982E1f101E93b906";

  for (const contract of proxy) {
    const deployTx = contract.getDeployTransaction(
      // Idk, must be not matter
      alice,
      bob,
      // This doesnt matter too for all proxy function have same signature
      RootChainManagerFactory.interface.encodeFunctionData("initialize", [
        alice,
      ])
    );
    const estimatedGas = await signer.estimateGas(deployTx);
    totalGas = totalGas.add(estimatedGas);
  }

  const gasPrice = await signer.getGasPrice();

  const deploymentPriceWei = gasPrice.mul(totalGas);
  const deploymentPrice = ethers.utils.formatEther(deploymentPriceWei);

  console.log(
    `Deployment price in native token on chainId ${await signer.getChainId()}\n${deploymentPrice}`
  );
};

const init = async () => {
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
};

const main = async () => {
  await init();
  await estimate();
};

main();
