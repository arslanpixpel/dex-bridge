import { getContracts } from './contracts'
import { etherAddress, mockChildIndex, mockChildSubIndex, mockETHChildIndex, mockETHChildSubIndex, mockValues } from './constants'
import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { BridgeProxyAdmin, ERC20Vault, EtherVault, MockErc20, RootChainManager, StateSender } from '../../typechain-types'
import { ethers } from 'hardhat'

export interface DeployContracts {
  rootChainManager: RootChainManager
  erc20Vault: ERC20Vault
  etherVault: EtherVault
  stateSender: StateSender
  dummyERC20: MockErc20
  proxyAdmin: BridgeProxyAdmin
}

export const deployFreshRootContracts = async (): Promise<DeployContracts> => {
  const contracts = await getContracts()
  const [owner] = await ethers.getSigners()
  const [
    rootChainManager,
    erc20Vault,
    etherVault,
    stateSender,
    dummyERC20,
    proxyAdmin
  ] = await Promise.all([
    contracts.RootChainManager.deploy(),
    contracts.ERC20Vault.deploy(),
    contracts.EtherVault.deploy(),
    contracts.StateSender.deploy(),
    contracts.MockErc20.deploy(),
    contracts.ProxyAdmin.deploy()
  ])

  const [
    rootChainManagerProxy,
    etherVaultProxy,
    erc20VaultProxy,
    stateSenderProxy
  ] = await Promise.all([
    contracts.RootChainManagerProxy.deploy(
      rootChainManager.address,
      proxyAdmin.address,
      rootChainManager.interface.encodeFunctionData('initialize', [owner.address])
    ),
    contracts.EtherVaultProxy.deploy(
      etherVault.address,
      proxyAdmin.address,
      etherVault.interface.encodeFunctionData('initialize', [owner.address])
    ),
    contracts.ERC20VaultProxy.deploy(
      erc20Vault.address,
      proxyAdmin.address,
      erc20Vault.interface.encodeFunctionData('initialize', [owner.address])
    ),
    contracts.StateSenderProxy.deploy(
      stateSender.address,
      proxyAdmin.address,
      stateSender.interface.encodeFunctionData('initialize', [owner.address])
    )
  ])

  return {
    rootChainManager: contracts.RootChainManager.attach(rootChainManagerProxy.address),
    erc20Vault: contracts.ERC20Vault.attach(erc20VaultProxy.address),
    etherVault: contracts.EtherVault.attach(etherVaultProxy.address),
    stateSender: contracts.StateSender.attach(stateSenderProxy.address),
    dummyERC20,
    proxyAdmin
  }
}

export const deployInitializedContracts = async (accounts: SignerWithAddress[]): Promise<DeployContracts> => {
  const root = await deployFreshRootContracts()

  await root.rootChainManager.setStateSender(root.stateSender.address)

  const MANAGER_ROLE = await root.erc20Vault.MANAGER_ROLE()
  const ERC20Type = await root.erc20Vault.TOKEN_TYPE()
  const EMITTER_ROLE = await root.stateSender.EMITTER_ROLE()
  const MERKLE_UPDATER = await root.rootChainManager.MERKLE_UPDATER()

  await root.rootChainManager.grantRole(MERKLE_UPDATER, accounts[0].address)
  await root.stateSender.grantRole(EMITTER_ROLE, root.rootChainManager.address)

  await root.erc20Vault.grantRole(MANAGER_ROLE, root.rootChainManager.address)
  await root.rootChainManager.registerVault(ERC20Type, root.erc20Vault.address)
  await root.rootChainManager.mapToken(root.dummyERC20.address, mockChildIndex, mockChildSubIndex, ERC20Type)

  const EtherType = await root.etherVault.TOKEN_TYPE()
  await root.etherVault.grantRole(MANAGER_ROLE, root.rootChainManager.address)
  await root.rootChainManager.registerVault(EtherType, root.etherVault.address)
  await root.rootChainManager.mapToken(etherAddress, mockETHChildIndex, mockETHChildSubIndex, EtherType)

  await root.dummyERC20.mint(accounts[0].address, mockValues.amounts[7])
  return root
}
