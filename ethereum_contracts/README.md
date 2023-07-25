
In order to install dependencies use:
```shell
npm install
npm install -g npx
```

Try running some of the following tasks:

To compile the smart contracts run the following command in the root folder:

```shell
npx hardhat compile
```

To cleanup the compiled contract folders run the following command in the root folder:

```shell
npx hardhat clean
```

To run the smart contract tests run the following command in the root folder:

```shell
npx hardhat test
```

To get a coverage report of the smart contract tests run the following command in the root folder:
```
npx hardhat coverage
```

To run the deployment script locally run the following command in the root folder:

```shell
npx hardhat run ./scripts/deploy_all.ts
```

To deploy the smart contracts to the goerli blockchain uncomment the `goerli` network in the `hardhat.config.ts` file, add the ETHEREUM_GOERLI_KEY, ALCHEMY_KEY, and ETHERSCAN_API_KEY to the `.env` file and run the following command in the root folder:

```shell
npx hardhat run ./scripts/deploy_all.ts --network goerli
```

To deploy the smart contracts to the Ehereum mainnet blockchain uncomment the `mainnet` network in the `hardhat.config.ts` file, add the ETHEREUM_MAINNET_KEY, ALCHEMY_KEY, and ETHERSCAN_API_KEY to the `.env` file and run the following command in the root folder:

```shell
npx hardhat run ./scripts/deploy_all.ts --network mainnet
```

## Linting

```shell
npm run eslint
npm run solhint
```