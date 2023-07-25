import { ethers, web3 } from 'hardhat'
import { MockErc20, MockErc20__factory, RootChainManager, RootChainManager__factory } from '../typechain-types'
import bs58check = require('bs58check')
import { config } from 'dotenv'
import { BigNumber } from 'ethers'
config({
  path: '.env'
})
const sleep = async (milliseconds: number): Promise<void> => {
  return await new Promise(resolve => setTimeout(resolve, milliseconds))
}
async function waitTx(tx: any): Promise<void> {
  let transactionReceipt = null
  while (transactionReceipt == null) { // Waiting expectedBlockTime until the transaction is mined
    transactionReceipt = await web3.eth.getTransactionReceipt(tx.hash)
    await sleep(2500)
    console.log('waiting transaction mined please be patient')
  }
  if (!transactionReceipt.status) {
    console.error(transactionReceipt)
    throw Error('TX failed')
  }
}

let MockErc20Factory: MockErc20__factory
let RootChainManagerFactory: RootChainManager__factory

let mockErc20: MockErc20
let rootManager: RootChainManager

async function setupToken(): Promise<MockErc20> {
  if (process.env.MOCK_ERC_20 != null) {
    return MockErc20Factory.attach(process.env.MOCK_ERC_20)
  }
  throw new Error('PLEASE SET MOCK_ERC_20 ENV')
}

async function setupRootManager(): Promise<RootChainManager> {
  if (process.env.MOCK_ERC_20 != null && process.env.ROOT_MANAGER_ADDRESS != null) {
    return RootChainManagerFactory.attach(process.env.ROOT_MANAGER_ADDRESS)
  }
  throw new Error('PLEASE SET MOCK_ERC_20 ENV')
}
async function main(): Promise<void> {
  MockErc20Factory = (await ethers.getContractFactory('MockErc20'))
  RootChainManagerFactory = (await ethers.getContractFactory('RootChainManager'))

  mockErc20 = await setupToken()
  rootManager = await setupRootManager()
  const [user] = await ethers.getSigners()
  const amount = BigNumber.from(10).pow(5).mul(1)
  console.log(await mockErc20.balanceOf(user.address))
  if ((await mockErc20.balanceOf(user.address)).lt(amount)) {
    console.log('Minting extra tokens to ', user.address)
    const tx = await mockErc20.mint(user.address, amount)
    await waitTx(tx)
  }
  if (process.env.ERC20_PREDICATE_ADDRESS == null || process.env.ERC20_PREDICATE_ADDRESS == null) {
    throw new Error('ERC20_PREDICATE_ADDRESS and ERC20_PREDICATE_ADDRESS must be provided')
  }
  if ((await mockErc20.allowance(user.address, process.env.ERC20_PREDICATE_ADDRESS)).lt(amount)) {
    console.log('Approving spending of tokens by the rootManager')
    const tx = await mockErc20.approve(process.env.ERC20_PREDICATE_ADDRESS, amount)
    await waitTx(tx)
  }
  const depositData = ethers.utils.defaultAbiCoder.encode(['uint256'], [amount])

  const ccdUser = '0x' + bs58check.decode('43A45q6ohC5tsZ9nTZ7T5EniADtzgFBBYPE9A41Mjeq1vz2hev').slice(1).toString('hex')

  const tx = await rootManager.depositFor(user.address, ccdUser, mockErc20.address, depositData, { gasLimit: 1000000 })
  await waitTx(tx)
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
