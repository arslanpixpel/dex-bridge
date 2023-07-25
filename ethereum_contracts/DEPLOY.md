# Deploy script

There is the `deploy_all.ts` script which will deploy all the contracts and do the necessary configurations.

If any contracts are already deployed they can be provided via environment variables (see `.env.sample`). In this case, no new contract will be deployed, only configuration. _Please Note_ that the proxy address of the contracts needs to be provided, not the implementation address

Sample script command:

`npx hardhat run --network goerli ./scripts/deploy_all.ts`

# Deploy Flow

This section details about the steps in the deploy flow

1. Deploy a ProxyAdmin. This is the contract that has permissions to upgrade contracts

2. Deploy the implementation of RootChainManager, Erc20Vault, EthVault, StateSender

3. Deploy the upgradable proxies for the above contracts and initialize them. The constructor of a proxy are the following impelementation address, proxy admin address, payload for the initialize function on the implementation contract. Currently this is the account which will receive the DEFAULT_ADMIN_ROLE (the wallet that is running the script). Sample code for deploying a proxy:

`await StateSenderFactoryProxy.deploy( stateSender.address, proxyAdmin.address, stateSender.interface.encodeFunctionData('initialize', [owner.address]) )`

4. Configure the root chain manager

- `setStateSender` is called to set the `StateSender`
- `registerVault` is called to register `EtherVault` and `Erc20Vault`

5. Role permissions are set:

- `StateSender` must grant `EMITTER_ROLE` to `RootChainManager`
- `Erc20Vault` and `EtherVault` must grant `MANAGER_ROLE` to `RootChainManager`

Deployed Kava:
StateSender - 0xb215e169dE9704a8921348c787996A6C64eAF58a
EtherVaultProxy - 0xf5F6BC48195Bc33Fd1098B5c7a59e944C8B102ad
ERC20VaultProxy - 0xFDc13cee803e8f2a556A736C7a9dbdc4FE49A0dE
ProxyAdmin - 0xf25B11e8b29e6150EaDb428FF0d035ea9beACe03
RootChainManagerProxy - 0x8469d2a9910251d990bCdb73fF61C4fBfBdBc8Ce
