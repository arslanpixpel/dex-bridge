import { NextPage } from "next";
import TransferProgress from "@components/templates/transfer-progress";
import { useRouter } from "next/router";
import { QueryRouter } from "src/types/config";
import { routes } from "src/constants/routes";
import { useEffect, useState } from "react";
import useRootManagerContract from "src/contracts/use-root-manager";
import { useApprovedWithdrawalsStore } from "src/store/approved-withdraws";
import { useEthMerkleProof, useWatchWithdraw } from "src/api-query/queries";

type Query = {
    tx: string;
};

const WithdrawTransactionStatus: NextPage = () => {
    const {
        query: { tx },
        replace,
        isReady,
    } = useRouter() as QueryRouter<Query>;
    const { data: txData } = useWatchWithdraw(tx !== undefined ? { tx_hash: tx } : undefined, {
        enabled: tx !== undefined,
    });
    const { withdraw } = useRootManagerContract();
    const { addApproved, transactions: approvedTransactions } = useApprovedWithdrawalsStore();
    const [pendingWallet, setPendingWallet] = useState(false);

    const { data: merkleProofData } = useEthMerkleProof(
        { event_id: txData?.concordium_event_id, tx_hash: tx },
        txData?.status !== "processed" // Disable the query when transaction has been processed.
    );

    useEffect(() => {
        if (tx === undefined && isReady) {
            replace(routes.withdraw.path);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const handleApprovalRequest = async (
        setError: (message: string) => void,
        setStatus: (message: string | undefined) => void
    ) => {
        if (merkleProofData?.proof === undefined || merkleProofData?.params === undefined || tx === undefined)
            throw new Error("Dependencies for withdrawal request not available");

        try {
            setPendingWallet(true);
            setStatus("Waiting for approval in Ethereum wallet");
            const approvalTx = await withdraw(merkleProofData.params, merkleProofData.proof);

            setStatus("Waiting for transaction to be confirmed");
            await approvalTx.wait(1);

            setStatus(undefined);
            addApproved(tx, approvalTx.hash);
        } catch {
            setError("Transaction rejected");
        } finally {
            setPendingWallet(false);
        }
    };

    const hasApproved = approvedTransactions[tx ?? ""] !== undefined;
    const canWithdraw = merkleProofData?.proof !== undefined && merkleProofData.params !== undefined && !hasApproved;

    return (
        <TransferProgress
            isWithdraw
            transferStatus={txData?.status}
            canWithdraw={canWithdraw}
            onRequestApproval={handleApprovalRequest}
            disableContinue={pendingWallet}
        />
    );
};

export default WithdrawTransactionStatus;
