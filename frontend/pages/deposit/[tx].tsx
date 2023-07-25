import { NextPage } from "next";
import TransferProgress from "@components/templates/transfer-progress";
import { useWatchDeposit } from "src/api-query/queries";
import { useRouter } from "next/router";
import { QueryRouter } from "src/types/config";
import { routes } from "src/constants/routes";
import { useEffect } from "react";

type Query = {
    tx: string;
};

const DepositTransactionStatus: NextPage = () => {
    const {
        query: { tx },
        isReady,
        replace,
    } = useRouter() as QueryRouter<Query>;
    const { data } = useWatchDeposit(tx !== undefined ? { tx_hash: tx } : undefined, {
        enabled: tx !== undefined,
    });

    useEffect(() => {
        if (tx === undefined && isReady) {
            replace(routes.deposit.path);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [isReady, tx]);

    return <TransferProgress transferStatus={data?.status} />;
};

export default DepositTransactionStatus;
