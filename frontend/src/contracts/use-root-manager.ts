import { BigNumber, ContractTransaction, ethers } from "ethers";
import useEthWallet from "../hooks/use-eth-wallet";
import ROOTMANAGER_ABI from "./abis/ROOTMANAGER_ABI.json";
import bs58check from "bs58check";
import addresses from "@config/addresses";
import { Components } from "src/api-query/__generated__/AxiosClient";
import useCCDWallet from "@hooks/use-ccd-wallet";
import transactionCosts from "@config/transaction-cost";

const useRootManagerContract = () => {
    const { context } = useEthWallet();
    const {
        ccdContext: { account: ccdAccount },
    } = useCCDWallet();

    const enabled = !!ccdAccount;
    const ccdUser = enabled
        ? "0x" + Buffer.from(Uint8Array.prototype.slice.call(bs58check.decode(ccdAccount || ""), 1)).toString("hex")
        : "";

    const typeToVault = async (): Promise<string> => {
        if (!context.library || !enabled) return "";

        const signer = context.library.getSigner();
        const rootContract = new ethers.Contract(addresses.root, ROOTMANAGER_ABI, signer);
        const typeToVault = await rootContract.typeToVault(addresses.erc20Vault);

        return typeToVault;
    };

    const depositFor = async (
        amount: bigint,
        selectedToken: Components.Schemas.TokenMapItem
    ): Promise<ContractTransaction> => {
        if (!context.library || !enabled || !ccdUser) {
            throw new Error("Expected deposit dependecies to be available");
        }

        const signer = context.library.getSigner();
        const rootContract = new ethers.Contract(addresses.root, ROOTMANAGER_ABI, signer);

        const depositData = ethers.utils.defaultAbiCoder.encode(["uint256"], [amount.toString()]);

        return rootContract.depositFor(context.account, ccdUser, selectedToken.eth_address, depositData);
    };

    const depositEtherFor = async (amount: bigint): Promise<ContractTransaction> => {
        if (!context.library || !enabled || !ccdUser) {
            throw new Error("Expected deposit dependecies to be available");
        }

        const signer = context.library.getSigner();
        const rootContract = new ethers.Contract(addresses.root, ROOTMANAGER_ABI, signer);

        return rootContract.depositEtherFor(context.account, ccdUser, { value: amount.toString() });
    };

    const withdraw = async (params: Components.Schemas.WithdrawParams, proof: string): Promise<ContractTransaction> => {
        if (!context.library) {
            throw new Error("Expected withdraw dependecies to be available");
        }

        const signer = context.library.getSigner();
        const rootContract = new ethers.Contract(addresses.root, ROOTMANAGER_ABI, signer);

        const partsLength = proof.length / 64;
        const parts = [];
        for (let i = 0; i < partsLength; i++) {
            parts.push("0x" + proof.substring(i * 64, (i + 1) * 64));
        }

        const parsedParams = {
            ccdIndex: params.ccd_index,
            ccdSubIndex: params.ccd_sub_index,
            amount: params.amount,
            userWallet: params.user_wallet,
            ccdTxHash: "0x" + params.ccd_tx_hash,
            ccdEventIndex: params.ccd_event_index,
            tokenId: params.token_id,
        };

        return rootContract.withdraw(parsedParams, parts);
    };

    const estimateGas = async (
        amount: bigint,
        selectedToken: Components.Schemas.TokenMapItem,
        type: "deposit" | "withdraw",
        params?: Components.Schemas.WithdrawParams,
        proof?: string
    ) => {
        if (!context.library) return;
        const provider = context.library.getSigner();

        const rootContract = new ethers.Contract(addresses.root, ROOTMANAGER_ABI, provider);

        let gasLimit: BigNumber;
        if (type === "deposit") {
            const stringAmount = amount.toString();
            if (selectedToken.eth_address === addresses.eth) {
                gasLimit = await rootContract.estimateGas.depositEtherFor(context.account, ccdUser, {
                    value: stringAmount,
                });
            } else {
                const depositData = ethers.utils.defaultAbiCoder.encode(["uint256"], [stringAmount]);
                gasLimit = await rootContract.estimateGas.depositFor(
                    context.account,
                    ccdUser,
                    selectedToken.eth_address,
                    depositData,
                    {}
                );
            }
        } else {
            if (proof === undefined || params === undefined) {
                throw new Error("Expected both params and proof arguments to be defined");
            }

            const partsLength = proof.length / 64;
            const parts = [];
            for (let i = 0; i < partsLength; i++) {
                parts.push("0x" + proof.substring(i * 64, (i + 1) * 64));
            }

            const parsedParams = {
                ccdIndex: params.ccd_index,
                ccdSubIndex: params.ccd_sub_index,
                amount: params.amount,
                userWallet: params.user_wallet,
                ccdTxHash: "0x" + params.ccd_tx_hash,
                ccdEventIndex: params.ccd_event_index,
                tokenId: params.token_id,
            };

            gasLimit = await rootContract.estimateGas.withdraw(parsedParams, parts);
        }

        const gasPrice: BigNumber | undefined = await provider?.getGasPrice();

        if (!gasPrice) {
            throw new Error("Error getting gas price");
        }

        const estimate = gasPrice.mul(gasLimit);
        return ethers.utils.formatEther(estimate);
    };

    const getDefaultWithdrawEstimate = async (token: Components.Schemas.TokenMapItem) => {
        if (!context.library) return;
        const provider = context.library.getSigner();

        const gasLimit =
            token.eth_address === addresses.eth
                ? transactionCosts.eth.rootManagerWithdrawEthGas
                : transactionCosts.eth.rootManagerWithdrawErc20Gas;

        const gasPrice: BigNumber | undefined = await provider?.getGasPrice();
        if (!gasPrice) {
            throw new Error("Error getting gas price");
        }

        const withdrawEstimate = gasPrice.mul(gasLimit);
        return ethers.utils.formatEther(withdrawEstimate);
    };

    return {
        ccdUser,
        typeToVault,
        depositFor,
        depositEtherFor,
        withdraw,
        estimateGas,
        getDefaultWithdrawEstimate,
    };
};

export default useRootManagerContract;
