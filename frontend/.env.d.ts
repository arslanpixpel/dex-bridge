declare global {
    namespace NodeJS {
        interface ProcessEnv {
            // Ethereum contract addresses
            // NEXT_PUBLIC_WETH: string; // TODO: Doesn't seem like this is used? Can this be removed?
            /**
             * Address of the bridge root manager contract on Ethereum.
             */
            NEXT_PUBLIC_ROOT_MANAGER_ADDRESS: string;
            NEXT_PUBLIC_GENERATE_ETHER_PREDICATE_ADDRESS: string; // TODO: What's this? Add documentation
            NEXT_PUBLIC_GENERATE_ERC20_PREDICATE_ADDRESS: string; // TODO: Add documentation
            // NEXT_PUBLIC_STATE_SENDER_ADDRESS: string; // TODO: Doesn't seem like this is used? Can this be removed?
            // NEXT_PUBLIC_PROXY_ADMIN_ADDRESS: string; // TODO: Doesn't seem like this is used? Can this be removed?

            // Ethereum token contract addresses
            /**
             * Address of the ETH ethereum contract
             */
            NEXT_PUBLIC_ETH_TOKEN_ADDRESS: string;

            // Providers
            /**
             * The ethereum network ID represented by an integer.
             */
            NEXT_PUBLIC_ETHEREUM_PROVIDER_NETWORK: string;
            /**
             * The ethereum block explorer corresponding to the provided network ID
             */
            NEXT_PUBLIC_ETHEREUM_EXPLORER_URL: string; // TODO: What's this? Document...
            // NEXT_PUBLIC_ETHEREUM_PROVIDER_KEY: string; // TODO: What's this? Document...
            // NEXT_PUBLIC_INFURA_PROJECT_ID: string; // TODO: What's this? Document...

            /**
             * Block hash of genesis block of the network.
             * Is used to check that the user has its browser wallet connected to the correct network.
             */
            NEXT_PUBLIC_NETWORK_GENESIS_BLOCK_HASH: string;
            /**
             * URL of concordium node to use, e.g. http://127.0.0.1
             */
            NEXT_PUBLIC_CCD_NODE_URL: string;
            /**
             * Port of gRPC v2 interface of Concordium node, e.g. 20000
             */
            NEXT_PUBLIC_CCD_NODE_PORT: string;


            // Concordium contract addresses
            /**
             * Bridge manager contract index.
             */
            NEXT_PUBLIC_BRIDGE_MANAGER_INDEX: string;
            /**
             * Bridge manager contract subindex.
             */
            NEXT_PUBLIC_BRIDGE_MANAGER_SUBINDEX: string;


            /**
             * URL of the bridge API.
             */
            NEXT_PUBLIC_API_URL: string;
            /**
             * URL of ccdscan for concordium network.
             */
            NEXT_PUBLIC_CCDSCAN_URL: string;

            // CCD contract schemas
            /**
             * Hex encoded schema of the bridge manager contract.
             */
            NEXT_PUBLIC_BRIDGE_MANAGER: string;
            /**
             * Hex encoded schema of the cis2-bridgeable contract.
             */
            NEXT_PUBLIC_CIS2_BRIDGEABLE: string;

            /**
             * Approximate difference in the cost (in eth gas units) of invoking `root-manager.deposit` and `erc20-token.transfer`.
             *
             * This is needed as we cannot estimate the gas required for `root-manager.deposit` when 
             * the root manager does not have an allowance of the specific ERC2 token.
             */
            NEXT_PUBLIC_ROOT_MANAGER_DEPOSIT_OVERHEAD_GAS: string;
            /**
             * Approximate cost (in eth gas units) of invoking `root-manager.withdraw` for an ERC20 token.
             * This is needed as we cannot estimate the gas required for `root-manager.withdraw` prior to having the necessary parameters available.
             */
            NEXT_PUBLIC_ROOT_MANAGER_WITHDRAW_ERC20_GAS: string;
            /**
             * Approximate cost (in eth gas units) of invoking `root-manager.withdraw` for an ETH token.
             * This is needed as we cannot estimate the gas required for `root-manager.withdraw` prior to having the necessary parameters available.
             */
            NEXT_PUBLIC_ROOT_MANAGER_WITHDRAW_ETH_GAS: string;
            /**
             * Approximate energy needed to invoke `cis2-bridgeable.withdraw`.
             *
             * This is needed as we cannot estimate the gas required for `bridge-manager.withdraw` when 
             * the bridge manager is not currently an operator of the account for the selected token.
             */
            NEXT_PUBLIC_BRIDGE_MANAGER_WITHDRAW_ENERGY: string;
        }
    }
}

// If this file has no import/export statements (i.e. is a script)
// convert it into a module by adding an empty export statement.
export {};
