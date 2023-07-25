/* eslint-disable @typescript-eslint/no-non-null-assertion */

import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { ethers, waffle } from 'hardhat'
import chai from 'chai'
import { mockValues } from '../helpers/constants'
import { RootChainManager, ERC20Vault, EtherVault, MockErc20, BridgeProxyAdmin, StateSender } from '../../typechain-types'
import { deployInitializedContracts } from '../helpers/deployer'
import bs58check from 'bs58check'
import { BigNumber } from 'ethers'
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const should = chai.should()

const blank = '0x0000000000000000000000000000000000000000'
const ccdUser = '0x' + bs58check.decode('43A45q6ohC5tsZ9nTZ7T5EniADtzgFBBYPE9A41Mjeq1vz2hev').slice(1).toString('hex')

describe('Proxy upgrade', () => {
  let accounts: SignerWithAddress[]
  before(async () => {
    accounts = await ethers.getSigners()
  })
  describe('StateSender', () => {
    let contracts: { rootChainManager: RootChainManager, erc20Vault: ERC20Vault, etherVault: EtherVault, stateSender: StateSender, dummyERC20: MockErc20, proxyAdmin: BridgeProxyAdmin }
    before(async () => {
      contracts = await deployInitializedContracts(accounts)
    })

    it('can upgrade', async () => {
      const factory = await ethers.getContractFactory('MockStateSenderUpgrade')
      const contract = await factory.deploy()
      const EMITTER_ROLE = await contracts.stateSender.EMITTER_ROLE()
      const TOKEN_TYPE = await contracts.etherVault.TOKEN_TYPE()
      await contracts.stateSender.grantRole(EMITTER_ROLE, accounts[0].address)
      let tx = await contracts.stateSender.emitTokenMapAdd(blank, 0, 0, TOKEN_TYPE)
      let receipt = await tx.wait()
      // @ts-expect-error
      const id: number = receipt.events[0].args?.id.toNumber()
      await contracts.proxyAdmin.upgrade(contracts.stateSender.address, contract.address)
      const stateSender = factory.attach(contracts.stateSender.address)
      tx = await stateSender.emitTest(42)
      receipt = await tx.wait()

      // @ts-expect-error
      const newId: number = receipt.events[0].args.id.toNumber()
      // @ts-expect-error
      const payload: number = receipt.events[0].args.payload.toNumber()
      newId.should.equal(id + 1)
      payload.should.equal(42)
    })
  })
  describe('Erc20Vault', () => {
    let contracts: { rootChainManager: RootChainManager, erc20Vault: ERC20Vault, etherVault: EtherVault, stateSender: StateSender, dummyERC20: MockErc20, proxyAdmin: BridgeProxyAdmin }
    before(async () => {
      contracts = await deployInitializedContracts(accounts)
    })

    it('can upgrade', async () => {
      const factory = await ethers.getContractFactory('ERC20Vault')
      const contract = await factory.deploy()
      const MANAGER_ROLE = await contracts.erc20Vault.MANAGER_ROLE()
      await contracts.erc20Vault.grantRole(MANAGER_ROLE, accounts[0].address)
      const initialBalance = await contracts.dummyERC20.balanceOf(accounts[0].address)

      const depositAmount = mockValues.amounts[1]
      const withdrawAmount = mockValues.amounts[0]
      await contracts.dummyERC20.approve(contracts.erc20Vault.address, depositAmount)
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await contracts.erc20Vault.lockTokens(
        accounts[0].address, accounts[0].address, ccdUser, contracts.dummyERC20.address, depositData
      )

      await contracts.proxyAdmin.upgrade(contracts.erc20Vault.address, contract.address)
      const vaultBalance = await contracts.dummyERC20.balanceOf(contracts.erc20Vault.address)
      vaultBalance.should.equal(depositAmount)
      await contracts.erc20Vault.exitTokens(accounts[0].address, contracts.dummyERC20.address, 0, withdrawAmount)
      const vaultBalance2 = await contracts.dummyERC20.balanceOf(contracts.erc20Vault.address)
      vaultBalance2.should.equal(depositAmount.sub(withdrawAmount))
      const currentBalance = await contracts.dummyERC20.balanceOf(accounts[0].address)
      currentBalance.should.equal(initialBalance.sub(depositAmount).add(withdrawAmount))
    })
  })

  describe('EtherVault', () => {
    let contracts: { rootChainManager: RootChainManager, erc20Vault: ERC20Vault, etherVault: EtherVault, stateSender: StateSender, dummyERC20: MockErc20, proxyAdmin: BridgeProxyAdmin }
    before(async () => {
      contracts = await deployInitializedContracts(accounts)
    })

    it('can upgrade', async () => {
      const factory = await ethers.getContractFactory('EtherVault')
      const contract = await factory.deploy()
      const MANAGER_ROLE = await contracts.etherVault.MANAGER_ROLE()
      await contracts.etherVault.grantRole(MANAGER_ROLE, accounts[0].address)
      let gasUsed = BigNumber.from(0)
      const depositAmount = mockValues.amounts[1]
      const withdrawAmount = mockValues.amounts[0]
      await contracts.dummyERC20.approve(contracts.etherVault.address, depositAmount)
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      const initialBalance = await waffle.provider.getBalance(accounts[0].address)

      let tx = await accounts[0].sendTransaction({
        to: contracts.etherVault.address,
        value: depositAmount
      })
      let receipt = await tx.wait()
      gasUsed = gasUsed.add(receipt.gasUsed.mul(receipt.effectiveGasPrice))
      tx = await contracts.etherVault.lockTokens(
        accounts[0].address, accounts[0].address, ccdUser, contracts.dummyERC20.address, depositData
      )
      receipt = await tx.wait()
      gasUsed = gasUsed.add(receipt.gasUsed.mul(receipt.effectiveGasPrice))
      tx = await contracts.proxyAdmin.upgrade(contracts.etherVault.address, contract.address)
      receipt = await tx.wait()
      gasUsed = gasUsed.add(receipt.gasUsed.mul(receipt.effectiveGasPrice))
      tx = await contracts.etherVault.exitTokens(accounts[0].address, contracts.dummyERC20.address, 0, withdrawAmount)
      receipt = await tx.wait()
      gasUsed = gasUsed.add(receipt.gasUsed.mul(receipt.effectiveGasPrice))

      const currentBalance = await waffle.provider.getBalance(accounts[0].address)
      currentBalance.should.equal(initialBalance.sub(depositAmount).sub(gasUsed).add(withdrawAmount))
    })
  })

  describe('RootChainManager', () => {
    let contracts: { rootChainManager: RootChainManager, erc20Vault: ERC20Vault, etherVault: EtherVault, stateSender: StateSender, dummyERC20: MockErc20, proxyAdmin: BridgeProxyAdmin }
    before(async () => {
      contracts = await deployInitializedContracts(accounts)
    })

    it('can upgrade', async () => {
      const factory = await ethers.getContractFactory('RootChainManager')
      const contract = await factory.deploy()
      const depositAmount = mockValues.amounts[1]

      // If everything is good with the upgrade then a deposit should validate roles are kept
      // and storage with the mapping was upgraded
      await contracts.proxyAdmin.upgrade(contracts.etherVault.address, contract.address)

      await contracts.dummyERC20.approve(contracts.erc20Vault.address, depositAmount)
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await contracts.rootChainManager.depositFor(accounts[0].address, ccdUser, contracts.dummyERC20.address, depositData)
    })
  })
})
