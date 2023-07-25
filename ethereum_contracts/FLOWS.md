# General overview of the smart contracts

There are 3 main categories for contracts:
 - RootChainManager. This the main contract. This is the main entry point for interacting with the bridge, all other contracts are not callable by external users
 - StateSender. This is a centralized place to emit events that are listened by the relayer
 - TokenVault. EtherVault, ERC20Vault, and more to come in the future. This are the contracts used to store the funds in the relayer


## User flows

There are two flows user's can use to interact with the contract:
- Deposit 
- Withdraw

### Deposit flow
Due to the special api of ETH there is a flow for depositing ETH and another flow for depositing other tokens. Currently only ERC20 is implemented. ERC721 and ERC1155 are planned for the future


Steps:
 1. User should first allow transfer of funds to the correct vault (Skipped for ETH). For example ERC20.
 2. User calls depositFor / depositEtherFor
 3. The RootChainManager validates that the token is correctly registered into the bridge. Then calls the Vault which transfer the funds and stores them. The RootChainManager emits a deposit event using the StateSender. 
 4. The relayer is responsible for watching the deposit events and then emitting tokens on the CCD chain

### Withdraw flow

This flow is initiated on the CCD chain and finalized on the ETH chain.

The relayer is responsible for keeping a Merkle tree with all pending withdraw TXs. The relayer will periodically update the root of the merkle tree on the RootChainManager


On the RootChainManager we store the last 2 merkle roots emitted by the relayer and we considered them both as being valid. This is done so that a merkle root update will not invalidate a user's inflight withdraw tx that was sent based on the previous root.


Complete flow:
1. User initiates the withdraw procedure on the CCD chain. CCD tokens are burned.
2. The relayer watches for withdraw events on the CCD chain and picks up the withdraw event.
3. Some time in the future the relayer will update the RootChainManager with a new Merkle root containing the CCD withdraw tx.
4. And this point the user is able to complete the withdraw process. By calling `withdraw` on the RootChainManger. The user needs to call the relayer api in order to retrieve the CCD tx data, as well as the merkle proof for that data.
5. The root chain manager verifies the withdraw data and the merkle proof. Afterwards it calls the vault in order to release tokens to the user.

## Admin Flows
### Map Tokens
This is the way to add new tokens to the bridge. The token address on ETH, the token address on CCD, and the vault type needs to be provided.

This requires a wallet with the MAPPER_ROLE

### CleanMap / ReMap Tokens
This required and ADMIN wallet on the contract. This functions should be used to remove a token from the bridge or to remap an existing token to a new token on CCD.

This functions should be very rarely used.
