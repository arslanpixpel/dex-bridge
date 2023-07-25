import type { NextPage } from "next";
import TransferOverview, {
    TransferOverviewLine,
    useTransferOverviewStatusState,
} from "@components/templates/transfer-overview";
import { ContractTransaction, errors } from "ethers";
import addresses from "@config/addresses";
import useRootManagerContract from "src/contracts/use-root-manager";
import { routes } from "src/constants/routes";
import useGenerateContract from "src/contracts/use-generate-contract";
import { useTransactionFlowStore } from "src/store/transaction-flow";
import { useRouter } from "next/router";
import { FC, useEffect, useState } from "react";
import { useAsyncMemo } from "@hooks/utils";
import { noOp } from "src/helpers/basic";
import { getPrice } from "src/helpers/price-usd";
import { Components } from "src/api-query/__generated__/AxiosClient";
import { renderGasFeeEstimate } from "src/helpers/fee";
import { useSubmittedDepositsStore } from "src/store/submitted-transactions";
import useEthWallet from "@hooks/use-eth-wallet";

const LINE_DETAILS_FALLBACK = "...";

type ApproveAllowanceLineProps = {
    token: Components.Schemas.TokenMapItem;
    needsAllowance: boolean | undefined;
    ethPrice: number;
    erc20Address: string | undefined;
};

const ApproveAllowanceLine: FC<ApproveAllowanceLineProps> = ({ token, needsAllowance, ethPrice, erc20Address }) => {
    const [error, setError] = useState<string>();
    const { estimateApprove } = useGenerateContract(token.eth_address as string, true);
    /**
     * Gets the gas fee required to make the deposit.
     * Throws `Error` if user rejected in the ethereum wallet
     */
    const gasFee = useAsyncMemo(
        async (): Promise<number | undefined> => {
            if (!erc20Address) {
                return undefined;
            }

            try {
                const gas = await estimateApprove(erc20Address);
                return parseFloat(gas as string);
            } catch (error) {
                // TODO: log actual error
                setError("Could not estimate cost");
            }
        },
        noOp,
        [estimateApprove, erc20Address]
    );

    return (
        <TransferOverviewLine
            isEth
            title={`Approve ${token.eth_name} allowance`}
            completed={needsAllowance === false}
            details={(gasFee !== undefined && renderGasFeeEstimate(gasFee, ethPrice)) || error || LINE_DETAILS_FALLBACK}
        />
    );
};

type DepositLineProps = {
    amount: bigint;
    token: Components.Schemas.TokenMapItem;
    hasAllowance: boolean;
    ethPrice: number;
    tokenVaultAddress: string | undefined;
};

const DepositLine: FC<DepositLineProps> = ({ amount, token, hasAllowance, ethPrice, tokenVaultAddress }) => {
    const [error, setError] = useState<string>();
    const { estimateGas } = useRootManagerContract();
    const { estimateTransferWithDepositOverhead } = useGenerateContract(token.eth_address as string, true);
    /**
     * Gets the gas fee required to make the deposit.
     * Throws `Error` if user rejected in the ethereum wallet
     */
    const gasFee = useAsyncMemo(
        async (): Promise<number | undefined> => {
            if (!amount || !token) {
                return undefined;
            }

            try {
                let gas: string | undefined;
                if (!hasAllowance && tokenVaultAddress !== undefined) {
                    gas = await estimateTransferWithDepositOverhead(amount, tokenVaultAddress);
                } else {
                    gas = await estimateGas(amount, token, "deposit");
                }
                return parseFloat(gas as string);
            } catch (error) {
                // TODO: log actual error
                setError("Could not estimate cost");
            }
        },
        noOp,
        [amount, token, hasAllowance, estimateGas]
    );

    return (
        <TransferOverviewLine
            isEth
            title={`Deposit ${token.eth_name}`}
            details={(gasFee !== undefined && renderGasFeeEstimate(gasFee, ethPrice)) || error || LINE_DETAILS_FALLBACK}
        />
    );
};

const DepositOverview: NextPage = () => {
    const { context } = useEthWallet();
    const { amount, token } = useTransactionFlowStore();
    const { checkAllowance, hasAllowance } = useGenerateContract(
        token?.eth_address as string, // address or empty string because the address is undefined on first renders
        !!token && !!amount // plus it's disabled on the first render anyway
    );
    const { typeToVault, depositFor, depositEtherFor } = useRootManagerContract();
    const [needsAllowance, setNeedsAllowance] = useState<boolean | undefined>(
        token?.eth_address !== addresses.eth ? undefined : false
    );
    const allowanceLoading = needsAllowance === undefined;
    const { prefetch } = useRouter();
    const { status, setInfo, setError } = useTransferOverviewStatusState();
    const { replace } = useRouter();
    const { add: addSubmitted } = useSubmittedDepositsStore();
    const ethPrice = useAsyncMemo(async () => getPrice("ETH"), noOp, []) ?? 0;
    const isErc20 = token?.eth_address !== addresses.eth;
    const erc20PredicateAddress = useAsyncMemo(async () => (isErc20 ? typeToVault() : undefined), noOp, [token]);
    const [pendingSignature, setPendingSignature] = useState(false);

    useEffect(() => {
        if (!isErc20) {
            setNeedsAllowance(false);
        } else if (erc20PredicateAddress) {
            hasAllowance(erc20PredicateAddress).then((allowance) => setNeedsAllowance(!allowance));
        }
    }, [hasAllowance, erc20PredicateAddress, isErc20]);

    useEffect(() => {
        if (!amount || !token) {
            replace(routes.deposit.path);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const getAllowance = async (): Promise<boolean> => {
        if (erc20PredicateAddress === undefined || !needsAllowance) {
            return false;
        }

        try {
            setPendingSignature(true);
            setInfo("Requesting allowance from Ethereum wallet.");
            const tx = await checkAllowance(erc20PredicateAddress);
            setPendingSignature(false);

            setInfo("Waiting for transaction to finalize");
            await tx.wait(1);

            setNeedsAllowance(false);
            return true;
        } catch {
            // TODO: log actual error
            setError("Allowance request rejected");
            setPendingSignature(false);

            return false;
        }
    };

    // Check necessary values are present from transfer page. These will not be available if this is the first page loaded in the browser.
    if (!amount || !token) {
        return null;
    }

    /**
     * Handles submission of the deposit transaction.
     */
    const onSubmit = async (): Promise<string | undefined> => {
        if (!context.account) {
            throw new Error("Expected eth account to be available");
        }
        if (needsAllowance && !(await getAllowance())) {
            return undefined;
        }

        try {
            setPendingSignature(true);
            setInfo("Awaiting signature in Ethereum wallet for deposit");
            let tx: ContractTransaction;
            if (token.eth_address === addresses.eth) {
                tx = await depositEtherFor(amount);
            } else {
                tx = await depositFor(amount, token); //deposit
            }
            setPendingSignature(false);

            prefetch(routes.deposit.tx(tx.hash));

            setInfo("Waiting for transaction to finalize");
            await tx.wait(1); // wait for confirmed transaction
            addSubmitted(context.account, tx.hash.replace("0x", ""), amount, token);

            return tx.hash;
        } catch (error) {
            setPendingSignature(false);
            // TODO: log actual error
            if (error.message.includes(errors.ACTION_REJECTED)) {
                setError("Transaction was rejected");
            } else {
                setError(error.message);
            }
        }
    };

    return (
        <TransferOverview
            title="Deposit overview"
            handleSubmit={onSubmit}
            timeToComplete="Deposit may take up to 5 minutes to complete."
            status={status}
            pendingWalletSignature={pendingSignature}
            isDeposit
        >
            {isErc20 && (
                <>
                    <ApproveAllowanceLine
                        token={token}
                        erc20Address={erc20PredicateAddress}
                        ethPrice={ethPrice}
                        needsAllowance={needsAllowance}
                    />
                    <br />
                </>
            )}
            <DepositLine
                amount={amount}
                token={token}
                hasAllowance={!needsAllowance && !allowanceLoading}
                tokenVaultAddress={erc20PredicateAddress}
                ethPrice={ethPrice}
            />
        </TransferOverview>
    );
};

export default DepositOverview;
