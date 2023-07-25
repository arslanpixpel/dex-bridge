import chai from 'chai'
import { ethers, web3 } from 'hardhat'
import * as deployer from '../helpers/deployer'
import { mockChildIndex, mockChildSubIndex, mockETHChildIndex, mockETHChildSubIndex, mockValues } from '../helpers/constants'
import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { BigNumber, ContractReceipt, ContractTransaction } from 'ethers'
import { deployInitializedContracts } from '../helpers/deployer'
import { IRootChainManager, MockErc20, RootChainManager } from '../../typechain-types'
import bs58check from 'bs58check'
import MerkleTree from 'merkletreejs'
import { getLogDecoder, LogDecoder } from '../helpers/log-decoder'

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { expectRevert } = require('@openzeppelin/test-helpers')
const abi = ethers.utils.defaultAbiCoder

const should = chai.should()

const ccdUser = '0x' + bs58check.decode('43A45q6ohC5tsZ9nTZ7T5EniADtzgFBBYPE9A41Mjeq1vz2hev').slice(1).toString('hex')
const ccdUser2 = '0x' + bs58check.decode('3yyk5wjVSLaRqLMhKn8Go5Fcnrtpwxg5SZZLGRH8ar1E17bSrf').slice(1).toString('hex')

const ccdIndex = mockChildIndex
const ccdSubIndex = mockChildSubIndex
const ccdIndexEth = mockETHChildIndex
const ccdSubIndexEth = mockETHChildSubIndex
function encode(withdrawTx: IRootChainManager.WithdrawParamsStruct): string {
  return ethers.utils.keccak256(abi.encode(['uint64', 'uint64', 'uint64', 'address', 'bytes32', 'uint64', 'uint64'],
    [
      withdrawTx.ccdIndex,
      withdrawTx.ccdSubIndex,
      withdrawTx.amount,
      withdrawTx.userWallet,
      withdrawTx.ccdTxHash,
      withdrawTx.ccdEventIndex,
      withdrawTx.tokenId
    ]))
}
describe('RootChainManager', () => {
  let accounts: SignerWithAddress[]
  before(async () => {
    accounts = await ethers.getSigners()
  })
  describe('Withdraw', () => {
    const depositAmount = mockValues.amounts[1]
    const depositAmount3 = mockValues.amounts[0]
    let totalDepositedAmount = BigNumber.from('0')
    const withdrawAmount = mockValues.amounts[1]
    let depositReceiver: SignerWithAddress
    let depositReceiverEth: SignerWithAddress
    const depositData = abi.encode(['uint256'], [depositAmount.toString()])
    let contracts: deployer.DeployContracts
    let dummyERC20: MockErc20
    let rootChainManager: RootChainManager
    let accountBalance: BigNumber
    let contractBalance: BigNumber
    let accountBalanceEth: BigNumber
    let contractBalanceEth: BigNumber
    let exitTx: ContractTransaction
    let exitReceipt: ContractReceipt
    let merkletree: MerkleTree
    let withdrawTxHash: string | Buffer
    let logDecoder: LogDecoder

    let withdrawTxs: IRootChainManager.WithdrawParamsStruct[]

    before(async () => {
      contracts = await deployInitializedContracts(accounts)
      dummyERC20 = contracts.dummyERC20
      rootChainManager = contracts.rootChainManager
      accountBalance = await dummyERC20.balanceOf(accounts[0].address)
      contractBalance = await dummyERC20.balanceOf(contracts.erc20Vault.address)
      depositReceiver = accounts[0]
      depositReceiverEth = accounts[3]
      logDecoder = await getLogDecoder()
    })

    it('Depositor should be able to approve and deposit', async () => {
      await dummyERC20.approve(contracts.erc20Vault.address, depositAmount)
      const depositTx = await rootChainManager.depositFor(depositReceiver.address, ccdUser, dummyERC20.address, depositData)
      should.exist(depositTx)
      totalDepositedAmount = totalDepositedAmount.add(depositAmount)
    })

    it('Second depositor should be able to approve and deposit', async () => {
      const depositAmount = mockValues.amounts[2]
      const depositData = abi.encode(['uint256'], [depositAmount.toString()])
      await dummyERC20.mint(accounts[2].address, depositAmount)
      await dummyERC20.connect(accounts[2]).approve(contracts.erc20Vault.address, depositAmount)
      const depositTx = await rootChainManager.connect(accounts[2]).depositFor(accounts[2].address, ccdUser, dummyERC20.address, depositData)
      should.exist(depositTx)
      totalDepositedAmount = totalDepositedAmount.add(depositAmount)
    })
    it('Third depositor should be able to approve and deposit', async () => {
      const depositTx = await rootChainManager.connect(accounts[3]).depositEtherFor(accounts[3].address, ccdUser2, {
        value: depositAmount3
      })
      should.exist(depositTx)
      accountBalanceEth = BigNumber.from(await web3.eth.getBalance(accounts[3].address))
      contractBalanceEth = BigNumber.from(await web3.eth.getBalance(contracts.etherVault.address))
    })

    it('Deposit amount should be deducted from depositor account', async () => {
      const newAccountBalance = await dummyERC20.balanceOf(accounts[0].address)
      newAccountBalance.should.equal(
        accountBalance.sub(depositAmount)
      )
      // update account balance
      accountBalance = newAccountBalance
    })

    it('Deposit amount should be credited to correct contract', async () => {
      const newContractBalance = await dummyERC20.balanceOf(contracts.erc20Vault.address)
      newContractBalance.should.equal(
        contractBalance.add(totalDepositedAmount)
      )

      // update balance
      contractBalance = newContractBalance
    })

    it('Can set merkle proof', async () => {
      withdrawTxs = [{
        ccdIndex,
        ccdSubIndex,
        amount: depositAmount,
        userWallet: depositReceiver.address,
        ccdTxHash: [
          55, 162, 168, 229, 46, 250, 217, 117, 219, 246, 88, 14, 119, 52, 228, 242, 73, 234, 165,
          234, 138, 118, 62, 147, 74, 134, 113, 205, 126, 68, 100, 153
        ],
        ccdEventIndex: 1,
        tokenId: 0
      },
      {
        ccdIndex: ccdIndexEth,
        ccdSubIndex: ccdSubIndexEth,
        amount: depositAmount3,
        userWallet: depositReceiverEth.address,
        ccdTxHash: [
          55, 162, 168, 229, 46, 250, 217, 117, 219, 246, 88, 14, 119, 52, 228, 242, 73, 234, 165,
          234, 138, 118, 62, 147, 74, 134, 113, 205, 126, 68, 100, 153
        ],
        ccdEventIndex: 1,
        tokenId: 0
      }, {
        ccdIndex,
        ccdSubIndex,
        amount: depositAmount,
        userWallet: accounts[2].address,
        ccdTxHash: [
          55, 162, 168, 229, 46, 250, 217, 117, 219, 246, 88, 14, 119, 52, 228, 242, 73, 234, 165,
          234, 138, 118, 62, 147, 74, 134, 113, 205, 126, 68, 100, 153
        ],
        ccdEventIndex: 1,
        tokenId: 0
      }, {
        ccdIndex,
        ccdSubIndex,
        amount: depositAmount,
        userWallet: accounts[2].address,
        ccdTxHash: [
          55, 162, 168, 229, 46, 250, 217, 117, 219, 246, 88, 14, 119, 52, 228, 242, 73, 234, 165,
          234, 138, 118, 62, 147, 74, 134, 113, 205, 126, 68, 100, 153
        ],
        ccdEventIndex: 2,
        tokenId: 0
      }, {
        ccdIndex,
        ccdSubIndex,
        amount: depositAmount,
        userWallet: accounts[2].address,
        ccdTxHash: [
          55, 162, 168, 229, 46, 250, 217, 117, 219, 246, 88, 14, 119, 52, 228, 242, 73, 234, 165,
          234, 138, 118, 62, 147, 74, 134, 113, 205, 126, 68, 100, 153
        ],
        ccdEventIndex: 3,
        tokenId: 0
      }]
      const hashes = withdrawTxs.map(encode)
      withdrawTxHash = hashes[0]
      merkletree = new MerkleTree(hashes, ethers.utils.keccak256, { sortPairs: true })
      const tx = await contracts.rootChainManager.setMerkleRoot(merkletree.getHexRoot())
      should.exist(tx)
      const receipt = await tx.wait()
      const logs = logDecoder.decodeLogs(receipt.logs)
      const merkleRootLog = logs.find(l => l.event === 'MerkleRoot')
      should.exist(merkleRootLog)
      merkleRootLog?.args.root.should.equal(merkletree.getHexRoot())
      const contractMerkleRoot = await contracts.rootChainManager.getMerkleRoot()
      contractMerkleRoot.should.equal(merkletree.getHexRoot())
      await expectRevert(contracts.rootChainManager.connect(accounts[2]).setMerkleRoot(merkletree.getHexRoot()), 'AccessControl: ')
    })

    it('Should fail: withdraw with a random data receipt', async () => {
      const params = {
        ccdIndex: ccdIndex + 1,
        ccdSubIndex,
        amount: depositAmount,
        userWallet: depositReceiver.address,
        ccdTxHash: [
          55, 162, 168, 229, 46, 250, 217, 117, 219, 246, 88, 14, 119, 52, 228, 242, 73, 234, 165,
          234, 138, 118, 62, 147, 74, 134, 113, 205, 126, 68, 100, 153
        ],
        ccdEventIndex: 1,
        tokenId: 0
      }
      // start exit
      await expectRevert(contracts.rootChainManager.connect(depositReceiver).withdraw(params, merkletree.getHexProof(withdrawTxHash)), 'RootChainManager: transaction proof verification failed')
    })

    it('Should fail: withdraw with a fake amount data in receipt', async () => {
      const params = {
        ccdIndex,
        ccdSubIndex,
        amount: depositAmount.mul(10),
        userWallet: depositReceiver.address,
        ccdTxHash: [
          55, 162, 168, 229, 46, 250, 217, 117, 219, 246, 88, 14, 119, 52, 228, 242, 73, 234, 165,
          234, 138, 118, 62, 147, 74, 134, 113, 205, 126, 68, 100, 153
        ],
        ccdEventIndex: 1,
        tokenId: 0
      }
      // start exit
      await expectRevert(contracts.rootChainManager.connect(depositReceiver).withdraw(params, merkletree.getHexProof(withdrawTxHash)), 'RootChainManager: transaction proof verification failed')
    })

    describe('ERC20 WITHDRAW', () => {
      it('Should start ERC20 withdraw', async () => {
        // start exit
        exitTx = await contracts.rootChainManager.connect(depositReceiver).withdraw(withdrawTxs[0], merkletree.getHexProof(withdrawTxHash))
        exitReceipt = await exitTx.wait()
        should.exist(exitTx)
      })

      it('Should fail: start exit again', async () => {
        await expectRevert(contracts.rootChainManager.connect(depositReceiver).withdraw(withdrawTxs[0], merkletree.getHexProof(withdrawTxHash)), 'RootChainManager: exit already processed')
      })

      it('Should emit Transfer log in exit tx', () => {
        const logs = logDecoder.decodeLogs(exitReceipt.logs)
        const exitTransferLog = logs.find(l => l.event === 'Transfer')
        should.exist(exitTransferLog)
      })

      it('Should emit WithdrawEvent log in exit tx', () => {
        const logs = logDecoder.decodeLogs(exitReceipt.logs)
        const exitTransferLog = logs.find(l => l.event === 'WithdrawEvent')
        should.exist(exitTransferLog)
      })

      it('Should have more amount in withdrawer account after withdraw', async () => {
        const newAccountBalance = await dummyERC20.balanceOf(depositReceiver.address)

        newAccountBalance.should.equal(
          accountBalance.add(depositAmount)
        )
      })

      it('Should have less amount in predicate contract after withdraw', async () => {
        const newContractBalance = await dummyERC20.balanceOf(contracts.erc20Vault.address)
        newContractBalance.gte(contractBalance.sub(withdrawAmount)).should.equal(true)
      })
    })
    describe('Ether WITHDRAW', () => {
      it('Should start Ether withdraw', async () => {
        // start exit
        exitTx = await contracts.rootChainManager.connect(depositReceiverEth).withdraw(withdrawTxs[1], merkletree.getHexProof(encode(withdrawTxs[1])))
        exitReceipt = await exitTx.wait()
        should.exist(exitTx)
      })

      it('Should fail: start exit again', async () => {
        await expectRevert(contracts.rootChainManager.connect(depositReceiverEth).withdraw(withdrawTxs[1], merkletree.getHexProof(encode(withdrawTxs[1]))), 'RootChainManager: exit already processed')
      })

      it('Should emit WithdrawEvent log in exit tx', () => {
        const logs = logDecoder.decodeLogs(exitReceipt.logs)
        const exitTransferLog = logs.find(l => l.event === 'WithdrawEvent')
        should.exist(exitTransferLog)
      })

      it('Should have more amount in withdrawer account after withdraw', async () => {
        const newAccountBalance = BigNumber.from(await web3.eth.getBalance(depositReceiverEth.address))

        newAccountBalance.gt(accountBalanceEth).should.equal(true)
      })

      it('Should have less amount in predicate contract after withdraw', async () => {
        const newContractBalance = BigNumber.from(await web3.eth.getBalance(contracts.erc20Vault.address))
        newContractBalance.gte(contractBalanceEth.sub(depositAmount3)).should.equal(true)
      })
    })
    describe('Supports using the previous merkle root', () => {
      it('should support withdraw with previous root', async () => {
        let tx = await contracts.rootChainManager.setMerkleRoot(encode(withdrawTxs[0]))
        should.exist(tx)

        tx = await contracts.rootChainManager.withdraw(withdrawTxs[2], merkletree.getHexProof(encode(withdrawTxs[2])))
        should.exist(tx)
      })
    })

    describe('Supports using withdraw fee', () => {
      const withdrawFee = mockValues.amounts[0]

      let newAccount: SignerWithAddress
      before(async () => {
        // @ts-expect-error
        newAccount = ethers.Wallet.createRandom() as SignerWithAddress
      })
      it('should set treasure and fee', async () => {
        await rootChainManager.setDepositFee(mockValues.amounts[1])
        await rootChainManager.setWithdrawFee(withdrawFee)
        await rootChainManager.setTreasurer(newAccount.address)
      })
      it('transaction should revert', async () => {
        await expectRevert(contracts.rootChainManager.withdraw(
          withdrawTxs[3],
          merkletree.getHexProof(encode(withdrawTxs[3])),
          {
            value: withdrawFee.div(2)
          }),
        'RootChainManager: ETH send needs to be at least withdrawFee')
      })
      it('transaction should work', async () => {
        await contracts.rootChainManager.withdraw(
          withdrawTxs[3],
          merkletree.getHexProof(encode(withdrawTxs[3])),
          {
            value: withdrawFee
          }
        )

        const balance = BigNumber.from(await web3.eth.getBalance(newAccount.address))
        balance.should.equal(withdrawFee)
      })
    })

    describe('It fails correctly if token has been unmapped', () => {
      const withdrawFee = mockValues.amounts[0]

      it('should throw an error', async () => {
        await contracts.rootChainManager.cleanMapToken(contracts.dummyERC20.address, mockChildIndex, mockChildSubIndex)
        await expectRevert(contracts.rootChainManager.withdraw(
          withdrawTxs[4],
          merkletree.getHexProof(encode(withdrawTxs[4])),
          {
            value: withdrawFee
          }
        ), 'RootChainManager: token not mapped')
      })
    })
  })
})
