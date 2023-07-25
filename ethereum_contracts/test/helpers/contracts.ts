
import { ethers } from 'hardhat'
import { BridgeProxyAdmin__factory, ERC20VaultProxy__factory, ERC20Vault__factory, EtherVaultProxy__factory, EtherVault__factory, MockErc20__factory, RootChainManagerProxy__factory, RootChainManager__factory, StateSenderProxy__factory, StateSender__factory } from '../../typechain-types'

export const getContracts = async (): Promise<{
  RootChainManager: RootChainManager__factory
  RootChainManagerProxy: RootChainManagerProxy__factory
  ERC20Vault: ERC20Vault__factory
  ERC20VaultProxy: ERC20VaultProxy__factory
  EtherVault: EtherVault__factory
  EtherVaultProxy: EtherVaultProxy__factory
  StateSender: StateSender__factory
  StateSenderProxy: StateSenderProxy__factory
  MockErc20: MockErc20__factory
  ProxyAdmin: BridgeProxyAdmin__factory
}> => {
  const RootChainManager = await ethers.getContractFactory('RootChainManager')
  const RootChainManagerProxy = await ethers.getContractFactory('RootChainManagerProxy')
  const ERC20Vault = await ethers.getContractFactory('ERC20Vault')
  const ERC20VaultProxy = await ethers.getContractFactory('ERC20VaultProxy')

  const EtherVault = await ethers.getContractFactory('EtherVault')
  const EtherVaultProxy = await ethers.getContractFactory('EtherVaultProxy')
  const StateSender = await ethers.getContractFactory('StateSender')
  const StateSenderProxy = await ethers.getContractFactory('StateSenderProxy')
  const MockErc20 = await ethers.getContractFactory('MockErc20')
  const ProxyAdmin = await ethers.getContractFactory('BridgeProxyAdmin')

  return {
    RootChainManager,
    RootChainManagerProxy,
    ERC20Vault,
    ERC20VaultProxy,
    EtherVault,
    EtherVaultProxy,
    StateSender,
    StateSenderProxy,
    MockErc20,
    ProxyAdmin
  }
}
