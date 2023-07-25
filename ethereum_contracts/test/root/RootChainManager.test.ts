/* eslint-disable @typescript-eslint/no-non-null-assertion */

import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { ethers, web3 } from 'hardhat'
import { RootChainManager, ERC20Vault, EtherVault, StateSender, MockErc20 } from '../../typechain-types'
import { etherAddress, mockValues } from '../helpers/constants'
import { DeployContracts, deployInitializedContracts } from '../helpers/deployer'
import chai from 'chai'
import { BigNumber, BytesLike, ContractReceipt, ContractTransaction } from 'ethers'
import bs58check from 'bs58check'
import { getLogDecoder, DecodedLog, LogDecoder } from '../helpers/log-decoder'
// eslint-disable-next-line @typescript-eslint/no-var-requires
const { expectRevert } = require('@openzeppelin/test-helpers')

const should = chai.should()
const ccdUser = '0x' + bs58check.decode('43A45q6ohC5tsZ9nTZ7T5EniADtzgFBBYPE9A41Mjeq1vz2hev').slice(1).toString('hex')
const blank = '0x0000000000000000000000000000000000000000'
const blankBytes32 = '0x0000000000000000000000000000000000000000000000000000000000000000'
describe('RootChainManager', () => {
  let accounts: SignerWithAddress[]
  before(async () => {
    accounts = await ethers.getSigners()
  })
  describe('Set values', () => {
    let contracts: { rootChainManager: RootChainManager, erc20Vault: ERC20Vault, etherVault: EtherVault, stateSender: StateSender, dummyERC20: MockErc20 }
    before(async () => {
      contracts = await deployInitializedContracts(accounts)
    })

    it('Can set stateSenderAddress', async () => {
      const mockStateSenderAddress = mockValues.addresses[0]
      await contracts.rootChainManager.setStateSender(mockStateSenderAddress)
      const stateSenderAddress = await contracts.rootChainManager.stateSenderAddress()
      stateSenderAddress.should.equal(mockStateSenderAddress)
      await contracts.rootChainManager.setStateSender(contracts.stateSender.address)
    })

    it('Should revert while setting stateSenderAddress from non admin account', async () => {
      const mockStateSenderAddress = mockValues.addresses[3]
      await expectRevert(
        contracts.rootChainManager.connect(accounts[4]).setStateSender(mockStateSenderAddress),
        'AccessControl: '
      )
    })
    it('Should revert while setting stateSenderAddress to 0x0', async () => {
      await expectRevert(
        contracts.rootChainManager.setStateSender(blank),
        'RootChainManager: stateSender can not be address(0)'
      )
    })
    it('Can register predicate', async () => {
      const mockType = mockValues.bytes32[2]
      const mockPredicate = mockValues.addresses[4]
      await contracts.rootChainManager.registerVault(mockType, mockPredicate)
      const predicate = await contracts.rootChainManager.typeToVault(mockType)
      predicate.should.equal(mockPredicate)
    })

    it('Should revert while registering predicate from non mapper account', async () => {
      const mockType = mockValues.bytes32[3]
      const mockPredicate = mockValues.addresses[5]
      await expectRevert(
        contracts.rootChainManager.connect(accounts[4]).registerVault(mockType, mockPredicate),
        'AccessControl: '
      )
    })
  })

  describe('Token Mapping', () => {
    describe('Map fresh token', () => {
      let contracts: { rootChainManager: RootChainManager, dummyStateSender?: any, erc20Vault?: ERC20Vault, etherVault?: EtherVault, stateSender?: StateSender, dummyERC20?: MockErc20 }
      // first set of mock values
      const mockParent = mockValues.addresses[3]
      const mockChildIndex = 45
      const mockChildSubIndex = 0
      const mockType = mockValues.bytes32[3]
      // second set of mock values
      // need to use new values for repeat interaction since tx reverts for same addresses
      const spockParent = mockValues.addresses[5]
      const spockChildIndex = 46
      const spockChildSubIndex = 0

      before(async () => {
        contracts = await deployInitializedContracts(accounts)

        const mockPredicate = mockValues.addresses[4]
        await contracts.rootChainManager.registerVault(mockType, mockPredicate)
      })

      it('Can map token', async () => {
        await contracts.rootChainManager.mapToken(mockParent, mockChildIndex, mockChildSubIndex, mockType)
      })

      it('Should set correct rootToChildToken map', async () => {
        const childTokenAddress = await contracts.rootChainManager.rootToChildToken(mockParent)
        childTokenAddress[0].should.equal(BigNumber.from(mockChildIndex))
        childTokenAddress[1].should.equal(BigNumber.from(mockChildSubIndex))
      })

      it('Should set correct childToRootToken map', async () => {
        const childHash = await contracts.rootChainManager.hashChild(mockChildIndex, mockChildSubIndex)
        const parentTokenAddress = await contracts.rootChainManager.childToRootToken(childHash)
        parentTokenAddress.should.equal(mockParent)
      })

      it('Should fail while mapping token from non mapper account', async () => {
        await expectRevert(
          contracts.rootChainManager.connect(accounts[4]).mapToken(spockParent, spockChildIndex, spockChildSubIndex, mockType),
          'AccessControl: '
        )
      })

      it('Should fail while mapping token using non existant predicate', async () => {
        const mockType = mockValues.bytes32[0]
        await expectRevert(
          contracts.rootChainManager.mapToken(spockParent, spockChildIndex, spockChildSubIndex, mockType),
          'RootChainManager: not supported token type'
        )
      })

      it('Should clean token mapping', async () => {
        await contracts.rootChainManager.cleanMapToken(mockParent, mockChildIndex, mockChildSubIndex)
        const [index, subindex] = await contracts.rootChainManager.rootToChildToken(mockParent)
        index.should.equal(0)
        subindex.should.equal(0)
        const tokenType = await contracts.rootChainManager.tokenToType(mockParent)
        tokenType.should.equal(blankBytes32)
      })
    })

    // Keep same child token, change mapped parent token
    describe('Tomato has Vegetable as parent, remap to have Fruit as parent', () => {
      let contracts: { rootChainManager: any, erc20Vault?: ERC20Vault, etherVault?: EtherVault, stateSender?: StateSender, dummyERC20?: MockErc20 }
      const vegetable = mockValues.addresses[3]
      const fruit = mockValues.addresses[4]
      const tomatoIndex = 47
      const tomatoSubIndex = 0
      const tokenType = mockValues.bytes32[3]

      before(async () => {
        contracts = await deployInitializedContracts(accounts)

        const predicate = mockValues.addresses[2]
        await contracts.rootChainManager.registerVault(tokenType, predicate)

        await contracts.rootChainManager.mapToken(vegetable, tomatoIndex, tomatoSubIndex, tokenType)
      })

      it('Should have Tomato as child of Vegetable', async () => {
        const childTokenAddress = await contracts.rootChainManager.rootToChildToken(vegetable)
        childTokenAddress[0].should.equal(BigNumber.from(tomatoIndex))
        childTokenAddress[1].should.equal(BigNumber.from(tomatoSubIndex))
      })

      it('Should have Vegetable as parent of Tomato', async () => {
        const hash = await contracts.rootChainManager.hashChild(tomatoIndex, tomatoSubIndex)
        const parentTokenAddress = await contracts.rootChainManager.childToRootToken(hash)
        parentTokenAddress.should.equal(vegetable)
      })

      it('Should fail to noramlly map Tomato as child of Fruit', async () => {
        await expectRevert(
          contracts.rootChainManager.mapToken(fruit, tomatoIndex, tomatoSubIndex, tokenType),
          'RootChainManager: already mapped'
        )
      })

      it('Should be able to explicitly remap Tomato as child of Fruit', async () => {
        await contracts.rootChainManager.remapToken(fruit, tomatoIndex, tomatoSubIndex, tokenType)
      })

      it('Should have Tomato as child of Fruit', async () => {
        const childTokenAddress = await contracts.rootChainManager.rootToChildToken(fruit)
        childTokenAddress[0].should.equal(tomatoIndex)
        childTokenAddress[1].should.equal(tomatoSubIndex)
      })

      it('Should have Fruit as parent of Tomato', async () => {
        const hash = await contracts.rootChainManager.hashChild(tomatoIndex, tomatoSubIndex)
        const parentTokenAddress = await contracts.rootChainManager.childToRootToken(hash)
        parentTokenAddress.should.equal(fruit)
      })

      it('Vegetable should not have any child', async () => {
        const parentTokenAddress = await contracts.rootChainManager.rootToChildToken(vegetable)
        parentTokenAddress[0].should.equal(BigNumber.from(0))
        parentTokenAddress[1].should.equal(BigNumber.from(0))
      })
    })

    // Keep same parent token, change mapped child token
    describe('Chimp has Baboon as child, remap to have Man as child', () => {
      let contracts: { rootChainManager: any, erc20Vault?: ERC20Vault, etherVault?: EtherVault, stateSender?: StateSender, dummyERC20?: MockErc20 }
      const chimp = mockValues.addresses[3]
      const baboonIndex = 48
      const baboonSubIndex = 0
      const manIndex = 49
      const manSubIndex = 0
      const tokenType = mockValues.bytes32[3]

      before(async () => {
        contracts = await deployInitializedContracts(accounts)

        const predicate = mockValues.addresses[2]
        await contracts.rootChainManager.registerVault(tokenType, predicate)

        await contracts.rootChainManager.mapToken(chimp, baboonIndex, baboonSubIndex, tokenType)
      })

      it('Should have Baboon as child of Chimp', async () => {
        const childTokenAddress = await contracts.rootChainManager.rootToChildToken(chimp)
        childTokenAddress[0].should.equal(BigNumber.from(baboonIndex))
        childTokenAddress[1].should.equal(BigNumber.from(baboonSubIndex))
      })

      it('Should have Chimp as parent of Baboon', async () => {
        const hash = await contracts.rootChainManager.hashChild(baboonIndex, baboonSubIndex)
        const parentTokenAddress = await contracts.rootChainManager.childToRootToken(hash)
        parentTokenAddress.should.equal(chimp)
      })

      it('Should fail to noramlly map Chimp to Man', async () => {
        await expectRevert(
          contracts.rootChainManager.mapToken(chimp, manIndex, manSubIndex, tokenType),
          'RootChainManager: already mapped'
        )
      })

      it('Should be able to explicitly remap Chimp to Man', async () => {
        await contracts.rootChainManager.remapToken(chimp, manIndex, manSubIndex, tokenType)
      })

      it('Should have Man as child of Chimp', async () => {
        const childTokenAddress = await contracts.rootChainManager.rootToChildToken(chimp)
        childTokenAddress[0].should.equal(BigNumber.from(manIndex))
        childTokenAddress[1].should.equal(BigNumber.from(manSubIndex))
      })

      it('Should have Chimp as parent of Man', async () => {
        const hash = await contracts.rootChainManager.hashChild(manIndex, manSubIndex)

        const parentTokenAddress = await contracts.rootChainManager.childToRootToken(hash)
        parentTokenAddress.should.equal(chimp)
      })

      it('Baboon should not have any parent', async () => {
        const hash = await contracts.rootChainManager.hashChild(baboonIndex, baboonSubIndex)

        const parentTokenAddress = await contracts.rootChainManager.childToRootToken(hash)
        parentTokenAddress.should.equal(mockValues.zeroAddress)
      })
    })
  })

  describe('Deposit ERC20', () => {
    const depositAmount = mockValues.amounts[1]
    const depositForAccount = mockValues.addresses[0]

    let contracts: { rootChainManager: RootChainManager, erc20Vault: ERC20Vault, etherVault: EtherVault, stateSender: StateSender, dummyERC20: MockErc20 }
    let dummyERC20: MockErc20
    let rootChainManager: RootChainManager
    let oldAccountBalance: BigNumber
    let oldContractBalance: BigNumber
    let depositTx: ContractTransaction
    let depositReceipt: ContractReceipt
    let lockedLog: DecodedLog
    let stateSyncedlog: DecodedLog
    let logDecoder: LogDecoder
    before(async () => {
      contracts = await deployInitializedContracts(accounts)
      dummyERC20 = contracts.dummyERC20
      rootChainManager = contracts.rootChainManager
      oldAccountBalance = await dummyERC20.balanceOf(accounts[0].address)
      oldContractBalance = await dummyERC20.balanceOf(contracts.erc20Vault.address)
      logDecoder = await getLogDecoder()
    })

    it('Depositor should have proper balance', () => {
      depositAmount.lt(oldAccountBalance).should.equal(true)
    })
    it('Should revert if paused', async () => {
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await rootChainManager.setPaused(true)
      await expectRevert(
        rootChainManager.depositFor(depositForAccount, ccdUser, dummyERC20.address, depositData),
        'RootChainManager: bridge is paused'
      )
      await rootChainManager.setPaused(false)
    })
    it('Depositor should be able to approve and deposit', async () => {
      await dummyERC20.approve(contracts.erc20Vault.address, depositAmount)
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      depositTx = await rootChainManager.depositFor(depositForAccount, ccdUser, dummyERC20.address, depositData)
      depositReceipt = await depositTx.wait()
      should.exist(depositTx)
    })

    it('Should emit LockedERC20 log', () => {
      const logs = logDecoder.decodeLogs(depositReceipt.logs)
      lockedLog = logs.find(l => l.event === 'LockedERC20')!
      should.exist(lockedLog)
    })

    describe('Correct values should be emitted in LockedERC20 log', () => {
      it('Event should be emitted by correct contract', () => {
        lockedLog.address.should.equal(
          contracts.erc20Vault.address.toLowerCase()
        )
      })

      it('Should emit proper depositor', () => {
        lockedLog.args.depositor.should.equal(accounts[0].address)
      })

      it('Should emit correct amount', () => {
        const lockedLogAmount = BigNumber.from(lockedLog.args.amount.toString())
        lockedLogAmount.should.equal(depositAmount)
      })

      it('Should emit correct deposit receiver', () => {
        lockedLog.args.depositReceiver.should.equal(depositForAccount)
      })

      it('Should emit correct root token', () => {
        lockedLog.args.rootToken.should.equal(dummyERC20.address)
      })
    })

    it('Should emit StateSynced log', () => {
      const logs = logDecoder.decodeLogs(depositReceipt.logs)
      stateSyncedlog = logs.find(l => l.event === 'LockedToken')!
      should.exist(stateSyncedlog)
    })

    describe('Correct values should be emitted in StateSynced log', () => {
      let depositReceiver: string, rootToken: string, depositData: BytesLike, depositReceiverCcd: BytesLike, vault: string
      before(() => {
        depositReceiver = stateSyncedlog.args.depositor
        depositReceiverCcd = stateSyncedlog.args.depositReceiver
        rootToken = stateSyncedlog.args.rootToken
        depositData = stateSyncedlog.args.depositData
        vault = stateSyncedlog.args.vault
      })

      it('Event should be emitted by correct contract', () => {
        stateSyncedlog.address.should.equal(
          contracts.stateSender.address.toLowerCase()
        )
      })

      it('Should emit correct deposit receiver', () => {
        depositReceiver.should.equal(depositForAccount)
      })

      it('Should emit correct deposit receiver ccd', () => {
        depositReceiverCcd.should.equal(ccdUser)
      })

      it('Should emit correct root token', () => {
        rootToken.should.equal(dummyERC20.address)
      })

      it('Should emit correct amount', () => {
        const [amount] = ethers.utils.defaultAbiCoder.decode(['uint256'], depositData)
        amount.should.equal(depositAmount)
      })

      it('Should emit correct vault', () => {
        vault.should.equal(contracts.erc20Vault.address)
      })
    })

    it('Deposit amount should be deducted from depositor account', async () => {
      const newAccountBalance = await dummyERC20.balanceOf(accounts[0].address)
      newAccountBalance.should.equal(
        oldAccountBalance.sub(depositAmount)
      )
    })

    it('Deposit amount should be credited to correct contract', async () => {
      const newContractBalance = await dummyERC20.balanceOf(contracts.erc20Vault.address)
      newContractBalance.should.equal(
        oldContractBalance.add(depositAmount)
      )
    })
  })

  describe('Deposit ERC20 for zero address', () => {
    const depositAmount = mockValues.amounts[1]
    const depositForAccount = mockValues.zeroAddress
    let rootChainManager: RootChainManager
    let dummyERC20: MockErc20

    before(async () => {
      const contracts = await deployInitializedContracts(accounts)
      rootChainManager = contracts.rootChainManager
      dummyERC20 = contracts.dummyERC20
      await dummyERC20.approve(contracts.erc20Vault.address, depositAmount)
    })

    it('transaction should revert', async () => {
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await expectRevert(
        rootChainManager.depositFor(depositForAccount, ccdUser, dummyERC20.address, depositData),
        'RootChainManager: invalid user'
      )
    })
  })

  describe('Deposit ERC20 with deposit fee', () => {
    const depositFee = mockValues.amounts[0]
    const depositAmount = mockValues.amounts[1]
    const depositForAccount = mockValues.addresses[0]
    let rootChainManager: RootChainManager
    let dummyERC20: MockErc20
    let newAccount: SignerWithAddress

    before(async () => {
      const contracts = await deployInitializedContracts(accounts)
      rootChainManager = contracts.rootChainManager
      dummyERC20 = contracts.dummyERC20
      await dummyERC20.approve(contracts.erc20Vault.address, depositAmount)
      // @ts-expect-error
      newAccount = ethers.Wallet.createRandom() as SignerWithAddress
    })
    it('should set treasure and fee', async () => {
      await rootChainManager.setDepositFee(depositFee)
      await rootChainManager.setWithdrawFee(mockValues.amounts[1])
      await rootChainManager.setTreasurer(newAccount.address)
    })
    it('should not be able to set treasurer to 0x0', async () => {
      await expectRevert(rootChainManager.setTreasurer(blank), 'RootChainManager: treasurer can not be address(0)')
    })
    it('transaction should revert', async () => {
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await expectRevert(rootChainManager.depositFor(depositForAccount, ccdUser, dummyERC20.address, depositData, {
        value: depositFee.div(2)
      }), 'RootChainManager: ETH send needs to be at least depositFee')
    })
    it('transaction should work', async () => {
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await rootChainManager.depositFor(depositForAccount, ccdUser, dummyERC20.address, depositData, {
        value: depositFee
      })
      const balance = BigNumber.from(await web3.eth.getBalance(newAccount.address))
      balance.should.equal(depositFee)
    })
  })

  describe('Deposit Ether', () => {
    const depositAmount = mockValues.amounts[1]
    const depositForAccount = mockValues.addresses[0]
    const gasPrice = BigNumber.from('300000000')
    let contracts: DeployContracts
    let rootChainManager: RootChainManager
    let oldAccountBalance: BigNumber
    let oldContractBalance: BigNumber
    let depositTx: ContractTransaction
    let depositReceipt: ContractReceipt
    let lockedLog: DecodedLog
    let stateSyncedlog: DecodedLog
    let logDecoder: LogDecoder

    before(async () => {
      contracts = await deployInitializedContracts(accounts)
      rootChainManager = contracts.rootChainManager
      oldAccountBalance = BigNumber.from(await web3.eth.getBalance(accounts[0].address))
      oldContractBalance = BigNumber.from(await web3.eth.getBalance(contracts.etherVault.address))
      logDecoder = await getLogDecoder()
    })

    it('Depositor should have proper balance', () => {
      depositAmount.lt(oldAccountBalance).should.equal(true)
    })

    it('Depositor should be able to deposit', async () => {
      depositTx = await rootChainManager.depositEtherFor(depositForAccount, ccdUser, {
        value: depositAmount,
        gasPrice
      })
      depositReceipt = await depositTx.wait()
      should.exist(depositTx)
    })

    it('Should emit LockedEther log', () => {
      const logs = logDecoder.decodeLogs(depositReceipt.logs)

      lockedLog = logs.find(l => l.event === 'LockedEther')!
      should.exist(lockedLog)
    })

    describe('Correct values should be emitted in LockedEther log', () => {
      it('Event should be emitted by correct contract', () => {
        lockedLog.address.should.equal(
          contracts.etherVault.address.toLowerCase()
        )
      })

      it('Should emit proper depositor', () => {
        lockedLog.args.depositor.should.equal(accounts[0].address)
      })

      it('Should emit correct amount', () => {
        const lockedLogAmount = BigNumber.from(lockedLog.args.amount.toString())
        lockedLogAmount.should.equal(depositAmount)
      })

      it('Should emit correct deposit receiver', () => {
        lockedLog.args.depositReceiver.should.equal(depositForAccount)
      })
    })

    it('Should emit StateSynced log', () => {
      const logs = logDecoder.decodeLogs(depositReceipt.logs)
      stateSyncedlog = logs.find(l => l.event === 'LockedToken')!
      should.exist(stateSyncedlog)
    })

    describe('Correct values should be emitted in StateSynced log', () => {
      let depositReceiver: string, rootToken: string, depositData: BytesLike
      before(() => {
        // const [, syncData] = abi.decode(['bytes32', 'bytes'], stateSyncedlog.args.data)
        // const data = abi.decode(['address', 'address', 'bytes'], syncData)
        depositReceiver = stateSyncedlog.args.depositor
        rootToken = stateSyncedlog.args.rootToken
        depositData = stateSyncedlog.args.depositData
      })

      it('Event should be emitted by correct contract', () => {
        stateSyncedlog.address.should.equal(
          contracts.stateSender.address.toLowerCase()
        )
      })

      it('Should emit correct deposit receiver', () => {
        depositReceiver.should.equal(depositForAccount)
      })

      it('Should emit correct root token', () => {
        rootToken.should.equal(etherAddress)
      })

      it('Should emit correct amount', () => {
        const [amount] = ethers.utils.defaultAbiCoder.decode(['uint256'], depositData)
        amount.should.equal(depositAmount)
      })
    })

    it('Deposit amount should be deducted from depositor account', async () => {
      const newAccountBalance = await web3.eth.getBalance(accounts[0].address)
      const gasUsed = depositReceipt.gasUsed
      const gasCost = gasPrice.mul(gasUsed)
      newAccountBalance.should.equal(
        oldAccountBalance.sub(depositAmount).sub(gasCost)
      )
    })

    it('Deposit amount should be credited to correct contract', async () => {
      const newContractBalance = await web3.eth.getBalance(contracts.etherVault.address)
      newContractBalance.should.equal(
        oldContractBalance.add(depositAmount)
      )
    })
    it('Should revert if paused', async () => {
      await rootChainManager.setPaused(true)
      await expectRevert(
        rootChainManager.depositEtherFor(depositForAccount, ccdUser, {
          value: depositAmount,
          gasPrice
        }),
        'RootChainManager: bridge is paused'
      )
      await rootChainManager.setPaused(false)
    })
  })

  describe('Deposit Ether by sending to RootChainManager', () => {
    const depositAmount = mockValues.amounts[1]
    const gasPrice = BigNumber.from('30000000000')
    let contracts: DeployContracts
    let rootChainManager: RootChainManager
    let oldAccountBalance: BigNumber
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    let oldContractBalance: BigNumber
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    let logDecoder: LogDecoder

    before(async () => {
      contracts = await deployInitializedContracts(accounts)
      rootChainManager = contracts.rootChainManager
      oldAccountBalance = BigNumber.from(await web3.eth.getBalance(accounts[0].address))
      oldContractBalance = BigNumber.from(await web3.eth.getBalance(contracts.etherVault.address))
      logDecoder = await getLogDecoder()
    })

    it('Depositor should have proper balance', () => {
      depositAmount.lt(oldAccountBalance).should.equal(true)
    })

    it('Depositor should not be able to deposit', async () => {
      await expectRevert(accounts[0].sendTransaction({
        to: rootChainManager.address,
        value: depositAmount,
        gasPrice
      }), 'RootChainManager: no direct ETH deposits')
    })
  })

  describe('Deposit Ether by directly calling depositFor', () => {
    const depositAmount = mockValues.amounts[1]
    const depositForAccount = mockValues.addresses[0]
    let rootChainManager: RootChainManager

    before(async () => {
      const contracts = await deployInitializedContracts(accounts)
      rootChainManager = contracts.rootChainManager
    })

    it('transaction should revert', async () => {
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await expectRevert(
        rootChainManager.depositFor(depositForAccount, ccdUser,
          etherAddress, depositData),
        'RootChainManager: invalid root token'
      )
    })
  })

  describe('Deposit Ether for zero address', () => {
    const depositAmount = mockValues.amounts[1]
    const depositForAccount = mockValues.zeroAddress
    let rootChainManager: RootChainManager

    before(async () => {
      const contracts = await deployInitializedContracts(accounts)
      rootChainManager = contracts.rootChainManager
    })

    it('transaction should revert', async () => {
      await expectRevert(
        rootChainManager.depositEtherFor(depositForAccount, ccdUser, { value: depositAmount }),
        'RootChainManager: invalid user'
      )
    })
  })

  describe('Deposit Ether with deposit fee', () => {
    const depositFee = mockValues.amounts[0]
    const depositAmount = mockValues.amounts[1]
    const depositForAccount = mockValues.addresses[0]
    let rootChainManager: RootChainManager
    let etherVault: EtherVault
    let newAccount: SignerWithAddress

    before(async () => {
      const contracts = await deployInitializedContracts(accounts)
      rootChainManager = contracts.rootChainManager
      etherVault = contracts.etherVault
      // @ts-expect-error
      newAccount = ethers.Wallet.createRandom() as SignerWithAddress
    })
    it('should set treasure and fee', async () => {
      await rootChainManager.setDepositFee(depositFee)
      await rootChainManager.setWithdrawFee(mockValues.amounts[1])
      await rootChainManager.setTreasurer(newAccount.address)
    })
    it('transaction should revert', async () => {
      await expectRevert(rootChainManager.depositEtherFor(depositForAccount, ccdUser, {
        value: depositFee.sub(1)
      }), 'RootChainManager: ETH send needs to be at least depositFee')
    })
    it('transaction should work', async () => {
      await rootChainManager.depositEtherFor(depositForAccount, ccdUser, {
        value: depositFee.add(depositAmount)
      })
      const balance = BigNumber.from(await web3.eth.getBalance(newAccount.address))
      balance.should.equal(depositFee)

      const vaultBalance = BigNumber.from(await web3.eth.getBalance(etherVault.address))
      vaultBalance.should.equal(depositAmount)
    })
  })

  describe('Deposit token whose predicate is disabled', () => {
    const depositAmount = mockValues.amounts[1]
    const depositForAccount = mockValues.addresses[0]
    const mockType = mockValues.bytes32[1]
    const mockChildIndex = 50
    const mockChildSubIndex = 0
    let contracts: DeployContracts
    let dummyERC20: MockErc20
    let rootChainManager: RootChainManager
    let erc20Vault: ERC20Vault

    before(async () => {
      contracts = await deployInitializedContracts(accounts)
      dummyERC20 = contracts.dummyERC20
      rootChainManager = contracts.rootChainManager
      erc20Vault = contracts.erc20Vault
      await dummyERC20.approve(erc20Vault.address, depositAmount)
      await rootChainManager.registerVault(mockType, erc20Vault.address)
      await rootChainManager.remapToken(dummyERC20.address, mockChildIndex, mockChildSubIndex, mockType)
      await rootChainManager.registerVault(mockType, mockValues.zeroAddress)
    })

    it('Should revert with correct reason', async () => {
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await expectRevert(
        rootChainManager.depositFor(depositForAccount, ccdUser, dummyERC20.address, depositData),
        'RootChainManager: invalid token type'
      )
    })

    it('Should revert if token is not mapped', async () => {
      const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [depositAmount.toString()])
      await rootChainManager.cleanMapToken(dummyERC20.address, mockChildIndex, mockChildSubIndex)
      await expectRevert(
        rootChainManager.depositFor(depositForAccount, ccdUser, dummyERC20.address, depositData),
        'RootChainManager: token not mapped'
      )
    })
  })
})
