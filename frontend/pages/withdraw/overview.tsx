import type { NextPage } from "next";
import TransferOverview, {
    TransferOverviewLine,
    useTransferOverviewStatusState,
} from "@components/templates/transfer-overview";
import { FC, useEffect, useMemo, useState } from "react";
import useCCDWallet from "@hooks/use-ccd-wallet";
import useCCDContract from "src/contracts/use-ccd-contract";
import { routes } from "src/constants/routes";
import useEthWallet from "@hooks/use-eth-wallet";
import { useRouter } from "next/router";
import { useNextMerkleRoot } from "src/api-query/queries";
import moment from "moment";
import { useTransactionFlowStore } from "src/store/transaction-flow";
import { Components } from "src/api-query/__generated__/AxiosClient";
import { ensureDefined, noOp } from "src/helpers/basic";
import { useAsyncMemo } from "@hooks/utils";
import { getPrice } from "src/helpers/price-usd";
import { getEnergyToMicroCcdRate, waitForTransactionFinalization } from "src/helpers/ccd-node";
import transactionCosts from "@config/transaction-cost";
import useRootManagerContract from "src/contracts/use-root-manager";
import { renderEnergyFeeEstimate, renderGasFeeEstimate } from "src/helpers/fee";
import Text from "src/components/atoms/text/text";
import { useSubmittedWithdrawalsStore } from "src/store/submitted-transactions";

const LINE_DETAILS_FALLBACK = "...";

const withdrawEnergyDefault = BigInt(transactionCosts.ccd.bridgeManagerWithdrawEnergy);

type ApprovalAllowanceLineProps = {
    hasAllowance: boolean;
    token: Components.Schemas.TokenMapItem;
    ccdPrice: number;
    microCcdPerEnergy: bigint | undefined;
};

const ApprovalAllowanceLine: FC<ApprovalAllowanceLineProps> = ({
    hasAllowance,
    token,
    ccdPrice,
    microCcdPerEnergy,
}) => {
    const { ccdContext } = useCCDWallet();
    const { estimateApprove } = useCCDContract(ccdContext.account, !!ccdContext.account);
    const [error, setError] = useState<string>();
    const microCcdFee = useAsyncMemo(
        async () => {
            if (microCcdPerEnergy === undefined) {
                return undefined;
            }

            const energy = await estimateApprove(token);
            if (energy === undefined) {
                return undefined;
            }

            return microCcdPerEnergy * energy.exact;
        },
        () => setError("Could not get fee estimate"),
        [token, microCcdPerEnergy]
    );

    const details = useMemo(
        () =>
            microCcdFee !== undefined ? renderEnergyFeeEstimate(microCcdFee, ccdPrice) : error || LINE_DETAILS_FALLBACK,
        [microCcdFee, ccdPrice, error]
    );

    return (
        <TransferOverviewLine title={`Add operator for ${token.ccd_name}`} details={details} completed={hasAllowance} />
    );
};

type WithdrawLineProps = {
    hasAllowance: boolean;
    token: Components.Schemas.TokenMapItem;
    amount: bigint;
    ethAccount: string;
    ccdPrice: number;
    microCcdPerEnergy: bigint | undefined;
};

const WithdrawLine: FC<WithdrawLineProps> = ({
    token,
    amount,
    ethAccount,
    ccdPrice,
    microCcdPerEnergy,
    hasAllowance,
}) => {
    const { ccdContext } = useCCDWallet();
    const { estimateWithdraw } = useCCDContract(ccdContext.account, !!ccdContext.account);
    const [error, setError] = useState<string>();
    const microCcdFee = useAsyncMemo(
        async () => {
            if (microCcdPerEnergy === undefined) {
                return undefined;
            }

            let energy: bigint;
            try {
                const estimate = await estimateWithdraw(amount, token, ethAccount);
                energy = estimate?.exact ?? withdrawEnergyDefault;
            } catch {
                energy = withdrawEnergyDefault;
            }

            return microCcdPerEnergy * energy;
        },
        () => setError("Could not get fee estimate"),
        [token, microCcdPerEnergy, hasAllowance]
    );

    const details = useMemo(
        () =>
            microCcdFee !== undefined ? renderEnergyFeeEstimate(microCcdFee, ccdPrice) : error || LINE_DETAILS_FALLBACK,
        [microCcdFee, ccdPrice, error]
    );

    return <TransferOverviewLine title={`Withdraw ${token.ccd_name}`} details={details} />;
};

type ApproveWithdrawLineProps = {
    token: Components.Schemas.TokenMapItem;
};

const ApproveWithdrawLine: FC<ApproveWithdrawLineProps> = ({ token }) => {
    const [error, setError] = useState<string>();
    const ethPrice = useAsyncMemo(async () => getPrice("ETH"), noOp, []) ?? 0;
    const { getDefaultWithdrawEstimate } = useRootManagerContract();
    const fee = useAsyncMemo(
        async () => {
            const g = await getDefaultWithdrawEstimate(token);
            const gas = ensureDefined(g, "Could not estimate gas");
            return parseFloat(gas);
        },
        () => setError("Could not estimate gas"),
        [token]
    );

    const details = useMemo(
        () => (fee !== undefined ? `${renderGasFeeEstimate(fee, ethPrice)}*` : error || LINE_DETAILS_FALLBACK),
        [fee, ethPrice, error]
    );

    return <TransferOverviewLine isEth title={`Approve withdraw ${token.eth_name}`} details={details} />;
};

const WithdrawOverview: NextPage = () => {
    const { ccdContext } = useCCDWallet();
    const { context } = useEthWallet();
    const { prefetch, replace } = useRouter();
    const { data: nextMerkleRoot, isLoading } = useNextMerkleRoot();
    const { amount, token } = useTransactionFlowStore();
    const { status, setInfo, setError } = useTransferOverviewStatusState();
    const [needsAllowance, setNeedsAllowance] = useState<boolean | undefined>();
    const ccdPrice = useAsyncMemo(async () => getPrice("CCD"), noOp, []) ?? 0;
    const microCcdPerEnergy = useAsyncMemo(getEnergyToMicroCcdRate, noOp, []);
    const { add: addSubmitted } = useSubmittedWithdrawalsStore();
    const [pendingSignature, setPendingSignature] = useState(false);

    const {
        withdraw: ccdWithdraw,
        approve: ccdApprove,
        hasApprove,
        estimateApprove,
        estimateWithdraw,
    } = useCCDContract(ccdContext.account, !!ccdContext.account);

    const timeToComplete = useMemo(() => {
        if (!isLoading && !nextMerkleRoot) {
            return "Could not get an estimated processing time";
        }
        if (nextMerkleRoot !== undefined) {
            const nextMerkleRootRelativeTime = moment(nextMerkleRoot * 1000).fromNow();
            return `Withdrawal expected to be ready for approval ${nextMerkleRootRelativeTime}`;
        }

        return "Getting estimated withdrawal processing time";
    }, [nextMerkleRoot, isLoading]);

    useEffect(() => {
        if (token !== undefined) {
            hasApprove({
                index: token.ccd_contract?.index,
                subindex: token.ccd_contract?.subindex,
            }).then((allowance) => setNeedsAllowance(!allowance));
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [token]);

    useEffect(() => {
        if (!amount || !token || !context.account) {
            replace(routes.withdraw.path);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    //
    // Check necessary values are present from transfer page. These will not be available if this is the first page loaded in the browser.
    if (!amount || !token || !context.account) {
        return null;
    }

    const requestWithdrawApproval = async () => {
        try {
            const approvalFee = await estimateApprove(token);

            setPendingSignature(true);
            setInfo("Awaiting allowance approval in Concordium wallet");
            const hash = await ccdApprove(token, approvalFee?.conservative);
            setPendingSignature(false);

            setInfo("Waiting for transaction to finalize");
            const hasApproval = await waitForTransactionFinalization(hash);

            setNeedsAllowance(false);
            return hasApproval;
        } catch {
            // Either the allowance approval was rejected, or a timeout happened while polling for allowance approval finalization
            setPendingSignature(false);
            setError("Allowance appproval rejected");

            return false;
        }
    };

    /**
     * Handles submission of the withdraw transaction.
     */
    const onSubmit = async (): Promise<string | undefined> => {
        if (!context?.account) {
            throw new Error("Expected eth account to be available");
        }

        if (needsAllowance && !(await requestWithdrawApproval())) {
            return undefined;
        }

        let hash: string | undefined;
        try {
            const withdrawFee = await estimateWithdraw(amount, token, context.account);

            setPendingSignature(true);
            setInfo("Awaiting signature in Concordium wallet for withdrawal");
            hash = await ccdWithdraw(amount, token, context.account, withdrawFee?.conservative);
            prefetch(routes.withdraw.tx(hash));
        } catch {
            setError("Transaction was rejected");
        } finally {
            setPendingSignature(false);
        }

        if (hash === undefined) {
            return;
        }
        try {
            setInfo("Waiting for transaction to finalize");
            await waitForTransactionFinalization(hash); // Wait for transaction finalization, as we do in the deposit flow.
            addSubmitted(context.account, hash, amount, token);

            return hash;
        } catch {
            setError("Could not get transaction status for withdrawal");
        }
    };

    return (
        <TransferOverview
            title="Withdrawal overview"
            handleSubmit={onSubmit}
            timeToComplete={timeToComplete}
            status={status}
            pendingWalletSignature={pendingSignature}
        >
            <ApprovalAllowanceLine
                hasAllowance={needsAllowance === false}
                token={token}
                ccdPrice={ccdPrice}
                microCcdPerEnergy={microCcdPerEnergy}
            />
            <br />
            <WithdrawLine
                hasAllowance={needsAllowance === false}
                token={token}
                ccdPrice={ccdPrice}
                microCcdPerEnergy={microCcdPerEnergy}
                ethAccount={context.account}
                amount={amount}
            />
            <br />
            <ApproveWithdrawLine token={token} />
            <Text fontFamily="Roboto" fontSize="9" fontWeight="light" fontColor="DarkGrey" fontLetterSpacing="0">
                *Price is based on history of transactions of similar types and can vary depending on network activity
                at the time of the transaction
            </Text>
            <div style={{ marginTop: 12 }} />
        </TransferOverview>
    );
};

export default WithdrawOverview;
