import type { NextPage } from "next";
import Transfer from "@components/templates/transfer";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { routes } from "src/constants/routes";
import { useTransactionFlowStore } from "src/store/transaction-flow";
import addresses from "@config/addresses";
import { useAsyncMemo } from "@hooks/utils";
import useRootManagerContract from "src/contracts/use-root-manager";
import useEthWallet from "@hooks/use-eth-wallet";
import useGenerateContract from "src/contracts/use-generate-contract";
import { ethers } from "ethers";
import { noOp } from "src/helpers/basic";
import { MaxTransferValue } from "@components/templates/transfer/Transfer";

const Deposit: NextPage = () => {
    const { prefetch } = useRouter();
    const { token } = useTransactionFlowStore();
    const { estimateGas } = useRootManagerContract();
    const { context } = useEthWallet();
    const { getBalance } = useGenerateContract(
        token?.eth_address || "", // address or empty string because the address is undefined on first renders
        !!token // plus it's disabled on the first render anyway
    );

    const maxTransferValue: MaxTransferValue | undefined = useAsyncMemo(
        async () => {
            if (token?.eth_address !== addresses.eth || !context.account) {
                return undefined;
            }

            const ethBalance = await getBalance();
            const gasFeeFractional = await estimateGas(ethBalance, token, "deposit");

            if (!gasFeeFractional) {
                return undefined;
            }

            const gasFee = ethers.utils.parseEther(gasFeeFractional).toBigInt();

            // Double the gas estimate as an attempt to align with metamask max estimate (for the most commonly used gas setting).
            // This max estimate seems to vary a lot (anywhere between 130% to 200% of the "base fee").
            const conservativeGasFee = gasFee * 2n;
            const max = ethBalance - conservativeGasFee;
            return { value: max > 0n ? max : 0n, deductedFee: conservativeGasFee };
        },
        noOp,
        [token?.eth_address, context.account]
    );

    useEffect(() => {
        prefetch(routes.deposit.overview);
        prefetch(routes.withdraw.path);
    }, [prefetch]);

    return <Transfer isDeposit maxTransferValue={maxTransferValue} />;
};

export default Deposit;
