import { ensureDefined } from "src/helpers/basic";

const eth = ensureDefined(
    process.env.NEXT_PUBLIC_ETH_TOKEN_ADDRESS,
    "Expected NEXT_PUBLIC_ETH_TOKEN_ADDRESS to be provided as an environment variable"
);
const root = ensureDefined(
    process.env.NEXT_PUBLIC_ROOT_MANAGER_ADDRESS,
    "Expected NEXT_PUBLIC_ROOT_MANAGER_ADDRESS to be provided as an environment variable"
);
const erc20Vault = ensureDefined(
    process.env.NEXT_PUBLIC_GENERATE_ERC20_PREDICATE_ADDRESS,
    "Expected NEXT_PUBLIC_GENERATE_ERC20_PREDICATE_ADDRESS to be provided as an environment variable"
);

const bridgeManagerIndex = ensureDefined(
    process.env.NEXT_PUBLIC_BRIDGE_MANAGER_INDEX,
    "Expected NEXT_PUBLIC_BRIDGE_MANAGER_INDEX to be provided as an environment variable"
);

const addresses = {
    /**
     * ETH Address (Ethereum)
     */
    eth,
    /**
     * Root Manager Address (Ethereum)
     */
    root,
    /**
     * ERC20 vault ID, used to get the address of the ERC20 vault (Ethereum)
     */
    erc20Vault,
    /**
     * Bridge Manager Address (Concordium)
     */
    bridgeManager: {
        index: bridgeManagerIndex,
        subindex: process.env.NEXT_PUBLIC_BRIDGE_MANAGER_SUBINDEX ?? "0",
    },
};

export default addresses;
