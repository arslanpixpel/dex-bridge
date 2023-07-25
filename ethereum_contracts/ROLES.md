# Roles used in the smart contracts

## ALL Contracts - DEFAULT_ADMIN_ROLE

This role should only be owned by the deployer wallet / wallets. This wallets should be stored offline and 
should only be ever used to deploy / setup contracts

This role is also used when modifying the fees of the bridge. This should very rarely change.

This role is also used to deregister a token from the bridge.

This role allows granting any role to anyone. This is basically root.

## RootChainManager - MAPPER_ROLE

This role should be given to the wallet / wallets that are used for registering new tokens to the bridge.

This role should only be used to call `RootChainManager.mapToken()`


## RootChainManager - MERKLE_UPDATER

This role should be given to a wallet that is managed by the Rust relayer. This role
allows setting the merkle root that signs the current pending withdraw transactions.

With access to this wallet, an atacker will be able to drain vaults.


## TokenVault - MANAGER_ROLE

This is an internal role. Only the RootChainManager contract should have the MANAGER_ROLE on each TokenVault.

This role allows calls to the vault's functions

## StateSender - EMITTER_ROLE

This is an internal role. Only the RootChainManager contract should have the EMITTER_ROLE on the StateSender.

This role allows call to the StateSender functions. The StateSender emits all the events that are watched by 
the Rust Relayer
 
