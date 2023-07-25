import { ethers, web3 } from 'hardhat'
import { MockErc20, MockErc20__factory, RootChainManager, RootChainManager__factory } from '../typechain-types'

import { config } from 'dotenv'
config({
  path: '.env'
})
const sleep = async (milliseconds: number): Promise<void> => {
  return await new Promise(resolve => setTimeout(resolve, milliseconds))
}
async function waitTx (tx: any): Promise<void> {
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
let rootChainManager: RootChainManager

async function createToken (): Promise<MockErc20> {
  if (process.env.MOCK_ERC_20 != null) {
    return MockErc20Factory.attach(process.env.MOCK_ERC_20)
  }
  console.log('Deploying contract StateSender')

  const token = await MockErc20Factory.deploy()
  console.log(token.address)
  await token.deployed()

  return token
}

async function setupToken (token: MockErc20): Promise<void> {
  const tx = await rootChainManager.mapToken(
    token.address,
    625,
    0,
    '0xa234e09165f88967a714e2a476288e4c6d88b4b69fe7c300a03190b858990bfc'
  )

  await waitTx(tx)
  // await sleep(120000)
  // await run("verify:verify", {
  //     address: token.address,
  //     constructorArguments: [
  //     ]
  // });
}
async function main (): Promise<void> {
  MockErc20Factory = (await ethers.getContractFactory('MockErc20'))
  RootChainManagerFactory = (await ethers.getContractFactory('RootChainManager'))
  if (process.env.ROOT_MANAGER_ADDRESS != null) {
    rootChainManager = RootChainManagerFactory.attach(process.env.ROOT_MANAGER_ADDRESS)
  }
  const token = await createToken()
  await setupToken(token)
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
