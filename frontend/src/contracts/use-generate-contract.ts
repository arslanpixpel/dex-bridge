import addresses from "@config/addresses";
import transactionCosts from "@config/transaction-cost";
import { BigNumber, ContractTransaction, ethers } from "ethers";
import { toWei } from "../helpers/number";
import useEthWallet from "../hooks/use-eth-wallet";
import MOCK_ABI from "./abis/MOCK_ABI.json";

const ERC20_ALLOWANCE = "1000000000000";

const useGenerateContract = (address: string, enabled: boolean) => {
    const { context } = useEthWallet();
    const amountToApprove = toWei(ERC20_ALLOWANCE);
    const signer = context.library?.getSigner();
    // console.log(signer);

    const getFormattedGasPrice = async (gas: BigNumber) => {
        const gasPrice: BigNumber | undefined = await signer.getGasPrice();

        if (!gasPrice) {
            throw new Error("Error getting gas price");
        }

        const estimatedGasPrice = gasPrice.mul(gas);
        return ethers.utils.formatEther(estimatedGasPrice);
    };

    const getBalance = async (): Promise<bigint> => {
        if (!enabled || !address || !context.library) throw new Error("Expected context not available");
        const provider = context.library;

        let balance;

        if (address === addresses.eth) {
            balance = await provider.getBalance(context.account);
        } else {
            const generatedContract = new ethers.Contract(address, MOCK_ABI, provider);
            balance = await generatedContract.balanceOf(context.account);
        }

        return BigInt(balance);
    };

    const hasAllowance = async (erc20PredicateAddress: string): Promise<boolean> => {
        if (!enabled || !erc20PredicateAddress || !address || !context.library) {
            throw new Error("Expected necessary parameters to be available");
        }

        const signer = context.library.getSigner();
        const generatedContract = new ethers.Contract(address, MOCK_ABI, signer);

        const response: BigNumber = await generatedContract.allowance(
            // Owner
            context.account,
            // Spender
            erc20PredicateAddress
        );

        // 0x00 is the hex value of the `BigNumber` from the response if an allowance hasn't been approved yet.
        return response._hex !== "0x00";
    };

    const estimateApprove = async (erc20PredicateAddress: string) => {
        if (!enabled || !erc20PredicateAddress || !address || !signer) return;

        const generatedContract = new ethers.Contract(address, MOCK_ABI, signer);
        const estimatedGas = await generatedContract.estimateGas.approve(
            // Spender
            erc20PredicateAddress,
            // Amount
            amountToApprove
        );

        return getFormattedGasPrice(estimatedGas);
    };

    const estimateTransferWithDepositOverhead = async (amount: bigint, vault: string) => {
        if (!enabled || !address || !signer) return;

        const generatedContract = new ethers.Contract(address, MOCK_ABI, signer);
        const depositData = ethers.utils.defaultAbiCoder.encode(["uint256"], [amount.toString()]);
        const estimatedGas = await generatedContract.estimateGas.transfer(
            // Spender
            vault,
            // Amount
            depositData
        );
        const withManagerOverhead = estimatedGas.add(transactionCosts.eth.rootManagerDepositOverheadGas);

        return getFormattedGasPrice(withManagerOverhead);
    };

    const approve = async (erc20PredicateAddress: string) => {
        if (!enabled || !erc20PredicateAddress || !address || !context.library) return;
        const signer = context.library.getSigner();

        const generatedContract = new ethers.Contract(address, MOCK_ABI, signer);

        const approve = await generatedContract.approve(
            // Spender
            erc20PredicateAddress,
            // Amount
            amountToApprove
        );
        return approve;
    };

    /**
     * Returns `ContractTransaction` or undefined if allowance has already been given.
     * Throws if necessary parameters are not available when function is invoked.
     */
    const checkAllowance = async (erc20PredicateAddress: string): Promise<ContractTransaction> => {
        if (!enabled || !erc20PredicateAddress) {
            throw new Error("Expected necessary parameters to be available");
        }

        try {
            return await approve(erc20PredicateAddress);
        } catch (err) {
            throw new Error("You need to approve token spending");
        }
    };

    return {
        hasAllowance,
        approve,
        estimateApprove,
        estimateTransferWithDepositOverhead,
        getBalance,
        checkAllowance,
    };
};

export default useGenerateContract;
