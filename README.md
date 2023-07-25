# ETH to Concordium bridge

This repository contains all components of a bridge between Ethereum and Concordium.
The bridge supports bridging any ERC20 compatible token, as well as the native ETH token, from Ethereum to a CIS2 token on Concordium, and withdrawing them back.

The bridge consists of the following 4 components

- [concordium contracts](./concordium_contracts) contains the implementation of Concordium contracts and their deployment scripts.
- [ethereum_contracts](./ethereum_contracts) contains the implementation of the Ethereum contracts and their corresponding deployment scripts.
- [relayer](./relayer) contains the implementation of the backend server (the relayer) and the api server that supports the frontend.
  The relayer is the component that monitors both chains and sends the necessary transactions to the relevant chain.
- [frontend](./frontend) is the UI component of the bridge. This is how the users will interact with the bridge to deposit and withdraw their tokens across it.

Each of the components has a README file with a more detailed explanation of the functionality, build requirements, and configuration options.
