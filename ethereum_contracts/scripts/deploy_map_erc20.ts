import { ethers, web3 } from 'hardhat'
import { ERC20Vault, ERC20Vault__factory, MockErc20, MockErc20__factory, RootChainManager, RootChainManager__factory } from '../typechain-types'

import { config } from 'dotenv'
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
let Erc20VaultFactory: ERC20Vault__factory
let rootChainManager: RootChainManager
let erc20Vault: ERC20Vault

async function setupToken(token: MockErc20): Promise<void> {
  const tokenType = await erc20Vault.TOKEN_TYPE()
  const tx = await rootChainManager.mapToken(
    token.address,
    1962,
    0,
    tokenType
  )

  await waitTx(tx)
}
async function main(): Promise<void> {
  MockErc20Factory = (await ethers.getContractFactory('MockErc20'))
  RootChainManagerFactory = (await ethers.getContractFactory('RootChainManager'))
  Erc20VaultFactory = (await ethers.getContractFactory('ERC20Vault'))
  if (process.env.ROOT_MANAGER_ADDRESS != null) {
    rootChainManager = RootChainManagerFactory.attach(process.env.ROOT_MANAGER_ADDRESS)
  }

  if (process.env.ERC20_PREDICATE_ADDRESS != null) {
    erc20Vault = Erc20VaultFactory.attach(process.env.ERC20_PREDICATE_ADDRESS)
  }
  let token: MockErc20
  if (process.env.MOCK_ERC_20 != null) {
    token = MockErc20Factory.attach(process.env.MOCK_ERC_20)
  } else {
    throw new Error('Token required in env')
  }
  await setupToken(token)
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
