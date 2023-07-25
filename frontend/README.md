# ETH-CCD bridge frontend

This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Dependencies

-   NodeJS stable v18.12 (easiest to manage this through [NVM](https://github.com/nvm-sh/nvm))
-   [Yarn classic stable](https://classic.yarnpkg.com/en/docs/install)
    -   If using NVM to manage node versions, it might be best to NOT use node to install yarn but instead go with an
        alternative solution like a package manager for your operating system (like brew or apt).

## Installation

```bash
yarn
```

## Running

First, run the nextjs development server:

```bash
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) (alternatively, the URL logged in your console) with your browser to see the result.

[API routes](https://nextjs.org/docs/api-routes/introduction) can be accessed on `http://localhost:3000/api/*`.
The `pages/api` directory is mapped to `/api/*`. Files in this directory are treated as [API routes](https://nextjs.org/docs/api-routes/introduction) instead of React pages.

### Runtime configuration

The project depends on a number of configuration options that can be set through environment variables.

**For Ethereum**

-   `NEXT_PUBLIC_ETHEREUM_PROVIDER_NETWORK`: Ethereum network ID to use (number). As an example, Goerli test network has the 5 as its ID. 
-   `NEXT_PUBLIC_ETHEREUM_EXPLORER_URL`: Ethereum block explorer to use. This is expected to include a placeholder for a transaction hash in the form of `{}`, i.e `https://goerli.etherscan.io/tx/{}` for the Goerli test network.
-   `NEXT_PUBLIC_ROOT_MANAGER_ADDRESS`: Address of main Ethereum contract [RootChainManager](../ethereum_contracts/contracts/root).
-   `NEXT_PUBLIC_GENERATE_ETHER_PREDICATE_ADDRESS`: Address of Ethereum contract for generating predicate addresses for ETH.
-   `NEXT_PUBLIC_GENERATE_ERC20_PREDICATE_ADDRESS`: Address of Ethereum contract for generating predicate addresses for ERC20 tokens.
-   `NEXT_PUBLIC_WETH_TOKEN_ADDRESS`: Ethereum contract address for wETH token.

**For Concordium**

-   `NEXT_PUBLIC_NETWORK_GENESIS_BLOCK_HASH`: Hex encoded block hash of the genesis block of the target Concordium network.
-   `NEXT_PUBLIC_CCDSCAN_URL`: Concordium scan URL to use. This is expected to include a placeholder for a transactions hash in the form of `{}`, i.e. `https://testnet.ccdscan.io/?dcount=1&dentity=transaction&dhash={}` for Concordium public testnet.
-   `NEXT_PUBLIC_BRIDGE_MANAGER_INDEX`: Index of the Concordium [bridge manager contract](../concordium_contracts/bridge-manager).
-   `NEXT_PUBLIC_API_URL`: URL of the Concordium bridge API.
-   `NEXT_PUBLIC_BRIDGE_MANAGER`: Hex encoded contract schema for Concordium [bridge manager contract](../concordium_contracts/bridge-manager).
-   `NEXT_PUBLIC_CIS2_BRIDGEABLE`: Hex encoded contract schema for Concordium [cis2-bridgeable contract](../concordium_contracts/cis2-bridgeable).

### Docker

The project can be built with docker by executing the following with the necessary environment variables described above:

```bash
docker build -t eth-ccd-bridge-app .
```

The set of environment variables required to build for the eth-goerli/ccd-testnet network pair can be found in `compose.yaml`, which can be built with:

```bash
docker compose build
```

Running the docker container is done by executing:

```bash
docker run -p 3000:3000 eth-ccd-bridge-app
```

## Learn More

To learn more about Next.js, take a look at the following resources:

-   [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
-   [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.
