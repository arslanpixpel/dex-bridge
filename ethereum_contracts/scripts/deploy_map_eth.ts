import { ethers, web3 } from 'hardhat'
import { EtherVault, EtherVault__factory, RootChainManager, RootChainManager__factory } from '../typechain-types'

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

let RootChainManagerFactory: RootChainManager__factory
let EtherVaultFactory: EtherVault__factory
let rootChainManager: RootChainManager
let etherVault: EtherVault

async function setupToken(): Promise<void> {
  const tokenType = await etherVault.TOKEN_TYPE()
  const tx = await rootChainManager.mapToken(
    '0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE',
    1960,
    0,
    tokenType
  )

  await waitTx(tx)
}
async function main(): Promise<void> {
  RootChainManagerFactory = (await ethers.getContractFactory('RootChainManager'))
  EtherVaultFactory = (await ethers.getContractFactory('EtherVault'))
  if (process.env.ROOT_MANAGER_ADDRESS != null) {
    rootChainManager = RootChainManagerFactory.attach(process.env.ROOT_MANAGER_ADDRESS)
  }

  if (process.env.ETHER_PREDICATE_ADDRESS != null) {
    etherVault = EtherVaultFactory.attach(process.env.ETHER_PREDICATE_ADDRESS)
  }
  await setupToken()
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
