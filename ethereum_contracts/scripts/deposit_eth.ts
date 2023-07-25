import { ethers, web3 } from 'hardhat'
import { RootChainManager, RootChainManager__factory } from '../typechain-types'
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

let RootChainManagerFactory: RootChainManager__factory

let rootManager: RootChainManager

async function setupRootManager(): Promise<RootChainManager> {
  if (process.env.MOCK_ERC_20 != null && process.env.ROOT_MANAGER_ADDRESS != null) {
    return RootChainManagerFactory.attach(process.env.ROOT_MANAGER_ADDRESS)
  }
  throw new Error('PLEASE SET MOCK_ERC_20 ENV')
}
async function main(): Promise<void> {
  RootChainManagerFactory = (await ethers.getContractFactory('RootChainManager'))

  rootManager = await setupRootManager()
  const [user] = await ethers.getSigners()
  const amount = BigNumber.from(10).pow(16).mul(1)

  // const depositData = ethers.utils.defaultAbiCoder.encode(["uint256"], [amount]);

  const ccdUser = '0x' + bs58check.decode('43A45q6ohC5tsZ9nTZ7T5EniADtzgFBBYPE9A41Mjeq1vz2hev').slice(1).toString('hex')

  const tx = await rootManager.depositEtherFor(user.address, ccdUser, { gasLimit: 1000000, value: amount })
  await waitTx(tx)
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
