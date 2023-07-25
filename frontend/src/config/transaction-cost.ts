import { ensureDefined } from "src/helpers/basic";

const transactionCosts = {
    eth: {
        rootManagerDepositOverheadGas: ensureDefined(
            process.env.NEXT_PUBLIC_ROOT_MANAGER_DEPOSIT_OVERHEAD_GAS,
            'Expected "NEXT_PUBLIC_ROOT_MANAGER_DEPOSIT_OVERHEAD_GAS" to be provided as an environment variable'
        ),
        rootManagerWithdrawErc20Gas: ensureDefined(
            process.env.NEXT_PUBLIC_ROOT_MANAGER_WITHDRAW_ERC20_GAS,
            'Expected "NEXT_PUBLIC_ROOT_MANAGER_WITHDRAW_ERC20_GAS" to be provided as an environment variable'
        ),
        rootManagerWithdrawEthGas: ensureDefined(
            process.env.NEXT_PUBLIC_ROOT_MANAGER_WITHDRAW_ETH_GAS,
            'Expected "NEXT_PUBLIC_ROOT_MANAGER_WITHDRAW_ETH_GAS" to be provided as an environment variable'
        ),
    },
    ccd: {
        bridgeManagerWithdrawEnergy: ensureDefined(
            process.env.NEXT_PUBLIC_BRIDGE_MANAGER_WITHDRAW_ENERGY,
            'Expected "NEXT_PUBLIC_BRIDGE_MANAGER_WITHDRAW_ENERGY" to be provided as an environment variable'
        ),
    },
};

export default transactionCosts;
